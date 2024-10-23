create table project (
  id text primary key not null,
  name varchar(200) not null,
  created_at datetime not null default now
);
