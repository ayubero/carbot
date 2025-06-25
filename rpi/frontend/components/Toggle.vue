<template>
  <label class="inline-flex items-center cursor-pointer">
    <input type="checkbox" v-model="isToggled" class="sr-only peer">
    <div class="relative w-11 h-6 bg-red-500 peer-focus:outline-none rounded-full peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-red-600 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-green-500 dark:peer-checked:bg-green-500">
    </div>
    <span class="ms-3 text-sm font-medium text-gray-900 dark:text-gray-300">{{ isToggled ? 'On' : 'Off' }}</span>
  </label>
</template>

<script setup>
const isToggled = ref(false)
</script>