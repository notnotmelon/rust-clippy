//@aux-build:proc_macros.rs
//@require-annotations-for-level: WARN
//@no-rustfix
#![warn(clippy::legacy_str_from_utf8)]
#![allow(unused, unused_mut)]

use proc_macros::external;

fn imports_that_should_be_fixed() {
    #[rustfmt::ignore]
    use core::str::{
        from_utf8,
        //~^ legacy_str_from_utf8
        from_utf8_mut,
        //~^ legacy_str_from_utf8
        from_utf8_unchecked,
        //~^ legacy_str_from_utf8
        from_utf8_unchecked_mut,
        //~^ legacy_str_from_utf8
    };

    let crab = from_utf8(&[0xF0, 0x9F, 0xA6, 0x80])?;
    let mut crab = from_utf8_mut(&mut [0xF0, 0x9F, 0xA6, 0x80])?;

    unsafe {
        let crab = from_utf8_unchecked(&[0xF0, 0x9F, 0xA6, 0x80]);
    }

    unsafe {
        let mut crab = from_utf8_unchecked_mut(&mut [0xF0, 0x9F, 0xA6, 0x80]);
    }
}

#[allow(unused)]
#[rustfmt::skip]
fn single_line_imports_that_should_be_fixed() {
    use core::str::from_utf8;
    //~^ legacy_str_from_utf8
    use core::str::from_utf8_mut;
    //~^ legacy_str_from_utf8
    use core::str::from_utf8_unchecked;
    //~^ legacy_str_from_utf8
    use core::str::from_utf8_unchecked_mut;
    //~^ legacy_str_from_utf8
    use core::str::from_utf8 as _;
    //~^ legacy_str_from_utf8
    use core::str::from_utf8_mut as _;
    //~^ legacy_str_from_utf8
    use core::str::from_utf8_unchecked as _;
    //~^ legacy_str_from_utf8
    use core::str::from_utf8_unchecked_mut as _;
    //~^ legacy_str_from_utf8
}

fn imports_that_should_not_be_fixed() {
    use core::str::*;
    use std::str::{from_utf8, from_utf8_mut, from_utf8_unchecked, from_utf8_unchecked_mut};

    let crab = from_utf8(&[0xF0, 0x9F, 0xA6, 0x80])?;
    let mut crab = from_utf8_mut(&mut [0xF0, 0x9F, 0xA6, 0x80])?;

    unsafe {
        let crab = from_utf8_unchecked(&[0xF0, 0x9F, 0xA6, 0x80]);
    }

    unsafe {
        let mut crab = from_utf8_unchecked_mut(&mut [0xF0, 0x9F, 0xA6, 0x80]);
    }
}

fn external_imports_that_should_not_be_fixed() {
    external! {
        use core::str::{
            from_utf8,
            from_utf8_mut,
            from_utf8_unchecked,
            from_utf8_unchecked_mut,
        };

        let crab = from_utf8(&[0xF0, 0x9F, 0xA6, 0x80])?;
        let mut crab = from_utf8_mut(&mut [0xF0, 0x9F, 0xA6, 0x80])?;

        unsafe {
            let crab = from_utf8_unchecked(&[0xF0, 0x9F, 0xA6, 0x80]);
        }

        unsafe {
            let mut crab = from_utf8_unchecked_mut(&mut [0xF0, 0x9F, 0xA6, 0x80]);
        }
    }
}

#[rustfmt::skip]
fn single_line_imports_that_should_not_be_fixed() {
    use std::str::from_utf8;
    use std::str::from_utf8_mut;
    use std::str::from_utf8_unchecked;
    use std::str::from_utf8_unchecked_mut;
    use std::str::from_utf8 as _;
    use std::str::from_utf8_mut as _;
    use std::str::from_utf8_unchecked as _;
    use std::str::from_utf8_unchecked_mut as _;
}

fn main() -> Result<(), std::str::Utf8Error> {
    let crab = core::str::from_utf8(&[0xF0, 0x9F, 0xA6, 0x80])?;
    //~^ legacy_str_from_utf8
    let crab = str::from_utf8(&[0xF0, 0x9F, 0xA6, 0x80])?;

    let mut crab = core::str::from_utf8_mut(&mut [0xF0, 0x9F, 0xA6, 0x80])?;
    //~^ legacy_str_from_utf8
    let mut crab = str::from_utf8_mut(&mut [0xF0, 0x9F, 0xA6, 0x80])?;

    unsafe {
        let crab = core::str::from_utf8_unchecked(&[0xF0, 0x9F, 0xA6, 0x80]);
        //~^ legacy_str_from_utf8
        let crab = str::from_utf8_unchecked(&[0xF0, 0x9F, 0xA6, 0x80]);
    }

    unsafe {
        let mut crab = core::str::from_utf8_unchecked_mut(&mut [0xF0, 0x9F, 0xA6, 0x80]);
        //~^ legacy_str_from_utf8
        let mut crab = str::from_utf8_unchecked_mut(&mut [0xF0, 0x9F, 0xA6, 0x80]);
    }

    Ok(())
}

#[clippy::msrv = "1.86.0"]
fn msrv_too_low() {
    let crab = core::str::from_utf8(&[0xF0, 0x9F, 0xA6, 0x80])?;
}

#[clippy::msrv = "1.87.0"]
fn msrv_just_right() {
    let crab = core::str::from_utf8(&[0xF0, 0x9F, 0xA6, 0x80])?;
    //~^ legacy_str_from_utf8
}
