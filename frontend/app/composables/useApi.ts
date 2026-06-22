// Centralized API configuration with JWT authentication
export const useApi = () => {
  const config = useRuntimeConfig()
  const API_BASE_URL =
    config.public.apiBaseUrl ||
    process.env.NUXT_PUBLIC_API_BASE_URL ||
    'http://127.0.0.1:8081'

  const { getToken, removeToken } = useAuthToken()

  // Build headers with JWT if available
  const authHeaders = (extra: Record<string, string> = {}): Record<string, string> => {
    const headers: Record<string, string> = { 'Content-Type': 'application/json', ...extra }
    const token = getToken()
    if (token) headers['Authorization'] = `Bearer ${token}`
    return headers
  }

  // Handle 401/403 responses
  const handleAuthError = () => {
    if (process.client) {
      removeToken()
      navigateTo('/login')
    }
  }

  const get = async (endpoint: string) => {
    const response = await fetch(`${API_BASE_URL}${endpoint}`, {
      headers: authHeaders()
    })
    
    if (response.status === 401 || response.status === 403) {
      handleAuthError()
      throw new Error('Unauthorized')
    }
    
    if (!response.ok) {
      const errorData = await response.json().catch(() => ({ error: 'Request failed' }))
      throw new Error(errorData.error || `Request failed: ${response.status}`)
    }
    return response.json()
  }

  const post = async (endpoint: string, data: any) => {
    const isLogin = endpoint === '/login'
    const headers: Record<string, string> = { 'Content-Type': 'application/json' }
    if (!isLogin) {
      const token = getToken()
      if (token) headers['Authorization'] = `Bearer ${token}`
    }
    
    const response = await fetch(`${API_BASE_URL}${endpoint}`, {
      method: 'POST',
      headers,
      body: JSON.stringify(data)
    })
    
    const responseText = await response.text()
    let responseData: any
    try { 
      responseData = JSON.parse(responseText) 
    } catch { 
      responseData = responseText 
    }
    
    if ((response.status === 401 || response.status === 403) && !isLogin) {
      handleAuthError()
    }
    
    return { ok: response.ok, status: response.status, data: responseData }
  }

  const put = async (endpoint: string, data: any) => {
    const response = await fetch(`${API_BASE_URL}${endpoint}`, {
      method: 'PUT',
      headers: authHeaders(),
      body: JSON.stringify(data)
    })
    
    const responseText = await response.text()
    let responseData: any
    try { 
      responseData = JSON.parse(responseText) 
    } catch { 
      responseData = responseText 
    }
    
    if (response.status === 401 || response.status === 403) {
      handleAuthError()
    }
    
    return { ok: response.ok, status: response.status, data: responseData }
  }

  const delete_ = async (endpoint: string) => {
    const response = await fetch(`${API_BASE_URL}${endpoint}`, {
      method: 'DELETE',
      headers: authHeaders()
    })
    
    const responseText = await response.text()
    let responseData: any
    try { 
      responseData = JSON.parse(responseText) 
    } catch { 
      responseData = responseText 
    }
    
    if (response.status === 401 || response.status === 403) {
      handleAuthError()
    }
    
    return { ok: response.ok, status: response.status, data: responseData }
  }

  return { API_BASE_URL, get, post, put, delete: delete_ }
}
