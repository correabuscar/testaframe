#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    warnings,
    future_incompatible,
    nonstandard_style,
    non_ascii_idents,
    clippy::restriction,
    rust_2018_compatibility,
    rust_2021_compatibility,
    unused
)]
#![allow(
    clippy::print_stdout,
    clippy::use_debug,
    clippy::missing_docs_in_private_items,
    clippy::absolute_paths, //FIXME: why can't I override this (ie. allow) since it's implied by clippy::restriction
    clippy::single_call_fn,
)]
#![allow(clippy::blanket_clippy_restriction_lints)] //workaround clippy
#![allow(clippy::needless_return)]
// might want to deny later:
#![allow(clippy::default_numeric_fallback)] // might want to deny later!
#![allow(clippy::dbg_macro)]
#![feature(stmt_expr_attributes)] //needed to attributes on expressions!

//#![warn(clippy::all, rust_2018_idioms)]

mod app;
#[allow(clippy::useless_attribute)] // need this for the #[allow(clippy::pub_use)] below to be allowed by clippy
#[allow(clippy::pub_use)] //FIXME: deny this! seems difficult to do!
pub use app::TemplateApp;
