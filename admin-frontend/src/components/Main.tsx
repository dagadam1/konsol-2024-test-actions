import React, { useContext, useEffect } from 'react';
import SlidesView from './SlidesView';
import '../styles/Main.css';
import NewSlide from './NewSlide';
import { GoogleLogin } from '@react-oauth/google';
import { UserContext } from '../contexts';
import { UserContextType } from '../types';

const logout = () => {
    fetch(`http://localhost:8080/api/auth/logout`, {
        method: 'POST',
        credentials: 'include',
        headers: {
            'Content-Type': 'application/json',
        },
    })
}

const Main: React.FC = () => {
    const {user, setUser} = useContext(UserContext) as UserContextType;

    useEffect(() => {
        fetch(`http://localhost:8080/api/auth/status`, {
            method: 'GET',
            credentials: 'include',
            headers: {
                'Content-Type': 'application/json',
            },
        }).then(res => {
            res.json().then(body => {
                if (res.ok) {
                    console.log("Session already authenticated");
                    setUser({email: body.email});
                }
            })
        }).catch(err => {
            console.error(err);
        });
    }, [setUser])

    return (
        <>
            <div id="floating-container">
                <div id='floating'>
                    <NewSlide />
                </div>
            </div>
            <h1>Konsol</h1>

        <div className="greeting"> {user ? "Authenticated as " + user.email : "Not authenticated"} <button onClick={logout}>Logout</button></div>
        <GoogleLogin 
            onSuccess={credentialResponse => {
                console.log(credentialResponse);
                var body: { id_token: any, client_id: any } = { id_token: credentialResponse.credential, client_id: credentialResponse.clientId };
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
                        setUser({email: body.email});
                    })
                }).catch(err => {
                    console.error(err);
                });
            }}
                
            onError={() => {
                console.log('Login Failed');
            }}
            useOneTap/>
            
            <SlidesView></SlidesView>;
    </>
    );
};

export default Main;