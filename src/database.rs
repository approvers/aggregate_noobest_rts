pub trait Database {
    type DBError: std::fmt::Display;

    fn new_entry(&mut self, _: NoobestRetweet) -> Result<(), Self::DBError>;

    fn entry_iter(&self) -> std::slice::Iter<'_, NoobestRetweet>;
}

pub struct NoobestRetweet {
    // リツイートされた時間
    pub time: chrono::DateTime<chrono::Utc>,

    // 作者
    pub author: NoobestRetweetedAuthor,

    // リツイートへのURL
    pub URL: String,
}

pub struct NoobestRetweetedAuthor {
    // 作者の名前
    pub name: String,

    // 作者のID
    pub id: String,

    // 作者ユーザーページへのURL
    pub URL: String,

    // ぬーべすとがフォローしているか
    pub is_noobest_follows: bool,
}
