use crate::complex::Complex;

#[test]
fn test_complex_01() {
    let a = Complex::new(10.0, 5.0);
    let b = Complex::new(3.0, 2.0);

    let c = a * b;
    let c_expected = Complex::new(20.0, 35.0);

    assert_eq!(c, c_expected)
}


#[test]
fn test_complex_02() {
    let a = Complex::new(4.25, 3.1);
    let b = Complex::new(5.5, 6.28);

    let c = a * b;
    let c_expected = Complex::new(3.907, 43.74);

    assert_eq!(c, c_expected)
}