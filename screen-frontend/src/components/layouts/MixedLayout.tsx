import SlData from "../../types/sl/SlData";
import { useState, useEffect } from "react";
import { SlideData } from "../../types/slides/SlideData";
import SlDepartureList from "../sl/SlDepartureList";
import Slideshow from "../slides/Slideshow";
import '../../styles/layouts/MixedLayout.css';
import fysikF from '../../assets/FrakturF2020.png';


function MixedLayout({slides, sl_data}: {slides: SlideData[], sl_data: SlData}) {

const [active, setActive] = useState("slide");

  useEffect(() => {
    const interval = setInterval(() => {
      setActive((prev) => (prev === "slide" ? "calendar" : "slide"));
    }, 15000);
    return () => clearInterval(interval); // cleanup on unmount
  }, []);

  return <>
    <div className="header">
      <img src={fysikF} alt="Fraktur F" className="fysikf"/>
      <h1>Konsol</h1>
    </div>
    
    <div className='left'>
      {active === "slide" ? (
        <Slideshow slides={slides} />
      ) : active === "calendar" ?(
        <div className="calendar-container">
          <iframe
            className="calendar-frame"
            src="https://calendar.google.com/calendar/embed?height=600&wkst=2&ctz=Europe%2FStockholm&showPrint=0&showNav=0&mode=AGENDA&hl=sv&title=Fysiksektionens%20kalender&showTabs=0&showCalendars=0&showTz=0&showDate=0&src=ZnlzaWtzZWt0aW9uZW4uc2VfMDE4N3ZibWRjaXZsOG10aW8xNDJlMjNjYXNAZ3JvdXAuY2FsZW5kYXIuZ29vZ2xlLmNvbQ&color=%23FF642B"
            scrolling="no">
          </iframe>
        </div>
      ): null
      }
      
    </div>

    <div className="right">
      <p className="last-update">{`Senast uppdaterad: ${sl_data.last_update ? sl_data.last_update.toLocaleTimeString() : "Aldrig"}`}</p>
      <SlDepartureList sl_data={sl_data}/>
    </div>
  </>;
}

export default MixedLayout;
