#[macro_use]
extern crate rocket;
 
extern crate diesel;

mod models;
mod schema;

use std::env;

use crate::models::*;
use crate::models::WeatherData;

use diesel::prelude::*;
use diesel::SqliteConnection;
use dotenvy::dotenv;
use rocket::fairing::Fairing;
use rocket::fairing::Info;
use rocket::fairing::Kind;
use rocket::http::{Header, Method, Status};
use rocket::serde::json::Json;
use rocket::Request;
use rocket::Response;
use serde::Serialize;

#[derive(Serialize)]
pub struct StatusResponse {
    status: i32,
    msg: String
}



#[post("/weatherdata", format="json", data = "<new_weather_data_point>")]
pub fn log_data_point(new_weather_data_point: Json<NewWeatherDataPoint>) -> Json<StatusResponse> {
    dotenv().ok();
    let connection_path = env::var("DATABASE_URL").expect("DATABASE_URL must be set in a .env config file!");
    let mut connection = SqliteConnection::establish(&connection_path).expect("Couldn't connect to database.");

    // let mut db_conn = || {SqliteConnection::establish("/home/walter/Projects/Rust/weather-rustler-api/data/weatherdata.db").expect("Couldn't connect to database.")};

    let nwp = new_weather_data_point.into_inner();
    println!("{:?}", nwp);
    diesel::insert_into(schema::weather_data::dsl::weather_data)
        .values(&nwp)
        .execute(&mut connection)
        .expect("Error Inserting Weather Data Point");

    Json(StatusResponse{status:200, msg:"Successfully added data log".to_string()})
}

#[get("/weatherdata?<log_window>")]
pub fn get_weather_data(log_window: Option<String>) -> Json<Vec<WeatherData>> {
    dotenv().ok();
    let connection_path = env::var("DATABASE_URL").expect("DATABASE_URL must be set in a .env config file!");
    println!("{}", &connection_path);
    let mut connection = SqliteConnection::establish(&connection_path).expect("Couldn't connect to database.");

    match log_window {
        Some(_log_window) => {
            let log_results = self::schema::weather_data::dsl::weather_data
                .load::<WeatherData>(&mut connection)
                .expect("Error loading weather data");
            Json(log_results)
        },
        None => {
            let log_results = self::schema::weather_data::dsl::weather_data
                .load::<WeatherData>(&mut connection)
                .expect("Error loading weather data");
            Json(log_results)
        }
    }
}

 pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if request.method() == Method::Options {
            response.set_status(Status::NoContent);
            response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "POST, PATCH, GET, DELETE",
            ));
            response.set_header(Header::new("Access-Control-Allow-HEADER", "*"));
        }
        response.set_header(Header::new("Access-Control-Allow-ORIGIN", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // let pool = SqlitePool::connect("sqlite://weatherdata.db").await.expect("Couldn't connect to the sqlite database");

    let _rocket = rocket::build()
        .attach(CORS)
        .mount("/api", routes![log_data_point, get_weather_data])
        .launch()
        .await?;

    Ok(())
}
