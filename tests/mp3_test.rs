use edar::{Extractor, Mp3Extractor};

#[test]
fn test_mp3_extractor() {
    let metadata =
        Mp3Extractor::extract_metadata("/home/darragh/Music/Yung Lean, Ecco2k - Af1s.mp3").unwrap();

    assert_eq!("Af1s", metadata.title.unwrap());
}
