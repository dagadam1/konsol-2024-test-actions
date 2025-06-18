import React from 'react';
import { useEffect, useState } from 'react';
import { SlideData } from '../types';
import Slide from '../components/Slide';
import '../styles/SlidesPage.css';

const SlidesPage: React.FC = () => {
      const [slides, setSlides] = useState<SlideData[]>([]);

    useEffect(() => {
        fetch('http://localhost:8080/api/screen/slides')
            .then(response => response.json())
            .then(json => setSlides(json));
        // setSlides([{ id: 'dummy-id', caption: 'Dummy Slide', start_date: new Date(), end_date: new Date(), active: true, filetype: 'image/png' }, { id: 'dummy-id2', caption: 'Dummy Slide', start_date: new Date(), end_date: new Date(), active: true, filetype: 'image/png' }, { id: 'dummy-id3', caption: 'Dummy Slide', start_date: new Date(), end_date: new Date(), active: true, filetype: 'image/png' }]);
        console.log(slides);
    }, []);

    return (
        <div className='slides-page'>
            <div className='slides'>
                {slides.map(slide => (
                    <Slide slide={slide} setSlides={setSlides}/>
                ))}
            </div>
        </div>
    );
};

export default SlidesPage;