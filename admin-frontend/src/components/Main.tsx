import React from 'react';
import SlidesView from './SlidesView';
import 'reactjs-popup/dist/index.css';
import '../styles/Main.css';
import NewSlide from './NewSlide';

const Main: React.FC = () => {
    return (
        <>
            <div id="floating-container">
                <div id='floating'>
                    <NewSlide />
                </div>
            </div>
            <h1>Konsol</h1>
            <SlidesView />
        </>
    );
};

export default Main;