import { Navigate, Outlet } from 'react-router-dom'
import { useAuth } from '../context/AuthContext'

/**
 * Envuelve rutas que requieren sesión activa.
 * Si el usuario no está autenticado → redirige a /login.
 * Si está autenticado → renderiza el contenido (Outlet).
 */
export default function ProtectedRoute() {
  const { isAuthenticated } = useAuth()

  if (!isAuthenticated) {
    return <Navigate to="/login" replace />
  }

  return <Outlet />
}
