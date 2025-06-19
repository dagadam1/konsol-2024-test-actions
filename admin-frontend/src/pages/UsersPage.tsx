import { useEffect, useState } from 'react';
import User from '../components/User';
import { UserData } from '../types';
import '../styles/UsersPage.css';

type Props = {}

const UsersPage = (props: Props) => {
    const [users, setUsers] = useState<UserData[]>([]);

    useEffect(() => {
        // fetch('http://localhost:8080/api/auth/list_users')
        //     .then(response => response.json())
        //     .then(json => setUsers(json));
        setUsers([{ id: 'dummy-id', email: 'user1@example.com', admin: true }, { id: 'dummy-id2', email: 'user2@example.com', admin: false }, { id: 'dummy-id3', email: 'user3@example.com', admin: true }]);
        console.log(users);
    }, []);


  return (
    <div className='users-page'>
      <div className="users-header">
        <h1>Users</h1>
        <button onClick={() => alert('Add user functionality not implemented yet')}>Add User</button>
      </div>
      {users.map(user => <User userData = {user}/>)}  
    </div>
  )
}

export default UsersPage