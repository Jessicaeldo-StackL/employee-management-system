<template>
  <div class="page-shell">
    <div class="page-container">
      <div class="toolbar mb-6">
        <div class="title-block">
          <p class="eyebrow">Administration</p>
          <h1 class="page-title">Leave Management</h1>
          <p class="page-subtitle">Review all employee leave requests and history.</p>
        </div>
        <div class="toolbar-actions">
          <NuxtLink to="/admin" class="btn-ghost">
            <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><path d="M3 11.5L12 4l9 7.5" stroke-linecap="round" stroke-linejoin="round" /><path d="M5 10.5V20h14v-9.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
            Back to Dashboard
          </NuxtLink>
          <NuxtLink to="/admin/leave-approval" class="btn-primary">
            <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><path d="M5 12l4 4L19 6" stroke-linecap="round" stroke-linejoin="round" /></svg>
            Leave Approval
          </NuxtLink>
        </div>
      </div>

      <div class="table-shell">
        <div v-if="isLoading" class="p-8 text-center text-slate-600">Loading leave requests...</div>
        <div v-else-if="leaves.length === 0" class="p-8 text-center text-slate-600">No leave requests available.</div>
        <div v-else class="overflow-x-auto">
          <table class="min-w-full text-sm">
            <thead class="table-head">
              <tr>
                <th class="table-head-cell">Employee Name</th>
                <th class="table-head-cell">Leave Type</th>
                <th class="table-head-cell">Start Date</th>
                <th class="table-head-cell">End Date</th>
                <th class="table-head-cell">Reason</th>
                <th class="table-head-cell">Status</th>
                <th class="table-head-cell">Created Date</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-slate-100 bg-white">
              <tr v-for="leave in leaves" :key="leave.id" class="table-row">
                <td class="table-cell font-semibold text-slate-900">{{ leave.employee_username }}</td>
                <td class="table-cell">{{ leave.leave_type }}</td>
                <td class="table-cell">{{ formatDate(leave.start_date) }}</td>
                <td class="table-cell">{{ formatDate(leave.end_date) }}</td>
                <td class="table-cell max-w-xs break-words">{{ leave.reason }}</td>
                <td class="table-cell">
                  <span class="status-badge" :class="statusClass(leave.status)">
                    {{ leave.status }}
                  </span>
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
import { onMounted, ref } from 'vue'

interface LeaveRequest {
  id: number
  employee_username: string
  leave_type: string
  start_date: string
  end_date: string
  reason: string
  status: 'Pending' | 'Approved' | 'Rejected' | 'Withdrawn'
  created_at: string
}

const { get } = useApi()
const { redirectIfUnauthorized } = useAuthGuard('admin')

const isLoading = ref(true)
const leaves = ref<LeaveRequest[]>([])

const statusClass = (status: string) => {
  if (status === 'Approved') return 'status-approved'
  if (status === 'Rejected') return 'status-rejected'
  if (status === 'Withdrawn') return 'status-withdrawn'
  return 'status-pending'
}

const formatDate = (date: string) => new Date(date).toLocaleDateString()
const formatDateTime = (date: string) => new Date(date).toLocaleString()

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

onMounted(async () => {
  redirectIfUnauthorized()
  await loadLeaves()
})
</script>