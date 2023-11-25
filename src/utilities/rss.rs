use serde::Serialize;

#[derive(Serialize)]
pub struct RssData {
    pub version: String,
    pub channel: Channel,
}

#[derive(Serialize)]
pub struct Channel {
    pub title: String,
    pub link: String,
    pub description: String,
    pub language: String,
    pub pub_date: String,
    pub last_build_date: String,
    pub managing_editor: String,
    pub web_master: String,
    pub items: Vec<Item>,
}

#[derive(Serialize)]
pub struct Item {
    pub title: String,
    pub link: String,
    pub description: String,
    pub pub_date: String,
    pub guid: Guid,
}

#[derive(Serialize)]
pub struct Guid {
    pub value: String,
    #[serde(rename = "isPermaLink")]
    pub is_permalink: bool,
}

impl RssData {
    pub fn to_string(&self) -> Result<String, serde_xml_rs::Error> {
        serde_xml_rs::to_string(&self)
    }
}
