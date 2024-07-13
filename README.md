# Deepgram Rust Captions

This package is the Rust implementation of Deepgram's WebVTT and SRT formatting. Given a transcription, this package can return a valid string to store as WebVTT or SRT caption files.

This implmentation is based (a direct port) on Deepgram's Python implementation of the same functionality. The original Python implementation can be found [here](https://github.com/deepgram/deepgram-python-captions/tree/main).

## Installation
Use cargo to install the package:

```bash
cargo add deepgram-rust-captions
```

## Usage
```rust
use deepgram_rust_captions::{converters::DeepgramConverter, srt::srt, webvtt::webvtt};
use deepgram::transcription::prerecorded::Response as DGResponse;


fn main() {
    let response: DGResponse  = ... 
    let converter = DeepgramConverter::new(&response);
    let srt_caption = srt(&converter, Some(10));
    let webvtt_caption = webvtt(&converter, Some(10));
    println!("{}", srt_caption);
    println!("{}", webvtt_caption);
}
```
