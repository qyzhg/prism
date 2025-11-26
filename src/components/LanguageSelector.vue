<template>
  <div class="language-selector">
    <div class="lang-group">
      <CustomSelect
        v-model="sourceLang"
        :options="languageOptions"
        placeholder="选择源语言"
      />
    </div>
    <button @click="handleSwap" class="swap-btn" title="交换语言">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M7 16V4m0 0L3 8m4-4l4 4m6 0v12m0 0l4-4m-4 4l-4-4"/>
        </svg>
      </button>
    <div class="lang-group">
      <CustomSelect
        v-model="targetLang"
        :options="languageOptions"
        placeholder="选择目标语言"
      />
    </div>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue'
import CustomSelect from './CustomSelect.vue'

const props = defineProps({
  sourceLanguage: String,
  targetLanguage: String
})

const emit = defineEmits(['update:source-language', 'update:target-language', 'swap-languages'])

const sourceLang = ref(props.sourceLanguage)
const targetLang = ref(props.targetLanguage)

const handleSwap = () => {
  if (sourceLang.value === "auto") return;
  
  const temp = sourceLang.value;
  sourceLang.value = targetLang.value;
  targetLang.value = temp;
  
  emit('update:source-language', sourceLang.value);
  emit('update:target-language', targetLang.value);
  emit('swap-languages');
}

const languages = [
  { code: 'auto', name: '自动检测' },
  { code: 'zh', name: '中文' },
  { code: 'en', name: '英语' },
  { code: 'ja', name: '日语' },
  { code: 'ko', name: '韩语' },
  { code: 'fr', name: '法语' },
  { code: 'de', name: '德语' },
  { code: 'es', name: '西班牙语' },
  { code: 'ru', name: '俄语' },
  { code: 'it', name: '意大利语' },
  { code: 'pt', name: '葡萄牙语' },
  { code: 'ar', name: '阿拉伯语' },
  { code: 'hi', name: '印地语' },
  { code: 'th', name: '泰语' },
  { code: 'vi', name: '越南语' }
]

const languageOptions = languages.map((lang) => ({
  value: lang.code,
  label: lang.name
}))

watch(
  () => props.sourceLanguage,
  (newVal) => {
  sourceLang.value = newVal
  }
)

watch(
  () => props.targetLanguage,
  (newVal) => {
  targetLang.value = newVal
  }
)

watch(
  sourceLang,
  (newVal) => {
    if (newVal !== props.sourceLanguage) {
      emit('update:source-language', newVal)
    }
  }
)

watch(
  targetLang,
  (newVal) => {
    if (newVal !== props.targetLanguage) {
      emit('update:target-language', newVal)
    }
  }
)
</script>

<style scoped>
.language-selector {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 15px;
}

.lang-group {
  flex: 1;
  min-width: 0;
}

.lang-select {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid #ddd;
  border-radius: 6px;
  background: white;
  font-size: 14px;
  cursor: pointer;
}

.lang-select:hover {
  border-color: #007bff;
}

.lang-select:focus {
  outline: none;
  border-color: #007bff;
  box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.25);
}

.swap-btn {
  padding: 8px;
  border: 1px solid #ddd;
  border-radius: 6px;
  background: white;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
  flex-shrink: 0;
  width: 36px;
  height: 36px;
}

.swap-btn:hover {
  background: #f8f9fa;
  border-color: #007bff;
}

.swap-btn:active {
  transform: scale(0.95);
}

</style>
