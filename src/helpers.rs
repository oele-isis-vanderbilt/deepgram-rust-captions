use chrono::{Duration, NaiveTime};

pub fn chunk_array<T>(array: Vec<T>, chunk_size: usize) -> Vec<Vec<T>> {
    let mut chunks = vec![];
    let mut chunk = vec![];
    for item in array {
        if chunk.len() == chunk_size {
            chunks.push(chunk);
            chunk = vec![];
        }
        chunk.push(item);
    }
    if !chunk.is_empty() {
        chunks.push(chunk);
    }
    chunks
}

pub fn seconds_to_timestamp(seconds: f64, format: &str) -> String {
    // Round the seconds to three decimal places
    let seconds = (seconds * 1000.0).round() / 1000.0;

    let duration = Duration::milliseconds((seconds * 1000.0) as i64);

    // Create a NaiveTime from the duration
    let time = NaiveTime::from_num_seconds_from_midnight_opt(
        duration.num_seconds() as u32 % 86400, // Modulo 86400 to ensure it wraps around 24 hours
        (duration.num_nanoseconds().unwrap_or(0) % 1_000_000_000) as u32,
    )
    .expect("Invalid time");

    // Format the time as HH:MM:SS,mmm
    time.format(format).to_string()
}
