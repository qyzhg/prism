<template>
  <div class="translation-result">
    <div class="result-header">
      <span class="result-label">翻译结果</span>
      <div class="result-actions">
        <button 
          class="action-btn"
          @click="$emit('copy')"
          :disabled="!translatedText"
          title="复制结果"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
            <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
          </svg>
        </button>
      </div>
    </div>
    <div class="result-content">
      <div v-if="isTranslating" class="loading">
        <div class="loading-spinner"></div>
        <span>翻译中...</span>
      </div>
      <div v-else-if="translatedText" class="result-text">
        {{ translatedText }}
      </div>
      <div v-else class="placeholder">
        翻译结果将显示在这里
      </div>
    </div>
  </div>
</template>

<script setup>
defineProps({
  translatedText: {
    type: String,
    default: ''
  },
  isTranslating: {
    type: Boolean,
    default: false
  }
})

defineEmits(['copy'])
</script>

<style scoped>
.translation-result {
  background: white;
  border-radius: 8px;
  padding: 16px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.result-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.result-label {
  font-weight: 600;
  color: #374151;
}

.result-actions {
  display: flex;
  gap: 8px;
}

.action-btn {
  background: none;
  border: none;
  color: #6b7280;
  cursor: pointer;
  padding: 4px;
  border-radius: 4px;
  transition: all 0.2s;
}

.action-btn:hover:not(:disabled) {
  background: #f3f4f6;
  color: #374151;
}

.action-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.result-content {
  height: 180px;
  position: relative;
  overflow: hidden;
}

.loading {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  color: #6b7280;
  font-size: 14px;
}

.loading-spinner {
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

.result-text {
  color: #374151;
  line-height: 1.6;
  white-space: pre-wrap;
  word-wrap: break-word;
  height: 100%;
  overflow-y: auto;
  padding-right: 4px;
}

.placeholder {
  color: #9ca3af;
  font-style: italic;
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  min-height: 120px;
}
</style>
