<template>
  <div class="bottom-toolbar">
    <div class="toolbar-left">
      <button 
        class="toolbar-btn"
        @click="$emit('togglePin')"
        :class="{ active: isPinned }"
        title="置顶窗口"
      >
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"></path>
        </svg>
        <span>{{ isPinned ? '取消置顶' : '置顶' }}</span>
      </button>
      
      <button 
        class="toolbar-btn"
        @click="$emit('showHistory')"
        title="查看历史记录"
      >
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="10"></circle>
          <polyline points="12 6 12 12 16 14"></polyline>
        </svg>
        <span>历史记录</span>
      </button>
    </div>
    
    <div class="toolbar-right">
      <button 
        class="toolbar-btn"
        @click="$emit('showSettings')"
        title="设置"
      >
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="3"></circle>
          <path d="M12 1v6m0 6v6m4.22-13.22l4.24 4.24M1.54 9.96l4.24 4.24M20.46 14.04l-4.24 4.24M7.78 7.78L3.54 3.54"></path>
        </svg>
        <span>设置</span>
      </button>
      
      <button 
        class="toolbar-btn ocr-btn"
        @click="$emit('start-ocr')"
        title="OCR截图翻译"
      >
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
          <circle cx="8.5" cy="8.5" r="1.5"></circle>
          <polyline points="21 15 16 10 5 21"></polyline>
        </svg>
        <span>OCR截图</span>
      </button>
    </div>
  </div>
</template>

<script setup>
defineProps({
  isPinned: {
    type: Boolean,
    default: false
  }
})

defineEmits(['togglePin', 'showHistory', 'showSettings', 'start-ocr'])
</script>

<style scoped>
.bottom-toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: #f8fafc;
  border-top: 1px solid #e2e8f0;
  gap: 16px;
}

.toolbar-left,
.toolbar-right {
  display: flex;
  gap: 8px;
}

.toolbar-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  background: white;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  color: #374151;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
}

.toolbar-btn:hover {
  background: #f9fafb;
  border-color: #9ca3af;
}

.toolbar-btn.active {
  background: #3b82f6;
  border-color: #3b82f6;
  color: white;
}

.ocr-btn {
  background: #10b981;
  border-color: #10b981;
  color: white;
}

.ocr-btn:hover {
  background: #059669;
  border-color: #059669;
}

.toolbar-btn svg {
  flex-shrink: 0;
}

@media (max-width: 640px) {
  .bottom-toolbar {
    flex-wrap: wrap;
    gap: 8px;
  }
  
  .toolbar-left,
  .toolbar-right {
    flex: 1;
    min-width: 0;
  }
  
  .toolbar-left {
    justify-content: flex-start;
  }
  
  .toolbar-right {
    justify-content: flex-end;
  }
  
  .toolbar-btn span {
    display: none;
  }
  
  .toolbar-btn {
    padding: 8px;
    justify-content: center;
  }
}
</style>
