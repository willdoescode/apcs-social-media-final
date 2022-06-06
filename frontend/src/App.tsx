import React from 'react';
import { Link } from 'react-router-dom';
import './App.css';

const App = () => {
  return (
    <div>
      <Link className='url' to="/createuser">Create User</Link>
      <Link className='url' to="/post">Create Post</Link>
      <Link className='url' to="/user">Find User</Link>
    </div>
  );
}

export default App;
