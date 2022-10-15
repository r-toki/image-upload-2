use crate::models::{
    lib::{get_current_date_time, get_new_id},
    table,
};

pub fn new(encoded_bytes: String, content_type: String) -> anyhow::Result<table::Blob> {
    Ok(table::Blob::new(
        get_new_id(),
        base64::decode(encoded_bytes)?,
        table::blob::Metadata::new(content_type).try_into()?,
        get_current_date_time(),
    ))
}
