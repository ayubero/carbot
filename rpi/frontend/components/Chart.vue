<template>
  <AreaChart
    :data="props.data"
    :categories="props.categories"
    :height="300"
    :xFormatter="xFormatter"
    :xLabel="props.xLabel"
    :yLabel="props.yLabel"
  />
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface DataPoint {
  x: string
  [key: string]: any
}

interface Category {
  name: string
  color: string
}

const props = defineProps<{
  data: DataPoint[]
  categories: Record<string, Category>
  xLabel: string
  yLabel: string
}>()

// Optional: Automatically format x-axis based on the `x` in `data`
const xFormatter = (i: number) => props.data[i]?.x
</script>
