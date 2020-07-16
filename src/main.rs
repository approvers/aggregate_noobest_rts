#![allow(dead_code)]
mod database;
mod make_error_enum;

use crate::database::*;
use chrono::prelude::*;
use database::mongo::MongoDB;
use egg_mode::{tweet, KeyPair, Token};
use log::info;
use std::env;
use std::sync::{Arc, RwLock};
use tokio::time::delay_for;

const NOOBEST_TWITTER_ID: u64 = 1122912368021737472;

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let db = MongoDB::new("mongodb://localhost").await.unwrap();

    noobest_check_routine(Arc::new(RwLock::new(db)))
        .await
        .unwrap();
}

async fn noobest_check_routine<D: Database>(db: Arc<RwLock<D>>) -> Result<(), D::Error> {
    let consumer_key = env::var("TWITTER_CONSUMER_KEY").expect("Set Twitter consumer key");
    let consumer_secret = env::var("TWITTER_CONSUMER_SECRET").expect("Set Twitter consumer secret");
    let access_token = env::var("TWITTER_ACCESS_TOKEN").expect("Set Twitter access token");
    let access_secret = env::var("TWITTER_ACCESS_SECRET").expect("Set Twitter access token");

    let consumer_pair = KeyPair::new(consumer_key, consumer_secret);
    let access_pair = KeyPair::new(access_token, access_secret);
    let token = Token::Access {
        consumer: consumer_pair,
        access: access_pair,
    };

    loop {
        let mut new_retweet_count = 0;
        let mut new_author_count = 0;

        info!("Getting timeline");
        let mut timeline = tweet::user_timeline(NOOBEST_TWITTER_ID, false, true, &token);
        timeline.count = 200;

        let timeline = {
            match timeline.call(None, None).await {
                Ok(a) => a,
                Err(e) => {
                    println!("Error getting timeline: {}", e);
                    continue;
                }
            }
            .response
            .iter()
            .filter(|x| x.retweeted_status.is_some())
            .cloned()
            .collect::<Vec<_>>()
        };
        info!("Got timeline");

        for tweet in timeline {
            let db = db.read().unwrap();

            //すでに登録されてる
            if db.contains_rt(StatusID(tweet.id)).await? {
                continue;
            }

            let author = tweet
                .retweeted_status
                .as_ref()
                .unwrap() // safe: already checked on above
                .user
                .as_ref()
                .unwrap(); // safe: timeline tweet must have user.

            //ツイートの作者がまだ登録されていない
            if !db.contains_author(UserID(author.id)).await? {
                let author = LocalTweetAuthor {
                    screen_name: author.screen_name.clone(),
                    name: author.name.clone(),
                    twitter_id: UserID(author.id),
                };

                db.save_author(author).await?;
                new_author_count += 1;
            }

            let db_author = db.get_author(UserID(author.id)).await?;
            let entry = LocalRetweet {
                time: tweet.created_at,
                author: db_author.id,
                twitter_id: StatusID(tweet.id),
            };

            db.save_retweet(entry).await?;
            new_retweet_count += 1;
        }

        info!(
            "Added {} RT and {} authors",
            new_retweet_count, new_author_count
        );

        delay_for(until_next_hour()).await;
    }
}

fn until_next_hour() -> std::time::Duration {
    let next_hour = (Local::now() + chrono::Duration::hours(1))
        .with_minute(0)
        .unwrap() // safe: 0 is valid.
        .with_second(0)
        .unwrap(); // safe: 0 is valid.

    (next_hour - Local::now()).to_std().unwrap()
}
