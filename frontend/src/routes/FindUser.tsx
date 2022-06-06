import React from "react";
import { Link, useNavigate } from "react-router-dom";

const FindUser = () => {
  const [username, setUsername] = React.useState("");

  const nav = useNavigate();

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    nav("/user/" + username);
  }

  return <div>
    <Link className="back" to="/">back</Link>

    <form onSubmit={e => handleSubmit(e)}>
      <input type="text" placeholder="username" onChange={e => setUsername(e.target.value)} />

      <button type="submit">Find User</button>
    </form>
  </div>;
}

export default FindUser;
