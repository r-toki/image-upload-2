create table blobs (
  id serial primary key,
  bytes bytea not null,
  metadata text,
  created_at timestamptz not null
);
