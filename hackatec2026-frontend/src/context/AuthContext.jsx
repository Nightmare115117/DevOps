import { createContext, useContext, useState } from 'react'

const AuthContext = createContext(null)

/**
 * Provee el estado de sesión a toda la app.
 * - token se guarda en localStorage para que persista al recargar.
 * - Reemplaza las funciones login/logout con tu lógica real de API.
 */
export function AuthProvider({ children }) {
  const [token, setToken] = useState(() => localStorage.getItem('auth_token'))

  /** Llama a esto cuando el usuario se autentique exitosamente */
  const login = (receivedToken) => {
    localStorage.setItem('auth_token', receivedToken)
    setToken(receivedToken)
  }

  /** Llama a esto en el botón de cerrar sesión */
  const logout = () => {
    localStorage.removeItem('auth_token')
    setToken(null)
  }

  const isAuthenticated = !!token

  return (
    <AuthContext.Provider value={{ token, isAuthenticated, login, logout }}>
      {children}
    </AuthContext.Provider>
  )
}

/** Hook para consumir el contexto fácilmente en cualquier componente */
export function useAuth() {
  const ctx = useContext(AuthContext)
  if (!ctx) throw new Error('useAuth debe usarse dentro de <AuthProvider>')
  return ctx
}
