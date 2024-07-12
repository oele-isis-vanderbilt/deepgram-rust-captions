use deepgram::transcription::prerecorded::response::{Response as DeepgramResponse, Word};

use crate::helpers::chunk_array;

#[derive(Debug, Clone)]
pub struct DeepgramConverter {
    pub response: DeepgramResponse,
}

impl DeepgramConverter {
    pub fn new(dg_response: &DeepgramResponse) -> Self {
        Self {
            response: dg_response.clone(),
        }
    }

    pub fn from_str(json_response: &str) -> Self {
        let dg_response: DeepgramResponse = serde_json::from_str(json_response).unwrap();
        Self::new(&dg_response)
    }

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

    pub fn get_headers(&self) -> Vec<String> {
        vec![
            "NOTE".to_string(),
            "Transcription provided by Deepgram".to_string(),
            self.response.metadata.request_id.to_string(),
            self.response.metadata.created.to_string(),
            self.response.metadata.duration.to_string(),
            self.response.metadata.channels.to_string(),
        ]
    }
}
