use std::{fmt, time::Duration};

pub mod extractor;

pub use extractor::Extractor;

#[derive(Debug, Clone, Default)]
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

impl fmt::Display for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.title.as_deref().unwrap_or(""))?;
        writeln!(f, "{}", self.artist.as_deref().unwrap_or(""))?;
        writeln!(f, "{}", self.album.as_deref().unwrap_or(""))?;
        writeln!(f, "{}", self.year.as_deref().unwrap_or(""))?;
        writeln!(f, "{}", self.genre.as_deref().unwrap_or(""))?;
        Ok(())
    }
}

pub trait FormatDuration {
    fn format(&self) -> String;
}

impl FormatDuration for Option<Duration> {
    fn format(&self) -> String {
        match self {
            Some(duration) => {
                let minutes = duration.as_secs() / 60;
                let seconds = duration.as_secs() % 60;
                format!("{:02}:{:02}", minutes, seconds)
            }
            None => "00:00".to_string(),
        }
    }
}
