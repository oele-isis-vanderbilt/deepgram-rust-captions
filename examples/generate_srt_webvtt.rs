use deepgram::{
    transcription::prerecorded::{
        audio_source::AudioSource,
        options::{Language, Model, Options},
    },
    Deepgram, DeepgramError,
};

use deepgram_rust_captions::{converters::DeepgramConverter, srt::srt, webvtt::webvtt};
use tokio::{fs::File, io::AsyncWriteExt};

static AUDIO_PATH: &str = "examples/files/Arthur.mp3";
static SRT_PATH: &str = "examples/files/Arthur-rust.srt";
static WEBVTT_PATH: &str = "examples/files/Arthur-rust.vtt";

#[tokio::main]
async fn main() -> Result<(), DeepgramError> {
    let api_key =
        std::env::var("DEEPGRAM_API_KEY").expect("DEEPGRAM_API_KEY environmental variable");
    let dg_client = Deepgram::new(&api_key);

    let file = File::open(AUDIO_PATH).await?;

    let source = AudioSource::from_buffer_with_mime_type(file, "audio/mpeg3");

    let options = Options::builder()
        .punctuate(true)
        .diarize(true)
        .utterances(true)
        .model(Model::Nova2)
        .language(Language::en_US)
        .build();

    println!("Transcribing {}...", AUDIO_PATH);

    let response = dg_client
        .transcription()
        .prerecorded(source, &options)
        .await?;

    let converter = DeepgramConverter::new(&response);

    let srt = srt(&converter, None);
    let webvtt = webvtt(&converter, None);

    let mut file = File::create(SRT_PATH).await?;

    file.write_all(srt.as_bytes()).await?;

    println!(
        "SRT file generated successfully for {} at: {}",
        AUDIO_PATH, SRT_PATH
    );

    let mut file = File::create(WEBVTT_PATH).await?;

    file.write_all(webvtt.as_bytes()).await?;

    println!(
        "WEBVTT file generated successfully for {} at: {}",
        AUDIO_PATH, WEBVTT_PATH
    );

    Ok(())
}
