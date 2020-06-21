mod database;
use std::env;

const NOOBEST_TWITTER_ID: u64 = 1122912368021737472;

fn main() {
    let consumer_key = env::var("TWITTER_CONSUMER_KEY").expect("Set Twitter consumer key");
    let consumer_secret = env::var("TWITTER_CONSUMER_SECRET").expect("Set Twitter consumer secret");
    let access_token = env::var("TWITTER_ACCESS_TOKEN").expect("Set Twitter access token");
    let access_secret = env::var("TWITTER_ACCESS_SECRET").expect("Set Twitter access token");

    let consumer_pair = egg_mode::KeyPair::new(consumer_key, consumer_secret);
    let access_pair = egg_mode::KeyPair::new(access_token, access_secret);
    let token = egg_mode::Token::Access {
        consumer: consumer_pair,
        access: access_pair,
    };

    let mut runtime =
        tokio::runtime::Runtime::new().expect("Failed to start tokio runtime for twitter");

    runtime.block_on(async {
        let mut timeline = egg_mode::tweet::user_timeline(NOOBEST_TWITTER_ID, false, true, &token);
        timeline.count = 200;
        let result = timeline.call(None, None).await.unwrap();

        println!("{:#?}", result.response);
    });
}
