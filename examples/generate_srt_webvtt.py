import os

import httpx
from deepgram import (
    DeepgramClient,
    DeepgramClientOptions,
    FileSource,
    PrerecordedOptions,
)
from deepgram_captions import DeepgramConverter, srt, webvtt

AUDIO_PATH = "examples/files/Arthur.mp3"
SRT_PATH = "examples/files/Arthur-python.srt"
WEBVTT_PATH = "examples/files/Arthur-python.vtt"


def main():
    try:
        client = DeepgramClient(
            "",
            DeepgramClientOptions(
                api_key=os.environ["DEEPGRAM_API_KEY"],
            ),
        )

        options = PrerecordedOptions(
            model="nova-2",
            language="en-US",
            punctuate=True,
            diarize=True,
            utterances=True,
        )

        with open(AUDIO_PATH, "rb") as f:
            buffer_data = f.read()

        payload: FileSource = {"buffer": buffer_data}
        print(f"Transcribing {AUDIO_PATH}...")
        response = client.listen.prerecorded.v("1").transcribe_file(
            payload, options, timeout=httpx.Timeout(300, connect=10.0)
        )

        converter = DeepgramConverter(response)

        srt_data = srt(converter)
        webvtt_data = webvtt(converter)

        with open(SRT_PATH, "w") as f:
            f.write(srt_data)

        print(f"SRT file generated successfully for {AUDIO_PATH} at {SRT_PATH}")

        with open(WEBVTT_PATH, "w") as f:
            f.write(webvtt_data)

        print(f"WebVTT file generated successfully for {AUDIO_PATH} at {WEBVTT_PATH}")

    except Exception as e:
        print(f"Exception: {e}")


if __name__ == "__main__":
    main()
