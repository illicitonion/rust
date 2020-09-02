// Test that From/Into cannot be derived other than for enums with an explicit int repr and no data.

#![allow(unused)]

#[derive(Into)]
//~^ ERROR `Into` can only be derived for enums [FIXME]
struct Struct {}

#[derive(Into)]
//~^ ERROR `Into` can only be derived for enums with an explicit integer representation [FIXME]
#[repr(C)]
enum NumberC {
    Zero,
    One,
}

#[derive(Into)]
//~^ ERROR `Into` can only be derived for enums with an explicit integer representation [FIXME]
enum NumberNoRepr {
    Zero,
    One,
}

#[derive(Into)]
//~^ ERROR `Into` can only be derived for data-free enums [FIXME]
#[repr(u8)]
enum NumberTuple {
    Zero,
    NonZero(u8),
}

#[derive(Into)]
//~^ ERROR `Into` can only be derived for data-free enums [FIXME]
#[repr(u8)]
enum NumberStruct {
    Zero,
    NonZero { value: u8 },
}

#[derive(Into)]
//~^ ERROR `Into` can only be derived for data-free enums [FIXME]
#[repr(u8)]
enum NumberEmptyTuple {
    Zero(),
    NonZero,
}

#[derive(Into)]
//~^ ERROR `Into` can only be derived for data-free enums [FIXME]
#[repr(u8)]
enum NumberEmptyStruct {
    Zero {},
    NonZero,
}

fn main() {}
