create table posts (
  id serial primary key,
  title varchar not null,
  body text not null,
  author varchar not null references users(username)
)
