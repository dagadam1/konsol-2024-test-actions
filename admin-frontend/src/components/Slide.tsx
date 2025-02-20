import React from 'react';
import { SlideData } from '../types';
import '../styles/Slide.css';

const Slide: React.FC<{ slide: SlideData }> = ({ slide }) => {
    const handleRemove = () => {
        fetch(`http://localhost:8080/api/slides/${slide.id}`, {
            method: 'DELETE'
        }).then(response => {
            if (response.ok) {
                console.log('Slide removed');
            } else {
                console.log('Failed to remove slide');
            }
        });
    }
    return (
        <div className='slide'>
            <div className='slide-header'>
                <img className='slide-image' src={`http://localhost:8080/api/screen/slides/images/${slide.id}.${slide.filetype}`} alt={slide.caption} />  {/*  http://localhost:8080/api/slides/${slide.id} */}
            </div>
            <h2>{slide.caption}</h2>
            <p>Start Date: {slide.start_date.toString()}</p>
            <p>End Date: {slide.end_date.toString()}</p>
            <p>Active: {slide.active ? 'Yes' : 'No'}</p>
            <p>Filetype: {slide.filetype}</p>
            <button onClick={handleRemove}>Remove</button>
        </div>
    )
};

export default Slide;