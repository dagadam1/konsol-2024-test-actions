import React from 'react'
import { UserData } from '../types'
import '../styles/User.css'

type Props = {
    userData: UserData
}

const User = (props: Props) => {
    return (
        <div className='user'>
            <p>{props.userData.email}</p>
            <p>Admin: {props.userData.admin ? 'Yes' : 'No'}</p>
            <button className = 'edit-button' onClick={() => alert('Edit user functionality not implemented yet')}>Edit</button>
            <button className = 'delete-button' onClick={() => alert('Delete user functionality not implemented yet')}>Delete</button>
        </div>
  )
}

export default User