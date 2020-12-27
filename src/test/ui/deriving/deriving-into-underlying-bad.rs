// Test that IntoUnderlying cannot be derived other than for enums with an int repr.

#![allow(unused)]

#[derive(IntoUnderlying)]
//~^ ERROR `IntoUnderlying` can only be derived for enums with an explicit integer representation [FIXME]
struct Struct {}

#[derive(IntoUnderlying)]
//~^ ERROR `IntoUnderlying` can only be derived for enums with an explicit integer representation [FIXME]
#[repr(C)]
enum NumberC {
    Zero,
    One,
}

#[derive(IntoUnderlying)]
//~^ ERROR `IntoUnderlying` can only be derived for enums with an explicit integer representation [FIXME]
enum NumberNoRepr {
    Zero,
    One,
}

fn main() {}
