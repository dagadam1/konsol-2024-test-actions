import { useEffect, useState } from 'react';

import '../../styles/slides/Slideshow.css';

import { SlideData } from '../../types/slides/SlideData.ts';

type Props = {
    slides: SlideData[];
}

const Slideshow = ({ slides }: Props) => {
    const [currentIndex, setCurrentIndex] = useState(0);

    const goToNextSlide = () => {
        setCurrentIndex((prevIndex) => (prevIndex + 1) % slides.length);
    };

    const goToPreviousSlide = () => {
        setCurrentIndex((prevIndex) => (prevIndex - 1 + slides.length) % slides.length);
    };

    
    useEffect(() => {
        const interval = setInterval(goToNextSlide, 3000);
        return () => clearInterval(interval);
    }, [slides]);
    
    if (slides.length === 0) {
        return <div>Loading slides...</div>;
    }
    
  return (
    <>
        <div key={slides[currentIndex].id}>
          <h2>{slides[currentIndex].caption}</h2>
          {/* TODO: use env var instead of hardcoded link, also https */}
          <img className='slide-image' src={'http://localhost:8080/api/screen/slides/images/' + slides[currentIndex].id +'.'+ slides[currentIndex].filetype} alt={slides[currentIndex].caption} />
        </div>
        <button onClick={goToPreviousSlide}>Previous</button>
        <button onClick={goToNextSlide}>Next</button>
      
    </>
  )
}

export default Slideshow;