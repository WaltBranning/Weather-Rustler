-- Your SQL goes here
CREATE TABLE IF NOT EXISTS weather_data (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, 
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL, 
    source_device INTEGER NOT NULL,
    humidity REAL NOT NULL, 
    temperature REAL NOT NULL, 
    pressure REAL NOT NULL,
    time_frame TEXT NOT NULL
    );