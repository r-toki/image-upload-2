create table blobs (
  id text primary key,
  bytes bytea not null,
  metadata json not null,
  created_at timestamptz not null
);
