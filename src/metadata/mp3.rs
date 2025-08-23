use lofty::{
    file::{AudioFile, TaggedFileExt},
    tag::Accessor,
};

use crate::metadata::{Extractor, Metadata, MetadataError};

pub struct Mp3Extractor;

impl Extractor for Mp3Extractor {
    fn extract_metadata(file_path: &str) -> Result<Metadata, MetadataError> {
        let file = lofty::read_from_path(file_path).map_err(|_| MetadataError::ReadError)?;

        let tag = match file.primary_tag() {
            Some(primary_tag) => primary_tag,
            None => return Err(MetadataError::NoTag),
        };
        let properties = file.properties();

        let duration = properties.duration();
        let sample_rate = properties.sample_rate();

        let channels = properties.channels();

        let metadata = Metadata {
            title: tag.title().map(|t| t.to_string()),
            artist: tag.artist().map(|a| a.to_string()),
            album: tag.album().map(|a| a.to_string()),
            year: tag.year().map(|y| y.to_string()),
            genre: tag.genre().map(|g| g.to_string()),
            cover: None,
            duration: Some(duration),
            sample_rate: sample_rate,
            channels: channels,
        };
        Ok(metadata)
    }
}
