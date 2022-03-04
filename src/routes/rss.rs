use actix_web::{web, Responder, Result};
use feed_rs::model::Feed;
use feed_rs::parser;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
pub struct PostBody {
    url: String,
}

async fn get_feed(url: &str) -> Result<Feed> {
    let content = reqwest::get(url)
        .await
        .expect("Could not get request")
        .bytes()
        .await
        .expect("Could not get bytes");

    Ok(parser::parse(&content[..]).expect("Could not parse feed"))
}

pub async fn feed_info(body: web::Json<PostBody>) -> Result<impl Responder> {
    let feed = get_feed(&body.url).await.expect("Could not get feed");

    let title_content = feed.title.expect("Could not get title");
    let title = title_content.content;

    let description_content = feed.description.expect("Could not get description");
    let description = description_content.content;

    let updated_content = feed.updated;
    let updated = match updated_content {
        Some(content) => content.to_rfc3339(),
        None => "".to_string(),
    };

    let entries = feed.entries;
    let mut data: Vec<EntryObj> = Vec::new();

    for entry in entries {
        data.push(EntryObj {
            id: entry.id,
            title: entry.title.unwrap().content,
            published: entry.published.unwrap().to_rfc3339(),
        });
    }

    let feed = json!({
        "title": title,
        "description": description,
        "updated": updated,
        "entries": data
    });

    Ok(web::Json(feed))
}

#[derive(Serialize)]
struct EntryObj {
    id: String,
    title: String,
    published: String,
}
