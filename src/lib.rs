//! impl Fallible collections on allocation errors, quite as describe
//! in [RFC 2116](https://github.com/rust-lang/rfcs/blob/master/text/2116-alloc-me-maybe.md)
//! This was used in the turbofish OS hobby project to mitigate the
//! the lack of faillible allocation in rust.
#![cfg_attr(not(test), no_std)]
#![feature(try_reserve)]
#![feature(specialization)]
#![feature(allocator_api)]
#![feature(dropck_eyepatch)]
#![feature(ptr_internals)]
#![feature(core_intrinsics)]
#![feature(maybe_uninit_ref)]
#![feature(maybe_uninit_slice)]
#![feature(maybe_uninit_extra)]
#![feature(internal_uninit_const)]

extern crate alloc;

pub mod boxed;
pub use boxed::*;
#[macro_use]
pub mod vec;
pub use vec::*;
pub mod rc;
pub use rc::*;
pub mod arc;
pub use arc::*;
pub mod btree;
#[macro_use]
pub mod format;
pub mod try_clone;

use alloc::collections::TryReserveError;

/// trait for trying to clone an elem, return an error instead of
/// panic if allocation failed
/// # Examples
///
/// ```
/// use fallible_collections::TryClone;
/// let mut vec = vec![42, 100];
/// assert_eq!(vec.try_clone().unwrap(), vec)
/// ```
pub trait TryClone {
    /// try clone method, (Self must be size because of Result
    /// constraint)
    fn try_clone(&self) -> Result<Self, TryReserveError>
    where
        Self: core::marker::Sized;
}
