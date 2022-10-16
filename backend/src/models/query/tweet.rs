use chrono::{DateTime, Utc};
use derive_new::new;
use serde::{Deserialize, Serialize};
use sqlx::{query, PgPool};

#[derive(new, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tweet {
    pub id: String,
    pub body: String,
    pub images: Vec<TweetImage>,
    pub created_at: DateTime<Utc>,
}

#[derive(new, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TweetImage {
    pub encoded_bytes: String,
    pub content_type: String,
}

#[derive(Debug, Deserialize)]
pub struct RawTweetImage {
    pub bytes: Vec<u8>,
    pub content_type: String,
}

pub async fn find_tweets(pool: &PgPool) -> anyhow::Result<Vec<Tweet>> {
    let raw_tweets = query!(
        r#"
select
t.id as id,
t.body as body,
json_agg(json_build_object('bytes', b.bytes, 'content_type', b.content_type)) as images,
t.created_at as created_at
from tweets as t, attachments as a, blobs as b
where a.record_type = 'tweets'
and a.record_name = 'images'
and a.record_id = t.id
and a.blob_id = b.id
group by t.id
        "#
    )
    .fetch_all(pool)
    .await?;

    let tweets: Vec<Tweet> = raw_tweets
        .into_iter()
        .map(|r_t| {
            let images = match r_t.images {
                Some(r_t_images) => {
                    let r_t_images = serde_json::from_value::<Vec<RawTweetImage>>(r_t_images);
                    match r_t_images {
                        Ok(r_t_images) => r_t_images
                            .into_iter()
                            .map(|r_t_image| {
                                TweetImage::new(
                                    base64::encode(r_t_image.bytes),
                                    r_t_image.content_type,
                                )
                            })
                            .collect(),
                        Err(e) => {
                            println!("{}", e);
                            vec![]
                        }
                    }
                }
                None => vec![],
            };
            Tweet::new(r_t.id, r_t.body, images, r_t.created_at)
        })
        .collect();

    Ok(tweets)
}
