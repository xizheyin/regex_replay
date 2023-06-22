fn main() {
    let _local2 = if let Ok(x) = regex::bytes::Regex::new("\0\0\0\0*$$$$") {
        x
    } else {
        use std::process;
        process::exit(0);
    };

    let _ = regex::bytes::Regex::captures_len(&(_local2));

    let _ = regex::bytes::Regex::shortest_match_at(
        &(_local2),
        &[6, 91, 0, 45, 87, 43, 99, 45, 122],
        122,
    );
}
