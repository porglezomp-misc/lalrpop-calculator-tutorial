pub mod calculator1;

#[test]
fn calculator1() {
    assert!(calculator1::parse_Term("22").is_ok());
    assert!(calculator1::parse_Term("((22))").is_ok());
    assert!(calculator1::parse_Term("((22)").is_err());
}
fn main() {
    println!("Hello, world!");
}
