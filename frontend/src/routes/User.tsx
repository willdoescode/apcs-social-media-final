import React, { useEffect, useState } from "react";
import { Link, useParams } from 'react-router-dom';
import axios from 'axios';

import './User.css'

interface UserI {
  username: string;
  bio: string | null;
  is_admin: boolean;
  password: string;
}

interface PostI {
  id: number;
  title: string;
  body: string;
  author: string;
}

const User = () => {
  let params = useParams();
  let [user, setUser] = useState({} as UserI);
  let [posts, setPosts] = useState([] as PostI[]);

  useEffect(() => {
    getUser();
    getPosts();
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [])

  const getUser = () => {
    axios.get(`http://127.0.0.1:8080/get_user/${params.username}`)
      .then(res => {
        setUser(res.data);
      })
  }

  const getPosts = () => {
    axios.get(`http://127.0.0.1:8080/get_posts/${params.username}`)
      .then(res => {
        setPosts(res.data);
      }) 
    }

  return (
    <div>
    <Link className="back" to="/">back</Link>

      {user && 
        <div>
          <h1>{user.username}</h1>
          <h3>{user.bio}</h3>
        </div>
      }

      <h1>Posts:</h1>

      {posts && posts.map(post => <div className="post" key={post.id}>
        <h3>{post.title}</h3>
        <br />
        <h4>{post.body}</h4>
        </div>)}
    </div>
  );
}

export default User;