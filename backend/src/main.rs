use bson::doc;
use chrono::{TimeZone, Utc};
use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use std::env;
use std::error::Error;
use tokio;
use dotenv::dotenv;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   dotenv().ok(); // This line loads the environment variables from the ".env" file.

   // Load the MongoDB connection string from an environment variable:
   let client_uri =
      env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
   // A Client is needed to connect to MongoDB:
   // An extra line of code to work around a DNS issue on Windows:
   let options =
      ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
         .await?;
   let client = Client::with_options(options)?;
   // Print the databases in our MongoDB cluster:
   println!("Databases:");
   for name in client.list_database_names(None, None).await? {
      println!("- {}", name);
   }

   // Get the 'movies' collection from the 'sample_mflix' database:
   let movies = client.database("sample_mflix").collection("movies");

   let new_doc = doc! {
      "title": "Yaya's",
      "year": 2025,
      "plot": "A sassy yaya.",
      "released": Utc.ymd(2020, 2, 7).and_hms_opt(0, 0, 0),
   };

   let insert_result = movies.insert_one(new_doc.clone(), None).await?;
   println!("New document ID: {}", insert_result.inserted_id);

   Ok(())
}
