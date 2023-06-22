
fn main() {
    let mut _local3 = regex::bytes::RegexBuilder::new("NN{4}^ế\\d\\z{4}");
    let _local5 = if let Ok(x) = regex::bytes::RegexBuilder::build(&(_local3)) {
        x
    } else {
        use std::process;
        process::exit(0);
    };
    let _ = regex::bytes::Regex::is_match_at(
        &(_local5),
        &[
            1, 0, 94, 117, 127, 117, 78, 0, 52, 125, 45, 117, 102, 117, 111, 64,
        ],
        78,
    );
}
