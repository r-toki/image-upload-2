create table blobs (
  id text primary key,
  bytes bytea not null,
  content_type text not null,
  created_at timestamptz not null
);
