// Test that IntoUnderlying can be derived to convert an int-repr'd enum into its repr.

// run-pass

#[derive(IntoUnderlying)]
#[repr(u8)]
enum PositiveNumber {
    Zero,
    One,
}

#[derive(IntoUnderlying)]
#[repr(i8)]
enum Number {
    MinusOne = -1,
    Zero,
    One,
}

fn main() {
    let n = PositiveNumber::Zero.into_underlying();
    assert_eq!(n, 0_u8);
    let n = PositiveNumber::One.into_underlying();
    assert_eq!(n, 1_u8);

    let n = Number::MinusOne.into_underlying();
    assert_eq!(n, -1_i8);
    let n = Number::Zero.into_underlying();
    assert_eq!(n, 0_i8);
    let n = Number::One.into_underlying();
    assert_eq!(n, 1_i8);
}
