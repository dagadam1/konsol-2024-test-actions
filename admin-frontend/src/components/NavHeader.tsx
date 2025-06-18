import React from 'react'
import '../styles/NavHeader.css'

type Props = {}

const NavHeader = (props: Props) => {
  return (
    <div className="nav-header">
        <a href="/slides" className="slides-page-button">Slides</a>
        <a href="/users" className="users-page-button">Users</a>
    </div>
  )
}

export default NavHeader