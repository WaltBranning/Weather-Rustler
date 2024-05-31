// @generated automatically by Diesel CLI.

diesel::table! {
    weather_data (id) {
        id -> Integer,
        timestamp -> Timestamp,
        source_device -> Integer,
        humidity -> Float,
        temperature -> Float,
        pressure -> Float,
        time_frame -> Text,
    }
}
