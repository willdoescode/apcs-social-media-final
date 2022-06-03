create table users (
  username varchar primary key,
  bio varchar,
  is_admin boolean not null default 'f'
)
