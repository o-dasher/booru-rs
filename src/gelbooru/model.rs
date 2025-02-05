//! Models for Gelbooru
use derive_more::From;
use serde::{Deserialize, Serialize};
use strum::Display;

use crate::shared::model::Rating;

/// Individual post from [`GelbooruResponse`]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GelbooruPost {
    /// The ID of the post
    pub id: u32,
    /// Datestamp of the post's creating date
    pub created_at: String,
    /// Post's score
    pub score: u32,
    /// Post's image width
    pub width: u32,
    /// Post's image height
    pub height: u32,
    /// Post's image md5
    pub md5: String,
    /// Post's image file url
    pub file_url: String,
    /// Post's tags
    pub tags: String,
    /// Post's image name (with extension)
    pub image: String,
    /// Post's image source
    pub source: String,
    /// Post's rating
    pub rating: GelbooruRating,
}

/// Gelbooru's API response with a list a posts
#[derive(Serialize, Deserialize, Debug)]
pub struct GelbooruResponse {
    #[serde(rename = "post")]
    pub posts: Vec<GelbooruPost>,
}

/// Post's rating. Check the [Gelbooru's ratings wiki](https://gelbooru.com/index.php?page=help&topic=rating)
#[derive(Serialize, Deserialize, Debug, Clone, Display, From)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum GelbooruRating {
    Explicit,
    Questionable,
    Safe,
    Sensitive,
    General,
}

impl From<Rating> for GelbooruRating {
    fn from(value: Rating) -> Self {
        match value {
            Rating::Explicit => Self::Explicit,
            Rating::Questionable => Self::Questionable,
            Rating::Safe => Self::Safe,
            Rating::Sensitive => Self::Sensitive,
            Rating::General => Self::General,
        }
    }
}
