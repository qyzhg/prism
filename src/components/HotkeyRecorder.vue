<template>
  <div class="hotkey-recorder">
    <div
      class="hotkey-display"
      :class="{
        recording: isRecording,
        empty: !modelValue
      }"
      tabindex="0"
      role="button"
      @click="startRecording"
      @keydown.enter.prevent="startRecording"
      @keydown.space.prevent="startRecording"
    >
      <span v-if="isRecording" class="recording-text">
        正在记录... 按 Esc 取消
      </span>
      <span v-else-if="modelValue" class="hotkey-value">
        {{ modelValue }}
      </span>
      <span v-else class="placeholder">
        {{ placeholderText }}
      </span>
    </div>
    <button
      type="button"
      class="hotkey-clear"
      :disabled="!modelValue"
      @click.stop="clearBinding"
    >
      清除
    </button>
  </div>
</template>

<script setup>
import { computed, onBeforeUnmount, ref } from 'vue'

const props = defineProps({
  modelValue: {
    type: String,
    default: ''
  },
  placeholder: {
    type: String,
    default: ''
  }
})

const emit = defineEmits(['update:modelValue'])

const isRecording = ref(false)

const placeholderText = computed(() => props.placeholder || '点击录制快捷键')

const startRecording = () => {
  if (isRecording.value) {
    return
  }
  isRecording.value = true
  window.addEventListener('keydown', handleKeydown, true)
  window.addEventListener('keyup', preventDuringRecording, true)
}

const stopRecording = () => {
  if (!isRecording.value) {
    return
  }
  isRecording.value = false
  window.removeEventListener('keydown', handleKeydown, true)
  window.removeEventListener('keyup', preventDuringRecording, true)
}

const preventDuringRecording = (event) => {
  if (isRecording.value) {
    event.preventDefault()
    event.stopPropagation()
  }
}

const clearBinding = () => {
  if (!props.modelValue) {
    return
  }
  emit('update:modelValue', '')
}

const handleKeydown = (event) => {
  event.preventDefault()
  event.stopPropagation()

  if (event.key === 'Escape') {
    stopRecording()
    return
  }

  if (
    event.key === 'Backspace' &&
    !event.shiftKey &&
    !event.ctrlKey &&
    !event.metaKey &&
    !event.altKey
  ) {
    clearBinding()
    stopRecording()
    return
  }

  const mainKey = normalizeKey(event)
  if (!mainKey) {
    return
  }

  const combo = buildCombination(event, mainKey)
  if (combo) {
    emit('update:modelValue', combo)
    stopRecording()
  }
}

const buildCombination = (event, key) => {
  const modifiers = []

  if (event.metaKey) {
    modifiers.push('Command')
  }
  if (event.ctrlKey) {
    modifiers.push('Ctrl')
  }
  if (event.altKey) {
    modifiers.push('Alt')
  }
  if (event.shiftKey) {
    modifiers.push('Shift')
  }

  return [...modifiers, key].join('+')
}

const normalizeKey = (event) => {
  const { key, code } = event

  if (['Shift', 'Control', 'Alt', 'Meta'].includes(key)) {
    return ''
  }

  if (/^Key[A-Z]$/.test(code)) {
    return code.replace('Key', '').toUpperCase()
  }

  if (/^Digit[0-9]$/.test(code)) {
    return code.replace('Digit', '')
  }

  if (/^F[0-9]{1,2}$/i.test(code)) {
    return code.toUpperCase()
  }

  if (/^Numpad[0-9]$/.test(code)) {
    return code.replace('Numpad', 'Numpad')
  }

  const specialCodeMap = {
    Backquote: 'Backquote',
    Minus: 'Minus',
    Equal: 'Equal',
    Backslash: 'Backslash',
    BracketLeft: 'BracketLeft',
    BracketRight: 'BracketRight',
    Semicolon: 'Semicolon',
    Quote: 'Quote',
    Comma: 'Comma',
    Period: 'Period',
    Slash: 'Slash',
    Space: 'Space',
    Tab: 'Tab',
    Enter: 'Enter',
    NumpadEnter: 'NumpadEnter',
    Escape: 'Escape',
    Backspace: 'Backspace',
    Delete: 'Delete',
    Insert: 'Insert',
    Home: 'Home',
    End: 'End',
    PageUp: 'PageUp',
    PageDown: 'PageDown',
    ArrowUp: 'ArrowUp',
    ArrowDown: 'ArrowDown',
    ArrowLeft: 'ArrowLeft',
    ArrowRight: 'ArrowRight',
    CapsLock: 'CapsLock',
    ContextMenu: 'ContextMenu',
    Pause: 'Pause',
    PrintScreen: 'PrintScreen',
    ScrollLock: 'ScrollLock',
    NumLock: 'NumLock'
  }

  if (specialCodeMap[code]) {
    return specialCodeMap[code]
  }

  if (code.startsWith('Numpad')) {
    const suffix = code.slice(6)
    const suffixMap = {
      Add: 'NumpadAdd',
      Subtract: 'NumpadSubtract',
      Multiply: 'NumpadMultiply',
      Divide: 'NumpadDivide',
      Decimal: 'NumpadDecimal'
    }
    if (suffixMap[suffix]) {
      return suffixMap[suffix]
    }
  }

  if (key === ' ') {
    return 'Space'
  }

  if (key && key.length === 1) {
    return key.toUpperCase()
  }

  if (key && key !== 'Unidentified') {
    return key.charAt(0).toUpperCase() + key.slice(1)
  }

  return ''
}

onBeforeUnmount(() => {
  stopRecording()
})
</script>

<style scoped>
.hotkey-recorder {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
}

.hotkey-display {
  flex: 1;
  padding: 6px 12px;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  font-size: 14px;
  min-height: 36px;
  display: flex;
  align-items: center;
  cursor: pointer;
  transition: all 0.2s ease;
  user-select: none;
}

.hotkey-display:hover {
  border-color: #9ca3af;
}

.hotkey-display.recording {
  border-color: #3b82f6;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  color: #2563eb;
}

.hotkey-display.empty {
  color: #9ca3af;
}

.recording-text {
  font-size: 13px;
}

.placeholder {
  color: #9ca3af;
}

.hotkey-clear {
  border: none;
  background: #f3f4f6;
  color: #4b5563;
  padding: 6px 10px;
  border-radius: 6px;
  font-size: 12px;
  cursor: pointer;
  transition: background 0.2s ease;
}

.hotkey-clear:hover {
  background: #e5e7eb;
}

.hotkey-clear:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  background: #f5f5f5;
}
</style>
