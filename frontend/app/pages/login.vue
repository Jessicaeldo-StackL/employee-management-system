<template>
  <div class="page-shell min-h-screen flex items-center py-4 sm:py-6 lg:py-8">
    <div class="mx-auto w-full max-w-6xl">
      <div class="grid gap-6 lg:grid-cols-[0.88fr_1.12fr] xl:gap-8">
        <section class="hero-panel-dark flex flex-col justify-start p-6 md:p-7 lg:p-8">
          <div class="space-y-6">
            <div class="flex items-center gap-3">
              <div class="brand-mark">EM</div>
              <div>
                <p class="text-xs font-bold uppercase tracking-[0.24em] text-blue-100">Enterprise HRMS</p>
                <h1 class="mt-1 text-xl font-semibold text-white">Employee Management System</h1>
              </div>
            </div>

            <p class="max-w-md text-sm leading-7 text-slate-200 md:text-base">
              Manage employees, leave requests, and approvals from a single secure platform.
            </p>
          </div>

          <div class="mt-6 rounded-2xl border border-white/15 bg-white/5 p-5">
            <p class="text-xs font-semibold uppercase tracking-[0.18em] text-blue-100">Core Features</p>
            <div class="mt-4 space-y-3">
              <div class="flex items-center gap-3 rounded-xl border border-white/10 bg-slate-900/30 px-3 py-2.5">
                <div class="icon-chip h-8 w-8 rounded-lg bg-white/10 text-white">
                  <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
                    <path d="M4 6h16" stroke-linecap="round" />
                    <path d="M4 12h16" stroke-linecap="round" />
                    <path d="M4 18h10" stroke-linecap="round" />
                  </svg>
                </div>
                <p class="text-sm font-medium text-slate-100">Employee Records</p>
              </div>

              <div class="flex items-center gap-3 rounded-xl border border-white/10 bg-slate-900/30 px-3 py-2.5">
                <div class="icon-chip h-8 w-8 rounded-lg bg-white/10 text-white">
                  <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
                    <rect x="4" y="5" width="16" height="15" rx="2" />
                    <path d="M8 3v4M16 3v4M4 10h16" stroke-linecap="round" stroke-linejoin="round" />
                  </svg>
                </div>
                <p class="text-sm font-medium text-slate-100">Leave Management</p>
              </div>

              <div class="flex items-center gap-3 rounded-xl border border-white/10 bg-slate-900/30 px-3 py-2.5">
                <div class="icon-chip h-8 w-8 rounded-lg bg-white/10 text-white">
                  <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
                    <path d="M7 11V8a5 5 0 1110 0v3" stroke-linecap="round" stroke-linejoin="round" />
                    <rect x="5" y="11" width="14" height="9" rx="2" />
                  </svg>
                </div>
                <p class="text-sm font-medium text-slate-100">Secure JWT Authentication</p>
              </div>
            </div>
          </div>
        </section>

        <section class="surface-card p-6 md:p-8 lg:p-10">
          <div class="mb-8 text-center lg:text-left">
            <div class="mx-auto mb-4 flex h-14 w-14 items-center justify-center rounded-2xl bg-blue-50 text-blue-700 lg:mx-0">
              <svg class="h-7 w-7" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
                <path d="M7 10V7a5 5 0 1110 0v3" stroke-linecap="round" stroke-linejoin="round" />
                <rect x="5" y="10" width="14" height="10" rx="2" />
                <path d="M12 14v2" stroke-linecap="round" />
              </svg>
            </div>
            <p class="eyebrow">Sign in</p>
            <h2 class="mt-2 text-3xl font-extrabold tracking-tight text-slate-950">Welcome back</h2>
            <p class="mt-2 text-sm leading-6 text-slate-600">Use your employee or admin account to continue.</p>
          </div>

          <form @submit.prevent="handleLogin" class="space-y-5">
            <div class="space-y-2">
              <label for="username" class="field-label">Username</label>
              <input
                id="username"
                v-model="form.username"
                type="text"
                placeholder="Enter your username"
                class="input-field"
                :disabled="isLoading"
              />
              <p v-if="errors.username" class="error-message">{{ errors.username }}</p>
            </div>

            <div class="space-y-2">
              <div class="flex items-center justify-between gap-4">
                <label for="password" class="field-label mb-0">Password</label>
                <button type="button" class="btn-link text-xs" @click="showPassword = !showPassword">
                  {{ showPassword ? 'Hide password' : 'Show password' }}
                </button>
              </div>
              <input
                id="password"
                v-model="form.password"
                :type="showPassword ? 'text' : 'password'"
                placeholder="Enter your password"
                class="input-field"
                :disabled="isLoading"
              />
              <p v-if="errors.password" class="error-message">{{ errors.password }}</p>
            </div>

            <div class="flex items-center">
              <label class="flex items-center gap-2 text-sm text-slate-600">
                <input
                  v-model="form.rememberMe"
                  type="checkbox"
                  class="h-4 w-4 rounded border-slate-300 text-blue-600 focus:ring-blue-500"
                  :disabled="isLoading"
                />
                Remember this device
              </label>
            </div>

            <transition name="fade">
              <div v-if="message.type === 'error'" class="rounded-2xl border border-rose-200 bg-rose-50 p-4">
                <p class="text-sm font-medium text-rose-800">{{ message.text }}</p>
              </div>
            </transition>

            <transition name="fade">
              <div v-if="message.type === 'success'" class="rounded-2xl border border-emerald-200 bg-emerald-50 p-4">
                <p class="text-sm font-medium text-emerald-800">{{ message.text }}</p>
              </div>
            </transition>

            <button
              type="submit"
              :disabled="isLoading || !form.username || !form.password"
              :class="[
                'btn-primary w-full py-3.5',
                isLoading || !form.username || !form.password
                  ? 'opacity-60 shadow-[0_10px_20px_rgba(37,99,235,0.16)]'
                  : 'shadow-[0_14px_28px_rgba(37,99,235,0.3)] hover:-translate-y-0.5 hover:shadow-[0_18px_32px_rgba(37,99,235,0.35)]'
              ]"
            >
              <svg v-if="!isLoading" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
                <path d="M15 3h4a2 2 0 012 2v4" stroke-linecap="round" stroke-linejoin="round" />
                <path d="M10 14L21 3" stroke-linecap="round" stroke-linejoin="round" />
                <path d="M21 14v5a2 2 0 01-2 2h-5" stroke-linecap="round" stroke-linejoin="round" />
                <path d="M3 10v11h11" stroke-linecap="round" stroke-linejoin="round" />
              </svg>
              <svg v-else class="h-4 w-4 animate-spin" viewBox="0 0 24 24" fill="none">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
              </svg>
              <span>{{ isLoading ? 'Signing in...' : 'Sign in securely' }}</span>
            </button>
          </form>

          <div class="mt-8 border-t border-slate-200 pt-6 text-center">
            <p class="text-sm text-slate-500">
              Secure access for authorized employees and administrators.
            </p>
          </div>
        </section>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { navigateTo } from '#app'

interface LoginForm {
  username: string
  password: string
  rememberMe: boolean
}

interface ErrorMessages {
  username?: string
  password?: string
}

interface Message {
  type: 'error' | 'success' | null
  text: string
}

const form = reactive<LoginForm>({
  username: '',
  password: '',
  rememberMe: false
})

const errors = reactive<ErrorMessages>({})
const isLoading = ref(false)
const showPassword = ref(false)
const message = reactive<Message>({
  type: null,
  text: ''
})

const validateForm = (): boolean => {
  errors.username = undefined
  errors.password = undefined

  if (!form.username.trim()) {
    errors.username = 'Username is required'
  }

  if (!form.password.trim()) {
    errors.password = 'Password is required'
  }

  if (form.password.length < 3) {
    errors.password = 'Password must be at least 3 characters'
  }
return !errors.username && !errors.password
}

const handleLogin = async () => {
  if (!validateForm()) {
    message.type = 'error'
    message.text = 'Please correct the errors above'
    return
  }

  isLoading.value = true
  message.type = null
  message.text = ''

  try {
    const { post } = useApi()
    const { setToken, setUserInfo } = useAuthToken()
    
    const loginData = await post('/login', {
      username: form.username.trim(),
      password: form.password
    })
    const response = { ok: loginData.ok, status: loginData.status }
    const data = typeof loginData.data === 'string' ? JSON.parse(loginData.data) : loginData.data

    if (data.success && data.token) {
      message.type = 'success'
      message.text = `Welcome ${data.username}! Redirecting...`

      // Store JWT token and user data
      setToken(data.token)
      console.log("TOKEN RECEIVED:", data.token)
console.log("TOKEN IN STORAGE:", localStorage.getItem("auth_token"))
      
      // Calculate token expiry (24 hours from now by default)
      const expiryTime = new Date(Date.now() + 24 * 60 * 60 * 1000).toISOString()
      setUserInfo(data.username, data.role, expiryTime)
      
      if (form.rememberMe) {
        localStorage.setItem('rememberMe', 'true')
      }

      // Redirect based on role
      await new Promise(resolve => setTimeout(resolve, 800))

      console.log("Login Response:", data)
      console.log("Username:", data.username)
      console.log("Role:", data.role)
      console.log("Token stored successfully")

      const role = String(data.role).toLowerCase().trim()

      console.log("Processed Role:", role)

      if (role === 'admin') {
        console.log("Redirecting to admin")
        window.location.href = '/admin'
      } else if (role === 'employee') {
        console.log("Redirecting to employee")
        window.location.href = '/employee'
      } else {
        console.log("Unknown role:", role)
        window.location.href = '/employees'
      }
    } else {
      message.type = 'error'
      message.text = data.message || 'Login failed. Please try again.'
    }
  } catch (error) {
    console.error('Login error:', error)
    message.type = 'error'
    message.text = 'Server connection failed. Please check your connection and try again.'
  } finally {
    isLoading.value = false
  }
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

input:disabled {
  @apply bg-slate-50 text-slate-400 cursor-not-allowed;
}
</style>