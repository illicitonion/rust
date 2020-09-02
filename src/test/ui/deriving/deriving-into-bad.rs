// Test that From/Into cannot be derived other than for enums with an int repr.

#![allow(unused)]

#[derive(Into)]
//~^ ERROR `Into` can only be derived for enums with an explicit integer representation [FIXME]
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

fn main() {}
