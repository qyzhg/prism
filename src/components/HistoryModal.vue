<template>
  <div v-if="show" class="modal-overlay" @click="$emit('close')">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <h3>翻译历史</h3>
        <button class="close-btn" @click="$emit('close')">
          <svg width="16" height="16" viewBox="0 0 16 16">
            <line x1="2" y1="2" x2="14" y2="14" stroke="currentColor" stroke-width="2"/>
            <line x1="14" y1="2" x2="2" y2="14" stroke="currentColor" stroke-width="2"/>
          </svg>
        </button>
      </div>
      
      <div class="modal-body">
        <div v-if="history.length === 0" class="empty-history">
          <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <path d="M12 2v20M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6"/>
          </svg>
          <p>暂无翻译历史</p>
        </div>
        
        <div v-else class="history-list">
          <div 
            v-for="(item, index) in history" 
            :key="index" 
            class="history-item"
          >
            <div class="history-text">
              <div class="source-text">{{ item.sourceText }}</div>
              <div class="target-text">{{ item.targetText }}</div>
              <div class="history-meta">
                <span class="languages">{{ item.sourceLanguage }} → {{ item.targetLanguage }}</span>
                <span class="time">{{ formatTime(item.timestamp) }}</span>
              </div>
            </div>
            <div class="history-actions">
              <button 
                class="action-btn"
                @click="$emit('copy-history', item.targetText)"
                title="复制翻译结果"
              >
                <svg width="16" height="16" viewBox="0 0 16 16">
                  <rect x="4" y="4" width="8" height="8" fill="none" stroke="currentColor" stroke-width="1.5" rx="1"/>
                  <path d="M6 2h6a2 2 0 0 1 2 2v6" fill="none" stroke="currentColor" stroke-width="1.5"/>
                </svg>
              </button>
              <button 
                class="action-btn"
                @click="$emit('use-history', item)"
                title="使用此翻译"
              >
                <svg width="16" height="16" viewBox="0 0 16 16">
                  <path d="M2 8l4 4 8-8" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                </svg>
              </button>
            </div>
          </div>
        </div>
      </div>
      
      <div class="modal-footer">
        <button 
          class="btn btn-secondary" 
          @click="$emit('clear-history')"
          :disabled="history.length === 0"
        >
          清空历史
        </button>
        <button class="btn btn-primary" @click="$emit('close')">
          关闭
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
defineProps({
  show: Boolean,
  history: Array
})

defineEmits(['close', 'clear-history', 'copy-history', 'use-history'])

const parseTimestamp = (timestamp) => {
  if (!timestamp) return null

  if (timestamp instanceof Date) {
    return isNaN(timestamp.getTime()) ? null : timestamp
  }

  if (typeof timestamp === 'number') {
    const fromNumber = new Date(timestamp)
    return isNaN(fromNumber.getTime()) ? null : fromNumber
  }

  let normalized = String(timestamp).trim()
  if (!normalized) return null

  if (/^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}$/.test(normalized)) {
    normalized = normalized.replace(' ', 'T')
  }

  if (!/[zZ]|[+-]\d{2}:\d{2}$/.test(normalized)) {
    normalized = `${normalized}Z`
  }

  const parsed = new Date(normalized)
  return isNaN(parsed.getTime()) ? null : parsed
}

const formatTime = (timestamp) => {
  const date = parseTimestamp(timestamp)
  if (!date) {
    return '未知时间'
  }

  const diff = Date.now() - date.getTime()

  if (diff < 60000) {
    return '刚刚'
  } else if (diff < 3600000) {
    return `${Math.max(1, Math.floor(diff / 60000))}分钟前`
  } else if (diff < 86400000) {
    return `${Math.max(1, Math.floor(diff / 3600000))}小时前`
  } else {
    return date.toLocaleDateString()
  }
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

.empty-history {
  text-align: center;
  padding: 48px 24px;
  color: #9ca3af;
}

.empty-history svg {
  margin-bottom: 16px;
  opacity: 0.5;
}

.empty-history p {
  margin: 0;
  font-size: 14px;
}

.history-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.history-item {
  background: #f9fafb;
  border: 1px solid #e5e7eb;
  border-radius: 8px;
  padding: 16px;
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  transition: all 0.2s;
}

.history-item:hover {
  background: #f3f4f6;
  border-color: #d1d5db;
}

.history-text {
  flex: 1;
  margin-right: 12px;
}

.source-text {
  font-size: 14px;
  color: #374151;
  margin-bottom: 8px;
  line-height: 1.4;
  word-break: break-word;
}

.target-text {
  font-size: 14px;
  color: #1f2937;
  font-weight: 500;
  margin-bottom: 8px;
  line-height: 1.4;
  word-break: break-word;
}

.history-meta {
  display: flex;
  gap: 12px;
  font-size: 12px;
  color: #6b7280;
}

.languages {
  font-weight: 500;
}

.history-actions {
  display: flex;
  gap: 4px;
  flex-shrink: 0;
}

.action-btn {
  background: white;
  border: 1px solid #d1d5db;
  padding: 6px;
  border-radius: 4px;
  cursor: pointer;
  color: #6b7280;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
}

.action-btn:hover {
  background: #f9fafb;
  border-color: #9ca3af;
  color: #374151;
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 24px 20px;
  border-top: 1px solid #e5e7eb;
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

.btn-secondary:hover:not(:disabled) {
  background: #e5e7eb;
}

.btn-secondary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-primary {
  background: #3b82f6;
  color: white;
}

.btn-primary:hover {
  background: #2563eb;
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
  
  .history-item {
    flex-direction: column;
    gap: 12px;
  }
  
  .history-text {
    margin-right: 0;
  }
  
  .history-actions {
    align-self: flex-end;
  }
}
</style>
