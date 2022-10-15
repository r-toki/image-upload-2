create table tweets (
  id text primary key,
  body text not null,
  created_at timestamptz not null,
  updated_at timestamptz not null
)
