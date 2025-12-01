<template>
  <div class="text-input">
    <div class="input-header">
      <div class="input-actions">
        <button @click="$emit('paste')" class="action-btn" title="粘贴">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"/>
            <rect x="8" y="2" width="8" height="4" rx="1" ry="1"/>
          </svg>
        </button>
        <button @click="$emit('clear')" class="action-btn" title="清空">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M3 6h18M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2m3 0v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6h14zM10 11v6M14 11v6"/>
          </svg>
        </button>
      </div>
      <button @click="$emit('translate')" class="translate-button" title="翻译">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M5 8l6 6m0 0l6-6m-6 6V3"/>
          <path d="M5 16l6-6m0 0l6 6m-6-6v9"/>
        </svg>
        <span>翻译</span>
      </button>
    </div>
    <div class="input-body">
      <div class="textarea-scroll">
        <div class="textarea-container">
          <textarea
            v-model="inputText"
            @input="handleInput"
            @keydown="handleKeydown"
            placeholder="请输入要翻译的文本..."
            class="input-textarea"
            :disabled="isOcrProcessing"
          ></textarea>
          <!-- OCR Loading Indicator -->
          <div v-if="isOcrProcessing" class="ocr-loading">
            <div class="ocr-loading-spinner"></div>
            <span>正在识别文字...</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue'

const props = defineProps({
  modelValue: {
    type: String,
    default: ''
  },
  isOcrProcessing: {
    type: Boolean,
    default: false
  }
})

const emit = defineEmits(['update:modelValue', 'paste', 'clear', 'translate'])

const inputText = ref(props.modelValue)

const handleInput = () => {
  emit('update:modelValue', inputText.value)
}

const handleKeydown = (event) => {
  if (event.key === 'Enter' && (event.ctrlKey || event.metaKey)) {
    event.preventDefault()
    emit('translate')
  }
}

// 监听props变化
watch(() => props.modelValue, (newVal) => {
  inputText.value = newVal
})
</script>

<style scoped>
.text-input {
  background: white;
  border-radius: 8px;
  border: 1px solid #ddd;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.input-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 12px;
  background: #f8f9fa;
  border-bottom: 1px solid #ddd;
}

.char-count {
  font-size: 12px;
  color: #666;
}

.input-actions {
  display: flex;
  gap: 5px;
}

.action-btn {
  padding: 4px 8px;
  border: none;
  background: transparent;
  cursor: pointer;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background-color 0.2s;
}

.action-btn:hover {
  background: #e9ecef;
}

.input-body {
  position: relative;
  padding: 0 12px 12px;
  display: flex;
  flex-direction: column;
  height: 260px;
}

.textarea-scroll {
  flex: 1;
  overflow: hidden;
}

.textarea-container {
  position: relative;
  height: 100%;
}

.input-textarea {
  width: 100%;
  height: 100%;
  padding: 12px;
  border: none;
  resize: none;
  font-size: 14px;
  line-height: 1.5;
  font-family: inherit;
  overflow-y: auto;
  box-sizing: border-box;
  scrollbar-width: thin;
  -ms-overflow-style: auto;
}

.input-textarea::-webkit-scrollbar {
  width: 6px;
}

.input-textarea::-webkit-scrollbar-track {
  background: transparent;
}

.input-textarea::-webkit-scrollbar-thumb {
  background: rgba(156, 163, 175, 0.7);
  border-radius: 4px;
}

.input-textarea::-webkit-scrollbar-thumb:hover {
  background: rgba(107, 114, 128, 0.85);
}

.input-textarea:focus {
  outline: none;
}

.input-textarea::placeholder {
  color: #999;
}

.translate-button {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 6px 12px;
  background: #3b82f6;
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 12px;
  cursor: pointer;
  transition: all 0.2s;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.translate-button:hover {
  background: #2563eb;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.15);
}

.translate-button:active {
  transform: translateY(1px);
}

.translate-button svg {
  flex-shrink: 0;
}

.translate-button span {
  font-weight: 500;
}

.ocr-loading {
  position: absolute;
  top: 20px;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  align-items: center;
  gap: 8px;
  color: #6b7280;
  font-size: 14px;
  background: rgba(255, 255, 255, 0.9);
  padding: 8px 12px;
  border-radius: 6px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  z-index: 10;
}

.ocr-loading-spinner {
  width: 16px;
  height: 16px;
  border: 2px solid #e5e7eb;
  border-top: 2px solid #3b82f6;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.input-textarea:disabled {
  background-color: #f9fafb;
  color: #6b7280;
  cursor: not-allowed;
}
</style>
