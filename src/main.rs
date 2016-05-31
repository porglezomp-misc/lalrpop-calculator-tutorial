pub mod calculator1;
pub mod calculator2;
pub mod calculator3;
pub mod calculator4;
pub mod ast;

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

#[test]
fn calculator3() {
    assert_eq!(calculator3::parse_Expr("22").unwrap(), 22);
    assert_eq!(calculator3::parse_Expr("11*2").unwrap(), 11*2);
    assert_eq!(calculator3::parse_Expr("1+2*3").unwrap(), 1+2*3);
    assert_eq!(calculator3::parse_Expr("1-1-1").unwrap(), 1-1-1);
    assert_eq!(calculator3::parse_Expr("(3+3)*(4+3)").unwrap(), (3+3)*(4+3));
    assert_eq!(calculator3::parse_Expr("((3-8)/2+1)").unwrap(), ((3-8)/2)+1);
    assert!(calculator3::parse_Expr("3*(/3)").is_err());
    assert!(calculator3::parse_Expr("*3").is_err());
    assert!(calculator3::parse_Expr("(").is_err());
}

#[test]
fn calculator4() {
    assert_eq!(&format!("{:?}", calculator4::parse_Expr("22 * 44 + 66").unwrap()),
               "((22 * 44) + 66)");
}

fn main() {
    println!("Hello, world!");
}
