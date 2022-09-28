use rolling_delta::get_file_content;
use rolling_delta::rolling_hash::rolling::Rolling;
use std::str::{self};

//GET ROLLING STRUCT
pub fn get_rolling() -> Rolling {
    let v1 = get_file_content("test.txt".to_string()).unwrap();
    let v2 = get_file_content("testV2.txt".to_string()).unwrap();
    let v2_bytes = v2.as_bytes();

    let rolling = Rolling::new(v1, v2_bytes);

    rolling
}

//TEST THAT THE FILE BUILD WITH DELTA HAVE THE SAME TEXT AS THE UPDATED FILE
#[test]
fn test_generated_file() {
    let rolling = get_rolling();
    let v2 = get_file_content("testV2.txt".to_string()).unwrap();

    assert_eq!(rolling.generate_new_version_with_delta().unwrap(), v2);
}

//TEST THAT THE DIFFERENCES BETWEEN THE UPDATED FILE AND THE FILE GENERATED WITH DELTA ARE THE EXPECTED ONES
#[test]
fn test_file_differences() {
    let rolling = get_rolling();

    let mut string_chunks: Vec<String> = vec![];

    for delta in rolling.delta {
        if delta.bytes.len() > 0 {
            string_chunks.push(str::from_utf8(&delta.bytes).unwrap().to_string())
        }
    }

    assert_eq!(
        string_chunks[0],
        "tten here but different to tes".to_string()
    );
    assert_eq!(string_chunks[1], "king properly".to_string());
}
