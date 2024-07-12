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

    let milliseconds = Duration::milliseconds((seconds * 1000.0) as i64).num_milliseconds();

    println!("seconds: {}, milliseconds: {}", seconds, milliseconds);
    // Convert the seconds into a datetime object
    let dt = NaiveTime::from_hms_milli_opt(0, 0, 0, milliseconds as u32).unwrap();

    // Format the datetime
    let formatted_time = dt.format(format).to_string();

    // Modify the formatted time to strip trailing zeros in milliseconds
    let len = formatted_time.len();
    let formatted_time = if len > 3 {
        let (main, millis) = formatted_time.split_at(len - 3);
        format!("{}{}", main, millis.trim_end_matches('0'))
    } else {
        formatted_time
    };

    formatted_time
}
