# Frontend JWT Integration Guide

## ✅ What's Been Integrated

Your Nuxt 3 frontend now has complete JWT authentication support:

- ✅ **JWT Token Storage** - Secure localStorage with encryption-ready design
- ✅ **Token Management** - Composable for getting/setting/removing tokens
- ✅ **API Client Integration** - Automatic Bearer token injection in all requests
- ✅ **Auth Guards** - Protect routes and validate token expiry
- ✅ **Login Handler** - Captures JWT from backend and stores it
- ✅ **401 Handling** - Auto-logout and redirect on unauthorized responses

## 📁 New Files Created

### 1. `app/composables/useAuthToken.ts`
**Purpose**: JWT token storage and management

**Exports**:
```typescript
useAuthToken() → {
  getToken()              // Get JWT from localStorage
  setToken(token)         // Save JWT to localStorage
  removeToken()           // Clear all auth data from localStorage
  setUserInfo(...)        // Save username, role, expiry
  getUserInfo()           // Retrieve stored user information
  isTokenExpired()        // Check if token has expired
  isAuthenticated()       // Check if user has valid token
}
```

**Usage**:
```typescript
const { getToken, isAuthenticated, removeToken } = useAuthToken()

if (isAuthenticated()) {
  // User has valid token
  const token = getToken()
} else {
  // Token expired or missing
  removeToken()
}
```

## 📝 Updated Files

### 1. `app/composables/useApi.ts`
**Changes**:
- Integrated `useAuthToken()` for token management
- Added `handleAuthError()` to centralize 401/403 handling
- All HTTP methods (GET, POST, PUT, DELETE) now include Bearer token
- Auto-logout on unauthorized responses

**Token Injection**:
```typescript
const authHeaders = (): Record<string, string> => {
  const headers = { 'Content-Type': 'application/json' }
  const token = getToken()
  if (token) headers['Authorization'] = `Bearer ${token}`
  return headers
}
```

### 2. `app/composables/useAuthGuard.ts`
**Changes**:
- Now checks token validity with `isTokenExpired()`
- Auto-clears expired tokens
- Better role-based route protection

**Usage**:
```typescript
export default definePageMeta({
  middleware: [(to, from) => {
    const { redirectIfUnauthorized } = useAuthGuard('admin')
    redirectIfUnauthorized()
  }]
})
```

### 3. `app/pages/login.vue`
**Changes**:
- Captures JWT token from login response
- Stores token with `setToken(data.token)`
- Stores user info with `setUserInfo(username, role, expiry)`
- Logs token storage for debugging

**Login Flow**:
```
User enters credentials
       ↓
Backend validates & returns JWT
       ↓
Frontend stores token in localStorage
       ↓
Token injected in all subsequent requests
       ↓
Backend validates token and serves protected resources
```

## 🔐 Local Storage Structure

```javascript
localStorage = {
  'auth_token': 'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...',
  'username': 'admin',
  'role': 'admin',
  'token_expiry': '2026-06-23T14:30:45.123Z',
  'rememberMe': 'true'  // optional
}
```

## 🧪 Testing Frontend JWT

### Test 1: Login and Check Storage
```javascript
// Open browser console (F12)
// Log in with: admin / admin123

// Then check:
console.log(localStorage.getItem('auth_token'))
// Should show: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...

console.log(localStorage.getItem('username'))
// Should show: admin

console.log(localStorage.getItem('role'))
// Should show: admin
```

### Test 2: Verify Token in Requests
```javascript
// Open Network tab (F12 → Network)
// Navigate to /employees or /admin
// Click on any API request
// Check Headers → Authorization
// Should see: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...
```

### Test 3: Decode Token
```javascript
// Visit https://jwt.io
// Paste token from localStorage.getItem('auth_token')
// Verify payload contains:
// {
//   "sub": "admin",
//   "role": "admin",
//   "exp": 1782194414,
//   "iat": 1782108014
// }
```

### Test 4: Check Token Expiry Handling
```javascript
// In browser console:
const { isTokenExpired } = useAuthToken()
console.log(isTokenExpired())
// Should show: false (token still valid)

// Manually set expiry to past time to test:
localStorage.setItem('token_expiry', '2020-01-01T00:00:00Z')
console.log(isTokenExpired())
// Should show: true (token expired)
```

## 🔄 Authentication Flow

```
┌─────────────────────────────────────────────────────────────┐
│ 1. USER VISITS LOGIN PAGE                                   │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│ 2. ENTERS CREDENTIALS                                       │
│    - username: admin                                         │
│    - password: admin123                                      │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│ 3. LOGIN FORM SUBMITS TO POST /login                        │
│    (useApi.post() without token - login endpoint)           │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│ 4. BACKEND VALIDATES & RETURNS JWT                          │
│    {                                                         │
│      "success": true,                                        │
│      "token": "eyJ0eXA...",                                  │
│      "username": "admin",                                    │
│      "role": "admin"                                         │
│    }                                                         │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│ 5. FRONTEND STORES TOKEN                                    │
│    - localStorage.auth_token = JWT                          │
│    - localStorage.username = admin                          │
│    - localStorage.role = admin                              │
│    - localStorage.token_expiry = 2026-06-23T...             │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│ 6. USER NAVIGATES TO PROTECTED PAGE                         │
│    e.g., /admin or /employees                               │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│ 7. AUTH GUARD RUNS                                          │
│    - Checks useAuthGuard()                                  │
│    - Verifies token not expired                             │
│    - Checks role matches required role                      │
│    - Allows or redirects                                    │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│ 8. API CALL INCLUDES TOKEN                                  │
│    GET /employees                                           │
│    Headers: Authorization: Bearer eyJ0eXA...                │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│ 9. BACKEND VALIDATES TOKEN                                  │
│    - Checks signature                                        │
│    - Verifies expiration                                    │
│    - Extracts user info from token                          │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│ 10. BACKEND RETURNS PROTECTED RESOURCE                      │
│     e.g., list of employees                                 │
└─────────────────────────────────────────────────────────────┘
```

## ⚠️ Error Handling

### Case 1: Token Expired
```
User tries to make request
Token is expired (from localStorage)
useAuthGuard detects expiry
Calls removeToken() to clear storage
Redirects to /login
```

### Case 2: Invalid Token
```
Backend returns 401 Unauthorized
useApi detects 401 response
Calls handleAuthError()
Clears localStorage
Redirects to /login
```

### Case 3: Insufficient Permissions
```
User has token but wrong role
Backend returns 403 Forbidden
useApi detects 403 response
Calls handleAuthError()
Redirects to /login
```

## 🛡️ Security Considerations

### ✅ What's Implemented
- Bearer token in Authorization header (not query parameter)
- Token stored in localStorage (accessible only to same-origin JavaScript)
- Automatic 401 handling with logout
- Token expiry validation on frontend
- Role-based access control checks

### ⚠️ Additional Recommendations
1. **HTTPS Only** - Never send tokens over HTTP
2. **Secure Cookies** - Consider storing token in httpOnly cookie instead of localStorage
3. **CSRF Protection** - Add CSRF tokens for state-changing operations
4. **Token Refresh** - Implement token refresh endpoints for long sessions
5. **Content Security Policy** - Set strict CSP headers
6. **Rate Limiting** - Implement on backend login endpoint

## 🔧 Customization

### Change Token Expiry
In `useAuthToken.ts`, modify token storage:
```typescript
// Store custom expiry time
const setToken = (token: string, expiryMinutes = 1440) => {
  const expiry = new Date(Date.now() + expiryMinutes * 60 * 1000).toISOString()
  localStorage.setItem('token_expiry', expiry)
  localStorage.setItem('auth_token', token)
}
```

### Add Token Refresh
```typescript
const refreshToken = async () => {
  const { post } = useApi()
  const result = await post('/auth/refresh', {})
  if (result.ok && result.data.token) {
    setToken(result.data.token)
    return true
  }
  return false
}
```

### Add Logout Button
```vue
<template>
  <button @click="logout" class="btn-logout">Logout</button>
</template>

<script setup>
const logout = () => {
  const { removeToken } = useAuthToken()
  removeToken()
  navigateTo('/login')
}
</script>
```

## 📋 Verification Checklist

After implementation:

- [ ] Login stores JWT token in localStorage
- [ ] Token appears in Authorization header for API calls
- [ ] Network inspector shows `Authorization: Bearer <token>`
- [ ] Logout clears token from localStorage
- [ ] Visiting /login without token redirects correctly
- [ ] Token expiry check works (test with past date)
- [ ] 401 responses auto-redirect to login
- [ ] Different roles redirect to correct pages
- [ ] Token visible in https://jwt.io

## 🐛 Troubleshooting

### Token Not Appearing in Headers
```javascript
// Check 1: Is token stored?
console.log(localStorage.getItem('auth_token'))

// Check 2: Is API method adding Bearer?
const { getToken } = useAuthToken()
console.log('Token:', getToken())
```

### Redirect Loop
```javascript
// Check 1: Is token expired?
const { isTokenExpired } = useAuthToken()
console.log('Expired:', isTokenExpired())

// Check 2: Is role correct?
console.log('Role:', localStorage.getItem('role'))
```

### Login Not Working
```javascript
// Check backend response:
// 1. Open Network tab (F12)
// 2. Attempt login
// 3. Click POST /login
// 4. View Response tab
// Should have: success, token, username, role
```

## 📚 Related Files

- Backend JWT Config: `backend/.env`
- JWT Implementation: `JWT_IMPLEMENTATION.md`
- Quick Start: `QUICK_START_JWT.md`
- Test Guide: `backend/JWT_TEST_GUIDE.md`

---

**Status**: ✅ Frontend JWT integration complete and ready to test

**Next Steps**:
1. Test login with admin / admin123
2. Check localStorage for token
3. Verify token in Network requests
4. Test protected routes
5. Test logout functionality
