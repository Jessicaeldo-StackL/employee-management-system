<template>
  <div class="page-shell">
    <div class="page-container">
      <div class="toolbar mb-6">
        <div class="title-block">
          <p class="eyebrow">Administration</p>
          <h1 class="page-title">Leave Approval</h1>
          <p class="page-subtitle">Approve or reject pending leave requests.</p>
        </div>
        <div class="toolbar-actions">
          <NuxtLink to="/admin" class="btn-ghost">
            <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><path d="M3 11.5L12 4l9 7.5" stroke-linecap="round" stroke-linejoin="round" /><path d="M5 10.5V20h14v-9.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
            Back to Dashboard
          </NuxtLink>
          <NuxtLink to="/admin/leaves" class="btn-secondary">
            <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><rect x="4" y="4" width="16" height="16" rx="2" /><path d="M8 8h8M8 12h8M8 16h5" stroke-linecap="round" /></svg>
            View Leave History
          </NuxtLink>
        </div>
      </div>

      <div v-if="message.text" :class="message.type === 'success' ? 'border-emerald-200 bg-emerald-50 text-emerald-800' : 'border-rose-200 bg-rose-50 text-rose-800'" class="mb-4 rounded-2xl border p-4 text-sm font-medium">
        {{ message.text }}
      </div>

      <div class="table-shell">
        <div v-if="isLoading" class="p-8 text-center text-slate-600">Loading pending requests...</div>
        <div v-else-if="pendingLeaves.length === 0" class="p-8 text-center text-slate-600">No pending leave requests.</div>

        <div v-else class="overflow-x-auto">
          <table class="min-w-full text-sm">
            <thead class="table-head">
              <tr>
                <th class="table-head-cell">Employee Name</th>
                <th class="table-head-cell">Leave Type</th>
                <th class="table-head-cell">Start Date</th>
                <th class="table-head-cell">End Date</th>
                <th class="table-head-cell">Reason</th>
                <th class="table-head-cell">Action</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-slate-100 bg-white">
              <tr v-for="leave in pendingLeaves" :key="leave.id" class="table-row">
                <td class="table-cell font-semibold text-slate-900">{{ leave.employee_username }}</td>
                <td class="table-cell">{{ leave.leave_type }}</td>
                <td class="table-cell">{{ formatDate(leave.start_date) }}</td>
                <td class="table-cell">{{ formatDate(leave.end_date) }}</td>
                <td class="table-cell max-w-xs break-words">{{ leave.reason }}</td>
                <td class="table-cell">
                  <div class="flex flex-wrap gap-2">
                    <button
                      class="btn-success px-3 py-2 text-xs"
                      :disabled="processingId === leave.id"
                      @click="updateStatus(leave.id, 'approve')"
                    >
                      Approve
                    </button>
                    <button
                      class="btn-danger px-3 py-2 text-xs"
                      :disabled="processingId === leave.id"
                      @click="updateStatus(leave.id, 'reject')"
                    >
                      Reject
                    </button>
                  </div>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, reactive, ref } from 'vue'

interface LeaveRequest {
  id: number
  employee_username: string
  leave_type: string
  start_date: string
  end_date: string
  reason: string
  status: 'Pending' | 'Approved' | 'Rejected' | 'Withdrawn'
}

const { get, put } = useApi()
const { redirectIfUnauthorized } = useAuthGuard('admin')

const isLoading = ref(true)
const processingId = ref<number | null>(null)
const leaves = ref<LeaveRequest[]>([])
const message = reactive<{ type: 'success' | 'error'; text: string }>({ type: 'success', text: '' })

const pendingLeaves = computed(() => leaves.value.filter((leave) => leave.status === 'Pending'))
const formatDate = (date: string) => new Date(date).toLocaleDateString()

const loadLeaves = async () => {
  isLoading.value = true
  try {
    const result = await get('/leave-requests')
    leaves.value = result?.data || []
  } catch {
    leaves.value = []
  } finally {
    isLoading.value = false
  }
}

const updateStatus = async (id: number, action: 'approve' | 'reject') => {
  processingId.value = id
  message.text = ''
  try {
    const endpoint = action === 'approve' ? `/leave-requests/${id}/approve` : `/leave-requests/${id}/reject`
    const result = await put(endpoint, {})
    if (!result.ok) {
      message.type = 'error'
      message.text = result.data?.message || `Failed to ${action} leave request`
      return
    }

    const index = leaves.value.findIndex((leave) => leave.id === id)
    if (index >= 0) {
      leaves.value[index].status = action === 'approve' ? 'Approved' : 'Rejected'
    }
    message.type = 'success'
    message.text = result.data?.message || `Leave request ${action}d successfully`
  } catch {
    message.type = 'error'
    message.text = `Unable to ${action} leave request`
  } finally {
    processingId.value = null
  }
}

onMounted(async () => {
  redirectIfUnauthorized()
  await loadLeaves()
})
</script>