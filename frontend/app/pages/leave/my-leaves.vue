<template>
  <div class="page-shell">
    <div class="page-container">
      <div class="toolbar mb-6">
        <div class="title-block">
          <p class="eyebrow">Employee Portal</p>
          <h1 class="page-title">My Leaves</h1>
          <p class="page-subtitle">Track all your leave requests and approval status.</p>
        </div>
        <div class="toolbar-actions">
          <NuxtLink to="/employee" class="btn-ghost">
            <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><path d="M3 11.5L12 4l9 7.5" stroke-linecap="round" stroke-linejoin="round" /><path d="M5 10.5V20h14v-9.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
            Back to Dashboard
          </NuxtLink>
          <NuxtLink to="/leave/apply" class="btn-primary">
            <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><path d="M12 5v14" stroke-linecap="round" /><path d="M5 12h14" stroke-linecap="round" /></svg>
            Apply Leave
          </NuxtLink>
        </div>
      </div>

      <div v-if="message.text" :class="message.type === 'success' ? 'border-emerald-200 bg-emerald-50 text-emerald-800' : 'border-rose-200 bg-rose-50 text-rose-800'" class="mb-4 rounded-2xl border p-4 text-sm font-medium">
        {{ message.text }}
      </div>

      <div class="table-shell">
        <div v-if="isLoading" class="p-8 text-center text-slate-600">Loading leave requests...</div>
        <div v-else-if="leaves.length === 0" class="p-8 text-center text-slate-600">No leave requests found.</div>

        <div v-else class="overflow-x-auto">
          <table class="min-w-full text-sm">
            <thead class="table-head">
              <tr>
                <th class="table-head-cell">Leave Type</th>
                <th class="table-head-cell">Start Date</th>
                <th class="table-head-cell">End Date</th>
                <th class="table-head-cell">Reason</th>
                <th class="table-head-cell">Status</th>
                <th class="table-head-cell">Action</th>
                <th class="table-head-cell">Created Date</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-slate-100 bg-white">
              <tr v-for="leave in leaves" :key="leave.id" class="table-row">
                <td class="table-cell font-semibold text-slate-900">{{ leave.leave_type }}</td>
                <td class="table-cell">{{ formatDate(leave.start_date) }}</td>
                <td class="table-cell">{{ formatDate(leave.end_date) }}</td>
                <td class="table-cell max-w-xs break-words">{{ leave.reason }}</td>
                <td class="table-cell">
                  <span class="status-badge" :class="statusClass(leave.status)">
                    {{ leave.status }}
                  </span>
                </td>
                <td class="table-cell">
                  <button
                    v-if="leave.status === 'Pending'"
                    class="btn-danger px-3 py-2 text-xs"
                    :disabled="processingId === leave.id"
                    @click="withdrawLeave(leave.id)"
                  >
                    {{ processingId === leave.id ? 'Withdrawing...' : 'Withdraw Leave' }}
                  </button>
                  <span v-else class="text-xs text-slate-500">-</span>
                </td>
                <td class="table-cell">{{ formatDateTime(leave.created_at) }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, reactive, ref } from 'vue'

interface LeaveRequest {
  id: number
  leave_type: string
  start_date: string
  end_date: string
  reason: string
  status: 'Pending' | 'Approved' | 'Rejected' | 'Withdrawn'
  created_at: string
}

const { get, put } = useApi()
const { redirectIfUnauthorized } = useAuthGuard('employee')

const isLoading = ref(true)
const processingId = ref<number | null>(null)
const leaves = ref<LeaveRequest[]>([])
const message = reactive<{ type: 'success' | 'error'; text: string }>({ type: 'success', text: '' })

const statusClass = (status: string) => {
  if (status === 'Approved') return 'status-approved'
  if (status === 'Rejected') return 'status-rejected'
  if (status === 'Withdrawn') return 'status-withdrawn'
  return 'status-pending'
}

const formatDate = (date: string) => {
  return new Date(date).toLocaleDateString()
}

const formatDateTime = (date: string) => {
  return new Date(date).toLocaleString()
}

const loadLeaves = async () => {
  isLoading.value = true
  try {
    const result = await get('/leave-requests/my')
    leaves.value = result?.data || []
  } catch {
    leaves.value = []
  } finally {
    isLoading.value = false
  }
}

const withdrawLeave = async (id: number) => {
  processingId.value = id
  message.text = ''
  try {
    const result = await put(`/leave-requests/${id}/withdraw`, {})
    if (!result.ok) {
      console.error('Withdraw leave failed:', {
        leaveId: id,
        status: result.status,
        response: result.data
      })
      message.type = 'error'
      message.text = result.data?.message || result.data?.error || `Failed to withdraw leave request (${result.status})`
      return
    }

    const index = leaves.value.findIndex((leave) => leave.id === id)
    if (index >= 0) {
      leaves.value[index].status = 'Withdrawn'
    }

    message.type = 'success'
    message.text = result.data?.message || 'Leave request withdrawn successfully'
  } catch {
    message.type = 'error'
    message.text = 'Unable to withdraw leave request'
  } finally {
    processingId.value = null
  }
}

onMounted(async () => {
  redirectIfUnauthorized()
  await loadLeaves()
})
</script>