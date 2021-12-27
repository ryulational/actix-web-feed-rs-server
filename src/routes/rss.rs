use actix_web::{web, Responder, Result};
use feed_rs::model::Feed;
use feed_rs::parser;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
pub struct PostBody {
    url: String,
}

async fn get_feed(url: &String) -> Result<Feed> {
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

    let updated_content = feed.updated.expect("Could not get updated");
    let updated = updated_content;

    let feed = json!({
        "title": title,
        "description": description,
        "updated": updated.to_rfc3339()
    });

    Ok(web::Json(feed))
}

#[derive(Serialize)]
struct EntryObj {
    id: String,
    title: String,
    published: String,
}

pub async fn feed_entries(body: web::Json<PostBody>) -> Result<impl Responder> {
    let feed = get_feed(&body.url).await.expect("Could not get feed");
    let entries = feed.entries;
    let mut data: Vec<EntryObj> = Vec::new();

    for entry in entries {
        data.push(EntryObj {
            id: entry.id,
            title: entry.title.unwrap().content,
            published: entry.published.unwrap().to_rfc3339(),
        });
    }

    let json = serde_json::to_string(&data)?;
    Ok(web::Json(json))
}
