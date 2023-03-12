use mongodb::{options::ClientOptions, Client, Database};


pub async fn print_hello()->Result<(), String> {
    println!("Hello, world!");
    Ok(())
}

pub async fn connect(url: String) -> Result<Client, mongodb::error::Error> {
    // Set up the client options and connect to MongoDB
    let client_options = ClientOptions::parse(url).await?;

    let client = Client::with_options(client_options)?;
    Ok(client)
}

pub async fn gell_all_db(client: &Client) -> Result<Vec<String>, mongodb::error::Error> {
    // List the names of the databases in that deployment.
    let list = client.list_database_names(None, None).await?;
    print!("sds");
    Ok(list)
}

pub async fn select_db(client: &Client, db_name: &str) -> Result<Database, mongodb::error::Error> {
    let db = client.database(db_name);
    Ok(db)
}

// pub async fn select_collection(
//     client: &Client,
//     db: String,
//     col: String,
// ) -> Result<(), mongodb::error::Error> {
//     let collection = db.collection::<Document>("books");
//     Ok(())
// }

// // Get a handle to a collection in the database.
// let collection = db.collection::<Document>("books");

// let docs = vec![
//     doc! { "title": "1984", "author": "George Orwell" },
//     doc! { "title": "Animal Farm", "author": "George Orwell" },
//     doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
// ];

// Insert some documents into the "mydb.books" collection.
// collection.insert_many(docs, None).await?;
