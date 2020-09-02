// Test that From/Into can be derived to convert an int-repr'd enum into its repr.

// run-pass

#[derive(Into)]
#[repr(u8)]
enum PositiveNumber {
    Zero,
    One,
}

#[derive(Into)]
#[repr(i8)]
enum Number {
    MinusOne = -1,
    Zero,
    One,
    Four = 4,
}

fn main() {
    let n = u8::from(PositiveNumber::Zero);
    assert_eq!(n, 0);
    let n = u8::from(PositiveNumber::One);
    assert_eq!(n, 1);

    let n: u8 = PositiveNumber::Zero.into();
    assert_eq!(n, 0);
    let n: u8 = PositiveNumber::One.into();
    assert_eq!(n, 1);

    let n = i8::from(Number::MinusOne);
    assert_eq!(n, -1);
    let n = i8::from(Number::Zero);
    assert_eq!(n, 0);
    let n = i8::from(Number::One);
    assert_eq!(n, 1);
    let n = i8::from(Number::Four);
    assert_eq!(n, 4);

    let n: i8 = Number::MinusOne.into();
    assert_eq!(n, -1);
    let n: i8 = Number::Zero.into();
    assert_eq!(n, 0);
    let n: i8 = Number::One.into();
    assert_eq!(n, 1);
    let n: i8 = Number::Four.into();
    assert_eq!(n, 4);
}
