use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Display;

pub mod mongo;

#[async_trait]
pub trait Database {
    type Error: Display + Error;

    async fn contains_rt(&self, _: StatusID) -> Result<bool, Self::Error>;
    async fn contains_author(&self, _: UserID) -> Result<bool, Self::Error>;

    async fn get_author(&self, _: UserID) -> Result<TweetAuthor, Self::Error>;

    async fn save_retweet(&self, _: LocalRetweet) -> Result<Retweet, Self::Error>;
    async fn save_author(&self, _: LocalTweetAuthor) -> Result<TweetAuthor, Self::Error>;

    async fn update_author(
        &self,
        author_id: DBID,
        screen_name: &str,
        name: &str,
    ) -> Result<(), Self::Error>;
}

#[derive(Serialize, Deserialize)]
pub struct UserID(pub u64);

#[derive(Serialize, Deserialize)]
pub struct StatusID(pub u64);

#[derive(Serialize, Deserialize)]
pub struct DBID(pub String);

#[derive(Deserialize)]
pub struct Retweet {
    pub id: DBID,
    pub time: DateTime<Utc>,
    pub author: DBID,
    pub twitter_id: StatusID,
}

#[derive(Deserialize)]
pub struct TweetAuthor {
    pub id: DBID,
    pub screen_name: String,
    pub name: String,
    pub twitter_id: UserID,
}

pub struct LocalRetweet {
    pub time: DateTime<Utc>,
    pub author: DBID,
    pub twitter_id: StatusID,
}

pub struct LocalTweetAuthor {
    pub screen_name: String,
    pub name: String,
    pub twitter_id: UserID,
}
