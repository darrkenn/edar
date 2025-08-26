use lofty::{
    file::{AudioFile, TaggedFileExt},
    tag::Accessor,
};

use crate::{Metadata, metadata::MetadataError};

pub struct Extractor;
impl Extractor {
    pub fn extract_metadata(file_path: &str) -> Result<Metadata, MetadataError> {
        let file = lofty::read_from_path(file_path).map_err(|_| MetadataError::ReadError)?;

        let properties = file.properties();

        let tag = file.primary_tag().or_else(|| file.first_tag());

        let metadata = if let Some(tag) = tag {
            let picture = tag.pictures().first().map(|p| p.data().to_vec());
            Metadata {
                title: tag.title().map(|t| t.to_string()),
                artist: tag.artist().map(|a| a.to_string()),
                album: tag.album().map(|a| a.to_string()),
                year: tag.year().map(|y| y.to_string()),
                genre: tag.genre().map(|g| g.to_string()),
                cover: picture,
                duration: Some(properties.duration()),
                sample_rate: properties.sample_rate(),
                channels: properties.channels(),
                has_tag: true,
            }
        } else {
            Metadata {
                title: None,
                artist: None,
                album: None,
                year: None,
                genre: None,
                cover: None,
                duration: Some(properties.duration()),
                sample_rate: properties.sample_rate(),
                channels: properties.channels(),
                has_tag: false,
            }
        };

        Ok(metadata)
    }
}
