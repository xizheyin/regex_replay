fn main() {
    let _local0 = regex::RegexSet::empty();
    let _local1 = regex::RegexSet::matches(&(_local0), "}?[[");
    let _ = regex::SetMatches::iter(&(_local1));

    let _ = regex::SetMatches::len(&(_local1));
    let _ = regex::SetMatches::matched(&(_local1), 1);
}
