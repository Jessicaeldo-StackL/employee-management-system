export const useAuthGuard = (requiredRole?: 'admin' | 'employee') => {
  const { isAuthenticated, removeToken, getUserInfo } = useAuthToken()

  const redirectIfUnauthorized = () => {
    if (!process.client) return

    if (!isAuthenticated()) {
      removeToken()
      navigateTo('/login')
      return
    }

    const { role } = getUserInfo()

    // Optional role check
    if (requiredRole && role !== requiredRole) {
      if (role === 'admin') {
        navigateTo('/admin')
      } else if (role === 'employee') {
        navigateTo('/employee')
      }
    }
  }

  return {
    redirectIfUnauthorized,
    isAuthenticated
  }
}