#![feature(type_alias_impl_trait)]
#![feature(min_type_alias_impl_trait)]
#![deny(
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications
)]
pub mod birds;
pub use crate::birds::*;
