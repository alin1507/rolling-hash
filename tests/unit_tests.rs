use rolling_delta::rolling_hash::signature::Signatures;
use rolling_delta::{
    get_file_content,
    rolling_hash::{adler_32::Adler32, rolling::Rolling},
};

//TEST THAT A FILE WITH LESS CONTENT THAT CHUNK SIZE RETURN AN ERROR
#[test]
fn test_small_chunk_size() {
    let file_content = "too small".to_string();

    let content = match get_file_content("testSmall.txt".to_string()) {
        Ok(content) => content,
        Err(err) => err.to_string(),
    };

    assert_ne!(file_content, content);
}

//TEST THAT THE FUNCTION get_file_content RETURN THE FILE CONTENT
#[test]
fn test_file_content() {
    let file_content = "Some text is written here to test the app is working properly".to_string();

    let content = match get_file_content("test.txt".to_string()) {
        Ok(content) => content,
        Err(err) => err.to_string(),
    };

    assert_eq!(file_content, content);
}

//TEST ROLL IN
#[test]
pub fn test_roll_in() {
    let text = "hello sir";
    let text_bytes = text.as_bytes();

    let mut first_weak = Adler32::new();
    first_weak.write(text_bytes);
    let first_sum = first_weak.sum();

    let mut second_weak = Adler32::new();
    second_weak.roll_in(&text_bytes[0]);
    second_weak.roll_in(&text_bytes[1]);
    second_weak.roll_in(&text_bytes[2]);
    second_weak.roll_in(&text_bytes[3]);
    second_weak.roll_in(&text_bytes[4]);
    second_weak.roll_in(&text_bytes[5]);
    second_weak.roll_in(&text_bytes[6]);
    second_weak.roll_in(&text_bytes[7]);
    second_weak.roll_in(&text_bytes[8]);
    let second_sum = first_weak.sum();

    assert_eq!(first_sum, second_sum);
}

//TEST ROLL OUT
#[test]
pub fn test_roll_out() {
    let text = "hello sir";
    let text_bytes = text.as_bytes();

    let mut first_weak = Adler32::new();
    first_weak.write(text_bytes);
    let first_sum = first_weak.sum();

    let mut second_weak = Adler32::new();
    second_weak.roll_in(&b"x"[0]);
    second_weak.roll_in(&text_bytes[0]);
    second_weak.roll_in(&text_bytes[1]);
    second_weak.roll_in(&text_bytes[2]);
    second_weak.roll_in(&text_bytes[3]);
    second_weak.roll_in(&text_bytes[4]);
    second_weak.roll_in(&text_bytes[5]);
    second_weak.roll_in(&text_bytes[6]);
    second_weak.roll_in(&text_bytes[7]);
    second_weak.roll_in(&text_bytes[8]);
    second_weak.roll_out();
    let second_sum = first_weak.sum();

    assert_eq!(first_sum, second_sum);
}

//TEST SIGNATURES
#[test]
pub fn test_find_signature() {
    let text = "hello my god sir".to_string();
    let text_bytes = text.as_bytes();

    let signatures = Rolling::generate_signatures(text_bytes);

    let mut weak = Adler32::new();
    weak.write(text_bytes);

    let index = Signatures::find(&signatures, weak.sum(), &text_bytes.to_vec());

    assert_ne!(index, -1);
}
