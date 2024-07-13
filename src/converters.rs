use deepgram::transcription::prerecorded::response::{Response as DeepgramResponse, Word};

use crate::helpers::chunk_array;

/// Converts a Deepgram response into a format that can be used to generate captions.
#[derive(Debug, Clone)]
pub struct DeepgramConverter {
    pub response: DeepgramResponse,
}

impl DeepgramConverter {
    /// Create a new DeepgramConverter from a DeepgramResponse.
    pub fn new(dg_response: &DeepgramResponse) -> Self {
        Self {
            response: dg_response.clone(),
        }
    }

    /// Create a new DeepgramConverter from a JSON string.
    pub fn from_str(json_response: &str) -> Self {
        let dg_response: DeepgramResponse = serde_json::from_str(json_response).unwrap();
        Self::new(&dg_response)
    }

    /// Create a new DeepgramConverter from a file path.
    pub fn from_path(path: &str) -> Self {
        let file_path = std::path::Path::new(path);
        if !file_path.exists() {
            panic!("File does not exist: {}", path);
        }

        let file = std::fs::File::open(file_path).unwrap();
        let reader = std::io::BufReader::new(file);
        let dg_response: DeepgramResponse = serde_json::from_reader(reader).unwrap();
        Self::new(&dg_response)
    }

    /// Get the lines of words from the response.
    pub fn get_lines(&self, line_length: u8) -> Vec<Vec<Word>> {
        let results = &self.response.results;

        let mut content: Vec<Vec<Word>> = vec![];

        if results.utterances.is_some() {
            let utterances = results.utterances.as_ref().unwrap();
            for utterance in utterances {
                if utterance.words.len() > line_length as usize {
                    let chunks = chunk_array(utterance.words.clone(), line_length as usize);
                    content.extend(chunks);
                } else {
                    content.push(utterance.words.clone());
                }
            }
        } else {
            let words = &results.channels[0].alternatives[0].words;
            let diarize = words[0].speaker.is_some();

            let mut buffer = vec![];
            let mut current_speaker = 0;

            for word in words {
                if diarize && word.speaker.is_some_and(|s| s != current_speaker) {
                    content.push(buffer.clone());
                    buffer.clear();
                }

                if buffer.len() == line_length as usize {
                    content.push(buffer.clone());
                    buffer.clear();
                }

                if diarize {
                    current_speaker = word.speaker.unwrap_or(0);
                }

                buffer.push(word.clone());
            }

            content.push(buffer)
        }

        content
    }

    /// Get the headers for the captions.
    pub fn get_headers(&self) -> Vec<String> {
        vec![
            "NOTE".to_string(),
            "Transcription provided by Deepgram".to_string(),
            format!(
                "Request Id: {}",
                self.response.metadata.request_id.to_string()
            ),
            format!("Created: {}", self.response.metadata.created.to_string()),
            format!("Duration: {}", self.response.metadata.duration.to_string()),
            format!("Channels: {}", self.response.metadata.channels.to_string()),
        ]
    }
}
