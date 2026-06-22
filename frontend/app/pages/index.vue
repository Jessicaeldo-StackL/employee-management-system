<template>
  <div class="min-h-screen bg-gradient-to-br from-slate-50 to-slate-100 flex items-center justify-center">
    <div class="text-center">
      <div class="w-16 h-16 bg-gradient-to-br from-blue-600 to-blue-700 rounded-2xl flex items-center justify-center mx-auto mb-6">
        <span class="text-white font-bold text-3xl">EM</span>
      </div>
      <h1 class="text-4xl font-bold text-slate-900 mb-4">Employee Management System</h1>
      <p class="text-xl text-slate-600 mb-8">Redirecting...</p>
      <svg class="animate-spin h-12 w-12 text-blue-600 mx-auto" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
      </svg>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'
import { onMounted } from 'vue'

const router = useRouter()

onMounted(() => {
  // Check if user is logged in
  const username = localStorage.getItem('username')
  const role = localStorage.getItem('role')

  if (!username) {
    // Not logged in, redirect to login
    router.push('/login')
  } else if (role === 'admin') {
    // Admin user, redirect to admin dashboard
    router.push('/admin')
  } else if (role === 'employee') {
    // Employee user, redirect to employee profile
    router.push('/employee')
  } else {
    // Fallback to login
    router.push('/login')
  }
})
</script>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
