<template>
  <div class="w-full">
    <input
      type="range"
      @input="$emit('update:value', $event.target.value)"
      :value="value"
      :min="min"
      :max="max"
      :step="step"
      class="w-full h-2.5 shadow-xs border-0 ring-1 ring-inset ring-gray-200 rounded-lg appearance-none cursor-pointer"
    />
  </div>
</template>

<script setup>
defineProps(['value', 'min', 'max', 'step']);
defineEmits(['update:value']);
</script>