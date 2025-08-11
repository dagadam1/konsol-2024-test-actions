import React from 'react'
import { User } from '../types'
import { GoogleLogin } from '@react-oauth/google';

type Props = {
    user: User | null | undefined
    setUser: (user: User | null | undefined) => void;
}

const UserStatus = ({ user, setUser }: Props) => {

    const logout = () => {
        fetch('http://localhost:8080/api/auth/logout', {
            method: 'POST',
            credentials: 'include',
        }).then(() => {
            setUser(null);
        }).catch(err => {
            console.error('Logout failed:', err);
        });
    };

    if (user === undefined) {
        return <p>Loading...</p>
    } else if (user === null) {
        return <GoogleLogin
            onSuccess={credentialResponse => {
                var body: { id_token: any } = { id_token: credentialResponse.credential };
                fetch(`http://localhost:8080/api/auth/verify`, {
                    method: 'POST',
                    credentials: 'include',
                    headers: {
                        'Content-Type': 'application/json',
                    },

                    body: JSON.stringify(body),
                }).then(res => {
                    res.json().then(body => {
                        console.log('Login successful');
                        setUser({ email: body.email, permission: body.permission });
                    })
                }).catch(err => {
                    console.error(err);
                });
            }}

            onError={() => {
                console.log('Login Failed');
            }}
            useOneTap />
    } else {
        return (<>
            <p onClick={logout}>{user.email}</p>
            {user.permission === "Admin" ? <p>Admin</p> : <></>}
        </>)
    }
}

export default UserStatus