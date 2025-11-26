<template>
  <div class="title-bar" data-tauri-drag-region>
    <div class="title">Prism</div>
    <div class="window-controls" data-tauri-drag-region="false">
      <button class="control-btn close" data-tauri-drag-region="false" @click="$emit('close')">
        <svg width="12" height="12" viewBox="0 0 12 12">
          <line x1="2" y1="2" x2="10" y2="10" stroke="currentColor" stroke-width="1.5"/>
          <line x1="10" y1="2" x2="2" y2="10" stroke="currentColor" stroke-width="1.5"/>
        </svg>
      </button>
    </div>
  </div>
</template>

<script setup>
defineEmits(['close'])
</script>

<style scoped>
.title-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 16px;
  background: white;
  color: #1f2937;
  font-weight: 500;
  user-select: none;
  -webkit-app-region: drag;
}

.window-controls {
  display: flex;
  gap: 8px;
  -webkit-app-region: no-drag;
}

.control-btn {
  width: 24px;
  height: 24px;
  border: none;
  border-radius: 4px;
  background: #f3f4f6;
  color: #6b7280;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
}

.control-btn:hover {
  background: #e5e7eb;
}

.control-btn.close:hover {
  background: #ff5f56;
  color: white;
}
</style>
