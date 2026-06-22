<template>
  <div class="page-shell">
    <div class="page-container-narrow">
      <div class="toolbar mb-6">
        <div class="title-block">
          <p class="eyebrow">Leave Management</p>
          <h1 class="page-title">Apply Leave</h1>
          <p class="page-subtitle">Submit your leave request for manager approval.</p>
        </div>
        <div class="toolbar-actions">
          <NuxtLink to="/employee" class="btn-ghost">
            <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><path d="M3 11.5L12 4l9 7.5" stroke-linecap="round" stroke-linejoin="round" /><path d="M5 10.5V20h14v-9.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
            Back to Dashboard
          </NuxtLink>
          <NuxtLink to="/leave/my-leaves" class="btn-secondary">
            <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><rect x="4" y="4" width="16" height="16" rx="2" /><path d="M8 8h8M8 12h8M8 16h5" stroke-linecap="round" /></svg>
            My Leaves
          </NuxtLink>
        </div>
      </div>

      <div class="form-panel">
        <form class="space-y-5" @submit.prevent="submitLeave">
          <div>
            <label class="field-label">Leave Type</label>
            <select
              v-model="form.leave_type"
              class="select-field"
            >
              <option value="">Select leave type</option>
              <option value="Sick Leave">Sick Leave</option>
              <option value="Vacation">Vacation</option>
              <option value="Personal Leave">Personal Leave</option>
              <option value="Maternity Leave">Maternity Leave</option>
              <option value="Other">Other</option>
            </select>
            <p v-if="errors.leave_type" class="text-red-600 text-sm mt-1">{{ errors.leave_type }}</p>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label class="field-label">Start Date</label>
              <input v-model="form.start_date" type="date" class="input-field" />
              <p v-if="errors.start_date" class="text-red-600 text-sm mt-1">{{ errors.start_date }}</p>
            </div>
            <div>
              <label class="field-label">End Date</label>
              <input v-model="form.end_date" type="date" class="input-field" />
              <p v-if="errors.end_date" class="text-red-600 text-sm mt-1">{{ errors.end_date }}</p>
            </div>
          </div>

          <div>
            <label class="field-label">Reason</label>
            <textarea
              v-model="form.reason"
              rows="4"
              placeholder="Reason for leave"
              class="textarea-field"
            ></textarea>
            <p v-if="errors.reason" class="text-red-600 text-sm mt-1">{{ errors.reason }}</p>
          </div>

          <div v-if="message.text" :class="message.type === 'success' ? 'border-emerald-200 bg-emerald-50 text-emerald-800' : 'border-rose-200 bg-rose-50 text-rose-800'" class="rounded-2xl border p-4 text-sm font-medium">
            {{ message.text }}
          </div>

          <button type="submit" :disabled="isSubmitting" class="btn-primary w-full py-3.5">
            <svg v-if="!isSubmitting" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><path d="M12 3v14" stroke-linecap="round" /><path d="M7 9l5-6 5 6" stroke-linecap="round" stroke-linejoin="round" /><path d="M5 21h14" stroke-linecap="round" /></svg>
            <svg v-else class="h-4 w-4 animate-spin" viewBox="0 0 24 24" fill="none"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" /><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" /></svg>
            {{ isSubmitting ? 'Submitting...' : 'Submit Leave Request' }}
          </button>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, reactive, ref } from 'vue'

const { post } = useApi()
const { redirectIfUnauthorized } = useAuthGuard('employee')

const isSubmitting = ref(false)
const message = reactive<{ type: 'success' | 'error'; text: string }>({ type: 'success', text: '' })

const form = reactive({
  leave_type: '',
  start_date: '',
  end_date: '',
  reason: ''
})

const errors = reactive({
  leave_type: '',
  start_date: '',
  end_date: '',
  reason: ''
})

const resetErrors = () => {
  errors.leave_type = ''
  errors.start_date = ''
  errors.end_date = ''
  errors.reason = ''
}

const validate = () => {
  resetErrors()
  if (!form.leave_type) errors.leave_type = 'Leave type is required'
  if (!form.start_date) errors.start_date = 'Start date is required'
  if (!form.end_date) errors.end_date = 'End date is required'
  if (!form.reason.trim()) errors.reason = 'Reason is required'
  if (form.start_date && form.end_date && form.start_date > form.end_date) {
    errors.end_date = 'End date cannot be before start date'
  }
  return !errors.leave_type && !errors.start_date && !errors.end_date && !errors.reason
}

const clearForm = () => {
  form.leave_type = ''
  form.start_date = ''
  form.end_date = ''
  form.reason = ''
}

const submitLeave = async () => {
  message.text = ''
  if (!validate()) return

  isSubmitting.value = true
  try {
    const result = await post('/leave-requests', {
      leave_type: form.leave_type,
      start_date: form.start_date,
      end_date: form.end_date,
      reason: form.reason.trim()
    })

    if (!result.ok) {
      message.type = 'error'
      message.text = result.data?.message || 'Failed to submit leave request'
      return
    }

    message.type = 'success'
    message.text = result.data?.message || 'Leave request submitted'
    clearForm()
  } catch (error) {
    message.type = 'error'
    message.text = 'Unable to submit leave request'
  } finally {
    isSubmitting.value = false
  }
}

onMounted(() => {
  redirectIfUnauthorized()
})
</script>