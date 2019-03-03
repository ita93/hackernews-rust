use rss::{Channel, Item};

use serde::Serialize;

pub type FetchRes<T> = Result<T, rss::Error>;

pub fn fetch_from(url: &str) -> FetchRes<Vec<RSSItem>> {
    Ok(Channel::from_url(url)?.items()
       .into_iter()
       .map(|item| RSSItem::from(item.clone())).
       collect())
}

#[derive(Serialize)]
pub struct RSSItem {
    pub title: String,
    pub link: String,
    pub description: String,
    pub pub_date: String,
}

impl From<Item> for RSSItem {
    fn from(item: Item) -> Self {
        RSSItem {
            title: item.title().unwrap_or_default().to_owned(),
            link: item.link().unwrap_or_default().to_owned(),
            description: item.description().unwrap_or_default().to_owned(),
            pub_date: item.pub_date().unwrap_or_default().to_owned(),
        }
    }
}
