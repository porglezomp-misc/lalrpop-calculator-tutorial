pub mod calculator1;

#[test]
fn calculator1() {
    assert!(calculator1::parse_Term("22").is_ok());
    assert!(calculator1::parse_Term("((22))").is_ok());
    assert!(calculator1::parse_Term("((22)").is_err());
}

pub mod calculator2;

#[test]
fn calclulator2() {
    assert!(calculator2::parse_Term("22").is_ok());
    assert!(calculator2::parse_Term("((22))").is_ok());
    assert!(calculator2::parse_Term("((22)").is_err());
}

pub mod calculator3;

macro_rules! test3 {
    ($expr:expr) => {
        assert_eq!(calculator3::parse_Expr(stringify!($expr)).unwrap(), $expr);
    }
}
#[test]
fn calculator3() {
    test3!(22);
    test3!(11 * 2);
    test3!(1 + 2 * 3);
    test3!(1 - 1 - 1);
    test3!((3 + 3) * (4 + 3));
    test3!(((3 - 8)/2 + 1));
    assert!(calculator3::parse_Expr("3*(/3)").is_err());
    assert!(calculator3::parse_Expr("*3").is_err());
    assert!(calculator3::parse_Expr("(").is_err());
}

pub mod calculator4;
pub mod ast;

#[test]
fn calculator4() {
    assert_eq!(&format!("{:?}", calculator4::parse_Expr("22 * 44 + 66").unwrap()),
               "((22 * 44) + 66)");
}

#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}
