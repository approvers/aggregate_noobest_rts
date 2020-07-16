use crate::database::{
    LocalRetweet, LocalTweetAuthor, Retweet, StatusID, TweetAuthor, UserID, DBID,
};
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

// because bson doesn't support u64, use string.
#[derive(Serialize, Deserialize)]
pub(super) struct MongoUserID(pub(super) String);

impl From<UserID> for MongoUserID {
    fn from(u: UserID) -> Self {
        MongoUserID(u.0.to_string())
    }
}

impl Into<UserID> for MongoUserID {
    fn into(self) -> UserID {
        UserID(self.0.parse().unwrap())
    }
}

#[derive(Serialize, Deserialize)]
pub(super) struct MongoStatusID(pub(super) String);
impl From<StatusID> for MongoStatusID {
    fn from(s: StatusID) -> Self {
        MongoStatusID(s.0.to_string())
    }
}

impl Into<StatusID> for MongoStatusID {
    fn into(self) -> StatusID {
        StatusID(self.0.parse().unwrap())
    }
}

#[derive(Serialize, Deserialize)]
pub(super) struct MongoDBID(pub(super) ObjectId);
impl From<DBID> for MongoDBID {
    fn from(i: DBID) -> Self {
        Self(ObjectId::with_string(&i.0).unwrap())
    }
}

impl Into<DBID> for MongoDBID {
    fn into(self) -> DBID {
        DBID(self.0.to_string())
    }
}

#[derive(Deserialize)]
pub(super) struct MongoRetweet {
    pub(super) _id: ObjectId,
    pub(super) time: DateTime<Utc>,
    pub(super) author: MongoDBID,
    pub(super) twitter_id: MongoStatusID,
}

impl Into<Retweet> for MongoRetweet {
    fn into(self) -> Retweet {
        Retweet {
            id: DBID(self._id.to_string()),
            time: self.time,
            author: self.author.into(),
            twitter_id: self.twitter_id.into(),
        }
    }
}

#[derive(Deserialize)]
pub(super) struct MongoTweetAuthor {
    pub(super) _id: ObjectId,
    pub(super) screen_name: String,
    pub(super) name: String,
    pub(super) twitter_id: MongoUserID,
}

impl Into<TweetAuthor> for MongoTweetAuthor {
    fn into(self) -> TweetAuthor {
        TweetAuthor {
            id: DBID(self._id.to_string()),
            screen_name: self.screen_name,
            name: self.name,
            twitter_id: self.twitter_id.into(),
        }
    }
}

#[derive(Serialize)]
pub(super) struct MongoLocalRetweet {
    pub(super) time: DateTime<Utc>,
    pub(super) author: MongoDBID,
    pub(super) twitter_id: MongoStatusID,
}

impl From<LocalRetweet> for MongoLocalRetweet {
    fn from(l: LocalRetweet) -> Self {
        MongoLocalRetweet {
            time: l.time,
            author: l.author.into(),
            twitter_id: l.twitter_id.into(),
        }
    }
}

#[derive(Serialize)]
pub(super) struct MongoLocalTweetAuthor {
    pub(super) screen_name: String,
    pub(super) name: String,
    pub(super) twitter_id: MongoUserID,
}

impl From<LocalTweetAuthor> for MongoLocalTweetAuthor {
    fn from(l: LocalTweetAuthor) -> Self {
        MongoLocalTweetAuthor {
            screen_name: l.screen_name,
            name: l.name,
            twitter_id: l.twitter_id.into(),
        }
    }
}
