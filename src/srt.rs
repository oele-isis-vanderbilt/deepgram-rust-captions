use crate::{converters::DeepgramConverter, helpers::seconds_to_timestamp};

pub fn srt(converter: &DeepgramConverter, line_length: Option<u8>) -> String {
    let mut output = Vec::<String>::new();

    let line_length = line_length.unwrap_or(8);

    let lines = converter.get_lines(line_length);

    let mut entry = 1;

    let mut current_speaker = None;

    for words in lines {
        output.push(entry.to_string());
        entry += 1;

        let first_word = words.first().unwrap();
        let last_word = words.last().unwrap();

        let start_time = seconds_to_timestamp(first_word.start, "%H:%M:%S,%3f");
        let end_time = seconds_to_timestamp(last_word.end, "%H:%M:%S,%3f");

        output.push(format!("{} --> {}", start_time, end_time));

        if first_word.speaker.is_some() {
            if current_speaker.is_none() || current_speaker != first_word.speaker {
                current_speaker = first_word.speaker;
                output.push(format!("[speaker {}]", current_speaker.unwrap()));
            }
        }

        let punctuated_words = words
            .iter()
            .map(|word| {
                word.punctuated_word
                    .as_ref()
                    .unwrap_or(&word.word)
                    .to_string()
            })
            .collect::<Vec<String>>();

        output.push(punctuated_words.join(" "));
        output.push("".to_string());
    }

    output.join("\n")
}
