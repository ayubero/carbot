<template>
  <input
    type="text"
    @input="$emit('update:value', $event.target.value)"
    :value="value"
    class="w-full h-10 py-2 px-3 pr-2 rounded-md bg-white text-left shadow-sm border-0 ring-1 ring-inset ring-gray-200 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-blue-500 sm:leading-6"
    :placeholder="placeholder"
  />
</template>

<script setup>
defineProps(['value', 'placeholder']);
defineEmits(['update:value']);
</script>
