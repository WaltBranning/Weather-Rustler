use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use crate::schema::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum TimeAggregate {
    Minute,
    TenMinute,
    Hour,
    FourHour,
    Day,
    Week
}

#[derive(Queryable, Serialize)]
pub struct WeatherData {
    pub id: i32,
    pub timestamp: NaiveDateTime,
    pub source_device: i32,
    pub humidity: f32,
    pub temperature: f32,
    pub pressure: f32,
    pub time_frame: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = weather_data)]
pub struct NewWeatherDataPoint {
    pub source_device: i32,
    pub humidity: f32,
    pub temperature: f32,
    pub pressure: f32,
    // pub time_frame: String,
}