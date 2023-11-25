use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename = "rss")]
pub struct Rss {
    #[serde(rename = "@version")]
    pub version: String,
    pub channel: Channel,
}

#[derive(Debug, Serialize)]
pub struct Channel {
    pub title: String,
    pub description: CDATA,
    pub copyright: String,
    pub link: String,
    pub ttl: i32,
    pub item: Vec<Item>,
}

#[derive(Debug, Serialize)]
pub struct Item {
    pub title: String,
    pub category: String,
    pub link: String,
    pub description: CDATA,
    #[serde(rename = "pubDate")]
    pub pub_date: String,
    pub source: Source,
    pub guid: Guid,
}

#[derive(Debug, Serialize)]
pub struct Source {
    #[serde(rename = "@url")]
    pub url: String,
    #[serde(rename = "$value")]
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct Guid {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "@isPermaLink")]
    pub is_permalink: bool,
}

#[derive(Debug, Serialize)]
pub struct CDATA {
    #[serde(rename = "$text")]
    pub value: String,
}

impl Rss {
    pub fn to_string(&self) -> Result<String, quick_xml::DeError> {
        quick_xml::se::to_string(&self)
    }
}
