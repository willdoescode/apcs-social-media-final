import axios from "axios";
import React, { useState } from "react";
import { Link, useNavigate } from "react-router-dom";

const CreatePost = () => {
  const [title, setTitle] = useState("");
  const [body, setBody] = useState("");
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

  const nav = useNavigate();

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    axios.post("http://127.0.0.1:8080/create_post", {
      username: username,
      password: password,
      title: title,
      body: body
    })
      .then(res => {
        nav("/user/" + res.data.author);
      })
  }

  return <div className="awesome">
    <Link className="back" to="/">back</Link>

    <form onSubmit={e => handleSubmit(e)}>
      <input type="text" placeholder="title" onChange={e => setTitle(e.target.value)} />
      <input type="text" placeholder="body" onChange={e => setBody(e.target.value)} />
      <input type="text" placeholder="username" onChange={e => setUsername(e.target.value)} />
      <input type="password" placeholder="password" onChange={e => setPassword(e.target.value)} />  

      <button type="submit">Create Post</button>
    </form>

  </div>;
}

export default CreatePost;