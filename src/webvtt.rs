use crate::{converters::DeepgramConverter, helpers::seconds_to_timestamp};

pub fn webvtt(converter: &DeepgramConverter, line_length: Option<u8>) -> String {
    let mut output = vec!["WEBVTT".to_string(), "".to_string()];

    let line_length = line_length.unwrap_or(8);

    output.extend(converter.get_headers());

    output.push("".to_string());

    let lines = converter.get_lines(line_length);

    let speaker_labels = lines[0][0].speaker.is_some();

    for words in lines {
        let first_word = words.first().unwrap();
        let last_word = words.last().unwrap();

        let start_time = seconds_to_timestamp(first_word.start, "%H:%M:%S.%3f");
        let end_time = seconds_to_timestamp(last_word.end, "%H:%M:%S.%3f");

        output.push(format!("{} --> {}", start_time, end_time));

        let line = words
            .iter()
            .map(|word| {
                word.punctuated_word
                    .as_ref()
                    .unwrap_or(&word.word)
                    .to_string()
            })
            .collect::<Vec<String>>();

        let speaker_label = if speaker_labels {
            let speaker = words[0].speaker.unwrap();
            format!("<v {}>", speaker)
        } else {
            "".to_string()
        };

        output.push(format!("{}{}", speaker_label, line.join(" ")));
        output.push("".to_string());
    }

    output.join("\n")
}
