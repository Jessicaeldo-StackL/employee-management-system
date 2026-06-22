<template>
  <header class="bg-white shadow-sm border-b border-slate-200">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-3 flex items-center justify-between">
      <div class="flex items-center space-x-3">
        <div class="w-9 h-9 bg-gradient-to-br from-blue-600 to-blue-700 rounded-lg flex items-center justify-center shadow-sm">
          <span class="text-white font-bold text-sm">EM</span>
        </div>
        <div>
          <h1 class="text-xl font-bold text-slate-900 leading-tight">{{ title }}</h1>
          <p v-if="subtitle" class="text-xs text-slate-500">{{ subtitle }}</p>
        </div>
      </div>
      <div class="flex items-center space-x-2">
        <slot />
      </div>
    </div>
  </header>
</template>

<script setup lang="ts">
defineProps<{
  title: string
  subtitle?: string
}>()
</script>
