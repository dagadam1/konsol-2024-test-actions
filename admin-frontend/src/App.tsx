import { Routes, Route, Link } from 'react-router-dom'
import Header from './components/Header'
import NavHeader from './components/NavHeader'
import SlidesPage from './pages/SlidesPage'
import './styles/App.css'
import UsersPage from './pages/UsersPage'
import { useState } from 'react'
import { User } from './types'

function App() {
  const [user, setUser] = useState<User | null>(null);

  return (
    <div className="app-container">
      <Header user={user} setUser={setUser} />
      <NavHeader />


      <Routes>
        <Route path="/" element={<SlidesPage />} />
        <Route path="/slides" element={<SlidesPage />} />
        <Route path="/users" element={<UsersPage />} />
      </Routes>
    </div>
  )
}

export default App
