import React from 'react'
import '../styles/Header.css'

type Props = {}

const Header = (props: Props) => {
  return (
    <div className="header">
      <img src="https://f.kth.se/wp-content/uploads/FysikMedium.png" className="logo" />
      <h1 className='title'>Konsol Admin</h1>
    </div>
  )
}

export default Header