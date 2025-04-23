import Main from './components/Main'
import './App.css'
import { UserContext } from './contexts'
import { User} from './types'
import { useState } from 'react';

function App() {
  const [user, setUser] = useState<User | null>(null);
  
  return (
    <>
      <UserContext.Provider value={{user, setUser}}>
        <Main />
      </UserContext.Provider>
    </>
  )
}

export default App
