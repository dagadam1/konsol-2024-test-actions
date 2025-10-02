import { useEffect, useState } from 'react'

import '../index.css';
import '../styles/App.css';

import Slideshow from './slides/Slideshow.tsx'
import { SlideData } from '../types/slides/SlideData.ts'

import SlDepartureList from "./sl/SlDepartureList.tsx";
import SlData from "../types/sl/SlData.ts";

function App() {
  const [slides, setSlides] = useState<SlideData[]>([]);

  const [sl, setSl] = useState<{ data: SlData }>({data: new SlData([
        { site_id: 9204, tracked_lines: undefined }, // tekniska högskolan
        { site_id: 9600, tracked_lines: undefined }, // östra station
        // { site_id: 1080, tracked_lines: undefined }  // stockholm city
    ])});

  useEffect(() => {
    fetch('http://localhost:8080/api/screen/slides')
      .then(response => response.json())
      .then(data => {
        setSlides(data);
        console.log(data);
      })
      .catch(error => {
        console.error('Error fetching slides:', error)
      })
  }, []);

  useEffect(() => {
    const handle = setInterval(() => {
      sl.data.update().then(() => {
        setSl({data: sl.data});
      });
    },
      5 * 1000)
    return () => clearInterval(handle);
  }, []);

  return <>
    <div className='left'>
      <Slideshow slides={slides} />
    </div>
    <div className="right">
      <p className="last-update">{`Senast uppdaterad: ${sl.data.last_update ? sl.data.last_update.toLocaleTimeString() : "Aldrig"}`}</p>
      <SlDepartureList sl_data={sl.data}/>
      <div className="calendar-container">
        <iframe
          className="calendar-frame"
          src="https://calendar.google.com/calendar/embed?height=600&wkst=2&ctz=Europe%2FStockholm&showPrint=0&showNav=0&mode=AGENDA&hl=sv&title=Fysiksektionens%20kalender&showTabs=0&showCalendars=0&showTz=0&showDate=0&src=ZnlzaWtzZWt0aW9uZW4uc2VfMDE4N3ZibWRjaXZsOG10aW8xNDJlMjNjYXNAZ3JvdXAuY2FsZW5kYXIuZ29vZ2xlLmNvbQ&color=%23FF642B"
          scrolling="no">
        </iframe>
      </div>
    </div>
  </>;
}

export default App
