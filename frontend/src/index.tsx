import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css'
import { BrowserRouter, Route, Routes } from 'react-router-dom';

import App from './App';
import User from './routes/User';
import FindUser from './routes/FindUser';
import CreateUser from './routes/CreateUser';
import CreatePost from './routes/CreatePost';

const root = ReactDOM.createRoot(
  document.getElementById('root') as HTMLElement
);

root.render(
  <BrowserRouter>
    <Routes>
      <Route path="/" element={<App />} /> 
      <Route path='user/:username' element={<User />} />
      <Route path='user' element={<FindUser />} />
      <Route path='createuser' element={<CreateUser />} />
      <Route path='post' element={<CreatePost />} />
    </Routes>
  </BrowserRouter>
);
