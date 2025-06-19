import { GoogleLogin } from '@react-oauth/google';
import '../styles/Header.css'
import { useState } from 'react';
import { User } from '../types';


type Props = {
  user: User | null;
  setUser: (user: User | null) => void;
}

const Header = (props: Props) => {
  const { user, setUser } = props;
  return (
    <div className="header">
      <div className="header-left">
        <img src="https://f.kth.se/wp-content/uploads/FysikMedium.png" className="logo" />
        <h1 className='title'>Konsol Admin</h1>
      </div>
      <div className="header-right">
        {user ?
          <div className='user-info'>
            <p>{user.email}</p>
            {user.permission == "Admin" ? <p>Admin</p> : <></>}
          </div>
          :<GoogleLogin
              onSuccess={credentialResponse => {
                  console.log(credentialResponse);
                  var body: { id_token: any } = { id_token: credentialResponse.credential };
                  fetch(`http://localhost:8080/api/auth/verify`, {
                      method: 'POST',
                      credentials: 'include',
                      headers: {
                          'Content-Type': 'application/json',
                      },
        
                      body: JSON.stringify(body),
                  }).then(res => {
                      console.log(res);
                      res.json().then(body => {
                          setUser({ email: body.email, permission: body.permission });
                      })
                  }).catch(err => {
                      console.error(err);
                  });
              }}
        
              onError={() => {
                  console.log('Login Failed');
              }}
              useOneTap/>
            }
      </div>

    </div>
  )
}

export default Header