<script setup>
import {computed, nextTick, onMounted, onUnmounted, ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {getCurrentWindow} from "@tauri-apps/api/window";
import {check as checkForAppUpdates} from "@tauri-apps/plugin-updater";
import {confirm} from "@tauri-apps/plugin-dialog";
import {relaunch} from "@tauri-apps/plugin-process";
import {enable as enableAutostart, disable as disableAutostart, isEnabled as isAutostartEnabled} from "@tauri-apps/plugin-autostart";

// 导入子组件
import TitleBar from './components/TitleBar.vue'
import LanguageSelector from './components/LanguageSelector.vue'
import TextInput from './components/TextInput.vue'
import TranslationResult from './components/TranslationResult.vue'
import BottomToolbar from './components/BottomToolbar.vue'
import HistoryModal from './components/HistoryModal.vue'
import SettingsModal from './components/SettingsModal.vue'

// 平台相关默认快捷键
const isMacPlatform =
  typeof navigator !== "undefined" &&
  /mac/i.test((navigator.userAgent || navigator.platform || "").toLowerCase());

const getDefaultHotkeys = () => {
  if (isMacPlatform) {
    return {
      popup_window: "Option+A",
      slide_translation: "Option+D",
      screenshot_translation: "Option+S"
    };
  }

  return {
    popup_window: "Alt+A",
    slide_translation: "Alt+D",
    screenshot_translation: "Alt+S"
  };
};

const MIN_MAX_TOKENS = 1000;

const createDefaultAutostartConfig = () => ({
  enabled: false
});

const createDefaultProxy = () => ({
  enabled: false,
  mode: "system",
  server: ""
});

const createDefaultTokenLimits = () => ({
  enable_user_max_tokens: false,
  user_max_tokens: 4096
});

const mergeProxyDefaults = (proxy) => {
  const base = createDefaultProxy();
  if (!proxy || typeof proxy !== "object") {
    return base;
  }
  return {
    ...base,
    ...proxy,
    mode: proxy.mode || base.mode,
    server: typeof proxy.server === "string" ? proxy.server : base.server
  };
};

const mergeAutostartConfig = (autostart) => {
  const base = createDefaultAutostartConfig();
  if (!autostart || typeof autostart !== "object") {
    return base;
  }
  return {
    ...base,
    ...autostart,
    enabled: Boolean(autostart.enabled)
  };
};

const createDefaultConfig = () => ({
  translation: {
    service: "openai",
    base_url: "https://api.openai.com/v1",
    api_key: "",
    model_id: "gpt-5-nano"
  },
  ocr: {
    base_url: "https://api.openai.com/v1",
    api_key: "",
    model_id: "gpt-4-vision-preview"
  },
  proxy: createDefaultProxy(),
  hotkeys: getDefaultHotkeys(),
  token_limits: createDefaultTokenLimits(),
  autostart: createDefaultAutostartConfig()
});

const mergeConfigWithDefaults = (config = {}) => {
  const defaults = createDefaultConfig();
  return {
    ...defaults,
    ...config,
    translation: {
      ...defaults.translation,
      ...(config.translation || {})
    },
    ocr: {
      ...defaults.ocr,
      ...(config.ocr || {})
    },
    proxy: mergeProxyDefaults({
      ...defaults.proxy,
      ...(config.proxy || {})
    }),
    hotkeys: {
      ...defaults.hotkeys,
      ...(config.hotkeys || {})
    },
    token_limits: {
      ...defaults.token_limits,
      ...(config.token_limits || {})
    },
    autostart: mergeAutostartConfig(config.autostart)
  };
};

const isTauriEnvironment = () =>
  typeof window !== "undefined" && typeof window.__TAURI__ !== "undefined";

// 响应式数据
const inputText = ref("");
const translatedText = ref("");
const isTranslating = ref(false);
const isOcrProcessing = ref(false);
const isPinned = ref(false);
const showHistoryModal = ref(false);
const showSettingsModal = ref(false);
const translationHistory = ref([]);
const supportedLanguages = ref([]);
const selectedFromLang = ref("auto");
const selectedToLang = ref("auto");
const selectedService = ref("openai");

// 设置相关数据
const appConfig = ref(createDefaultConfig());

const tempConfig = ref(createDefaultConfig());
const hasChanges = ref(false);
const isSaving = ref(false);
const saveMessage = ref("");
const copyMessage = ref(null);
let copyMessageTimer = null;

const ensureAutostartSection = (config) => {
  if (!config) {
    return createDefaultAutostartConfig();
  }
  if (!config.autostart || typeof config.autostart !== "object") {
    config.autostart = createDefaultAutostartConfig();
  } else {
    config.autostart = mergeAutostartConfig(config.autostart);
  }
  return config.autostart;
};

const updateLocalAutostartState = (enabled) => {
  if (appConfig.value) {
    ensureAutostartSection(appConfig.value).enabled = enabled;
  }
  if (tempConfig.value) {
    ensureAutostartSection(tempConfig.value).enabled = enabled;
  }
};

const syncAutostartStateFromSystem = async () => {
  if (!isTauriEnvironment()) {
    return null;
  }
  try {
    const enabled = await isAutostartEnabled();
    updateLocalAutostartState(enabled);
    return enabled;
  } catch (error) {
    console.warn("读取开机自启状态失败:", error);
    return null;
  }
};

const applyAutostartPreference = async (shouldEnable) => {
  if (!isTauriEnvironment()) {
    throw new Error("当前环境不支持开机自启设置");
  }
  if (shouldEnable) {
    await enableAutostart();
  } else {
    await disableAutostart();
  }
};

const parseErrorMessage = (error) => {
  if (!error) return "未知错误";
  if (typeof error === "string") return error;
  if (error.message) return error.message;
  try {
    return JSON.stringify(error);
  } catch (_) {
    return String(error);
  }
};

let hasAutoCheckedUpdates = false;

const autoCheckForAppUpdates = async () => {
  if (hasAutoCheckedUpdates || import.meta.env.DEV) {
    return;
  }
  hasAutoCheckedUpdates = true;
  try {
    const update = await checkForAppUpdates();
    if (update) {
      const shouldInstall = await confirm(
        `检测到新版本 v${update.version}，是否立即下载并安装？`,
        {
          title: "发现新版本",
          type: "info",
          okLabel: "立即更新",
          cancelLabel: "稍后"
        }
      );

      if (shouldInstall) {
        await update.downloadAndInstall();
        await relaunch();
      } else {
        try {
          await update.close();
        } catch (error) {
          console.warn("关闭更新资源失败:", error);
        }
      }
    }
  } catch (error) {
    console.error("自动更新检查失败:", error);
  }
};

// 输入框引用
const inputTextarea = ref(null);

// 检测文本是否包含汉字
const containsChinese = (text) => {
  return /[\u4e00-\u9fa5]/.test(text);
};

const normalizeLanguageCode = (lang) => (lang || "").toLowerCase();

const isChineseLanguage = (lang) => {
  const normalized = normalizeLanguageCode(lang);
  return [
    "zh",
    "zh-cn",
    "zh_cn",
    "zh-hans",
    "zh_hans",
    "zh-tw",
    "zh_tw",
    "zh-hant",
    "zh_hant"
  ].includes(normalized);
};

const resolveTargetLanguage = (hasChineseInput) => {
  const preferredTarget = selectedToLang.value || "auto";
  const normalizedTarget = normalizeLanguageCode(preferredTarget);
  const targetIsAuto = normalizedTarget === "auto";
  const targetIsChinese = isChineseLanguage(preferredTarget);
  const shouldAutoResolve = targetIsAuto || targetIsChinese;

  if (selectedFromLang.value === "auto") {
    if (hasChineseInput && shouldAutoResolve) {
      return "en";
    }
    if (!hasChineseInput && shouldAutoResolve) {
      return "zh-CN";
    }
    return preferredTarget;
  }

  if (targetIsAuto) {
    return hasChineseInput ? "en" : "zh-CN";
  }

  return preferredTarget;
};

const resolveFromLanguage = (hasChineseInput) => {
  const preferredSource = selectedFromLang.value || "auto";
  if (preferredSource !== "auto") {
    return preferredSource;
  }
  return hasChineseInput ? "zh-CN" : "auto";
};

// 预处理历史记录数据
const processedHistory = computed(() => {
  if (!Array.isArray(translationHistory.value)) return [];
  
  return translationHistory.value
    .filter(record => record && typeof record === 'object')
    .map(record => {
      // 确保record对象存在且具有所有必需的属性
      const safeRecord = record || {};
      return {
        id: safeRecord.id || Math.random().toString(36).substr(2, 9),
        sourceText: safeRecord.original_text || '未知原文',
        targetText: safeRecord.translated_text || '未知译文',
        sourceLanguage: safeRecord.from_language || '未知语言',
        targetLanguage: safeRecord.to_language || '未知语言',
        timestamp: safeRecord.created_at || new Date().toISOString(),
        service: safeRecord.service || '未知服务'
      };
    })
    .filter(record => record && typeof record === 'object'); // 再次过滤确保结果有效
});

// 自动调整输入框高度
const autoResize = () => {
  nextTick(() => {
    if (inputTextarea.value) {
      inputTextarea.value.style.height = 'auto';
      inputTextarea.value.style.height = inputTextarea.value.scrollHeight + 'px';
    }
  });
};

// 翻译功能
const translate = async () => {
  if (!inputText.value.trim()) return;
  
  try {
    isTranslating.value = true;
    
    // 语言自动检测
    const hasChinese = containsChinese(inputText.value);
    const fromLang = resolveFromLanguage(hasChinese);
    const targetLang = resolveTargetLanguage(hasChinese);
    
    const result = await invoke("translate_text", {
      text: inputText.value,
      fromLanguage: fromLang,
      toLanguage: targetLang,
      service: selectedService.value
    });
    
    translatedText.value = result.translated_text;
    
    // 保存翻译记录到历史记录
    try {
      await invoke("save_translation", {
        originalText: inputText.value,
        translatedText: result.translated_text,
        fromLanguage: fromLang,
        toLanguage: targetLang,
        service: result.service || selectedService.value
      });
    } catch (saveError) {
      console.error("保存翻译记录失败:", saveError);
      // 不影响翻译功能，只记录错误
    }
    
  } catch (error) {
    console.error("翻译失败:", error);
    translatedText.value = `翻译失败: ${error}`;
  } finally {
    isTranslating.value = false;
  }
};

// 清空输入
const clearInput = () => {
  inputText.value = "";
  translatedText.value = "";
  autoResize();
};

const showCopyResultMessage = (text, type = 'success') => {
  copyMessage.value = { text, type };
  if (copyMessageTimer) {
    clearTimeout(copyMessageTimer);
  }
  copyMessageTimer = setTimeout(() => {
    copyMessage.value = null;
    copyMessageTimer = null;
  }, 2000);
};

const extractWords = (text) => {
  if (!text) return [];
  return (text.match(/[A-Za-z0-9]+/g) || []).filter(Boolean);
};

const toSnakeCase = (text) => {
  const words = extractWords(text);
  if (!words.length) return '';
  return words.map((word) => word.toLowerCase()).join('_');
};

const toCamelCase = (text) => {
  const words = extractWords(text);
  if (!words.length) return '';
  const [first, ...rest] = words;
  return [
    first.toLowerCase(),
    ...rest.map((word) => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
  ].join('');
};

// 复制结果
const copyResult = async () => {
  if (!translatedText.value) return;
  
  try {
    await navigator.clipboard.writeText(translatedText.value);
    showCopyResultMessage('复制成功');
  } catch (error) {
    console.error("复制失败:", error);
    showCopyResultMessage('复制失败', 'error');
  }
};

const copySnakeCaseResult = async () => {
  if (!translatedText.value) return;
  const snakeText = toSnakeCase(translatedText.value);
  if (!snakeText) {
    showCopyResultMessage('无法生成蛇形命名', 'error');
    return;
  }

  try {
    await navigator.clipboard.writeText(snakeText);
    showCopyResultMessage('复制蛇形成功');
  } catch (error) {
    console.error('复制蛇形失败:', error);
    showCopyResultMessage('复制蛇形失败', 'error');
  }
};

const copyCamelCaseResult = async () => {
  if (!translatedText.value) return;
  const camelText = toCamelCase(translatedText.value);
  if (!camelText) {
    showCopyResultMessage('无法生成驼峰命名', 'error');
    return;
  }

  try {
    await navigator.clipboard.writeText(camelText);
    showCopyResultMessage('复制驼峰成功');
  } catch (error) {
    console.error('复制驼峰失败:', error);
    showCopyResultMessage('复制驼峰失败', 'error');
  }
};

// 切换置顶
const togglePin = async () => {
  try {
    const window = getCurrentWindow();
    await window.setAlwaysOnTop(!isPinned.value);
    isPinned.value = !isPinned.value;
  } catch (error) {
    console.error("设置置顶失败:", error);
  }
};

const hideToTray = async () => {
  try {
    await getCurrentWindow().hide();
  } catch (error) {
    console.error("隐藏窗口失败:", error);
  }
};

// 区域截图功能
const areaScreenshot = async () => {
  try {
    clearInput();
    await invoke("start_area_selection");
  } catch (error) {
    console.error("启动区域选择失败:", error);
    alert(`启动区域选择失败: ${error}`);
  }
};

// 显示历史记录
const showHistory = async () => {
  try {
    const history = await invoke("get_translation_history", { limit: 50, offset: 0 });
    // 确保历史记录数据有效
    translationHistory.value = Array.isArray(history) ? history.filter(record => record && typeof record === 'object') : [];
    showHistoryModal.value = true;
  } catch (error) {
    console.error("获取历史记录失败:", error);
    translationHistory.value = [];
    showHistoryModal.value = true;
  }
};

// 加载支持的语言
const loadSupportedLanguages = async () => {
  try {
    supportedLanguages.value = await invoke("get_supported_languages");
  } catch (error) {
    console.error("获取支持的语言失败:", error);
  }
};

// 监听输入变化
const onInput = () => {
  autoResize();
  // 可以添加自动翻译功能
};

// 交换语言
const swapLanguages = () => {
  if (selectedFromLang.value === "auto") return;
  
  // 如果有翻译结果，交换原文和译文
  const currentInput = inputText.value;
  const currentResult = translatedText.value;
  inputText.value = currentResult || "";
  translatedText.value = currentInput || "";
  autoResize();
};

// 粘贴功能
const pasteText = async () => {
  try {
    const text = await navigator.clipboard.readText();
    inputText.value += text;
    autoResize();
  } catch (error) {
    console.error("粘贴失败:", error);
  }
};

// 设置相关函数
const openSettings = () => {
  // 深拷贝配置文件
  tempConfig.value = JSON.parse(JSON.stringify(appConfig.value));
  showSettingsModal.value = true
}

const closeSettings = () => {
    showSettingsModal.value = false;
};

const showSettings = () => {
    openSettings();
};

const loadSettings = async () => {
  try {
    const config = await invoke('get_app_config')
    const normalizedConfig = mergeConfigWithDefaults(config)
    appConfig.value = normalizedConfig
    tempConfig.value = JSON.parse(JSON.stringify(normalizedConfig))

    // 从配置中初始化 useTranslationForOcr
    if (normalizedConfig.ocr && typeof normalizedConfig.ocr.reuse_translation !== 'undefined') {
        useTranslationForOcr.value = normalizedConfig.ocr.reuse_translation
    }

    await syncAutostartStateFromSystem()
  } catch (error) {
    console.error('加载设置失败:', error)
    alert('加载配置失败: ' + error);
    // 仅在首次运行且配置不存在时自动打开设置页面
    if (error.message && error.message.includes("应用配置不存在")) {
      console.log('首次运行，自动打开设置页面')
      showSettingsModal.value = true
    }
  }
}

// ESC键关闭支持
const handleKeydown = (event) => {
  if (event.key === 'Escape' && showSettingsModal.value) {
    closeSettings()
  }
  if (event.ctrlKey && event.key === 'Enter') {
    translate()
  }
}

const saveSettings = async (newConfig) => {
  // 如果没有传入newConfig，则使用tempConfig (兼容旧调用方式)
  const configToProcess = mergeConfigWithDefaults(newConfig || tempConfig.value);
  configToProcess.proxy = mergeProxyDefaults(configToProcess.proxy)
  configToProcess.autostart = mergeAutostartConfig(configToProcess.autostart)

  if (!configToProcess?.translation?.api_key?.trim()) {
    saveMessage.value = { text: '请输入翻译API密钥', type: 'error' }
    setTimeout(() => saveMessage.value = '', 3000)
    return
  }
  
  // 检查OCR配置
  const reuseTranslation = configToProcess.ocr.reuse_translation;
  if (!reuseTranslation && !configToProcess?.ocr?.api_key?.trim()) {
    saveMessage.value = { text: '请输入OCR API密钥', type: 'error' }
    setTimeout(() => saveMessage.value = '', 3000)
    return
  }

  if (
    configToProcess?.proxy?.enabled &&
    configToProcess?.proxy?.mode !== 'system' &&
    !configToProcess?.proxy?.server?.trim()
  ) {
    saveMessage.value = { text: '请输入代理地址', type: 'error' }
    setTimeout(() => saveMessage.value = '', 3000)
    return
  }

  const tokenLimits = {
    ...createDefaultTokenLimits(),
    ...(configToProcess.token_limits || {})
  }
  tokenLimits.user_max_tokens = Number(tokenLimits.user_max_tokens)
  if (!Number.isFinite(tokenLimits.user_max_tokens)) {
    tokenLimits.user_max_tokens = createDefaultTokenLimits().user_max_tokens
  } else {
    tokenLimits.user_max_tokens = Math.floor(tokenLimits.user_max_tokens)
  }
  if (
    tokenLimits.enable_user_max_tokens &&
    tokenLimits.user_max_tokens < MIN_MAX_TOKENS
  ) {
    saveMessage.value = { text: `自定义最大Token必须不小于${MIN_MAX_TOKENS}`, type: 'error' }
    setTimeout(() => saveMessage.value = '', 3000)
    return
  }
  tokenLimits.user_max_tokens = Math.max(tokenLimits.user_max_tokens, MIN_MAX_TOKENS)
  configToProcess.token_limits = tokenLimits

  const previousAutostartEnabled = Boolean(appConfig.value?.autostart?.enabled)
  let desiredAutostartEnabled = Boolean(configToProcess.autostart?.enabled)
  const userRequestedAutostartChange = desiredAutostartEnabled !== previousAutostartEnabled
  let autostartChangeSuccess = true
  let autostartErrorMessage = ''
  
  isSaving.value = true
  try {
    if (userRequestedAutostartChange) {
      try {
        await applyAutostartPreference(desiredAutostartEnabled)
      } catch (error) {
        autostartChangeSuccess = false
        autostartErrorMessage = parseErrorMessage(error)
        desiredAutostartEnabled = previousAutostartEnabled
      }
    }

    configToProcess.autostart = mergeAutostartConfig({
      enabled: desiredAutostartEnabled
    })

    // 准备要保存的配置
    const configToSave = JSON.parse(JSON.stringify(configToProcess))
    configToSave.proxy = mergeProxyDefaults(configToSave.proxy)
    
    await invoke('save_app_config', { config: configToSave })
    
    // 更新本地状态
    appConfig.value = JSON.parse(JSON.stringify(configToSave))
    tempConfig.value = JSON.parse(JSON.stringify(configToSave))
    useTranslationForOcr.value = configToSave.ocr.reuse_translation
    
    // 重新应用这些快捷方式的功能
    try {
      await invoke('reload_shortcuts')
    } catch (error) {
      console.error('重新加载快捷键失败:', error)
    }
    
    hasChanges.value = false
    const finalMessage = !autostartChangeSuccess && userRequestedAutostartChange
      ? { text: `设置已保存，但开机自启设置更新失败：${autostartErrorMessage || '未知错误'}`, type: 'error' }
      : { text: '设置已保存', type: 'success' }
    saveMessage.value = finalMessage
    setTimeout(() => saveMessage.value = '', 3000)
  } catch (error) {
    console.error('保存设置详细错误:', error);
    // 尝试获取更详细的错误信息
    let errorMsg = error;
    if (typeof error === 'object') {
      errorMsg = JSON.stringify(error);
    }
    alert('保存设置失败: ' + errorMsg);
    saveMessage.value = { text: '保存失败: ' + errorMsg, type: 'error' }
    setTimeout(() => saveMessage.value = '', 3000)
  } finally {
    isSaving.value = false
  }
}

// OCR复用翻译配置
const useTranslationForOcr = ref(true)

const onTranslationServiceChange = () => {
  // 确保tempConfig.value.translation存在
  if (!tempConfig.value || !tempConfig.value.translation) {
    return
  }
  
  // 根据服务更新默认配置
  const serviceDefaults = {
    openai: {
      base_url: 'https://api.openai.com/v1',
      model_id: 'gpt-5-nano'
    },
    // google: {
    //   base_url: 'https://translation.googleapis.com/language/translate/v2',
    //   model_id: ''
    // },
    // baidu: {
    //   base_url: 'https://fanyi-api.baidu.com/api/trans/vip/translate',
    //   model_id: ''
    // }
  }
  
  const service = tempConfig.value.translation.service || 'openai'
  const defaults = serviceDefaults[service]
  if (defaults) {
    const currentServiceDefaults = serviceDefaults[appConfig.value?.translation?.service] || {}
    if (!tempConfig.value.translation.base_url || tempConfig.value.translation.base_url === currentServiceDefaults.base_url) {
      tempConfig.value.translation.base_url = defaults.base_url
    }
    if (!tempConfig.value.translation.model_id || tempConfig.value.translation.model_id === currentServiceDefaults.model_id) {
      tempConfig.value.translation.model_id = defaults.model_id
    }
  }
  
  checkChanges()
}

// 检测配置变化
const checkChanges = () => {
  hasChanges.value = JSON.stringify(appConfig.value) !== JSON.stringify(tempConfig.value)
}

// 历史记录相关函数
const clearHistory = async () => {
  try {
    await invoke("clear_history");
    translationHistory.value = [];
  } catch (error) {
    console.error("清空历史记录失败:", error);
    alert(`清空历史记录失败: ${error}`)
  }
};

const copyHistoryText = async (text) => {
  try {
    await navigator.clipboard.writeText(text);
  } catch (error) {
    console.error("复制失败:", error);
  }
};

const useHistoryItem = (item) => {
  inputText.value = item.sourceText;
  translatedText.value = item.targetText;
  showHistoryModal.value = false;
  autoResize();
};

// 组件挂载时初始化
onMounted(async () => {
  await loadSupportedLanguages();
  await loadSettings();
  if (!import.meta.env.DEV) {
    autoCheckForAppUpdates();
  }
  autoResize();
  
  // 初始化表头和最下面的
  try {
    const window = getCurrentWindow();
    isPinned.value = await window.isAlwaysOnTop();
  } catch (error) {
    console.error("获取置顶状态失败:", error);
  }
  
  // 添加键盘事件监听器
  document.addEventListener('keydown', handleKeydown);
  
  // 监听OCR结果事件
  await getCurrentWindow().listen('ocr-result', (event) => {
    console.log('收到ocr-result事件:', event.payload);
    const ocrText = event.payload;
    console.log('OCR文本:', ocrText, '长度:', ocrText ? ocrText.length : 'null');
    
    // 改进条件检查，确保所有OCR结果都能被处理
    if (ocrText !== null && ocrText !== undefined) {
      if (ocrText.startsWith("Error:")) {
        console.log('显示OCR错误:', ocrText);
        inputText.value = ocrText; // Show error
        } else {
          const trimmedText = ocrText.trim();
          console.log('设置OCR结果:', trimmedText);
          // 直接更新inputText的值，TextInput组件会通过watch监听到变化
          inputText.value = trimmedText;
          console.log('inputText.value已更新为:', inputText.value);
          if (trimmedText.length > 0) {
            translate();
          }
        }
      autoResize();
      isOcrProcessing.value = false; // Stop OCR loading
    } else {
      console.log('OCR文本为null或undefined，忽略');
    }
  });
// 监听ocr识别结果
  await getCurrentWindow().listen('ocr-pending', () => {
    clearInput();
    isOcrProcessing.value = true;
  });

  await getCurrentWindow().listen('prefill-text', (event) => {
    clearInput();
    const incomingText = typeof event.payload === 'string' ? event.payload : '';
    const hasText = incomingText.trim().length > 0;
    inputText.value = incomingText || '';
    autoResize();
    if (hasText) {
      translate();
    }
  });
});

// 组件卸载时清理
onUnmounted(() => {
  // 移除键盘事件监听器
  document.removeEventListener('keydown', handleKeydown);
  if (copyMessageTimer) {
    clearTimeout(copyMessageTimer);
    copyMessageTimer = null;
  }
});
</script>

<template>
  <div class="app-container">
    <!-- 顶部标题栏 -->
    <TitleBar 
      @close="hideToTray"
    />

    <!-- 主内容区 -->
    <div class="main-content">
      <!-- 语言选择器 -->
      <LanguageSelector
        :source-language="selectedFromLang"
        :target-language="selectedToLang"
        :supported-languages="supportedLanguages"
        :selected-service="selectedService"
        @update:source-language="selectedFromLang = $event"
        @update:target-language="selectedToLang = $event"
        @update:selected-service="selectedService = $event"
        @swap-languages="swapLanguages"
      />

      <!-- 输入区域 -->
      <TextInput
        v-model="inputText"
        :is-translating="isTranslating"
        :is-ocr-processing="isOcrProcessing"
        @translate="translate"
        @paste="pasteText"
        @clear="clearInput"
        @keydown="handleKeydown"
      />

      <!-- 结果区域 -->
      <TranslationResult
        :translated-text="translatedText"
        :is-translating="isTranslating"
        :copy-message="copyMessage"
        @copy="copyResult"
        @copy-snake="copySnakeCaseResult"
        @copy-camel="copyCamelCaseResult"
      />
    </div>

    <!-- 底部工具栏 -->
    <BottomToolbar
      :is-pinned="isPinned"
      @toggle-pin="togglePin"
      @show-history="showHistory"
      @show-settings="showSettings"
      @start-ocr="areaScreenshot"
    />

    <!-- 历史记录模态框 -->
    <HistoryModal
      :show="showHistoryModal"
      :history="processedHistory"
      @close="showHistoryModal = false"
      @clear-history="clearHistory"
      @copy-history="copyHistoryText"
      @use-history="useHistoryItem"
    />

    <!-- 设置模态框 -->
    <SettingsModal
      :show="showSettingsModal"
      :config="tempConfig"
      :use-translation-for-ocr="useTranslationForOcr"
      :has-changes="hasChanges"
      :is-saving="isSaving"
      :save-message="saveMessage"
      @close="closeSettings"
      @save="saveSettings($event)"
      @update:use-translation-for-ocr="useTranslationForOcr = $event"
      @update:config="tempConfig = $event"
      @check-changes="checkChanges"
      @on-translation-service-change="onTranslationServiceChange"
    />
  </div>
</template>

<style>
/* 仅隐藏窗口滚动条，内部区域保持可滚动 */
html,
body {
  overflow: hidden;
  scrollbar-width: none; /* Firefox */
  -ms-overflow-style: none; /* IE and Edge */
}

html::-webkit-scrollbar,
body::-webkit-scrollbar {
  display: none;
}
</style>

<style scoped>
.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: #f5f5f5;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  border-radius: 12px;
  overflow: hidden;
}

/* 主内容区 */
.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 12px;
  overflow: hidden; /* Hide scrollbar */
  /* Space for bottom toolbar */
  padding: 12px 12px 60px;
}

/* 通用样式 */
.spacer {
  flex: 1;
}
</style>
