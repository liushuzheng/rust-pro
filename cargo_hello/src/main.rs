use encoding_rs::*;

fn main() {
    let expectation = "\u{30CF}\u{30ED}\u{30FC}\u{30FB}\u{30EF}\u{30FC}\u{30EB}\u{30C9}";
    let bytes = b"\x83n\x83\x8D\x81[\x81E\x83\x8F\x81[\x83\x8B\x83h";

    let (cow, encoding_used, had_errors) = SHIFT_JIS.decode(bytes);
    assert_eq!(&cow[..], expectation);
    assert_eq!(encoding_used, SHIFT_JIS);
    assert!(!had_errors);
}
