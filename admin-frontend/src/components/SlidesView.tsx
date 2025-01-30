import { useEffect, useState } from 'react';
import React from 'react';


interface Slide {
    id: string;
    caption: string;
    start_date: Date;
    end_date: Date;
    active: boolean;
    filetype: string;
}

const SlidesView: React.FC = () => {

    const [slides, setSlides] = useState<Slide[]>([]);

    useEffect(() => {
        // fetch('http://localhost:3001/slides')
        //     .then(response => response.json())
        //     .then(json => {
        //         setSlides([{
        //             id: 'dummy-id',
        //             caption: 'Dummy Slide',
        //             start_date: new Date(),
        //             end_date: new Date(),
        //             active: true,
        //             filetype: 'image/png'
        //         }]);
        //     });
        setSlides([{ id: 'dummy-id', caption: 'Dummy Slide', start_date: new Date(), end_date: new Date(), active: true, filetype: 'image/png' }]);
        console.log(slides);
    }, []);

    return (
        <div>
            <h1>Slides</h1>
            <div>
                {slides.map(slide => (
                    <div key={slide.id}>
                        <h2>{slide.caption}</h2>
                        <p>Start Date: {slide.start_date.toString()}</p>
                        <p>End Date: {slide.end_date.toString()}</p>
                        <p>Active: {slide.active ? 'Yes' : 'No'}</p>
                        <p>Filetype: {slide.filetype}</p>
                    </div>
                ))}
            </div>
        </div>
    );
};

export default SlidesView;