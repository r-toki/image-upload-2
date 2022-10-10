create table blobs (
  id text primary key,
  bytes bytea not null,
  metadata text,
  created_at timestamptz not null
);
