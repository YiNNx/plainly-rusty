use crate::config::global_config;
use crate::entities::{posts, prelude::Posts};
use crate::utilities::rss::*;
use actix_web::{web, HttpResponse, Result};
use sea_orm::*;

pub async fn rss(db: web::Data<sea_orm::DatabaseConnection>) -> Result<HttpResponse> {
    let link = &global_config().application.url;
    let items: Vec<Item> = Posts::find()
        .filter(posts::Column::Status.eq("PUBLIC"))
        .order_by_desc(posts::Column::Time)
        .all(db.as_ref())
        .await
        .unwrap_or(vec![])
        .into_iter()
        .map(|post| {
            let post_link = format!("{}/post/{}", link, post.id);
            let mut html_output = String::new();
            pulldown_cmark::html::push_html(
                &mut html_output,
                pulldown_cmark::Parser::new(&post.content),
            );
            Item {
                title: post.title,
                link: post_link.clone(),
                description: CDATA { value: html_output },
                pub_date: post.time.unwrap().to_string(),
                guid: Guid {
                    value: post_link.clone(),
                    is_permalink: false,
                },
                category: "".into(),
                source: Source {
                    url: post_link,
                    text: global_config().rss_channel.title.clone(),
                },
            }
        })
        .collect();
    let rss_data = Rss {
        version: "2.0".into(),
        channel: Channel {
            title: global_config().rss_channel.title.clone(),
            link: link.clone(),
            item: items,
            description: CDATA {
                value: global_config().rss_channel.description.clone(),
            },
            copyright: global_config().rss_channel.copyright.clone(),
            ttl: 5,
        },
    };
    match rss_data.to_string() {
        Ok(resp) => Ok(HttpResponse::Ok().content_type("text/xml").body(resp)),
        Err(e) => Ok(HttpResponse::InternalServerError()
            .content_type("text/xml")
            .body(e.to_string())),
    }
}
