mod structs;

use crate::database::*;
use async_trait::async_trait;
use mongodb::bson::{self, doc, Bson};
use mongodb::options::ClientOptions;
use mongodb::Client;
use mongodb::Collection;
use structs::*;
use thiserror::Error;
use tokio::stream::StreamExt;

pub struct MongoDB {
    rt_collection: Collection,
    author_collection: Collection,
}

#[derive(Error, Debug)]
pub enum MongoDBError {
    #[error("接続に失敗しました")]
    Request(mongodb::error::Error),

    #[error("URLのパースに失敗しました")]
    URLParse(mongodb::error::Error),

    #[error("Clientの作成に失敗しました")]
    ClientCreate(mongodb::error::Error),

    #[error("シリアライズに失敗しました")]
    Serialize(mongodb::bson::ser::Error),

    #[error("シリアライズ時にエラーが発生しました")]
    InvalidSerialize(&'static str),

    #[error("デシリアライズに失敗しました")]
    Deserialize(mongodb::bson::de::Error),

    #[error("デシリアライズ時にエラーが発生しました")]
    InvalidDeserialize(&'static str),

    #[error("指定されたものは見つかりませんでした")]
    NotFound,

    #[error("無効なエントリが見つかりました")]
    InvalidEntry(&'static str),

    #[error("アップデートに失敗しました")]
    Update(mongodb::error::Error),
}

impl MongoDB {
    pub async fn new(url: &str) -> Result<MongoDB, MongoDBError> {
        let mut client_options = ClientOptions::parse(url)
            .await
            .map_err(|e| MongoDBError::URLParse(e))?;

        client_options.app_name = Some("Noobest RT Aggregator".into());

        let database = Client::with_options(client_options)
            .map_err(|e| MongoDBError::ClientCreate(e))?
            .database("noobest_rt");

        let result = MongoDB {
            rt_collection: database.collection("retweets"),
            author_collection: database.collection("authors"),
        };

        Ok(result)
    }
}

#[async_trait]
impl Database for MongoDB {
    type Error = MongoDBError;

    async fn contains_rt(&self, status_id: StatusID) -> Result<bool, Self::Error> {
        let status_id = MongoStatusID::from(status_id);

        let aggregate_result = self
            .author_collection
            .aggregate(
                vec![
                    doc! {"$match": doc! { "twitter_id": status_id.0 }  },
                    doc! {"$count": "count"},
                ],
                None,
            )
            .await
            .map_err(|e| MongoDBError::Request(e))?
            .next()
            .await
            .unwrap_or_else(|| Ok(doc! {"count": 0}))
            .map_err(|e| MongoDBError::Request(e))?;

        match aggregate_result
            .get_i32("count")
            .map_err(|_| MongoDBError::NotFound)?
        {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(MongoDBError::InvalidEntry(
                "同じTweetIDを持つエントリが複数あります",
            )),
        }
    }

    async fn contains_author(&self, user_id: UserID) -> Result<bool, Self::Error> {
        let user_id = MongoUserID::from(user_id);

        let aggregate_result = self
            .author_collection
            .aggregate(
                vec![
                    doc! {"$match": doc! { "twitter_id": user_id.0 }  },
                    doc! {"$count": "count"},
                ],
                None,
            )
            .await
            .map_err(|e| MongoDBError::Request(e))?
            .next()
            .await
            .unwrap_or_else(|| Ok(doc! {"count": 0}))
            .map_err(|e| MongoDBError::Request(e))?;

        match aggregate_result
            .get_i32("count")
            .map_err(|_| MongoDBError::NotFound)?
        {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(MongoDBError::InvalidEntry(
                "同じAuthorIDを持つエントリが複数あります",
            )),
        }
    }

    async fn get_author(&self, user_id: UserID) -> Result<TweetAuthor, Self::Error> {
        let user_id = MongoUserID::from(user_id);

        let aggregate_result = self
            .author_collection
            .aggregate(
                vec![doc! {
                    "$match": doc! {
                        "twitter_id": user_id.0
                    }
                }],
                None,
            )
            .await
            .map_err(|e| MongoDBError::Request(e))?
            .next()
            .await
            .ok_or(MongoDBError::NotFound)?
            .map_err(|e| MongoDBError::Request(e))?;

        let bson = Bson::Document(aggregate_result);
        let author =
            bson::from_bson::<MongoTweetAuthor>(bson).map_err(|e| MongoDBError::Deserialize(e))?;

        Ok(author.into())
    }

    async fn save_retweet(&self, entry: LocalRetweet) -> Result<Retweet, Self::Error> {
        let entry = MongoLocalRetweet::from(entry);

        let bson = bson::to_bson(&entry)
            .map_err(|e| MongoDBError::Serialize(e))?
            .as_document()
            .ok_or_else(|| MongoDBError::InvalidSerialize("bson::to_bson returned not document"))?
            .clone();

        let id = self
            .rt_collection
            .insert_one(bson, None)
            .await
            .map_err(|e| MongoDBError::Request(e))?
            .inserted_id;

        let entry = self
            .rt_collection
            .find_one(doc! {"_id": id}, None)
            .await
            .map_err(|e| MongoDBError::Request(e))?
            .ok_or_else(|| MongoDBError::InvalidSerialize("not found _id"))?
            .clone();

        let document = Bson::Document(entry);
        let entry =
            bson::from_bson::<MongoRetweet>(document).map_err(|e| MongoDBError::Deserialize(e))?;

        Ok(entry.into())
    }

    async fn save_author(&self, entry: LocalTweetAuthor) -> Result<TweetAuthor, Self::Error> {
        let entry = MongoLocalTweetAuthor::from(entry);

        let bson = bson::to_bson(&entry)
            .map_err(|e| MongoDBError::Serialize(e))?
            .as_document()
            .ok_or_else(|| MongoDBError::InvalidSerialize("bson::to_bson returned not document"))?
            .clone();

        let id = self
            .author_collection
            .insert_one(bson, None)
            .await
            .map_err(|e| MongoDBError::Request(e))?
            .inserted_id;

        let entry = self
            .author_collection
            .find_one(doc! {"_id": id}, None)
            .await
            .map_err(|e| MongoDBError::Request(e))?
            .ok_or_else(|| MongoDBError::InvalidSerialize("not found _id"))?
            .clone();

        let entry: MongoTweetAuthor =
            bson::from_bson(Bson::Document(entry)).map_err(|e| MongoDBError::Deserialize(e))?;

        Ok(entry.into())
    }

    async fn update_author(
        &self,
        author_id: DBID,
        screen_name: &str,
        name: &str,
    ) -> Result<(), Self::Error> {
        let author_id = MongoDBID::from(author_id).0;

        self.author_collection
            .find_one_and_update(
                doc! {"_id": author_id},
                doc! {"screen_name": screen_name, "name": name},
                None,
            )
            .await
            .map_err(|e| MongoDBError::Update(e))?;

        Ok(())
    }
}
