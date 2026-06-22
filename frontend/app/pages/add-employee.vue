<template>
  <div class="min-h-screen bg-gradient-to-br from-slate-50 to-slate-100">
    <!-- Header Navigation -->
    <header class="bg-white shadow-sm border-b border-slate-200">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4 flex items-center justify-between">
        <div class="flex items-center space-x-3">
          <div class="w-10 h-10 bg-gradient-to-br from-blue-600 to-blue-700 rounded-lg flex items-center justify-center">
            <span class="text-white font-bold text-lg">EM</span>
          </div>
          <h1 class="text-2xl font-bold text-slate-900">Add New Employee</h1>
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
        <form @submit.prevent="addEmployee" class="space-y-6">
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
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
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
              <div>
                <label class="block text-sm font-semibold text-slate-700 mb-2">Mobile Number *</label>
                <input
                  v-model="employee.mobile_number"
                  type="tel"
                  placeholder="Enter mobile number"
                  class="input-field"
                  required
                />
              </div>
            </div>
            <div class="mt-6">
              <label class="block text-sm font-semibold text-slate-700 mb-2">Address *</label>
              <textarea
                v-model="employee.address"
                placeholder="Enter full address"
                rows="3"
                class="w-full px-4 py-3 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200"
                required
              ></textarea>
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
                <label class="block text-sm font-semibold text-slate-700 mb-2">Username *</label>
                <input
                  v-model="employee.username"
                  type="text"
                  placeholder="Enter username"
                  class="input-field"
                  required
                />
              </div>
              <div>
                <label class="block text-sm font-semibold text-slate-700 mb-2">Password *</label>
                <input
                  v-model="employee.password"
                  type="password"
                  placeholder="Enter password"
                  class="input-field"
                  required
                />
              </div>
            </div>
          </div>

          <!-- File Uploads Section -->
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
              class="flex-1 px-6 py-3 bg-green-600 hover:bg-green-700 text-white font-semibold rounded-lg transition-all duration-200 shadow-md hover:shadow-lg disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center space-x-2"
            >
              <span v-if="!isSubmitting">Add Employee</span>
              <span v-else>Adding...</span>
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
import { ref } from 'vue'
import { navigateTo } from '#app'
import { useApi } from '~/composables/useApi'

interface Employee {
  first_name: string
  last_name: string
  email: string
  department: string
  salary: number
  mobile_number: string
  address: string
  username: string
  password: string
  id_proof_1: string
  id_proof_2: string
  profile_image: string
}

interface FileUpload {
  file: File | null
  preview: string | null
  fileName: string
  uploading: boolean
}

const employee = ref<Employee>({
  first_name: '',
  last_name: '',
  email: '',
  department: '',
  salary: 0,
  mobile_number: '',
  address: '',
  username: '',
  password: '',
  id_proof_1: '',
  id_proof_2: '',
  profile_image: ''
})

const message = ref('')
const isSubmitting = ref(false)

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

const handleFileUpload = (event: Event, fileRef: any, fileType: 'pdf' | 'image') => {
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

  fileRef.value.file = file
  fileRef.value.fileName = generateTimestampFileName(file.name)

  // Create preview for images
  if (fileType === 'image') {
    const reader = new FileReader()
    reader.onload = (e) => {
      fileRef.value.preview = e.target?.result as string
    }
    reader.readAsDataURL(file)
  } else {
    // For PDFs, show a document icon indicator
    fileRef.value.preview = 'pdf'
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

const addEmployee = async () => {
  isSubmitting.value = true
  message.value = ''

  try {
    // Prepare employee data with proper type conversion
    const employeeData = {
      ...employee.value,
      salary: parseFloat(String(employee.value.salary)) // Ensure salary is a number
    }

    // Convert files to base64 if they exist
    if (idProof1.value.file) {
      employeeData.id_proof_1 = await convertFileToBase64(idProof1.value.file)
    }
    if (idProof2.value.file) {
      employeeData.id_proof_2 = await convertFileToBase64(idProof2.value.file)
    }
    if (profileImage.value.file) {
      employeeData.profile_image = await convertFileToBase64(profileImage.value.file)
    }

    const { post } = useApi()
    const response = await post('/employees', employeeData)

    message.value = response.data

    if (response.ok) {
        const firstName = employee.value.first_name
        const lastName = employee.value.last_name
      message.value = `Employee ${firstName} ${lastName} added successfully! Redirecting...`
      setTimeout(() => {
        navigateTo('/employees')
        }, 1500)
    } else {
      message.value = `Failed: ${response.data}`
    }
  } catch (error) {
    console.error('Error adding employee:', error)
    message.value = 'Failed to add employee. Please try again.'
  } finally {
    isSubmitting.value = false
  }
}

const resetForm = () => {
  employee.value = {
    first_name: '',
    last_name: '',
    email: '',
    department: '',
    salary: 0,
    mobile_number: '',
    address: '',
    username: '',
    password: '',
    id_proof_1: '',
    id_proof_2: '',
    profile_image: ''
  }
  message.value = ''
}

const handleCancel = async () => {
  await navigateTo('/employees')
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