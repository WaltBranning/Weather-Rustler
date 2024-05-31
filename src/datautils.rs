use chrono::{Days, NaiveDateTime};
use diesel::{
    deserialize::FromSqlRow, 
    dsl::{avg, max}, 
    prelude::*, 
    result::Error, 
    sql_types::{self, Double, SqlType}, 
    QueryDsl, 
    RunQueryDsl, 
    SqliteConnection,
};


use crate::schema::{self, weather_data::{self, *}};

// #[derive(Debug, Selectable)]
#[derive(Debug, FromSqlRow)]
// #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
// #[diesel(table_name = weather_data)]
pub struct WeatherDataFields {
    pub temperature: Option<weather_data::temperature>,
    pub humidity: Option<weather_data::humidity>,
    pub pressure: Option<weather_data::pressure>,
}

#[derive(Queryable, Debug)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = weather_data)]
pub struct Timestamps {
    timestamp: Option<NaiveDateTime>,
    time_frame: String,
}

#[derive(Default, Debug)]
struct TimeFrames {
    minute: Option<NaiveDateTime>,
    hour: Option<NaiveDateTime>,
    day: Option<NaiveDateTime>,
}

impl TimeFrames {
    fn get_timeframes(&mut self, timestamps: Result<Vec<Timestamps>, Error> ) -> &mut Self {
        for ts in timestamps.expect("Didn't return timestamp") {
            
            match ts.time_frame.as_str() {
                "Minute" => {
                    self.minute = ts.timestamp;
                    // print!("Minute timeframe max value is {:?}", ts.timestamp.unwrap());
                    // if ts.timestamp.unwrap() > 
                },
                "Hour" => {
                    self.hour = ts.timestamp;
                    // print!("Hour timeframe max value is {:?}", ts.timestamp)
                },
                "Day" => {
                    self.day = ts.timestamp;
                    // print!("Day timeframe max value is {:?}", ts.timestamp)
                },
                &_ => print!("No timeframe specified")
            }
    
        }
        self
    }

    fn check_timestamps(&mut self, connection: &mut SqliteConnection) {
        if self.hour > self.day {
            print!("The max hour is greater than day");
            let get_avg_hour = weather_data::dsl::weather_data
                .select((
                    avg(weather_data::temperature),
                    avg(weather_data::humidity),
                    avg(weather_data::pressure)
                ))
                .filter(time_frame.eq("Hour"))
                .filter(timestamp.between(self.hour.unwrap_or_default() + Days::new(7), self.day.unwrap_or_default()))
                .load::<WeatherDataFields>(connection);
        }
        if self.minute.unwrap_or_default() + Days::new(7) > self.hour.unwrap_or_default() {
            print!("The max minute is greater than hour")
        }
    }
    
}

pub fn get_max_timestamps(connection: &mut SqliteConnection) {

    let timestamps = schema::weather_data::dsl::weather_data
        .group_by(weather_data::time_frame)
        .select((max(timestamp).nullable(), time_frame))
        .load::<Timestamps>(connection);

    let mut timeframe = TimeFrames::default();
    timeframe
        .get_timeframes( timestamps)
        .check_timestamps(connection);

    // println!("{:?}", timeframe);

    // for ts in timestamps.expect("Didn't return timestamp") {

    //     match ts.time_frame.unwrap().as_str() {
    //         "Minute" => {
    //             print!("Minute timeframe max value is {:?}", ts.timestamp.unwrap());
    //             // if ts.timestamp.unwrap() > 
    //         },
    //         "Hour" => print!("Hour timeframe max value is {:?}", ts.timestamp.unwrap()),
    //         "Day" => print!("Day timeframe max value is {:?}", ts.timestamp.unwrap()),
    //         &_ => print!("No timeframe specified")
    //     }

    // }

    // print!("{:?}", timestamps);
    // timestamps
}