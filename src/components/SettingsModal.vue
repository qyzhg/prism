<template>
  <div v-if="show" class="modal-overlay" @click="$emit('close')">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <h3>设置</h3>
        <button class="close-btn" @click="$emit('close')">
          <svg width="16" height="16" viewBox="0 0 16 16">
            <line x1="2" y1="2" x2="14" y2="14" stroke="currentColor" stroke-width="2"/>
            <line x1="14" y1="2" x2="2" y2="14" stroke="currentColor" stroke-width="2"/>
          </svg>
        </button>
      </div>
      
      <div class="modal-body">
        <div class="settings-section translation-card">
          <div class="card">
            <div class="card-header">
              <div>
                <h4>翻译设置</h4>
              </div>
              <span class="card-badge">
                {{ localConfig.translation.service === 'openai' ? 'OpenAI兼容' : '自定义' }}
              </span>
            </div>

            <div class="card-body">
              <div class="card-grid">
                <div class="setting-item grid-span-2">
                  <label class="setting-label">
                    <span>翻译服务</span>
                    <select v-model="localConfig.translation.service" class="setting-select">
                      <option value="openai">OpenAI兼容模式</option>
<!--                      <option value="google">Google 翻译</option>-->
<!--                      <option value="baidu">百度翻译</option>-->
<!--                      <option value="youdao">有道翻译</option>-->
                    </select>
                  </label>
                </div>

                <div class="setting-item">
                  <label class="setting-label">
                    <span>API Base URL</span>
                    <input 
                      type="text" 
                      v-model="localConfig.translation.base_url"
                      class="setting-input"
                      placeholder="https://api.openai.com/v1"
                    >
                  </label>
                </div>

                <div class="setting-item">
                  <label class="setting-label">
                    <span>API Key</span>
                    <input 
                      type="password" 
                      v-model="localConfig.translation.api_key"
                      class="setting-input"
                      placeholder="输入API密钥"
                    >
                  </label>
                </div>

                <div class="setting-item grid-span-2">
                  <label class="setting-label">
                    <span>模型ID</span>
                    <div class="model-input-group">
                      <input 
                        type="text" 
                        v-model="localConfig.translation.model_id"
                        class="setting-input"
                        placeholder="gpt-5-nano"
                      >
                      <button
                        type="button"
                        class="model-fetch-btn"
                        :disabled="!canFetchTranslationModels || translationModelsLoading"
                        @click="openTranslationModelModal"
                      >
                        获取
                      </button>
                    </div>
                  </label>
                  <p
                    v-if="translationModelsError"
                    class="setting-hint setting-hint-error"
                  >
                    {{ translationModelsError }}
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div class="settings-section token-card">
          <div class="card">
            <div class="card-header">
              <div>
                <h4>Token限制</h4>
              </div>
              <label class="toggle-switch">
                <input 
                  type="checkbox" 
                  v-model="localConfig.token_limits.enable_user_max_tokens"
                  class="switch-input"
                >
                <span class="switch-track">
                  <span class="switch-thumb"></span>
                </span>
                <span class="switch-label">启用自定义</span>
              </label>
            </div>

            <div class="card-body">
              <div 
                class="setting-item"
                v-if="localConfig.token_limits.enable_user_max_tokens"
              >
                <label class="setting-label">
                  <span>最大输出Token</span>
                  <input
                    type="number"
                    min="1000"
                    step="100"
                    class="setting-input"
                    v-model.number="localConfig.token_limits.user_max_tokens"
                    placeholder="4096"
                  >
                </label>
                <p class="setting-hint">
                  动态 max_tokens 至少为 {{ MIN_MAX_TOKENS }}，启用后不会超过此上限
                </p>
              </div>
              <p
                v-else
                class="setting-hint"
              >
                关闭自定义时根据文本长度自动估算；开启后可输入自定义上限（不少于 {{ MIN_MAX_TOKENS }}）
              </p>
            </div>
          </div>
        </div>

        <div class="settings-section ocr-card">
          <div class="card">
            <div class="card-header">
              <div>
                <h4>OCR设置</h4>
              </div>
              <label class="toggle-switch">
                <input 
                  type="checkbox" 
                  v-model="localConfig.ocr.reuse_translation"
                  class="switch-input"
                >
                <span class="switch-track">
                  <span class="switch-thumb"></span>
                </span>
                <span class="switch-label">复用翻译设置</span>
              </label>
            </div>
            
            <div class="card-body">
              <div class="card-grid">
                <template v-if="!localConfig.ocr.reuse_translation">
                  <div class="setting-item">
                    <label class="setting-label">
                      <span>OCR Base URL</span>
                      <input 
                        type="text" 
                        v-model="localConfig.ocr.base_url"
                        class="setting-input"
                        placeholder="https://api.openai.com/v1"
                      >
                    </label>
                  </div>
                  
                  <div class="setting-item">
                    <label class="setting-label">
                      <span>OCR API Key</span>
                      <input 
                        type="password" 
                        v-model="localConfig.ocr.api_key"
                        class="setting-input"
                        placeholder="输入OCR API密钥"
                      >
                    </label>
                  </div>
                </template>

                <div class="setting-item grid-span-2">
                  <label class="setting-label">
                    <span>OCR模型ID</span>
                    <div class="model-input-group">
                      <input 
                        type="text" 
                        v-model="localConfig.ocr.model_id"
                        class="setting-input"
                        placeholder="gpt-4-vision-preview"
                      >
                      <button
                        type="button"
                        class="model-fetch-btn"
                        :disabled="!canFetchOcrModels || ocrModelsLoading"
                        @click="openOcrModelModal"
                      >
                        获取
                      </button>
                    </div>
                  </label>
                  <!-- Removed inline select -->
                  <p
                    v-if="ocrModelsError"
                    class="setting-hint setting-hint-error"
                  >
                    {{ ocrModelsError }}
                  </p>
                  <p
                    v-else-if="localConfig.ocr.reuse_translation"
                    class="setting-hint"
                  >
                    当前复用翻译配置，将使用翻译的Base URL和API Key获取模型
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>
        
        <div class="settings-section network-card">
          <div class="card">
            <div class="card-header">
              <div>
                <h4>网络设置</h4>
              </div>
              <label class="toggle-switch">
                <input
                  type="checkbox"
                  v-model="localConfig.proxy.enabled"
                  class="switch-input"
                >
                <span class="switch-track">
                  <span class="switch-thumb"></span>
                </span>
                <span class="switch-label">使用代理</span>
              </label>
            </div>

            <div class="card-body">
              <div v-if="localConfig.proxy.enabled" class="card-grid">
                <div class="setting-item grid-span-2">
                  <label class="setting-label">
                    <span>代理模式</span>
                    <select v-model="localConfig.proxy.mode" class="setting-select">
                      <option value="system">使用系统代理</option>
                      <option value="https">使用HTTPS代理</option>
                      <option value="http">使用HTTP代理</option>
                      <option value="socks5">使用SOCKS5代理</option>
                    </select>
                  </label>
                </div>

                <div
                  class="setting-item grid-span-2"
                  v-if="localConfig.proxy.mode !== 'system'"
                >
                  <label class="setting-label">
                    <span>代理地址</span>
                    <input
                      type="text"
                      v-model="localConfig.proxy.server"
                      class="setting-input"
                      :placeholder="proxyPlaceholder"
                    >
                  </label>
                  <p class="setting-hint">
                    请输入完整的代理URL，例如 http://127.0.0.1:7890
                  </p>
                </div>
              </div>
              <p v-else class="setting-hint">
                当前未启用代理，默认使用系统直连配置
              </p>
            </div>
          </div>
        </div>
        
        <div class="settings-section hotkey-card">
          <div class="card">
            <div class="card-header">
              <div>
                <h4>快捷键设置</h4>
              </div>
            </div>

            <div class="card-body">
              <div class="card-grid">
                <div class="setting-item">
                  <label class="setting-label">
                    <span>弹出窗口</span>
                    <HotkeyRecorder
                      v-model="localConfig.hotkeys.popup_window"
                      :placeholder="platformHotkeys.popup_window"
                    />
                  </label>
                  <p class="setting-hint">按下快捷键弹出翻译窗口</p>
                </div>

                <div class="setting-item">
                  <label class="setting-label">
                    <span>划词翻译</span>
                    <HotkeyRecorder
                      v-model="localConfig.hotkeys.slide_translation"
                      :placeholder="platformHotkeys.slide_translation"
                    />
                  </label>
                  <p class="setting-hint">按下快捷键启动划词翻译</p>
                </div>

                <div class="setting-item">
                  <label class="setting-label">
                    <span>截图翻译</span>
                    <HotkeyRecorder
                      v-model="localConfig.hotkeys.screenshot_translation"
                      :placeholder="platformHotkeys.screenshot_translation"
                    />
                  </label>
                  <p class="setting-hint">按下快捷键启动截图翻译</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
      
      <div class="modal-footer">
        <button class="btn btn-secondary footer-reset" @click="resetToDefaults">
          恢复默认
        </button>
        <div class="footer-right">
          <button class="btn btn-secondary" @click="$emit('close')">
            取消
          </button>
          <button
            class="btn btn-primary"
            :disabled="isSaving"
            @click="saveSettings"
          >
            {{ isSaving ? '保存中...' : '保存' }}
          </button>
        </div>
      </div>
    </div>
  </div>
    <ModelSelectorModal
      :show="showTranslationModelModal"
      title="选择翻译模型"
      :models="translationModels"
      :loading="translationModelsLoading"
      :error="translationModelsError"
      @close="showTranslationModelModal = false"
      @select="selectTranslationModel"
      @retry="fetchTranslationModels"
    />

    <ModelSelectorModal
      :show="showOcrModelModal"
      title="选择OCR模型"
      :models="ocrModels"
      :loading="ocrModelsLoading"
      :error="ocrModelsError"
      @close="showOcrModelModal = false"
      @select="selectOcrModel"
      @retry="fetchOcrModels"
    />
</template>

<script setup>
import { computed, ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import HotkeyRecorder from './HotkeyRecorder.vue'
import ModelSelectorModal from './ModelSelectorModal.vue'

const props = defineProps({
  show: Boolean,
  config: [Object, Array],
  isSaving: {
    type: Boolean,
    default: false
  },
  saveMessage: {
    type: [String, Object],
    default: ''
  }
})

const emit = defineEmits(['close', 'save'])

const isMacPlatform =
  typeof navigator !== 'undefined' &&
  /mac/i.test((navigator.userAgent || navigator.platform || '').toLowerCase())

const MIN_MAX_TOKENS = 1000

const getDefaultHotkeys = () => {
  if (isMacPlatform) {
    return {
      popup_window: "Option+A",
      slide_translation: "Option+D",
      screenshot_translation: "Option+S"
    }
  }

  return {
    popup_window: "Alt+A",
    slide_translation: "Alt+D",
    screenshot_translation: "Alt+S"
  }
}

// 默认配置 - 与App.vue中的appConfig结构匹配
const defaultConfig = {
  translation: {
    service: "openai",
    base_url: "https://api.openai.com/v1",
    api_key: "",
    model_id: "gpt-5-nano"
  },
  ocr: {
    base_url: "https://api.openai.com/v1",
    api_key: "",
    model_id: "gpt-4-vision-preview",
    reuse_translation: false
  },
  proxy: {
    enabled: false,
    mode: "system",
    server: ""
  },
  hotkeys: getDefaultHotkeys(),
  token_limits: {
    enable_user_max_tokens: false,
    user_max_tokens: 4096
  }
}

const platformHotkeys = defaultConfig.hotkeys

const localConfig = ref(JSON.parse(JSON.stringify(defaultConfig)))
const validationError = ref('')
const translationModels = ref([])
const ocrModels = ref([])
const translationModelsLoading = ref(false)
const ocrModelsLoading = ref(false)
const translationModelsError = ref('')
const ocrModelsError = ref('')
const translationModelSelection = ref('')
const ocrModelSelection = ref('')
const showTranslationModelModal = ref(false)
const showOcrModelModal = ref(false)
const proxyPlaceholder = computed(() => {
  const mode = localConfig.value?.proxy?.mode
  if (mode === 'https') return 'https://127.0.0.1:7890'
  if (mode === 'http') return 'http://127.0.0.1:7890'
  if (mode === 'socks5') return 'socks5://127.0.0.1:7890'
  return 'http://127.0.0.1:7890'
})

const getOcrConfigForFetch = () => {
  if (!localConfig.value) return { base_url: '', api_key: '' }
  if (localConfig.value?.ocr?.reuse_translation) {
    return localConfig.value.translation || { base_url: '', api_key: '' }
  }
  return localConfig.value.ocr || { base_url: '', api_key: '' }
}

const canFetchTranslationModels = computed(() => {
  const translation = localConfig.value?.translation || {}
  return Boolean(translation.base_url?.trim() && translation.api_key?.trim())
})

const canFetchOcrModels = computed(() => {
  const credentials = getOcrConfigForFetch()
  return Boolean(credentials.base_url?.trim() && credentials.api_key?.trim())
})

const normalizeConfig = (config) => {
  if (!config) return null

  if (config.translation && config.ocr && config.hotkeys) {
    return config
  }

  if (
    typeof config === 'object' &&
    config !== null &&
    'value' in config &&
    config.value &&
    typeof config.value === 'object'
  ) {
    return config.value
  }

  return null
}

const mergeWithDefaults = (config = {}) => {
  const base = JSON.parse(JSON.stringify(defaultConfig))
  return {
    ...base,
    ...config,
    translation: {
      ...base.translation,
      ...(config.translation || {})
    },
    ocr: {
      ...base.ocr,
      ...(config.ocr || {})
    },
    proxy: {
      ...base.proxy,
      ...(config.proxy || {})
    },
    hotkeys: {
      ...base.hotkeys,
      ...(config.hotkeys || {})
    },
    token_limits: {
      ...base.token_limits,
      ...(config.token_limits || {})
    }
  }
}

const syncLocalConfig = () => {
  const normalized = normalizeConfig(props.config)
  if (normalized) {
    localConfig.value = mergeWithDefaults(normalized)
  } else {
    localConfig.value = JSON.parse(JSON.stringify(defaultConfig))
  }
}

watch(
  () => props.show,
  (newShow) => {
    if (newShow) {
      syncLocalConfig()
    }
  },
  { immediate: true }
)

watch(
  () => props.config,
  () => {
    if (props.show) {
      syncLocalConfig()
    }
  },
  { deep: true }
)

const resetTranslationModelsState = () => {
  translationModels.value = []
  translationModelSelection.value = ''
  translationModelsError.value = ''
}

const resetOcrModelsState = () => {
  ocrModels.value = []
  ocrModelSelection.value = ''
  ocrModelsError.value = ''
}

watch(
  () => [
    localConfig.value?.translation?.base_url,
    localConfig.value?.translation?.api_key
  ],
  resetTranslationModelsState
)

watch(
  () => [
    localConfig.value?.ocr?.base_url,
    localConfig.value?.ocr?.api_key,
    localConfig.value?.ocr?.reuse_translation,
    localConfig.value?.translation?.base_url,
    localConfig.value?.translation?.api_key
  ],
  resetOcrModelsState
)

const syncTranslationSelection = () => {
  const current = localConfig.value?.translation?.model_id || ''
  if (translationModels.value.some(model => model.id === current)) {
    translationModelSelection.value = current
  } else {
    translationModelSelection.value = ''
  }
}

const syncOcrSelection = () => {
  const current = localConfig.value?.ocr?.model_id || ''
  if (ocrModels.value.some(model => model.id === current)) {
    ocrModelSelection.value = current
  } else {
    ocrModelSelection.value = ''
  }
}

watch(
  () => localConfig.value?.translation?.model_id,
  () => syncTranslationSelection(),
  { immediate: true }
)

watch(translationModels, () => syncTranslationSelection())

watch(
  () => localConfig.value?.ocr?.model_id,
  () => syncOcrSelection(),
  { immediate: true }
)

watch(ocrModels, () => syncOcrSelection())

const normalizeBaseUrl = (url = '') => {
  return (url || '').trim().replace(/\/+$/, '')
}

const extractModelList = (payload) => {
  if (!payload) return []

  let rawList = []
  if (Array.isArray(payload)) {
    rawList = payload
  } else if (Array.isArray(payload.data)) {
    rawList = payload.data
  } else if (Array.isArray(payload.models)) {
    rawList = payload.models
  } else if (
    payload.data &&
    typeof payload.data === 'object' &&
    !Array.isArray(payload.data)
  ) {
    rawList = Object.values(payload.data)
  }

  return rawList
    .map(item => {
      if (typeof item === 'string') {
        return { id: item, label: item }
      }
      if (!item || typeof item !== 'object') {
        return null
      }
      const id = item.id || item.model || item.name || item.slug
      if (!id) {
        return null
      }
      const ownedBy = item.owned_by || item.organization || item.provider
      const label = ownedBy ? `${id} (${ownedBy})` : id
      return { id, label }
    })
    .filter(Boolean)
}

const fetchModels = async (target) => {
  const isTranslation = target === 'translation'
  const state = isTranslation
    ? {
        modelsRef: translationModels,
        loadingRef: translationModelsLoading,
        errorRef: translationModelsError,
        credentials: localConfig.value?.translation
      }
    : {
        modelsRef: ocrModels,
        loadingRef: ocrModelsLoading,
        errorRef: ocrModelsError,
        credentials: getOcrConfigForFetch()
      }

  const credentials = state.credentials || {}
  if (!credentials.base_url?.trim() || !credentials.api_key?.trim()) {
    state.errorRef.value = '请先输入Base URL和API Key'
    return
  }

  state.loadingRef.value = true
  state.errorRef.value = ''

  try {
    const endpoint = `${normalizeBaseUrl(credentials.base_url)}/models`
    const response = await fetch(endpoint, {
      method: 'GET',
      headers: {
        Authorization: `Bearer ${credentials.api_key.trim()}`,
        'Content-Type': 'application/json',
        Accept: 'application/json'
      }
    })

    if (!response.ok) {
      let errorText = ''
      try {
        errorText = await response.text()
      } catch (e) {
        console.warn('无法读取错误响应正文', e)
      }
      throw new Error(errorText || `请求失败: ${response.status}`)
    }

    const payload = await response.json()
    const list = extractModelList(payload)
    if (!list.length) {
      throw new Error('未从接口获取到模型列表')
    }

    state.modelsRef.value = list
  } catch (error) {
    const message =
      (error && error.message) ||
      (typeof error === 'string' ? error : '') ||
      '获取模型列表失败'
    state.errorRef.value = message
  } finally {
    state.loadingRef.value = false
  }
}

const fetchTranslationModels = () => fetchModels('translation')
const fetchOcrModels = () => fetchModels('ocr')

const applyModelSelection = (target, value) => {
  if (!value) return
  if (target === 'translation') {
    localConfig.value.translation.model_id = value
  } else if (target === 'ocr') {
    localConfig.value.ocr.model_id = value
  }
}

const openTranslationModelModal = () => {
  showTranslationModelModal.value = true
  fetchTranslationModels()
}

const openOcrModelModal = () => {
  showOcrModelModal.value = true
  fetchOcrModels()
}

const selectTranslationModel = (modelId) => {
  applyModelSelection('translation', modelId)
}

const selectOcrModel = (modelId) => {
  applyModelSelection('ocr', modelId)
}

const resetToDefaults = () => {
  localConfig.value = JSON.parse(JSON.stringify(defaultConfig))
}

const saveSettings = () => {
  validationError.value = ''
  const payload = JSON.parse(JSON.stringify(localConfig.value || defaultConfig))

  if (!payload.translation?.api_key?.trim()) {
    validationError.value = '请输入翻译API密钥'
    return
  }

  if (
    !payload.ocr?.reuse_translation &&
    !payload.ocr?.api_key?.trim()
  ) {
    validationError.value = '请输入OCR API密钥'
    return
  }

  if (
    payload.proxy?.enabled &&
    payload.proxy?.mode !== 'system' &&
    !payload.proxy?.server?.trim()
  ) {
    validationError.value = '请输入代理地址'
    return
  }

  const tokenLimits = {
    ...defaultConfig.token_limits,
    ...(payload.token_limits || {})
  }
  tokenLimits.user_max_tokens = Number(tokenLimits.user_max_tokens)
  if (!Number.isFinite(tokenLimits.user_max_tokens)) {
    tokenLimits.user_max_tokens = defaultConfig.token_limits.user_max_tokens
  } else {
    tokenLimits.user_max_tokens = Math.floor(tokenLimits.user_max_tokens)
  }
  if (
    tokenLimits.enable_user_max_tokens &&
    tokenLimits.user_max_tokens < MIN_MAX_TOKENS
  ) {
    validationError.value = `自定义最大Token必须不小于${MIN_MAX_TOKENS}`
    return
  }
  tokenLimits.user_max_tokens = Math.max(tokenLimits.user_max_tokens, MIN_MAX_TOKENS)
  payload.token_limits = tokenLimits

  emit('save', payload)
  emit('close')
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background: white;
  border-radius: 12px;
  width: 90%;
  max-width: 600px;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px 16px;
  border-bottom: 1px solid #e5e7eb;
}

.modal-header h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: #1f2937;
}

.close-btn {
  background: none;
  border: none;
  padding: 8px;
  border-radius: 6px;
  cursor: pointer;
  color: #6b7280;
  transition: all 0.2s;
}

.close-btn:hover {
  background: #f3f4f6;
  color: #374151;
}

.modal-body {
  flex: 1;
  overflow-y: auto;
  padding: 16px 24px;
}

.settings-section {
  margin-bottom: 24px;
}

.translation-card .card,
.token-card .card,
.ocr-card .card,
.network-card .card,
.hotkey-card .card {
  background: linear-gradient(135deg, #f8fafc 0%, #ffffff 60%);
  border: 1px solid #ffffff;
  border-radius: 16px;
  box-shadow: 0 15px 25px rgba(15, 23, 42, 0.05);
  overflow: hidden;
}

.card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 18px 20px 12px;
  border-bottom: 1px solid rgba(226, 232, 240, 0.9);
}

.card-header h4 {
  margin: 0;
}

.card-subtitle {
  margin: 4px 0 0;
  font-size: 13px;
  color: #6b7280;
}

.card-badge {
  font-size: 12px;
  font-weight: 600;
  color: #2563eb;
  background: rgba(37, 99, 235, 0.12);
  border-radius: 999px;
  padding: 4px 12px;
}

.card-body {
  padding: 18px 20px 20px;
}

.card-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
  gap: 16px;
}

.card-grid .setting-item {
  margin-bottom: 0;
}

.grid-span-2 {
  grid-column: span 2;
}

.card-grid .setting-label {
  margin-bottom: 6px;
}

.settings-section:last-child {
  margin-bottom: 0;
}

.settings-section h4 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: #1f2937;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
  padding-bottom: 8px;
  border-bottom: 1px solid #e5e7eb;
}

.header-checkbox-label {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  color: #374151;
  cursor: pointer;
}

.toggle-switch {
  display: inline-flex;
  align-items: center;
  gap: 10px;
  cursor: pointer;
  user-select: none;
}

.switch-input {
  display: none;
}

.switch-track {
  width: 40px;
  height: 20px;
  border-radius: 999px;
  background: #d1d5db;
  position: relative;
  transition: background 0.2s ease;
}

.switch-thumb {
  position: absolute;
  top: 2px;
  left: 2px;
  width: 16px;
  height: 16px;
  border-radius: 50%;
  background: #ffffff;
  box-shadow: 0 2px 4px rgba(15, 23, 42, 0.2);
  transition: transform 0.2s ease;
}

.switch-input:checked + .switch-track {
  background: #3b82f6;
}

.switch-input:checked + .switch-track .switch-thumb {
  transform: translateX(20px);
}

.switch-label {
  font-size: 14px;
  color: #1f2937;
}

.setting-item {
  margin-bottom: 16px;
}

.setting-item:last-child {
  margin-bottom: 0;
}

.setting-label {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 14px;
  color: #374151;
  margin-bottom: 4px;
}

.setting-select,
.setting-input {
  padding: 6px 12px;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  font-size: 14px;
  background: white;
  transition: all 0.2s;
  width: 100%;
  box-sizing: border-box;
}

.setting-select:focus,
.setting-input:focus {
  outline: none;
  border-color: #3b82f6;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.setting-input[readonly] {
  background: #f9fafb;
  color: #6b7280;
  cursor: not-allowed;
}

.setting-checkbox {
  width: 16px;
  height: 16px;
  accent-color: #3b82f6;
}

.setting-hint {
  margin: 4px 0 0 0;
  font-size: 12px;
  color: #6b7280;
  line-height: 1.4;
}

.modal-footer {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 16px 24px 20px;
  border-top: 1px solid #e5e7eb;
}

.footer-right {
  margin-left: auto;
  display: flex;
  gap: 12px;
}

.footer-reset {
  flex-shrink: 0;
}

.footer-message {
  font-size: 12px;
  color: #6b7280;
}

.footer-message.error {
  color: #dc2626;
}

.footer-message.success {
  color: #10b981;
}

.btn {
  padding: 8px 16px;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-secondary {
  background: #f3f4f6;
  color: #374151;
}

.btn-secondary:hover {
  background: #e5e7eb;
}

.btn-primary {
  background: #3b82f6;
  color: white;
}

.btn-primary[disabled] {
  opacity: 0.7;
  cursor: not-allowed;
}

.btn-primary:hover {
  background: #2563eb;
}

.model-input-group {
  display: flex;
  gap: 8px;
  width: 100%;
}

.model-input-group .setting-input {
  flex: 1;
}

.model-fetch-btn {
  padding: 6px 12px;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  background: #f9fafb;
  font-size: 13px;
  color: #374151;
  cursor: pointer;
  transition: all 0.2s;
  min-width: 60px;
}

.model-fetch-btn:hover {
  background: #f3f4f6;
}

.model-fetch-btn:disabled {
  cursor: not-allowed;
  opacity: 0.6;
}

.model-select {
  margin-top: 8px;
}

.model-select select {
  width: 100%;
  padding: 6px 12px;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  font-size: 14px;
  background: white;
}

.setting-hint-error {
  color: #dc2626;
}

@media (max-width: 768px) {
  .modal-content {
    width: 95%;
    max-height: 90vh;
  }
  
  .modal-header,
  .modal-body,
  .modal-footer {
    padding-left: 16px;
    padding-right: 16px;
  }
  
  .setting-label {
    flex-direction: column;
    align-items: flex-start;
    gap: 8px;
  }
  
  .setting-select,
  .setting-input {
    width: 100%;
  }
  
  .footer-right {
    width: auto;
  }

  .card-grid {
    grid-template-columns: 1fr;
  }

  .grid-span-2 {
    grid-column: span 1;
  }
}
</style>
