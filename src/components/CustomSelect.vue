<template>
  <div class="custom-select" ref="dropdownRef">
    <button
      type="button"
      class="custom-select-trigger"
      :class="{ disabled }"
      @click="toggleDropdown"
      :disabled="disabled"
    >
      <transition name="fade-text" mode="out-in">
        <span class="selected-label" :key="selectedLabel">
          {{ selectedLabel }}
        </span>
      </transition>
      <svg
        class="chevron"
        :class="{ open: isOpen }"
        width="16"
        height="16"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
      >
        <path d="M6 9l6 6 6-6" />
      </svg>
    </button>
    <transition name="fade-slide">
      <ul v-if="isOpen" class="custom-select-options">
        <li
          v-for="option in options"
          :key="option.value"
          :class="['custom-option', { active: option.value === modelValue }]"
          @click="selectOption(option.value)"
        >
          {{ option.label }}
        </li>
      </ul>
    </transition>
  </div>
</template>

<script setup>
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'

const props = defineProps({
  modelValue: {
    type: [String, Number, null],
    default: ''
  },
  options: {
    type: Array,
    default: () => []
  },
  placeholder: {
    type: String,
    default: '请选择'
  },
  disabled: {
    type: Boolean,
    default: false
  }
})

const emit = defineEmits(['update:modelValue', 'change'])

const isOpen = ref(false)
const dropdownRef = ref(null)

const selectedLabel = computed(() => {
  const current = props.options.find((option) => option.value === props.modelValue)
  return current ? current.label : props.placeholder
})

const toggleDropdown = () => {
  if (props.disabled) return
  isOpen.value = !isOpen.value
}

const selectOption = (value) => {
  emit('update:modelValue', value)
  emit('change', value)
  isOpen.value = false
}

const handleClickOutside = (event) => {
  if (dropdownRef.value && !dropdownRef.value.contains(event.target)) {
    isOpen.value = false
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
})

onBeforeUnmount(() => {
  document.removeEventListener('click', handleClickOutside)
})
</script>

<style scoped>
.custom-select {
  position: relative;
}

.custom-select-trigger {
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 12px;
  border: 1px solid #ddd;
  border-radius: 6px;
  background: white;
  font-size: 14px;
  cursor: pointer;
  transition: border-color 0.2s ease, box-shadow 0.2s ease, color 0.2s ease;
}

.custom-select-trigger:hover:not(.disabled) {
  border-color: #007bff;
}

.custom-select-trigger:focus-visible {
  outline: none;
  border-color: #007bff;
  box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.25);
}

.custom-select-trigger.disabled {
  cursor: not-allowed;
  color: #9ca3af;
  background: #f9fafb;
}

.selected-label {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.chevron {
  transition: transform 0.2s ease;
}

.chevron.open {
  transform: rotate(180deg);
}

.custom-select-options {
  position: absolute;
  z-index: 10;
  width: 100%;
  box-sizing: border-box;
  margin: 6px 0 0;
  padding: 4px;
  background: white;
  border: 1px solid #e5e7eb;
  border-radius: 8px;
  box-shadow: 0 10px 30px rgba(15, 23, 42, 0.1);
  max-height: 220px;
  overflow-y: auto;
}

.custom-option {
  padding: 8px 12px;
  border-radius: 4px;
  font-size: 14px;
  color: #1f2937;
  cursor: pointer;
  transition: background 0.2s ease, color 0.2s ease;
}

.custom-option:hover {
  background: #f3f4f6;
}

.custom-option.active {
  background: #d1d5db;
  color: #1f2937;
}

.fade-text-enter-active,
.fade-text-leave-active {
  transition: opacity 0.2s ease;
}

.fade-text-enter-from,
.fade-text-leave-to {
  opacity: 0;
}

.fade-slide-enter-active,
.fade-slide-leave-active {
  transition: opacity 0.15s ease, transform 0.15s ease;
}

.fade-slide-enter-from,
.fade-slide-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}
</style>

