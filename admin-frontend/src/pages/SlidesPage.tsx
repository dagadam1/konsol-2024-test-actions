import React from 'react';
import { useEffect, useState } from 'react';
import { SlideData } from '../types';
import Slide from '../components/Slide';
import '../styles/SlidesPage.css';
import Popup from 'reactjs-popup';

const SlidesPage = () => {

    const [slides, setSlides] = useState<SlideData[]>([]);

    useEffect(() => {
        fetch('http://localhost:8080/api/screen/slides', {
            method: 'GET',
            credentials: 'include'
        })
            .then(response => response.json())
            .then(json => setSlides(json));
        // setSlides([{ id: 'dummy-id', caption: 'Dummy Slide', start_date: new Date(), end_date: new Date(), active: true, filetype: 'image/png' }, { id: 'dummy-id2', caption: 'Dummy Slide', start_date: new Date(), end_date: new Date(), active: true, filetype: 'image/png' }, { id: 'dummy-id3', caption: 'Dummy Slide', start_date: new Date(), end_date: new Date(), active: true, filetype: 'image/png' }]);
    }, []);

    const handleAddSlide = (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        const data = new FormData(event.currentTarget);

        // Visible is an empty string if the checkbox is not checked; we want it to be false instead
        data.set('visible', data.get('visible') || 'false');

        console.log(data);

        fetch('http://localhost:8080/api/screen/slides/save', {
            method: 'POST',
            body: data
        }).then(response => {
            if (response.ok) {
                console.log('Slide saved');
                //TODO update slides when a new slide is added
            } else {
                console.log('Failed to save slide');
            }
        });
    };

    return (
        <div className='slides-page'>
            <div className="slides-header">
                <h1>Slides</h1>
                <Popup className="add-slide-popup" trigger={<button className='add-slide-button'>Add Slide</button>} modal>   
                        <h2>Add Slide</h2>
                        <form onSubmit={(event) => handleAddSlide(event)}>
                            <label htmlFor='caption'>Caption</label>
                            <input type='text' id='caption' name='caption' />
                            <label htmlFor='startDate'>Start Date</label>
                            <input type='date' id='startDate' name='start' />
                            <label htmlFor='endDate'>End Date</label>
                            <input type='date' id='endDate' name='end' />
                            {/* <label htmlFor='active'>Active</label>
                            <input type='checkbox' id='active' name='visible' value='true' /> */}
                            <label htmlFor='file'>Image</label>
                            <input type='file' id='file' name='imageFile' />
                            <button type='submit'>Submit</button>
                        </form>
                </Popup>

            </div>
            <div className='slides'>
                {slides.map(slide => (
                    <Slide slide={slide} setSlides={setSlides} />
                ))}
            </div>
        </div>
    );
};

export default SlidesPage;