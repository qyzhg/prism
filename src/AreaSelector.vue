<template>
  <div
    class="area-selector"
    @mousedown="startSelection"
    @mousemove="updateSelection"
    @mouseup="endSelection"
    @contextmenu.prevent="cancelSelection"
  >
    <div
      v-if="isSelecting"
      class="selection-box"
      :style="{
        left: selection.x + 'px',
        top: selection.y + 'px',
        width: selection.width + 'px',
        height: selection.height + 'px',
      }"
    ></div>
    <div class="instructions">
      按住鼠标左键拖动选择区域，松开完成截图<br />
      按 ESC 或右键取消
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

const isSelecting = ref(false);
const startPos = ref({ x: 0, y: 0 });
const selection = ref({ x: 0, y: 0, width: 0, height: 0 });

const startSelection = (e) => {
  isSelecting.value = true;
  startPos.value = { x: e.clientX, y: e.clientY };
  selection.value = {
    x: e.clientX,
    y: e.clientY,
    width: 0,
    height: 0,
  };
};

const updateSelection = (e) => {
  if (!isSelecting.value) return;

  const currentX = e.clientX;
  const currentY = e.clientY;

  const x = Math.min(startPos.value.x, currentX);
  const y = Math.min(startPos.value.y, currentY);
  const width = Math.abs(currentX - startPos.value.x);
  const height = Math.abs(currentY - startPos.value.y);

  selection.value = { x, y, width, height };
};

const endSelection = async () => {
  if (!isSelecting.value) return;
  isSelecting.value = false;

  if (selection.value.width > 0 && selection.value.height > 0) {
    try {
      // Hide window immediately to avoid capturing the overlay itself
      await getCurrentWindow().hide();
      
      // Submit area for OCR
      // Note: We need to convert to integers as expected by the backend command
      await invoke("submit_area_for_ocr", {
        x: Math.round(selection.value.x),
        y: Math.round(selection.value.y),
        width: Math.round(selection.value.width),
        height: Math.round(selection.value.height),
      });
      
      // Close the selector window
      await getCurrentWindow().close();
    } catch (error) {
      console.error("Failed to submit area:", error);
      await getCurrentWindow().close();
    }
  }
};

const cancelSelection = async () => {
  await getCurrentWindow().close();
};

const handleKeydown = async (e) => {
  if (e.key === "Escape") {
    await cancelSelection();
  }
};

onMounted(() => {
  window.addEventListener("keydown", handleKeydown);
});

onUnmounted(() => {
  window.removeEventListener("keydown", handleKeydown);
});
</script>

<style scoped>
.area-selector {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  cursor: crosshair;
  background-color: rgba(0, 0, 0, 0.3); /* Semi-transparent overlay */
  z-index: 9999;
}

.selection-box {
  position: absolute;
  border: 2px solid #007bff;
  background-color: rgba(0, 123, 255, 0.1);
  pointer-events: none; /* Allow clicks to pass through */
}

.instructions {
  position: absolute;
  top: 20px;
  left: 50%;
  transform: translateX(-50%);
  background-color: rgba(0, 0, 0, 0.7);
  color: white;
  padding: 10px 20px;
  border-radius: 5px;
  pointer-events: none;
  text-align: center;
  font-family: sans-serif;
  font-size: 14px;
}
</style>
