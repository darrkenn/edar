use std::time::Duration;

pub mod extractor;

pub use extractor::Extractor;

#[derive(Debug, Clone)]
pub struct Metadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub year: Option<String>,
    pub genre: Option<String>,
    pub cover: Option<Vec<u8>>,

    pub duration: Option<Duration>,
    pub sample_rate: Option<u32>,
    pub channels: Option<u8>,
}
#[derive(Debug)]
pub enum MetadataError {
    UnsupportedFormat,
    ReadError,
    ExtensionError,
    NoTag,
}
