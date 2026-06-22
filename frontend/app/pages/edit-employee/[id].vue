<template>
  <div class="min-h-screen bg-gradient-to-br from-slate-50 to-slate-100">
    <!-- Header Navigation -->
    <header class="bg-white shadow-sm border-b border-slate-200">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4 flex items-center justify-between">
        <div class="flex items-center space-x-3">
          <div class="w-10 h-10 bg-gradient-to-br from-blue-600 to-blue-700 rounded-lg flex items-center justify-center">
            <span class="text-white font-bold text-lg">EM</span>
          </div>
          <h1 class="text-2xl font-bold text-slate-900">Edit Employee</h1>
        </div>
        <div class="flex items-center space-x-3">
          <NuxtLink to="/">
            <button class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg transition-all duration-200 shadow-sm hover:shadow-md">
              Home
            </button>
          </NuxtLink>
          <button
            @click="handleCancel"
            class="px-4 py-2 bg-slate-600 hover:bg-slate-700 text-white font-medium rounded-lg transition-all duration-200 shadow-sm hover:shadow-md"
          >
            Cancel
          </button>
        </div>
      </div>
    </header>

    <!-- Main Content -->
    <main class="max-w-2xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <div class="bg-white rounded-xl shadow-card-lg p-8">
        <!-- Loading State -->
        <div v-if="isLoading" class="flex items-center justify-center min-h-96">
          <div class="text-center">
            <svg class="animate-spin h-12 w-12 text-blue-600 mx-auto mb-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
            </svg>
            <p class="text-slate-600 font-medium">Loading employee data...</p>
          </div>
        </div>

        <!-- Form -->
        <form v-else @submit.prevent="updateEmployee" class="space-y-6">
          <!-- Personal Information Section -->
          <div>
            <h2 class="text-lg font-bold text-slate-900 mb-4 flex items-center">
              Personal Information
            </h2>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div>
                <label class="block text-sm font-semibold text-slate-700 mb-2">First Name *</label>
                <input
                  v-model="employee.first_name"
                  type="text"
                  placeholder="Enter first name"
                  class="input-field"
                  required
                />
              </div>
              <div>
                <label class="block text-sm font-semibold text-slate-700 mb-2">Last Name *</label>
                <input
                  v-model="employee.last_name"
                  type="text"
                  placeholder="Enter last name"
                  class="input-field"
                  required
                />
              </div>
            </div>
          </div>

          <!-- Contact Information Section -->
          <div class="pt-6 border-t border-slate-200">
            <h2 class="text-lg font-bold text-slate-900 mb-4 flex items-center">
              Contact Information
            </h2>
            <div>
              <label class="block text-sm font-semibold text-slate-700 mb-2">Email *</label>
              <input
                v-model="employee.email"
                type="email"
                placeholder="Enter email address"
                class="input-field"
                required
              />
            </div>
          </div>

          <!-- Contact Details Section -->
          <div class="pt-6 border-t border-slate-200">
            <h2 class="text-lg font-bold text-slate-900 mb-4 flex items-center">
              Additional Contact Details
            </h2>
            <div class="space-y-6">
              <div>
                <label class="block text-sm font-semibold text-slate-700 mb-2">Mobile Number</label>
                <input
                  v-model="employee.mobile_number"
                  type="tel"
                  placeholder="Enter mobile number"
                  class="input-field"
                />
              </div>
              <div>
                <label class="block text-sm font-semibold text-slate-700 mb-2">Address</label>
                <textarea
                  v-model="employee.address"
                  placeholder="Enter full address"
                  rows="3"
                  class="w-full px-4 py-3 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200"
                ></textarea>
              </div>
            </div>
          </div>

          <!-- Job Information Section -->
          <div class="pt-6 border-t border-slate-200">
            <h2 class="text-lg font-bold text-slate-900 mb-4 flex items-center">
              Job Information
            </h2>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div>
                <label class="block text-sm font-semibold text-slate-700 mb-2">Department *</label>
                <input
                  v-model="employee.department"
                  type="text"
                  placeholder="Enter department"
                  class="input-field"
                  required
                />
              </div>
              <div>
                <label class="block text-sm font-semibold text-slate-700 mb-2">Salary *</label>
                <input
                  v-model="employee.salary"
                  type="number"
                  placeholder="Enter salary"
                  class="input-field"
                  required
                />
              </div>
            </div>
          </div>

          <!-- Security Information Section -->
          <div class="pt-6 border-t border-slate-200">
            <h2 class="text-lg font-bold text-slate-900 mb-4 flex items-center">
              Security Information
            </h2>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div>
                <label class="block text-sm font-semibold text-slate-700 mb-2">Username</label>
                <input
                  v-model="employee.username"
                  type="text"
                  placeholder="Enter username"
                  class="input-field"
                  disabled
                />
                <p class="text-xs text-slate-500 mt-1">Username cannot be changed</p>
              </div>
              <div>
                <label class="block text-sm font-semibold text-slate-700 mb-2">Password</label>
                <input
                  v-model="employee.password"
                  type="password"
                  placeholder="Enter password"
                  class="input-field"
                />
                <p class="text-xs text-slate-500 mt-1">Leave blank to keep current password</p>
              </div>
            </div>
          </div>

          <!-- Current Uploaded Files -->
          <div class="pt-6 border-t border-slate-200">
            <h2 class="text-lg font-bold text-slate-900 mb-4 flex items-center">
              Current Uploaded Files
            </h2>
            <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
              <div class="bg-white rounded-xl shadow-card-lg p-6 border border-slate-100 space-y-4">
                <div class="flex items-center mb-4 pb-4 border-b border-slate-200">
                  <h3 class="text-lg font-bold text-slate-900">ID Proof 1</h3>
                </div>
                <div v-if="isStoredFileValue(employee.id_proof_1)" class="flex flex-wrap gap-3">
                  <button
                    type="button"
                    class="inline-flex items-center px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg transition-all duration-200"
                    @click="viewFile(employee.id_proof_1)"
                  >
                    View File
                  </button>
                  <button
                    type="button"
                    class="inline-flex items-center px-4 py-2 bg-slate-700 hover:bg-slate-800 text-white font-medium rounded-lg transition-all duration-200"
                    @click="downloadFile(employee.id_proof_1)"
                  >
                    Download File
                  </button>
                </div>
                <p v-else class="text-sm text-slate-500 italic">No file uploaded</p>
              </div>

              <div class="bg-white rounded-xl shadow-card-lg p-6 border border-slate-100 space-y-4">
                <div class="flex items-center mb-4 pb-4 border-b border-slate-200">
                  <h3 class="text-lg font-bold text-slate-900">ID Proof 2</h3>
                </div>
                <div v-if="isStoredFileValue(employee.id_proof_2)" class="flex flex-wrap gap-3">
                  <button
                    type="button"
                    class="inline-flex items-center px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg transition-all duration-200"
                    @click="viewFile(employee.id_proof_2)"
                  >
                    View File
                  </button>
                  <button
                    type="button"
                    class="inline-flex items-center px-4 py-2 bg-slate-700 hover:bg-slate-800 text-white font-medium rounded-lg transition-all duration-200"
                    @click="downloadFile(employee.id_proof_2)"
                  >
                    Download File
                  </button>
                </div>
                <p v-else class="text-sm text-slate-500 italic">No file uploaded</p>
              </div>

              <div class="bg-white rounded-xl shadow-card-lg p-6 border border-slate-100 space-y-4">
                <div class="flex items-center mb-4 pb-4 border-b border-slate-200">
                  <h3 class="text-lg font-bold text-slate-900">Profile Image</h3>
                </div>
                <div v-if="isStoredFileValue(employee.profile_image)" class="flex flex-wrap gap-3">
                  <button
                    type="button"
                    class="inline-flex items-center px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg transition-all duration-200"
                    @click="viewFile(employee.profile_image)"
                  >
                    View File
                  </button>
                  <button
                    type="button"
                    class="inline-flex items-center px-4 py-2 bg-slate-700 hover:bg-slate-800 text-white font-medium rounded-lg transition-all duration-200"
                    @click="downloadFile(employee.profile_image)"
                  >
                    Download File
                  </button>
                </div>
                <p v-else class="text-sm text-slate-500 italic">No file uploaded</p>
              </div>
            </div>
          </div>

          <!-- Additional Information Section -->
          <div class="pt-6 border-t border-slate-200">
            <h2 class="text-lg font-bold text-slate-900 mb-4 flex items-center">
              File Uploads
            </h2>
            <div class="space-y-6">
              <!-- ID Proof 1 Upload -->
              <div>
                <label class="block text-sm font-semibold text-slate-700 mb-3">ID Proof 1 (PDF)</label>
                <div class="relative">
                  <input
                    type="file"
                    accept=".pdf,application/pdf"
                    @change="(e) => handleFileUpload(e, idProof1, 'pdf')"
                    class="hidden"
                    id="idProof1Input"
                  />
                  <label
                    for="idProof1Input"
                    class="flex items-center justify-center w-full px-4 py-8 border-2 border-dashed border-slate-300 rounded-lg bg-slate-50 hover:bg-slate-100 cursor-pointer transition-all duration-200 hover:border-blue-400"
                  >
                    <div class="text-center">
                      <svg v-if="!idProof1.preview" class="mx-auto h-12 w-12 text-slate-400 mb-2" stroke="currentColor" fill="none" viewBox="0 0 48 48">
                        <path d="M28 8H12a4 4 0 00-4 4v24a4 4 0 004 4h24a4 4 0 004-4V20m-14-8v12m0 0l-4-4m4 4l4-4" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
                      </svg>
                      <svg v-else class="mx-auto h-12 w-12 text-green-500 mb-2" fill="currentColor" viewBox="0 0 20 20">
                        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
                      </svg>
                      <p class="text-sm font-medium text-slate-900">{{ idProof1.fileName || 'Click to upload or drag PDF' }}</p>
                      <p class="text-xs text-slate-500 mt-1">PDF up to 5MB</p>
                    </div>
                  </label>
                </div>
              </div>

              <!-- ID Proof 2 Upload -->
              <div>
                <label class="block text-sm font-semibold text-slate-700 mb-3">ID Proof 2 (PDF)</label>
                <div class="relative">
                  <input
                    type="file"
                    accept=".pdf,application/pdf"
                    @change="(e) => handleFileUpload(e, idProof2, 'pdf')"
                    class="hidden"
                    id="idProof2Input"
                  />
                  <label
                    for="idProof2Input"
                    class="flex items-center justify-center w-full px-4 py-8 border-2 border-dashed border-slate-300 rounded-lg bg-slate-50 hover:bg-slate-100 cursor-pointer transition-all duration-200 hover:border-blue-400"
                  >
                    <div class="text-center">
                      <svg v-if="!idProof2.preview" class="mx-auto h-12 w-12 text-slate-400 mb-2" stroke="currentColor" fill="none" viewBox="0 0 48 48">
                        <path d="M28 8H12a4 4 0 00-4 4v24a4 4 0 004 4h24a4 4 0 004-4V20m-14-8v12m0 0l-4-4m4 4l4-4" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
                      </svg>
                      <svg v-else class="mx-auto h-12 w-12 text-green-500 mb-2" fill="currentColor" viewBox="0 0 20 20">
                        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
                      </svg>
                      <p class="text-sm font-medium text-slate-900">{{ idProof2.fileName || 'Click to upload or drag PDF' }}</p>
                      <p class="text-xs text-slate-500 mt-1">PDF up to 5MB</p>
                    </div>
                  </label>
                </div>
              </div>

              <!-- Profile Image Upload -->
              <div>
                <label class="block text-sm font-semibold text-slate-700 mb-3">Profile Image</label>
                <div class="relative">
                  <input
                    type="file"
                    accept="image/*"
                    @change="(e) => handleFileUpload(e, profileImage, 'image')"
                    class="hidden"
                    id="profileImageInput"
                  />
                  <label
                    for="profileImageInput"
                    class="flex items-center justify-center w-full px-4 py-8 border-2 border-dashed border-slate-300 rounded-lg bg-slate-50 hover:bg-slate-100 cursor-pointer transition-all duration-200 hover:border-blue-400"
                  >
                    <div class="text-center w-full">
                      <img v-if="profileImage.preview && profileImage.preview !== 'pdf'" :src="profileImage.preview" alt="Profile preview" class="mx-auto h-24 w-24 object-cover rounded-lg mb-2" />
                      <svg v-else-if="!profileImage.preview" class="mx-auto h-12 w-12 text-slate-400 mb-2" stroke="currentColor" fill="none" viewBox="0 0 48 48">
                        <path d="M28 8H12a4 4 0 00-4 4v24a4 4 0 004 4h24a4 4 0 004-4V20m-14-8v12m0 0l-4-4m4 4l4-4" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
                      </svg>
                      <p class="text-sm font-medium text-slate-900">{{ profileImage.fileName || 'Click to upload or drag image' }}</p>
                      <p class="text-xs text-slate-500 mt-1">JPG, PNG, WebP up to 5MB</p>
                    </div>
                  </label>
                </div>
              </div>
            </div>
          </div>

          <!-- Alert Messages -->
          <transition name="fade">
            <div v-if="message && message.includes('success')" class="p-4 bg-green-50 border border-green-200 rounded-lg">
              <p class="text-green-800 text-sm font-medium flex items-center">
                <span class="mr-2">Success:</span>
                {{ message }}
              </p>
            </div>
          </transition>

          <transition name="fade">
            <div v-if="message && !message.includes('success')" class="p-4 bg-red-50 border border-red-200 rounded-lg">
              <p class="text-red-800 text-sm font-medium flex items-center">
                <span class="mr-2">Error:</span>
                {{ message }}
              </p>
            </div>
          </transition>

          <!-- Action Buttons -->
          <div class="flex space-x-4 pt-6 border-t border-slate-200">
            <button
              type="submit"
              :disabled="isSubmitting"
              class="flex-1 px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white font-semibold rounded-lg transition-all duration-200 shadow-md hover:shadow-lg disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center space-x-2"
            >
              <span v-if="!isSubmitting">Update Employee</span>
              <span v-else>Updating...</span>
            </button>
            <button
              type="button"
              @click="handleCancel"
              class="flex-1 px-6 py-3 bg-slate-600 hover:bg-slate-700 text-white font-semibold rounded-lg transition-all duration-200 shadow-md hover:shadow-lg flex items-center justify-center space-x-2"
            >
              <span>Cancel</span>
            </button>
          </div>
        </form>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useApi } from '~/composables/useApi'
import { useAuthToken } from '~/composables/useAuthToken'

interface Employee {
  first_name: string
  last_name: string
  email: string
  department: string
  salary: number | string
  mobile_number?: string
  address?: string
  username?: string
  password?: string
  id_proof_1?: string
  id_proof_2?: string
  profile_image?: string
}

interface FileUpload {
  file: File | null
  preview: string | null
  fileName: string
  uploading: boolean
}

const route = useRoute()
const router = useRouter()
const { API_BASE_URL } = useApi()
const { getToken, removeToken } = useAuthToken()

const message = ref('')
const isLoading = ref(true)
const isSubmitting = ref(false)

const employee = ref<Employee>({
  first_name: '',
  last_name: '',
  email: '',
  department: '',
  salary: ''
})

const idProof1 = ref<FileUpload>({
  file: null,
  preview: null,
  fileName: '',
  uploading: false
})

const idProof2 = ref<FileUpload>({
  file: null,
  preview: null,
  fileName: '',
  uploading: false
})

const profileImage = ref<FileUpload>({
  file: null,
  preview: null,
  fileName: '',
  uploading: false
})

const generateTimestampFileName = (originalName: string): string => {
  const timestamp = new Date().toISOString().replace(/[:.]/g, '-').split('T')[0] + '_' + Date.now()
  const extension = originalName.split('.').pop()
  return `${timestamp}.${extension}`
}

const getDisplayFileName = (value?: string | null): string => {
  if (!value) return ''

  const cleanedValue = value.split('?')[0]
  const fileName = cleanedValue.split('/').pop()

  return fileName || value
}

const getFileUrl = (value?: string | null): string => {
  if (!value) return ''

  if (value.startsWith('data:')) {
    return value
  }

  if (value.startsWith('/uploads/')) {
    return `${API_BASE_URL}${value}`
  }

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

const getApiErrorMessage = (data: any, fallback = 'Request failed'): string => {
  if (typeof data === 'string' && data.trim()) {
    return data
  }

  if (data && typeof data === 'object') {
    if (typeof data.error === 'string' && data.error.trim()) {
      return data.error
    }
    if (typeof data.message === 'string' && data.message.trim()) {
      return data.message
    }
    try {
      return JSON.stringify(data)
    } catch {
      return fallback
    }
  }

  return fallback
}

const handleAuthFailure = async () => {
  removeToken()
  await navigateTo('/login')
}

const fetchProtectedFileBlob = async (url: string): Promise<Blob> => {
  const token = getToken()
  const headers: Record<string, string> = {}
  if (token) {
    headers.Authorization = `Bearer ${token}`
  }

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

const isStoredFileValue = (value?: string | null): boolean => {
  if (!value) return false

  return value.startsWith('/uploads/') || value.startsWith('data:')
}

const viewFile = async (value?: string | null) => {
  const url = getViewFileUrl(value)
  if (!url || !process.client) return

  if (url.startsWith('data:')) {
    window.open(url, '_blank')
    return
  }

  const newTab = window.open('about:blank', '_blank')
  if (!newTab) {
    message.value = 'Unable to open a new tab. Please allow pop-ups for this site.'
    return
  }

  newTab.document.write('<title>Loading file...</title><p style="font-family: sans-serif; padding: 16px;">Loading file...</p>')

  try {
    const blob = await fetchProtectedFileBlob(url)
    const blobUrl = window.URL.createObjectURL(blob)
    newTab.location.href = blobUrl

    setTimeout(() => {
      window.URL.revokeObjectURL(blobUrl)
    }, 60_000)
  } catch (error) {
    console.error('Error viewing file:', error)
    newTab.close()
    message.value = error instanceof Error ? error.message : 'Failed to open file. Please try again.'
  }
}

const syncExistingFileState = (data: Employee) => {
  idProof1.value.file = null
  idProof1.value.preview = null
  idProof1.value.fileName = isStoredFileValue(data.id_proof_1) ? getDisplayFileName(data.id_proof_1) : ''

  idProof2.value.file = null
  idProof2.value.preview = null
  idProof2.value.fileName = isStoredFileValue(data.id_proof_2) ? getDisplayFileName(data.id_proof_2) : ''

  profileImage.value.file = null
  profileImage.value.fileName = isStoredFileValue(data.profile_image) ? getDisplayFileName(data.profile_image) : ''
  profileImage.value.preview = null
}

const downloadFile = async (value?: string | null) => {
  const url = getDownloadFileUrl(value)
  if (!url || !process.client) return

  const fileName = getDisplayFileName(value) || 'download'

  try {
    if (url.startsWith('data:')) {
      const link = document.createElement('a')
      link.href = url
      link.download = fileName
      link.rel = 'noopener noreferrer'
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
    link.rel = 'noopener noreferrer'
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)

    window.URL.revokeObjectURL(blobUrl)
  } catch (error) {
    console.error('Error downloading file:', error)
    message.value = error instanceof Error ? error.message : 'Failed to download file. Please try again.'
  }
}

const handleFileUpload = (event: Event, fileState: FileUpload, fileType: 'pdf' | 'image') => {
  const input = event.target as HTMLInputElement
  const file = input.files?.[0]

  if (!file) return

  // Validate file type
  if (fileType === 'pdf' && file.type !== 'application/pdf') {
    message.value = '⚠️ Please upload a valid PDF file'
    return
  }

  if (fileType === 'image' && !file.type.startsWith('image/')) {
    message.value = '⚠️ Please upload a valid image file'
    return
  }

  // Validate file size (5MB max)
  const maxSize = 5 * 1024 * 1024
  if (file.size > maxSize) {
    message.value = '⚠️ File size must be less than 5MB'
    return
  }

  fileState.file = file
  fileState.fileName = generateTimestampFileName(file.name)

  // Create preview for images
  if (fileType === 'image') {
    const reader = new FileReader()
    reader.onload = (e) => {
      fileState.preview = e.target?.result as string
    }
    reader.readAsDataURL(file)
  } else {
    // For PDFs, show a document icon indicator
    fileState.preview = 'pdf'
  }

  message.value = ''
}

const convertFileToBase64 = (file: File): Promise<string> => {
  return new Promise((resolve, reject) => {
    const reader = new FileReader()
    reader.onload = () => {
      resolve(reader.result as string)
    }
    reader.onerror = reject
    reader.readAsDataURL(file)
  })
}

onMounted(async () => {
  try {
    const { get } = useApi()
    const data = await get(`/employees/${route.params.id}`)
    employee.value = data
    syncExistingFileState(data)
  } catch (error) {
    console.error('Error loading employee:', error)
    message.value = 'Failed to load employee data'
  } finally {
    isLoading.value = false
  }
})

const updateEmployee = async () => {
  isSubmitting.value = true
  message.value = ''

  try {
    // Prepare employee data with proper type conversion
    const employeeData: Record<string, any> = {
      first_name: employee.value.first_name,
      last_name: employee.value.last_name,
      email: employee.value.email,
      department: employee.value.department,
      salary: parseFloat(String(employee.value.salary)),
      mobile_number: employee.value.mobile_number || '',
      address: employee.value.address || '',
      username: employee.value.username || '',
      password: employee.value.password || ''
    }

    // Send the selected file, or keep an already-stored file path, but never send legacy labels like Aadhaar/PAN
    if (idProof1.value.file) {
      employeeData.id_proof_1 = await convertFileToBase64(idProof1.value.file)
    } else if (isStoredFileValue(employee.value.id_proof_1)) {
      employeeData.id_proof_1 = employee.value.id_proof_1
    }
    if (!employeeData.id_proof_1) {
      delete employeeData.id_proof_1
    }
    if (idProof2.value.file) {
      employeeData.id_proof_2 = await convertFileToBase64(idProof2.value.file)
    } else if (isStoredFileValue(employee.value.id_proof_2)) {
      employeeData.id_proof_2 = employee.value.id_proof_2
    }
    if (!employeeData.id_proof_2) {
      delete employeeData.id_proof_2
    }
    if (profileImage.value.file) {
      employeeData.profile_image = await convertFileToBase64(profileImage.value.file)
    } else if (isStoredFileValue(employee.value.profile_image)) {
      employeeData.profile_image = employee.value.profile_image
    }
    if (!employeeData.profile_image) {
      delete employeeData.profile_image
    }

    const { put } = useApi()
    const response = await put(`/employees/${route.params.id}`, employeeData)
    console.log('Update employee API response:', response)

    if (response.ok) {
      message.value = `Employee ${employee.value.first_name} ${employee.value.last_name} updated successfully! Redirecting...`
      setTimeout(() => {
        router.push('/employees')
      }, 2000)
    } else {
      const errorMessage = getApiErrorMessage(response.data, `Request failed (${response.status})`)
      console.error('Update employee failed:', {
        status: response.status,
        data: response.data,
        payload: employeeData
      })
      message.value = `Failed: ${errorMessage}`
    }
  } catch (error) {
    console.error('Error updating employee:', error)
    message.value = 'Failed to update employee. Please try again.'
  } finally {
    isSubmitting.value = false
  }
}

const handleCancel = () => {
  router.push('/employees')
}
</script>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>