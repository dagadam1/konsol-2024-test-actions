import React from 'react';
import SlidesView from './SlidesView';
import '../styles/Main.css';
import NewSlide from './NewSlide';
import { GoogleLogin } from '@react-oauth/google';

const Main: React.FC = () => {
    return (
        <>
            <div id="floating-container">
                <div id='floating'>
                    <NewSlide />
                </div>
            </div>
            <h1>Konsol</h1>

        <GoogleLogin 
            onSuccess={credentialResponse => {
                console.log(credentialResponse);
                var body: { id_token: any, client_id: any } = { id_token: credentialResponse.credential, client_id: credentialResponse.clientId };
                fetch(`http://localhost:8080/api/auth/verify`, {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify(body),
                }).then(res => {
                    console.log(res);
                }).catch(err => {
                    console.error(err);
                });
            }}
                
            onError={() => {
                console.log('Login Failed');
            }}
            useOneTap/>;
    </>
    );
};

export default Main;