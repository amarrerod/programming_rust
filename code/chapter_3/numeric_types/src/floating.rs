pub fn limits() -> bool {
    assert_eq!((-1. / f32::INFINITY).is_sign_negative(), true);
    assert_eq!(-f32::MIN, f32::MAX);
    true
}
