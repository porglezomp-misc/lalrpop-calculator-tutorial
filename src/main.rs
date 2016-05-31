pub mod calculator1;
pub mod calculator2;

#[test]
fn calculator1() {
    assert!(calculator1::parse_Term("22").is_ok());
    assert!(calculator1::parse_Term("((22))").is_ok());
    assert!(calculator1::parse_Term("((22)").is_err());
}

#[test]
fn calclulator2() {
    assert!(calculator2::parse_Term("22").is_ok());
    assert!(calculator2::parse_Term("((22))").is_ok());
    assert!(calculator2::parse_Term("((22)").is_err());
}

fn main() {
    println!("Hello, world!");
}
