<template>
  <div class="page-shell">
    <header class="app-header mb-8">
      <div class="mx-auto flex max-w-7xl flex-col gap-4 px-4 py-4 sm:px-6 lg:flex-row lg:items-center lg:justify-between lg:px-8">
        <div class="flex items-center gap-4">
          <div class="brand-mark">EM</div>
          <div>
            <p class="eyebrow">Administration</p>
            <h1 class="text-2xl font-extrabold tracking-tight text-slate-950">Employee Details</h1>
            <p class="text-sm text-slate-600">Read-only overview of the selected employee.</p>
          </div>
        </div>
        <div class="toolbar-actions">
          <NuxtLink to="/employees" class="btn-ghost">
            Back to List
          </NuxtLink>
          <NuxtLink :to="`/edit-employee/${route.params.id}`" class="btn-primary inline-flex w-auto">
            Edit
          </NuxtLink>
        </div>
      </div>
    </header>

    <main class="page-shell pt-0">
      <div v-if="isLoading" class="flex items-center justify-center min-h-96">
        <div class="text-center">
          <svg class="animate-spin h-12 w-12 text-blue-600 mx-auto mb-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
          </svg>
          <p class="text-slate-600 font-medium">Loading employee details...</p>
        </div>
      </div>

      <div v-else-if="employee" class="space-y-6">
        <div class="section-card border-l-4 border-blue-600 p-8">
          <div class="flex flex-col gap-6 lg:flex-row lg:items-start lg:justify-between">
            <div class="flex items-start gap-6">
              <div class="flex h-24 w-24 items-center justify-center rounded-2xl bg-blue-50 text-center text-sm font-bold tracking-[0.18em] text-blue-700 shadow-sm">
                EMPLOYEE
              </div>
              <div>
                <h2 class="text-3xl font-extrabold tracking-tight text-slate-950 mb-2">
                  {{ employee.first_name }} {{ employee.last_name }}
                </h2>
                <p class="text-lg font-semibold text-blue-700 mb-2">{{ employee.department }}</p>
                <div class="flex flex-wrap gap-4 text-sm text-slate-600">
                  <span>ID: {{ employee.id }}</span>
                  <span>{{ employee.username }}</span>
                </div>
              </div>
            </div>
            <span class="status-approved self-start">Active</span>
          </div>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          <div class="section-card p-6">
            <div class="mb-4 flex items-center pb-4 border-b border-slate-200">
              <h3 class="card-title">Contact Information</h3>
            </div>
            <div class="space-y-4">
              <div>
                <p class="text-sm text-slate-500 font-medium mb-1">Email</p>
                <p class="text-slate-900 font-semibold break-all">{{ employee.email }}</p>
              </div>
              <div>
                <p class="text-sm text-slate-500 font-medium mb-1">Mobile Number</p>
                <p class="text-slate-900 font-semibold">{{ employee.mobile_number }}</p>
              </div>
              <div>
                <p class="text-sm text-slate-500 font-medium mb-1">Address</p>
                <p class="text-slate-900 font-semibold leading-relaxed">{{ employee.address }}</p>
              </div>
            </div>
          </div>

          <div class="section-card p-6">
            <div class="mb-4 flex items-center pb-4 border-b border-slate-200">
              <h3 class="card-title">Compensation</h3>
            </div>
            <div class="rounded-2xl border border-emerald-200 bg-emerald-50 p-4">
              <p class="text-sm text-green-600 font-medium mb-1">Annual Salary</p>
              <p class="text-2xl font-bold text-green-700">{{ formatCurrency(employee.salary) }}</p>
            </div>
          </div>

          <div class="section-card p-6">
            <div class="mb-4 flex items-center pb-4 border-b border-slate-200">
              <h3 class="card-title">Security</h3>
            </div>
            <div class="space-y-4">
              <div>
                <p class="text-sm text-slate-500 font-medium mb-1">Username</p>
                <p class="text-slate-900 font-semibold bg-slate-50 p-2 rounded">{{ employee.username }}</p>
              </div>
              <div class="bg-yellow-50 border border-yellow-200 rounded-lg p-3">
                <p class="text-xs text-yellow-700 font-medium">Password is confidential and cannot be displayed</p>
              </div>
            </div>
          </div>

          <div class="section-card p-6 md:col-span-2 lg:col-span-3">
            <div class="mb-4 flex items-center pb-4 border-b border-slate-200">
              <h3 class="card-title">Current Files</h3>
            </div>
            <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
              <div class="rounded-2xl border border-slate-200 bg-slate-50 p-5">
                <p class="text-sm font-semibold text-slate-700 mb-2">ID Proof 1</p>
                <p class="text-sm text-slate-500 mb-3">{{ isStoredFileValue(employee.id_proof_1) ? 'File available' : 'No file uploaded' }}</p>
                <div v-if="isStoredFileValue(employee.id_proof_1)" class="flex flex-wrap gap-3">
                  <button
                    type="button"
                    class="btn-primary inline-flex w-auto"
                    @click="viewFile(employee.id_proof_1)"
                  >
                    View File
                  </button>
                  <button
                    type="button"
                    class="btn-secondary inline-flex w-auto"
                    @click="downloadFile(employee.id_proof_1)"
                  >
                    Download File
                  </button>
                </div>
              </div>

              <div class="rounded-2xl border border-slate-200 bg-slate-50 p-5">
                <p class="text-sm font-semibold text-slate-700 mb-2">ID Proof 2</p>
                <p class="text-sm text-slate-500 mb-3">{{ isStoredFileValue(employee.id_proof_2) ? 'File available' : 'No file uploaded' }}</p>
                <div v-if="isStoredFileValue(employee.id_proof_2)" class="flex flex-wrap gap-3">
                  <button
                    type="button"
                    class="btn-primary inline-flex w-auto"
                    @click="viewFile(employee.id_proof_2)"
                  >
                    View File
                  </button>
                  <button
                    type="button"
                    class="btn-secondary inline-flex w-auto"
                    @click="downloadFile(employee.id_proof_2)"
                  >
                    Download File
                  </button>
                </div>
              </div>

              <div class="rounded-2xl border border-slate-200 bg-slate-50 p-5">
                <p class="text-sm font-semibold text-slate-700 mb-2">Profile Image</p>
                <p class="text-sm text-slate-500 mb-3">{{ isStoredFileValue(employee.profile_image) ? 'File available' : 'No file uploaded' }}</p>
                <div v-if="isStoredFileValue(employee.profile_image)" class="flex flex-wrap gap-3">
                  <button
                    type="button"
                    class="btn-primary inline-flex w-auto"
                    @click="viewFile(employee.profile_image)"
                  >
                    View File
                  </button>
                  <button
                    type="button"
                    class="btn-secondary inline-flex w-auto"
                    @click="downloadFile(employee.profile_image)"
                  >
                    Download File
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div class="rounded-2xl border border-blue-200 bg-blue-50 p-6">
          <div class="flex items-start space-x-4">
            <div>
              <h3 class="font-bold text-blue-900 mb-2">Admin View</h3>
              <p class="text-blue-800 text-sm">This page is read-only. Use Edit only if you need to replace employee information or uploaded documents.</p>
            </div>
          </div>
        </div>
      </div>

      <div v-else class="section-card p-8 text-center">
        <p class="text-lg text-slate-600 mb-4">Unable to load employee details</p>
        <NuxtLink to="/employees" class="btn-primary inline-flex w-auto px-6 py-2.5">Back to List</NuxtLink>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, navigateTo } from '#app'
import { useApi } from '~/composables/useApi'
import { useAuthToken } from '~/composables/useAuthToken'

interface Employee {
  id: number
  first_name: string
  last_name: string
  email: string
  department: string
  salary: number | string
  mobile_number: string
  address: string
  username: string
  password: string
  id_proof_1: string
  id_proof_2: string
  profile_image: string
}

const route = useRoute()
const employee = ref<Employee | null>(null)
const isLoading = ref(true)
const { API_BASE_URL } = useApi()
const { getToken, removeToken } = useAuthToken()

const isStoredFileValue = (value?: string | null): boolean => {
  if (!value) return false
  return value.startsWith('/uploads/') || value.startsWith('data:')
}

const getFileUrl = (value?: string | null): string => {
  if (!value) return ''
  if (value.startsWith('data:')) return value
  if (value.startsWith('/uploads/')) return `${API_BASE_URL}${value}`
  return value
}

const getViewFileUrl = (value?: string | null): string => {
  if (!value) return ''

  if (value.startsWith('/uploads/')) {
    return `${API_BASE_URL}/files/view?path=${encodeURIComponent(value)}`
  }

  return getFileUrl(value)
}

const getDownloadFileUrl = (value?: string | null): string => {
  if (!value) return ''

  if (value.startsWith('/uploads/')) {
    return `${API_BASE_URL}/files/download?path=${encodeURIComponent(value)}`
  }

  return getFileUrl(value)
}

const getDisplayFileName = (value?: string | null): string => {
  if (!value) return 'download'
  const cleanedValue = value.split('?')[0]
  return cleanedValue.split('/').pop() || 'download'
}

const handleAuthFailure = async () => {
  removeToken()
  await navigateTo('/login')
}

const getApiErrorMessage = (data: any, fallback = 'Request failed'): string => {
  if (typeof data === 'string' && data.trim()) return data
  if (data && typeof data === 'object') {
    if (typeof data.error === 'string' && data.error.trim()) return data.error
    if (typeof data.message === 'string' && data.message.trim()) return data.message
    try {
      return JSON.stringify(data)
    } catch {
      return fallback
    }
  }
  return fallback
}

const fetchProtectedFileBlob = async (url: string): Promise<Blob> => {
  const token = getToken()
  const headers: Record<string, string> = {}
  if (token) headers.Authorization = `Bearer ${token}`

  const response = await fetch(url, { headers })

  if (response.status === 401 || response.status === 403) {
    await handleAuthFailure()
    throw new Error('Unauthorized')
  }

  if (!response.ok) {
    const responseText = await response.text().catch(() => '')
    let responseData: any = responseText
    if (responseText) {
      try {
        responseData = JSON.parse(responseText)
      } catch {
        responseData = responseText
      }
    }

    throw new Error(getApiErrorMessage(responseData, `Request failed (${response.status})`))
  }

  return response.blob()
}

const viewFile = async (value?: string | null) => {
  const url = getViewFileUrl(value)
  if (!url || !process.client) return

  if (url.startsWith('data:')) {
    window.open(url, '_blank')
    return
  }

  const newTab = window.open('about:blank', '_blank')
  if (!newTab) return

  newTab.document.write('<title>Loading file...</title><p style="font-family: sans-serif; padding: 16px;">Loading file...</p>')

  try {
    const blob = await fetchProtectedFileBlob(url)
    const blobUrl = window.URL.createObjectURL(blob)
    newTab.location.href = blobUrl
    setTimeout(() => window.URL.revokeObjectURL(blobUrl), 60_000)
  } catch (error) {
    console.error('Error viewing file:', error)
    newTab.close()
  }
}

const downloadFile = async (value?: string | null) => {
  const url = getDownloadFileUrl(value)
  if (!url || !process.client) return

  const fileName = getDisplayFileName(value)

  try {
    if (url.startsWith('data:')) {
      const link = document.createElement('a')
      link.href = url
      link.download = fileName
      document.body.appendChild(link)
      link.click()
      document.body.removeChild(link)
      return
    }

    const blob = await fetchProtectedFileBlob(url)
    const blobUrl = window.URL.createObjectURL(blob)

    const link = document.createElement('a')
    link.href = blobUrl
    link.download = fileName
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)
    window.URL.revokeObjectURL(blobUrl)
  } catch (error) {
    console.error('Error downloading file:', error)
  }
}

const formatCurrency = (amount: number | string): string => {
  const num = typeof amount === 'string' ? parseFloat(amount) : amount
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: 'USD',
    minimumFractionDigits: 0,
    maximumFractionDigits: 2
  }).format(num)
}

const loadEmployee = async () => {
  isLoading.value = true
  try {
    const { get } = useApi()
    const data = await get(`/employees/${route.params.id}`)
    employee.value = data
  } catch (error) {
    console.error('Error loading employee details:', error)
    employee.value = null
  } finally {
    isLoading.value = false
  }
}

onMounted(async () => {
  await loadEmployee()
})
</script>
