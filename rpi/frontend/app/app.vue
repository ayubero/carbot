<template>
  <div class="w-full h-screen p-4 pb-12 pr-12 sm:pr-4 flex flex-col sm:flex-row justify-between">
    <div class="main-container">
      <h2>Control</h2>
      <!--<Toggle class="py-4" @toggle="handleOnOff" />-->
      <div class="py-4">
        <p>Speed (cm/s): {{ speed }}</p>
        <Slider v-model:value="speed" min="0" max="2" step="0.1" />
      </div>
      <DirectionControl @move="sendMessage" :speed="speed" />
      <div class="py-4">
        <p>Console:</p>
        <div class="console">
          {{ lastMessage }}
        </div>
      </div>
      
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
const speed = ref(1.0);
const lastMessage = ref('');

const sendMessage = (message) => {
  lastMessage.value = message;
}

const handleOnOff = (isToggled) => {
  if (isToggled) {
    sendMessage('TURN_ON\n')
  } else {
    sendMessage('TURN_OFF\n')
  }
}
</script>