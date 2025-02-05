use anyhow::{ensure, Result};
use derive_more::From;

use async_trait::async_trait;
use itertools::Itertools;
use reqwest::{header, header::HeaderMap};

use super::model::*;
use crate::shared::client::*;

// This is only here because of Danbooru, thanks Danbooru, really cool :)
pub fn get_headers() -> HeaderMap {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static("PostmanRuntime/7.30.0"),
    );
    headers
}

/// Client that sends requests to the Danbooru API to retrieve the data.
#[derive(From)]
pub struct DanbooruClient(ClientBuilder<Self>);

impl<'a> ClientInformation for DanbooruClient {
    const URL: &'static str = "https://danbooru.donmai.us";
    const SORT: &'static str = "order:";

    type Rating = DanbooruRating;
    type Post = DanbooruPost;
}

#[async_trait]
impl Client for DanbooruClient {
    fn validate(validates: ValidationType<'_, Self>) -> Result<()> {
        match validates {
            ValidationType::Tags(tags) => {
                ensure!(
                    tags.0.iter().filter(|t| t.is_plain()).collect_vec().len() <= 1,
                    "Danbooru only allows two tags per query"
                );
            }
        }

        Ok(())
    }

    /// Directly get a post by its unique Id
    async fn get_by_id(&self, id: u32) -> Result<Self::Post, reqwest::Error> {
        let builder = &self.0;
        let url = &builder.url;

        let response = builder
            .client
            .get(format!("{url}/posts/{id}.json"))
            .headers(get_headers())
            .send()
            .await?
            .json::<DanbooruPost>()
            .await?;

        Ok(response)
    }

    /// Pack the [`ClientBuilder`] and sent the request to the API to retrieve the posts
    async fn get(&self) -> Result<Vec<Self::Post>, reqwest::Error> {
        let builder = &self.0;
        let url = &builder.url;

        let response = builder
            .client
            .get(format!("{url}/posts.json"))
            .headers(get_headers())
            .query(&[
                ("limit", &builder.limit.to_string()),
                ("tags", &builder.tags.unpack()),
            ])
            .send()
            .await?
            .json::<Vec<DanbooruPost>>()
            .await?;

        Ok(response)
    }
}
