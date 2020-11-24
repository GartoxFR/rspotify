//! All objects related to artist defined by Spotify API
use serde::{Deserialize, Serialize};

/// Audio Feature Object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/object-model/#audio-features-object)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AudioFeatures {
    pub acousticness: f32,
    pub analysis_url: String,
    pub danceability: f32,
    pub duration_ms: u32,
    pub energy: f32,
    pub id: String,
    pub instrumentalness: f32,
    pub key: i32,
    pub liveness: f32,
    pub loudness: f32,
    pub mode: f32,
    pub speechiness: f32,
    pub tempo: f32,
    pub time_signature: i32,
    pub track_href: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub uri: String,
    pub valence: f32,
}

/// Audio feature object wrapped by `Vec`
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/tracks/get-several-audio-features/)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub(in crate) struct AudioFeaturesPayload {
    pub audio_features: Vec<AudioFeatures>,
}

/// Audio analysis object
///
/// [Reference](https://developer.spotify.com/web-api/get-audio-analysis/)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AudioAnalysis {
    pub bars: Vec<TimeInterval>,
    pub beats: Vec<TimeInterval>,
    pub meta: AudioAnalysisMeta,
    pub sections: Vec<AudioAnalysisSection>,
    pub segments: Vec<AudioAnalysisSegment>,
    pub tatums: Vec<TimeInterval>,
    pub track: AudioAnalysisTrack,
}

/// Time interval object
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/tracks/get-audio-analysis/#time-interval-object)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TimeInterval {
    pub start: f32,
    pub duration: f32,
    pub confidence: f32,
}

/// Audio analysis section object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/tracks/get-audio-analysis/#section-object)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AudioAnalysisSection {
    #[serde(flatten)]
    pub time_interval: TimeInterval,
    pub loudness: f32,
    pub tempo: f32,
    pub tempo_confidence: f32,
    pub key: i32,
    pub key_confidence: f32,
    pub mode: f32,
    pub mode_confidence: f32,
    pub time_signature: i32,
    pub time_signature_confidence: f32,
}

/// Audio analysis meta object
///
/// [Reference](https://developer.spotify.com/web-api/get-audio-analysis/)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AudioAnalysisMeta {
    pub analyzer_version: String,
    pub platform: String,
    pub detailed_status: String,
    pub status_code: i32,
    pub timestamp: u64,
    pub analysis_time: f32,
    pub input_process: String,
}
/// Audio analysis segment object
///
///[Reference](https://developer.spotify.com/documentation/web-api/reference/tracks/get-audio-analysis/#segment-object)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AudioAnalysisSegment {
    #[serde(flatten)]
    pub time_interval: TimeInterval,
    pub loudness_start: f32,
    pub loudness_max_time: f32,
    pub loudness_max: f32,
    pub loudness_end: Option<f32>,
    pub pitches: Vec<f32>,
    pub timbre: Vec<f32>,
}

/// Audio analysis track object
///
/// [Reference](https://developer.spotify.com/web-api/get-audio-analysis/)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AudioAnalysisTrack {
    pub num_samples: u32,
    pub duration: f32,
    pub sample_md5: String,
    pub offset_seconds: u32,
    pub window_seconds: u32,
    pub analysis_sample_rate: i32,
    pub analysis_channels: u32,
    pub end_of_fade_in: f32,
    pub start_of_fade_out: f32,
    pub loudness: f32,
    pub tempo: f32,
    pub tempo_confidence: f32,
    pub time_signature: i32,
    pub time_signature_confidence: f32,
    pub key: u32,
    pub key_confidence: f32,
    pub mode: f32,
    pub mode_confidence: f32,
    pub codestring: String,
    pub code_version: f32,
    pub echoprintstring: String,
    pub echoprint_version: f32,
    pub synchstring: String,
    pub synch_version: f32,
    pub rhythmstring: String,
    pub rhythm_version: f32,
}
