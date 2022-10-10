create table attachments (
  id serial primary key,
  record_type text,
  record_name text,
  record_id text,
  blob_id text
);
