<template>
  <div class="w-full h-screen p-4 pb-12 pr-12 sm:pr-4 flex flex-col sm:flex-row justify-between">
    <div class="main-container">
      <h2>Control</h2>
      <Toggle class="py-4" />
      <!--Joystick
        :size="100"
        base-color="oklch(88.2% 0.059 254.128)"
        stick-color="oklch(54.6% 0.245 262.881)"
        :throttle="100"
        @start="start"
        @stop="stop"
        @move="move"
      />-->
      <DirectionControl />
    </div>
    <div class="main-container">
      <h2>Charts</h2>
      <Chart
        :data="[
          { x: '0', speed: 50 },
          { x: '1', speed: 55 },
          { x: '2', speed: 80 },
          { x: '3', speed: 40 },
          { x: '4', speed: 30 },
        ]"
        :categories="{speed: { name: 'Speed', color: '#155dfc'}}"
        xLabel="Time"
        yLabel="Speed (cm/s)"
        class="py-4"
      />
      <Chart
        :data="[
          { x: '0', acceleration: 50 },
          { x: '1', acceleration: 55 },
          { x: '2', acceleration: 80 },
          { x: '3', acceleration: 40 },
          { x: '4', acceleration: 30 },
        ]"
        :categories="{acceleration: { name: 'Acceleration', color: '#155dfc'}}"
        xLabel="Time"
        yLabel="Acceleration (cm²/s)"
        class="py-4"
      />
    </div>
  </div>
</template>

<script setup>

</script>