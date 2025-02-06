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
        <div>
          <p>Calendar placeholder</p>
        </div>
      </div>
    </>
  )
}

export default App
