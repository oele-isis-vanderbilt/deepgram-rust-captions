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

        let start_time = seconds_to_timestamp(first_word.start, "%H:%M:%S,%f");
        let end_time = seconds_to_timestamp(last_word.end, "%H:%M:%S,%f");

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::converters::DeepgramConverter;
    use regex::Regex;

    fn test_srt_format(converter: &DeepgramConverter) {
        let result = srt(converter, None);

        result
            .split("\n\n")
            .into_iter()
            .enumerate()
            .for_each(|(index, caption)| {
                if !caption.trim().is_empty() {
                    let lines = caption.split("\n").collect::<Vec<&str>>();
                    // timestamp_pattern = r"\d{2}:\d{2}:\d{2},\d{3} --> \d{2}:\d{2}:\d{2},\d{3}"
                    // assert (
                    //     re.match(timestamp_pattern, lines[1]) is not None
                    // ), f"Timestamp format is incorrect: {lines[1]}"

                    let timestamp_pattern = r"\d{2}:\d{2}:\d{2},\d{3} --> \d{2}:\d{2}:\d{2},\d{3}";
                    let re = Regex::new(timestamp_pattern).expect("Failed to create regex pattern");

                    assert!(
                        re.is_match(&lines[1]),
                        "Timestamp format is incorrect: {}",
                        &lines[1]
                    );
                }
            });
    }

    #[test]
    fn test_srt_format_no_utterances() {
        let dg_response = DeepgramConverter::from_path("test/files/dg_speakers_no_utterances.json");
        test_srt_format(&dg_response);
    }
}
