#[cfg(test)]
mod tests {
    use deepgram_rust_captions::{converters::DeepgramConverter, srt::srt};
    use regex::Regex;

    const FILE_PATHS_ALL: &'static [&'static str] = &[
        "tests/files/dg_speakers.json",
        "tests/files/dg_speakers_no_utterances.json",
        "tests/files/dg_transcription.json",
        "tests/files/dg_utterances.json",
        "tests/files/dg_whisper_transcription.json",
    ];

    fn test_srt_format(converter: &DeepgramConverter) {
        let result = srt(converter, None);

        result
            .split("\n\n")
            .into_iter()
            .enumerate()
            .for_each(|(index, caption)| {
                if !caption.trim().is_empty() {
                    let lines = caption.split("\n").collect::<Vec<&str>>();
                    assert!(
                        lines[0] == (index + 1).to_string(),
                        "Entry number is incorrect: {}",
                        lines[0]
                    );

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
    fn test_srt_format_all() {
        FILE_PATHS_ALL
            .iter()
            .map(|path| DeepgramConverter::from_path(path))
            .for_each(|dg_response| {
                test_srt_format(&dg_response);
            });
    }

    #[test]
    fn test_srt_first_caption_number() {
        FILE_PATHS_ALL
            .iter()
            .map(|path| DeepgramConverter::from_path(path))
            .for_each(|dg_response| {
                let result = srt(&dg_response, None);
                let first_caption = result.split("\n\n").next().unwrap();
                assert!(
                    first_caption.starts_with("1"),
                    "First caption does not start with 1: {}",
                    first_caption
                );
            });
    }

    #[test]
    fn test_srt_speaker_format() {
        vec![
            "tests/files/dg_speakers.json",
            "tests/files/dg_speakers_no_utterances.json",
        ]
        .iter()
        .map(|path| DeepgramConverter::from_path(path))
        .for_each(|dg_response| {
            let result = srt(&dg_response, None);
            let speaker_lines = result
                .split("\n")
                .filter(|line| line.starts_with("[speaker"))
                .collect::<Vec<&str>>();
            assert!(
                speaker_lines.len() > 2,
                "No speaker lines found in result: {}",
                result
            );
        });
    }
}
