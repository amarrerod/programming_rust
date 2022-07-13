pub fn overflow_example() {
    let mut i: i32 = 1;
    loop {
        // panic: Multiplication overflowed in any build
        i = i.checked_mul(10).expect("Multiplication overflowed");
    }
}

pub fn checking_example() {
    assert_eq!(10_u8.checked_add(20), Some(30));
    assert_eq!(100_u8.checked_add(200), None);
}

pub fn wrapping_example() {
    assert_eq!(100_u16.wrapping_mul(200), 20000);
    // No puede ser representado en u16 por lo que se hace el modulo
    assert_eq!(500_u16.wrapping_mul(500), 53392);
}
