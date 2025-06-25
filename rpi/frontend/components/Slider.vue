<template>
  <div class="w-full">
    <input
      type="range"
      @input="$emit('update:value', $event.target.value)"
      :value="value"
      :min="min"
      :max="max"
      :step="step"
      class="w-full h-2.5 shadow-sm border-0 ring-1 ring-inset ring-gray-200 rounded-lg appearance-none cursor-pointer dark:bg-gray-900 dark:ring-gray-950 dark:ring-1 dark:ring-inset"
    />
  </div>
</template>

<style>
/* Thumb Styles */
/* Chrome, Safari, Opera, and Edge Chromium */
input[type='range']::-webkit-slider-thumb {
  -webkit-appearance: none; /* Override default look */
  appearance: none;
  @apply bg-primary w-4 h-4 rounded-full;
}
/* Firefox */
input[type='range']::-moz-range-thumb {
  @apply bg-primary w-4 h-4 rounded-full border-0;
}
</style>

<script setup>
defineProps(['value', 'min', 'max', 'step']);
defineEmits(['update:value']);
</script>