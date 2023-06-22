fn main() {
    let _local1 = regex::bytes::RegexSet::empty();
    let _ = regex::bytes::RegexSet::is_empty(&(_local1));
    let _ = regex::bytes::RegexSet::len(&(_local1));
    let _ = regex::bytes::RegexSet::is_match(&(_local1), &[106, 97, 118, 97, 44, 32, 74, 97]);
    let _local5 =
        regex::bytes::RegexSet::matches(&(_local1), &[118, 97, 110, 101, 115, 101, 141, 138]);
    let _ = regex::bytes::SetMatches::matched(&(_local5), 8);
}
