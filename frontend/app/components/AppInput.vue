<template>
  <div>
    <label v-if="label" :for="id" class="block text-sm font-semibold text-slate-700 mb-1.5">
      {{ label }} <span v-if="required" class="text-red-500">*</span>
    </label>
    <input
      :id="id"
      v-bind="$attrs"
      :value="modelValue"
      :class="[
        'w-full px-4 py-2.5 border rounded-lg text-sm transition-all duration-200',
        'focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent',
        'placeholder:text-slate-400',
        error ? 'border-red-400 bg-red-50' : 'border-slate-300 bg-white hover:border-slate-400',
        $attrs.disabled ? 'bg-slate-100 cursor-not-allowed text-slate-500' : '',
      ]"
      @input="$emit('update:modelValue', ($event.target as HTMLInputElement).value)"
    />
    <p v-if="error" class="mt-1 text-xs text-red-600 font-medium">{{ error }}</p>
    <p v-else-if="hint" class="mt-1 text-xs text-slate-500">{{ hint }}</p>
  </div>
</template>

<script setup lang="ts">
defineOptions({ inheritAttrs: false })
defineProps<{
  id?: string
  label?: string
  modelValue?: string | number
  error?: string
  hint?: string
  required?: boolean
}>()
defineEmits(['update:modelValue'])
</script>
