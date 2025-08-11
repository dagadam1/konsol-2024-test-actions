import React from 'react'
import Popup from 'reactjs-popup';
import 'reactjs-popup/dist/index.css';
import { UserData } from '../types'
import '../styles/User.css'

type Props = {
    userData: UserData
}

const User = (props: Props) => {
    return (
        <div className='user'>
            <p>{props.userData.email}</p>
            <p>{props.userData.admin ? 'Admin' : 'Not admin'}</p>
            <Popup trigger={<button className='remove-user-button'>Remove User</button>} modal>
                <div className='remove-user-popup'>
                    <h2>Are you sure you want to remove this user?</h2>
                    <p>{props.userData.email}</p>
                    <button className='confirm-remove-button'>Yes, Remove</button>
                    <button className='cancel-remove-button'>Cancel</button>
                </div>
            </Popup>

        </div>
  )
}

export default User