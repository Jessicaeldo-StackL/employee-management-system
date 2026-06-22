<template>
  <NuxtPage v-if="isEmployeeDetailsRoute" />
  <div v-else class="page-shell">
    <header class="app-header mb-8">
      <div class="mx-auto flex max-w-7xl flex-col gap-4 px-4 py-4 sm:px-6 lg:flex-row lg:items-center lg:justify-between lg:px-8">
        <div class="flex items-center gap-4">
          <div class="brand-mark">EM</div>
          <div>
            <p class="eyebrow">Administration</p>
            <h1 class="text-2xl font-extrabold tracking-tight text-slate-950">Employee List</h1>
            <p class="text-sm text-slate-600">Browse and manage the employee directory.</p>
          </div>
        </div>
        <div class="toolbar-actions">
          <NuxtLink to="/" class="btn-ghost">
            <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><path d="M3 11.5L12 4l9 7.5" stroke-linecap="round" stroke-linejoin="round" /><path d="M5 10.5V20h14v-9.5" stroke-linecap="round" stroke-linejoin="round" /><path d="M10 20v-6h4v6" stroke-linecap="round" stroke-linejoin="round" /></svg>
            Home
          </NuxtLink>
          <button @click="handleLogout" class="btn-danger">
            <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><path d="M10 16l-4-4 4-4" stroke-linecap="round" stroke-linejoin="round" /><path d="M6 12h10" stroke-linecap="round" /><path d="M14 4h4a2 2 0 012 2v12a2 2 0 01-2 2h-4" stroke-linecap="round" stroke-linejoin="round" /></svg>
            Logout
          </button>
        </div>
      </div>
    </header>

    <main class="page-shell pt-0">
      <div class="toolbar mb-8">
        <div class="title-block">
          <p class="eyebrow">Employee Directory</p>
          <h2 class="page-title">All Employees</h2>
          <p class="page-subtitle">Total: <span class="font-bold text-blue-700">{{ employees.length }}</span> employees</p>
        </div>
        <NuxtLink to="/add-employee" class="btn-primary w-full sm:w-auto">
          <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><path d="M12 5v14" stroke-linecap="round" /><path d="M5 12h14" stroke-linecap="round" /></svg>
          Add New Employee
        </NuxtLink>
      </div>

      <div class="section-card p-6 mb-8">
        <input
          v-model="searchQuery"
          type="text"
          placeholder="Search employees by name, email, or department..."
          class="input-field"
        />
      </div>

      <!-- Loading State -->
      <div v-if="isLoading" class="flex items-center justify-center min-h-96">
        <div class="text-center">
          <svg class="animate-spin h-12 w-12 text-blue-600 mx-auto mb-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
          </svg>
          <p class="text-slate-600 font-medium">Loading employees...</p>
        </div>
      </div>

      <div v-else-if="filteredEmployees.length > 0" class="table-shell">
        <div class="overflow-x-auto">
          <table class="min-w-full">
            <thead class="table-head">
              <tr>
                <th class="table-head-cell">ID</th>
                <th class="table-head-cell">Name</th>
                <th class="table-head-cell">Email</th>
                <th class="table-head-cell">Department</th>
                <th class="table-head-cell">Salary</th>
                <th class="table-head-cell">Actions</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-slate-100 bg-white">
              <tr
                v-for="(employee, index) in filteredEmployees"
                :key="employee.id"
                class="table-row"
              >
                <td class="table-cell font-semibold text-slate-900">
                  #{{ employee.id }}
                </td>
                <td class="table-cell">
                  <div class="font-semibold text-slate-900">{{ employee.first_name }} {{ employee.last_name }}</div>
                  <div class="text-xs text-slate-500">{{ employee.username }}</div>
                </td>
                <td class="table-cell text-slate-600">
                  {{ employee.email }}
                </td>
                <td class="table-cell">
                  <span class="status-pending">
                    {{ employee.department }}
                  </span>
                </td>
                <td class="table-cell font-semibold text-emerald-700">
                  {{ formatCurrency(employee.salary) }}
                </td>
                <td class="table-cell">
                  <div class="flex flex-wrap gap-2">
                  <NuxtLink
                    :to="`/employees/${employee.id}`"
                    class="btn-success"
                  >
                    View
                  </NuxtLink>
                  <NuxtLink :to="`/edit-employee/${employee.id}`">
                    <button class="btn-primary">
                      Edit
                    </button>
                  </NuxtLink>
                  <button
                    @click="confirmDelete(employee.id, employee.first_name)"
                    class="btn-danger"
                  >
                    Delete
                  </button>
                  </div>
                </td>
              </tr>
            </tbody>
          </table>
        </div>

        <div class="bg-slate-50 px-6 py-4 border-t border-slate-200">
          <p class="text-sm text-slate-600">
            Showing <span class="font-bold">{{ filteredEmployees.length }}</span> of <span class="font-bold">{{ employees.length }}</span> employees
          </p>
        </div>
      </div>

      <div v-else class="section-card p-12 text-center">
        <div class="icon-chip mx-auto mb-4 bg-blue-50 text-blue-700">
          <svg class="h-6 w-6" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><circle cx="11" cy="11" r="7" /><path d="M20 20l-3.5-3.5" stroke-linecap="round" stroke-linejoin="round" /></svg>
        </div>
        <p class="text-lg text-slate-600 mb-6">No employees found</p>
        <NuxtLink to="/add-employee" class="btn-primary inline-flex w-auto">
          Add First Employee
        </NuxtLink>
      </div>

      <div class="mt-8 flex flex-col gap-4 sm:flex-row">
        <button @click="goToAdmin" class="btn-secondary flex-1 py-3.5">
          Back to Admin Dashboard
        </button>
        <button @click="handleLogout" class="btn-danger flex-1 py-3.5">
          Logout
        </button>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { navigateTo, useRoute } from '#app'
import { useApi } from '~/composables/useApi'
interface Employee {
  id: number
  first_name: string
  last_name: string
  email: string
  department: string
  salary: number
  username: string
}

const employees = ref<Employee[]>([])
const isLoading = ref(true)
const searchQuery = ref('')
const route = useRoute()
const isEmployeeDetailsRoute = computed(() => Boolean(route.params.id))

const formatCurrency = (amount: number | string): string => {
  const num = typeof amount === 'string' ? parseFloat(amount) : amount
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: 'USD',
    minimumFractionDigits: 0,
    maximumFractionDigits: 2
  }).format(num)
}

const filteredEmployees = computed(() => {
  if (!searchQuery.value.trim()) {
    return employees.value
  }

  const query = searchQuery.value.toLowerCase()
  return employees.value.filter(emp =>
    emp.first_name.toLowerCase().includes(query) ||
    emp.last_name.toLowerCase().includes(query) ||
    emp.email.toLowerCase().includes(query) ||
    emp.department.toLowerCase().includes(query) ||
    emp.username.toLowerCase().includes(query)
  )
})

const loadEmployees = async () => {
  if (isEmployeeDetailsRoute.value) return

  isLoading.value = true
  try {
    const { get } = useApi()
    const data = await get('/employees')
    employees.value = data
  } catch (error) {
    console.error('Error loading employees:', error)
  } finally {
    isLoading.value = false
  }
}

const confirmDelete = async (id: number, name: string) => {
  const confirmed = confirm(
    `Are you sure you want to delete ${name}? This action cannot be undone.`
  )

  if (!confirmed) return

  try {
    const { delete: deleteEmployee } = useApi()
    const response = await deleteEmployee(`/employees/${id}`)

    if (response.ok) {
      alert(`Employee ${name} has been deleted successfully.`)
      await loadEmployees()
    } else {
      alert('Failed to delete employee. Please try again.')
    }
  } catch (error) {
    console.error('Error deleting employee:', error)
    alert('Error deleting employee. Please try again.')
  }
}

const goToAdmin = async () => {
  await navigateTo('/admin')
}

const goToEmployeeDetails = async (id: number) => {
  await navigateTo(`/employees/${id}`)
}

const handleLogout = async () => {
  localStorage.removeItem('auth_token')
  localStorage.removeItem('username')
  localStorage.removeItem('role')
  localStorage.removeItem('rememberMe')
  await navigateTo('/login')
}

onMounted(async () => {
  await loadEmployees()
})
</script>