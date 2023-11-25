use crate::entities::{posts, prelude::Posts};
use crate::utilities::rss::*;
use actix_web::{web, HttpResponse, Result};
use sea_orm::*;

pub async fn rss(db: web::Data<sea_orm::DatabaseConnection>) -> Result<HttpResponse> {
    let items: Vec<Item> = Posts::find()
        .filter(posts::Column::Status.eq("PUBLIC"))
        .order_by_desc(posts::Column::Time)
        .all(db.as_ref())
        .await
        .unwrap_or(vec![])
        .into_iter()
        .map(|post| Item {
            title: post.title,
            link: format!("http://localhost:8000/post/{}", post.id),
            description: post.summary.unwrap_or("".into()),
            pub_date: post.time.unwrap().to_string(),
            guid: Guid {
                value: post.id.to_string(),
                is_permalink: false,
            },
        })
        .collect();

    let rss_data = RssData {
        version: "1.0".to_string(),
        channel: Channel {
            title: "just-plain.fun".to_string(),
            link: "https://just-plain.fun".to_string(),
            description: "".to_string(),
            language: "zh-cn".to_string(),
            pub_date: "".to_string(),
            last_build_date: "".to_string(),
            managing_editor: "".to_string(),
            web_master: "".to_string(),
            items: items,
        },
    };
    let resp = rss_data.to_string().unwrap_or("".into());
    Ok(HttpResponse::Ok().content_type("text/xml").body(resp))
}
