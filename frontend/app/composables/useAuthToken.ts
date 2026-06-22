// JWT token storage and management composable
export const useAuthToken = () => {
  const getToken = (): string | null => {
    if (process.client) {
      return localStorage.getItem('auth_token')
    }
    return null
  }

  const setToken = (token: string) => {
    if (process.client) {
      localStorage.setItem('auth_token', token)
    }
  }

  const removeToken = () => {
    if (process.client) {
      localStorage.removeItem('auth_token')
      localStorage.removeItem('username')
      localStorage.removeItem('role')
      localStorage.removeItem('token_expiry')
    }
  }

  const setUserInfo = (username: string, role: string, expiry: string) => {
    if (process.client) {
      localStorage.setItem('username', username)
      localStorage.setItem('role', role)
      localStorage.setItem('token_expiry', expiry)
    }
  }

  const getUserInfo = () => {
    if (process.client) {
      return {
        username: localStorage.getItem('username'),
        role: localStorage.getItem('role'),
        expiry: localStorage.getItem('token_expiry')
      }
    }
    return { username: null, role: null, expiry: null }
  }

  const isTokenExpired = (): boolean => {
    if (!process.client) return true
    
    const expiry = localStorage.getItem('token_expiry')
    if (!expiry) return true
    
    try {
      const expiryTime = new Date(expiry).getTime()
      const now = new Date().getTime()
      return now >= expiryTime
    } catch (e) {
      return true
    }
  }

  const isAuthenticated = (): boolean => {
    const token = getToken()
    return !!token && !isTokenExpired()
  }

  return {
    getToken,
    setToken,
    removeToken,
    setUserInfo,
    getUserInfo,
    isTokenExpired,
    isAuthenticated
  }
}
