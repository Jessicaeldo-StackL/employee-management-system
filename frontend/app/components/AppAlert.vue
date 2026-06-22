<template>
  <div :class="['p-4 rounded-lg border flex items-start space-x-3', styles[type].bg, styles[type].border]">
    <span :class="['text-sm font-medium leading-5', styles[type].text]">
      <span class="font-semibold uppercase tracking-wide text-xs block mb-1">{{ label }}</span>
      <slot />
    </span>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  type: 'error' | 'success' | 'warning' | 'info'
}>()

const label = computed(() => ({
  error: 'Error',
  success: 'Success',
  warning: 'Warning',
  info: 'Info'
}[props.type]))

const styles = {
  error:   { bg: 'bg-red-50',    border: 'border-red-200',   text: 'text-red-800'   },
  success: { bg: 'bg-green-50',  border: 'border-green-200', text: 'text-green-800' },
  warning: { bg: 'bg-yellow-50', border: 'border-yellow-200',text: 'text-yellow-800'},
  info:    { bg: 'bg-blue-50',   border: 'border-blue-200',  text: 'text-blue-800'  },
}
</script>
