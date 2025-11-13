import React from 'react'
import '../styles/NavHeader.css'
import { User } from '../types';
import { Link } from 'react-router';

type Props = {
  user: User | null | undefined
}

const NavHeader = ({ user }: Props) => {
  return (
    <div className="nav-header">
        <Link to="/slides" className="slides-page-button">Slides</Link>
        {user?.permission === 'Admin' && <Link to="/users" className="users-page-button">Users</Link>}
    </div>
  )
}

export default NavHeader