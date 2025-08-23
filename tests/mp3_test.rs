use std::time::Duration;

use edar::{Extractor, FormatDuration};

#[test]
fn test_mp3_extractor() {
    let metadata = Extractor::extract_metadata("tests/assets/test_song.mp3").unwrap();
    assert_eq!("A_Song", metadata.title.unwrap());
    assert_eq!("darrkenn", metadata.artist.unwrap());
    assert_eq!("A_Album", metadata.album.unwrap());
    assert_eq!("2020", metadata.year.unwrap());
    assert_eq!("A_Genre", metadata.genre.unwrap());
    assert!(metadata.cover.is_some());
    let duration = Duration::from_secs(1) + Duration::from_millis(272);

    assert_eq!(duration, metadata.duration.unwrap());

    assert_eq!(48000, metadata.sample_rate.unwrap());
    assert_eq!(1, metadata.channels.unwrap());

    assert_eq!("00:01", metadata.duration.format());
}

#[test]
fn test_ogg_extractor() {
    let metadata = Extractor::extract_metadata("tests/assets/test_song.ogg").unwrap();
    assert_eq!("A_Song", metadata.title.unwrap());
    assert_eq!("darrkenn", metadata.artist.unwrap());
    assert_eq!("A_Album", metadata.album.unwrap());
    assert_eq!("2020", metadata.year.unwrap());
    assert_eq!("A_Genre", metadata.genre.unwrap());
    assert!(metadata.cover.is_some());
    let duration = Duration::from_secs(1) + Duration::from_millis(272);

    assert_eq!(duration, metadata.duration.unwrap());

    assert_eq!(48000, metadata.sample_rate.unwrap());
    assert_eq!(1, metadata.channels.unwrap());

    assert_eq!("00:01", metadata.duration.format());
}

#[test]
fn test_flac_extractor() {
    let metadata = Extractor::extract_metadata("tests/assets/test_song.flac").unwrap();
    assert_eq!("A_Song", metadata.title.unwrap());
    assert_eq!("darrkenn", metadata.artist.unwrap());
    assert_eq!("A_Album", metadata.album.unwrap());
    assert_eq!("2020", metadata.year.unwrap());
    assert_eq!("A_Genre", metadata.genre.unwrap());
    assert!(metadata.cover.is_some());
    let duration = Duration::from_secs(1) + Duration::from_millis(272);

    assert_eq!(duration, metadata.duration.unwrap());

    assert_eq!(48000, metadata.sample_rate.unwrap());
    assert_eq!(1, metadata.channels.unwrap());

    assert_eq!("00:01", metadata.duration.format());
}
