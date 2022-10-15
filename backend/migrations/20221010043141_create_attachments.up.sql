create table attachments (
  id text primary key,
  record_type text not null,
  record_name text not null,
  record_id text not null,
  blob_id text not null references blobs(id) on delete cascade,
  unique(record_type, record_name, record_id, blob_id)
);
