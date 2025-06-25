<template>
  <div class="grid-container">
    <div class="grid-item"></div>
    <div class="grid-item" @click="emitMove('MOVE_FORWARD')">
      <Triangle :fill="triangleColor" />
    </div>
    <div class="grid-item"></div>
    <div class="grid-item" @click="emitMove('MOVE_LEFT')">
      <Triangle :fill="triangleColor" class="rotate-270" />
    </div>
    <div class="grid-item">
      <Joystick
        :size="100"
        base-color="oklch(88.2% 0.059 254.128)"
        stick-color="oklch(54.6% 0.245 262.881)"
        :throttle="100"
        @start="start"
        @stop="stop"
        @move="move"
      />
    </div>
    <div class="grid-item" @click="emitMove('MOVE_RIGTH')">
      <Triangle :fill="triangleColor" class="rotate-90" />
    </div>
    <div class="grid-item"></div>
    <div class="grid-item" @click="emitMove('MOVE_BACKWARD')">
      <Triangle :fill="triangleColor" class="rotate-180" />
    </div>
    <div class="grid-item"></div>
  </div>
</template>

<style>
.grid-container {
  display: grid;
  grid-template-columns: repeat(3, 1fr); /* 3 equal columns */
  gap: 10px; /* space between items */
  width: 320px; /* optional fixed size */
  margin: 20px auto; /* center on page */
}

.grid-item {
  width: 100px;
  height: 100px;
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>

<script setup>
import Joystick from 'vue-joystick-component';

const emit = defineEmits(['move']);

const triangleColor = 'oklch(54.6% 0.245 262.881)';

const start = () => { /*console.log('start')*/ }

const stop = () => {
  emitMove('STOP');
}

const move = ({ x, y, direction, distance }) => {
  //console.log('move', { x, y, direction, distance })
  emitMove('MOVE ' + x + ' ' + y);
}

const emitMove = (move) => {
  console.log(move);
  emit('move', move);
}
</script>