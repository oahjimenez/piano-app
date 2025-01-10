use actix::Response;
use actix_cors::{Cors, CorsError};
use actix_web::{error::{HttpError, JsonPayloadError}, get, middleware, web::{self, get}, App, HttpServer, Responder, Result};
use bson::{doc, oid::ObjectId, Bson, Document};
use chrono::{TimeZone, Utc};
use mongodb::{error::Error, options::{ClientOptions, ResolverConfig}, Client};
use serde::{Deserialize, Serialize};
use std::{env, ptr::null};
use tokio;
use std::fmt;
use dotenv::dotenv;

#[derive(Serialize, Deserialize, Debug)]
struct Movie {

   #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
   id: Option<ObjectId>,
   title : String,
   year: i32,
   plot: String,
   #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
   released: chrono::DateTime<Utc>

}

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Movie {
   // This trait requires `fmt` with this exact signature.
   fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
       // Write strictly the first element into the supplied output
       // stream: `f`. Returns `fmt::Result` which indicates whether the
       // operation succeeded or failed. Note that `write!` uses syntax which
       // is very similar to `println!`.
       write!(formatter, "MyMovie {} {} {} {}", self.title, self.year, self.plot, self.released)
   }
}


#[get("/movies")]
pub async fn movie_list() -> Result<impl Responder, HttpError> {
   let movie = get_movie().await .expect("Could not get yaya movie");
   Ok(web::Json(movie))
}

async fn get_movie() -> Result<Movie,Error> {
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

   let _new_doc = doc! {
      "title": "Yaya's",
      "year": 2025,
      "plot": "A sassy yaya.",
      "released": Utc.ymd(2020, 2, 7).and_hms_opt(0, 0, 0),
   };

   //let insert_result = movies.insert_one(new_doc.clone(), None).await?;
   //println!("New document ID: {}", insert_result.inserted_id);

   let loaded_movie = movies.find_one(Some(
      doc ! {
         "title" : "Yaya's"
      })
      , None).await ? .expect("Error retrieving yaya film");

   let yaya_movie: Movie = bson::from_bson(Bson::Document(loaded_movie))?;

   println!("Yaya movie: {}",yaya_movie);
   return Ok(yaya_movie);

}


#[tokio::main]
pub async fn main() -> std::io::Result<()> {
   use actix_web::{App, HttpServer};

   HttpServer::new(|| { 
      let cors = Cors::default()
      .allowed_origin("http://localhost")
      .allowed_origin("http://localhost:8080")
      .allowed_origin("http://localhost:3000")
      .allowed_methods(vec!["GET", "POST"])
      .max_age(36000);

      App::new()
      .wrap(cors)
      .service(movie_list) 
   
   })
       .bind(("127.0.0.1", 8080))?
       .run()
       .await
}
