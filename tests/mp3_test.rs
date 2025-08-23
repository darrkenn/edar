use edar::{Extractor, Mp3Extractor};

#[test]
fn test_mp3_extractor() {
    let metadata = Mp3Extractor::extract_metadata("test_song.mp3").unwrap();

    assert_eq!("Af1s", metadata.title.unwrap());
}
