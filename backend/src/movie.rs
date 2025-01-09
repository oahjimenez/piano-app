mod movie.rs

pub type Movies = Response<Movie>;

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

#[get("/movies")]
pub async fn list() -> HttpResponse {
    // TODO find the last 50 tweets and return them

    let tweets = Tweets { results: vec![] };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(tweets)
}
