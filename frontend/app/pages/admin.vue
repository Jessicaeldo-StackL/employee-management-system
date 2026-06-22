<template>
  <NuxtPage v-if="!isDashboardRoute" />
  <div v-else class="page-shell">
    <header class="app-header">
      <div class="mx-auto flex max-w-7xl flex-col gap-4 px-4 py-4 sm:px-6 lg:flex-row lg:items-center lg:justify-between lg:px-8">
        <div class="flex items-center gap-4">
          <div class="brand-mark">EM</div>
          <div>
            <p class="eyebrow">Administration</p>
            <h1 class="text-2xl font-extrabold tracking-tight text-slate-950">Admin Dashboard</h1>
            <p class="text-sm text-slate-600">Executive controls for employees, leaves, and system governance.</p>
          </div>
        </div>
        <div class="toolbar-actions">
          <NuxtLink to="/" class="btn-ghost">
            <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
              <path d="M3 11.5L12 4l9 7.5" stroke-linecap="round" stroke-linejoin="round" />
              <path d="M5 10.5V20h14v-9.5" stroke-linecap="round" stroke-linejoin="round" />
              <path d="M10 20v-6h4v6" stroke-linecap="round" stroke-linejoin="round" />
            </svg>
            Home
          </NuxtLink>
          <button @click="handleLogout" class="btn-danger">
            <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
              <path d="M10 16l-4-4 4-4" stroke-linecap="round" stroke-linejoin="round" />
              <path d="M6 12h10" stroke-linecap="round" />
              <path d="M14 4h4a2 2 0 012 2v12a2 2 0 01-2 2h-4" stroke-linecap="round" stroke-linejoin="round" />
            </svg>
            Logout
          </button>
        </div>
      </div>
    </header>

    <main class="page-shell">
      <section class="page-hero mb-8 grid gap-6 lg:grid-cols-[1.2fr_0.8fr] lg:items-end">
        <div class="space-y-4">
          <p class="eyebrow">Overview</p>
          <h2 class="page-title">Welcome, Admin</h2>
          <p class="page-subtitle">Manage employee records, monitor leave activity, and keep the workforce platform running smoothly.</p>
        </div>
        <div class="grid gap-4 sm:grid-cols-3 lg:grid-cols-1 xl:grid-cols-3">
          <div class="rounded-2xl border border-slate-200 bg-slate-50 p-4">
            <p class="text-xs font-semibold uppercase tracking-[0.14em] text-slate-500">Total Employees</p>
            <p class="mt-2 text-3xl font-extrabold tracking-tight text-slate-950">{{ totalEmployees }}</p>
          </div>
          <div class="rounded-2xl border border-slate-200 bg-slate-50 p-4">
            <p class="text-xs font-semibold uppercase tracking-[0.14em] text-slate-500">Admin Accounts</p>
            <p class="mt-2 text-3xl font-extrabold tracking-tight text-slate-950">1</p>
          </div>
          <div class="rounded-2xl border border-emerald-200 bg-emerald-50 p-4">
            <p class="text-xs font-semibold uppercase tracking-[0.14em] text-emerald-700">System Status</p>
            <span class="status-approved mt-2">Active</span>
          </div>
        </div>
      </section>

      <div class="grid gap-6 md:grid-cols-2">
        <section class="dashboard-card border-t-4 border-blue-600">
          <div class="space-y-4">
            <div class="icon-chip bg-blue-50 text-blue-700">
              <svg class="h-6 w-6" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
                <path d="M16 11V8a4 4 0 10-8 0v3" stroke-linecap="round" stroke-linejoin="round" />
                <path d="M5 11h14v9H5z" stroke-linejoin="round" />
              </svg>
            </div>
            <div>
              <h3 class="section-title">Employee Management</h3>
              <p class="section-copy mt-2">View, add, edit, and delete employee records from a streamlined workspace.</p>
            </div>
          </div>
          <NuxtLink to="/employees" class="btn-primary mt-6 w-full">
            <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
              <path d="M4 7h16M4 12h16M4 17h10" stroke-linecap="round" />
            </svg>
            Manage Employees
          </NuxtLink>
        </section>

        <section class="dashboard-card border-t-4 border-emerald-600">
          <div class="space-y-4">
            <div class="icon-chip bg-emerald-50 text-emerald-700">
              <svg class="h-6 w-6" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
                <path d="M7 4h10a2 2 0 012 2v13l-4-3-4 3-4-3-4 3V6a2 2 0 012-2z" stroke-linejoin="round" />
              </svg>
            </div>
            <div>
              <h3 class="section-title">Leave Management</h3>
              <p class="section-copy mt-2">Review leave history and approve or reject pending applications with clarity.</p>
            </div>
          </div>
          <div class="mt-6 grid gap-3 sm:grid-cols-2">
            <NuxtLink to="/admin/leaves" class="btn-soft w-full">
              <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
                <path d="M7 3v4M17 3v4M4 8h16" stroke-linecap="round" stroke-linejoin="round" />
                <rect x="4" y="6" width="16" height="14" rx="2" />
              </svg>
              Leave History
            </NuxtLink>
            <NuxtLink to="/admin/leave-approval" class="btn-primary w-full">
              <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
                <path d="M20 6L9 17l-5-5" stroke-linecap="round" stroke-linejoin="round" />
              </svg>
              Approvals
            </NuxtLink>
          </div>
        </section>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { navigateTo } from '#app'

const totalEmployees = ref(0)
const { removeToken } = useAuthToken()
const route = useRoute()
const isDashboardRoute = computed(() => route.path === '/admin')

const loadStats = async () => {
  try {
    const { get } = useApi()
    const employees = await get('/employees')
    totalEmployees.value = employees.length
  } catch (error) {
    console.error('Error loading stats:', error)
  }
}

const handleLogout = async () => {
  removeToken()
  localStorage.removeItem('rememberMe')
  await navigateTo('/login')
}

onMounted(async () => {
  if (isDashboardRoute.value) {
    await loadStats()
  }
})
</script>