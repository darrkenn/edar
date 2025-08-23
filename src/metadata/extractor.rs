use lofty::{
    file::{AudioFile, TaggedFileExt},
    tag::Accessor,
};

use crate::{Metadata, metadata::MetadataError};

pub struct Extractor;
impl Extractor {
    pub fn extract_metadata(file_path: &str) -> Result<Metadata, MetadataError> {
        let file = lofty::read_from_path(file_path).map_err(|_| MetadataError::ReadError)?;

        let tag = match file.primary_tag() {
            Some(primary_tag) => primary_tag,
            None => return Err(MetadataError::NoTag),
        };
        let properties = file.properties();

        let picture = tag.pictures().first().map(|p| p.data().to_vec());

        let metadata = Metadata {
            title: tag.title().map(|t| t.to_string()),
            artist: tag.artist().map(|a| a.to_string()),
            album: tag.album().map(|a| a.to_string()),
            year: tag.year().map(|y| y.to_string()),
            genre: tag.genre().map(|g| g.to_string()),
            cover: picture,
            duration: Some(properties.duration()),
            sample_rate: properties.sample_rate(),
            channels: properties.channels(),
        };
        Ok(metadata)
    }
}
