use std::f64::consts;

fn square_area_to_circle(size: f64) -> f64 {
    consts::PI * size / 4.
}

fn assert_close(a:f64, b:f64, epsilon:f64) {
    assert!( (a-b).abs() < epsilon, format!("Expected: {}, got: {}",b,a) );
}


#[test]
fn returns_expected_9() {
    assert_close(square_area_to_circle(9.0), 7.0685834705770345, 1e-8);
}
#[test]
fn returns_expected_20() {
    assert_close(square_area_to_circle(20.0), 15.70796326794897, 1e-8);
}

#[test]
fn returns_expected_100() {
    assert_close(square_area_to_circle(100.0), 78.53981633974483, 1e-8);
}

#[test]
fn returns_expected_2() {
    assert_close(square_area_to_circle(2.0), 1.5707963267948966, 1e-8);
}

#[test]
fn returns_expected_1000() {
    assert_close(square_area_to_circle(1000.0), 785.3981633974482, 1e-8);
}
