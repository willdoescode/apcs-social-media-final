import axios from "axios";
import React, { useState } from "react";
import { Link, useNavigate } from "react-router-dom";
import './CreateUser.css'

const CreateUser = () => {
  let [username, setUsername] = useState('');
  let [bio, setBio] = useState('');
  let [password, setPassword] = useState('');
  let nav = useNavigate();

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    axios.post('http://127.0.0.1:8080/create_user', {
      username: username,
      bio: bio,
      password: password
    }).then(res => {
      nav('/user/' + res.data.username)
    });
  }

  return <div className="awesome">
    <Link className="back" to="/">back</Link>

    <form onSubmit={e => handleSubmit(e)}>
      <input type="text" placeholder="username" onChange={e => setUsername(e.target.value)} />
      <input type="text" placeholder="bio" onChange={e => setBio(e.target.value)} />
      
      <input type="password" placeholder="password" onChange={e => setPassword(e.target.value)} />

      <button type="submit">Create User</button>
    </form>

  </div>;
}

export default CreateUser;