import React, { useEffect, useState } from "react";
import { useParams } from 'react-router-dom';
import axios from 'axios';

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
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [])

  const getUser = () => {
    axios.get(`http://127.0.0.1:8080/get_user/${params.username}`)
      .then(res => {
        setUser(res.data);
      })
  }

  return (
    <div>
      <h1>{user && 
      <div>
        <h1>{user.username}</h1>
        <h2>{user.bio}</h2>
      </div>
      }</h1>
    </div>
  );
}

export default User;