import { useState, useContext, useEffect, createContext} from 'react'
import './App.css'

const settingsContext = createContext({})

function App() {
  const [count, setCount] = useState(0)

  return (
    <>
      <div className='left'>
        <img id="slideph" src={"https://www.publicdomainpictures.net/pictures/40000/velka/white-duck-in-pond.jpg"} alt='React Logo' />
        <p>slideshow placeholder</p>
      </div>
      <div className="right">
        <div className='sl'>
          <p>Sl-placeholder</p>
        </div>
        <div className="calendar-placeholder">
          <iframe
            className="calendar-frame"
            src="https://calendar.google.com/calendar/embed?height=600&wkst=2&ctz=Europe%2FStockholm&showPrint=0&showNav=0&mode=AGENDA&hl=sv&title=Fysiksektionens%20kalender&showTabs=0&showCalendars=0&showTz=0&showDate=0&src=ZnlzaWtzZWt0aW9uZW4uc2VfMDE4N3ZibWRjaXZsOG10aW8xNDJlMjNjYXNAZ3JvdXAuY2FsZW5kYXIuZ29vZ2xlLmNvbQ&color=%23FF642B"
            scrolling="no">
          </iframe>
        </div>
      </div>
    </>
  )
}

export default App
