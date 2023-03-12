use dotenv::dotenv;
use futures::stream::TryStreamExt;
use mongodb::bson::{doc, Document};
use mongodb::options::FindOptions;
use serde::{Deserialize, Serialize};

use std::env;
use tokio;

#[path = "./db/mongo_db.rs"]
mod mongo_db;

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let MONGO_DB_URL = env::var("MONGO_DB_URL").unwrap();
    let DB_NAME = env::var("DB_NAME").unwrap();

    let client = mongo_db::connect(MONGO_DB_URL).await?;

    let dbList = mongo_db::gell_all_db(&client).await?;

    for db_name in dbList {
        println!("{}", db_name);
    }

    let selected_db = mongo_db::select_db(&client, &DB_NAME).await?;

    let col: mongodb::Collection<()> = selected_db.collection("books");

    // Query the books in the collection with a filter and an option.
    let filter = doc! { "author": "George Orwell" };

    let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
    let mut result = col.find(filter, find_options).await?;



    Ok(())
}
