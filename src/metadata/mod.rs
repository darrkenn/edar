use std::{path::Path, time::Duration};

use crate::metadata::mp3::Mp3Extractor;

pub mod flac;
pub mod mp3;
pub mod ogg;
pub mod wav;

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

pub trait Extractor {
    fn extract_metadata(file_path: &str) -> Result<Metadata, MetadataError>;
}

pub fn extract(file_path: &str) -> Result<Metadata, MetadataError> {
    let extension = Path::new(file_path)
        .extension()
        .and_then(|e| e.to_str())
        .ok_or(MetadataError::ExtensionError)?;

    match extension {
        "mp3" => Mp3Extractor::extract_metadata(file_path),
        _ => todo!(),
    }
}
