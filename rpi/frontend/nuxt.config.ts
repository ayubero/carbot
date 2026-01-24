import tailwindcss from "@tailwindcss/vite";

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-05-15',
  devtools: { enabled: true },
  css: ['~/assets/css/main.css'],

  vite: {    
    plugins: [      
      tailwindcss(),
    ],  
  },

  build: {
    transpile: ['vue-joystick-component'],
  },

  runtimeConfig: {
    // Server-side only
    apiBaseServer: 'http://backend:5000',
    
    // Public config available on both server and client
    public: {
      apiBase: process.env.NUXT_PUBLIC_API_BASE || 'http://localhost:5000'
    }
  },

  modules: ['nuxt-charts'],
})