<template>
  <div class="w-full h-screen p-4 pb-12 pr-12 sm:pr-4 flex flex-col sm:flex-row justify-between">
    <div class="main-container">
      <h2>Control</h2>
      <div class="py-4">
        <p>Backend URL:</p>
        <div class="flex flex-row">
          <InputText v-model:value="apiUrl"/>
          <Button class="ml-4" @click="getUsbDevices">Fetch</Button>
        </div>
      </div>

      <!--<Toggle class="py-4" @toggle="handleOnOff" />-->
      <div class="py-4">
        <p>Serial device:</p>
        <div class="flex flex-row">
          <SelectMenu v-model:value="serialDevice" :options="options"/>
          <Button class="ml-4" @click="connectToSerialPort">
            {{ connected ? 'Disconnect' : 'Connect' }}
          </Button>
        </div>
      </div>
      
      <div class="py-4">
        <p>Speed: {{ speed }}</p>
        <Slider v-model:value="speed" min="100" max="3000" step="100" />
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
      <h2>Camera</h2>
      <div>
        <img class="mt-2 rounded-lg" v-if="imageSrc" :src="imageSrc" alt="Camera Stream" />
        <p v-else>Connecting to camera was not possible.</p>
      </div>
      <h2 class="mt-4">Charts</h2>
      <!--<Chart
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
      />-->
    </div>
  </div>
</template>

<script setup>
const connected = ref(false);
const speed = ref(1.0);
const lastMessage = ref('');
//const appConfig = useAppConfig()
const apiUrl = ref('http://localhost:5000')
const serialDevice = ref('/dev/ttyS0');

// Camera stream
const imageSrc = ref('');
let currentBlobUrl = null;
let socket = null;

// Get serial devices
const options = ref([
  { value: '/dev/ttyS0', text: '/dev/ttyS0' },
  { value: '/dev/ttyUSB0', text: '/dev/ttyUSB0' },
]);
//const apiUrl = config.public.apiBase;
const getUsbDevices = async () => {
  const res = await $fetch(apiUrl.value + '/list', {
    method: 'GET',
  });
  options.value = res.map(item => ({
    value: item.port_name,
    text: item.port_name
  }));
};

const connectToSerialPort = async () => {
  if (connected.value) { // Disconnect
    try {
      const res = await $fetch(apiUrl.value + '/disconnect', {
        method: 'POST',
      });
      console.log(res);
      connected.value = false; // Set connected to false on success
    } catch (error) {
      console.error("Failed to disconnect:", error);
    }
  } else { // Connect
    try {
      const res = await $fetch(apiUrl.value + '/connect', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ port_path: serialDevice.value })
      });
      console.log(res);
      connected.value = true; // Set connected to true on success
    } catch (error) {
      console.error("Failed to connect:", error);
      connected.value = false; // Ensure connected is false on failure
    }
  }
};

const sendMessage = async (message) => {
  const res = await $fetch(apiUrl.value + '/send', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({ message: message })
  });
  console.log(res);
};

const connectWebSocket = () => {
  if (socket) {
    socket.close();
  }

  const wsUrl = apiUrl.value.replace('http://', 'ws://').replace('https://', 'wss://');
  socket = new WebSocket(wsUrl + '/camera_ws');

  socket.binaryType = 'arraybuffer';

  socket.onopen = () => {
    console.log('WebSocket connection established');
  };

  socket.onmessage = (event) => {
    if (currentBlobUrl) {
      URL.revokeObjectURL(currentBlobUrl);
    }
    const arrayBuffer = event.data;
    const blob = new Blob([arrayBuffer], { type: 'image/jpeg' });
    console.log(blob);
    currentBlobUrl = URL.createObjectURL(blob);
    imageSrc.value = currentBlobUrl;
  };

  socket.onerror = (error) => {
    console.error('WebSocket error:', error);
  };

  socket.onclose = () => {
    console.log('WebSocket connection closed. Reconnecting...');
    if (reconnectAttempts < maxReconnectAttempts) {
        reconnectAttempts++;
        setTimeout(connectWebSocket, 1000);
    } else {
        console.error('Max reconnection attempts reached');
    }
  };
};

watch(speed, (speed, prevSpeed) => {
  sendMessage('speed ' + speed)
});

watch(apiUrl, (url) => {
  connectWebSocket();
});

onMounted(() => {
  // Fetch USB devices
  getUsbDevices();
  connectWebSocket();
});

onUnmounted(() => {
  if (socket) {
    socket.close();
  }
})
</script>