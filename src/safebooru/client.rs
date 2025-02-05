use async_trait::async_trait;
use derive_more::From;

use crate::shared::client::{Client, ClientBuilder, ClientInformation};

use super::model::{SafebooruPost, SafebooruRating};

#[derive(From)]
pub struct SafebooruClient(ClientBuilder<Self>);

impl ClientInformation for SafebooruClient {
    const URL: &'static str = "https://safebooru.org";
    const SORT: &'static str = "sort:";

    type Post = SafebooruPost;
    type Rating = SafebooruRating;
}

#[async_trait]
impl Client for SafebooruClient {
    async fn get_by_id(&self, id: u32) -> Result<Self::Post, reqwest::Error> {
        let builder = &self.0;
        let url = &builder.url;

        let response = builder
            .client
            .get(format!("{url}/index.php"))
            .query(&[
                ("page", "dapi"),
                ("s", "post"),
                ("q", "index"),
                ("id", &id.to_string()),
                ("json", "1"),
            ])
            .send()
            .await?
            .json::<Vec<SafebooruPost>>()
            .await?;

        // FIXME: Assumes there is a post with the given id. Same is true for the
        // Gelbooru client.
        Ok(response.into_iter().next().unwrap())
    }

    async fn get(&self) -> Result<Vec<Self::Post>, reqwest::Error> {
        let builder = &self.0;
        let url = &builder.url;

        let response = builder
            .client
            .get(format!("{url}/index.php"))
            .query(&[
                ("page", "dapi"),
                ("s", "post"),
                ("q", "index"),
                ("limit", &builder.limit.to_string()),
                ("tags", &builder.tags.unpack()),
                ("json", "1"),
            ])
            .send()
            .await?
            .json::<Vec<SafebooruPost>>()
            .await?;

        Ok(response)
    }
}
