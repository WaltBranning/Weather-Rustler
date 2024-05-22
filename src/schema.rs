// @generated automatically by Diesel CLI.

diesel::table! {
    weather_data (id) {
        id -> Integer,
        timestamp -> Timestamp,
        source_device -> Integer,
        humidity -> Nullable<Float>,
        temperature -> Nullable<Float>,
        pressure -> Nullable<Float>,
        time_frame -> Nullable<Text>,
    }
}
