import React from 'react';
import { SlideData } from '../types';
import '../styles/Slide.css';

const Slide: React.FC<{ slide: SlideData; setSlides: any}> = ({ slide, setSlides }) => {
    const handleRemove = () => {
        fetch(`http://localhost:8080/api/screen/slides/${slide.id}`, {
            method: 'DELETE'
        }).then(response => {
            if (response.ok) {
                console.log('Slide removed');
                // Remove this slide from the slide_array
                console.log("removing")
                setSlides((prevSlides: SlideData[]) => prevSlides.filter(s => s.id !== slide.id));
            } else {
                console.log('Failed to remove slide');
            }
        });
    }

    const truncateCaption = (caption: string) => {
        return caption.length > 30 ? caption.substring(0, 30) + '...' : caption;
    }
    
    return (
        <div className='slide'>
            <div className={`indicator ${slide.active ? 'active' : 'inactive'}`} />
            <h2>{ truncateCaption(slide.caption) }</h2>
            <img className='slide-image' src={`http://localhost:8080/api/screen/slides/images/${slide.id}.${slide.filetype}`} alt={slide.caption} />  {/*  http://localhost:8080/api/slides/${slide.id} */}
            <p>Start Date: {slide.start_date.toString()}</p>
            <p>End Date: {slide.end_date.toString()}</p>
            <p>Active: {slide.active ? 'Yes' : 'No'}</p>
            <p>Filetype: {slide.filetype}</p>
            <button className='remove-button' onClick={handleRemove}>X</button>
        </div>
    )
};

export default Slide;