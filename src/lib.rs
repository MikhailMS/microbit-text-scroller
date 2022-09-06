//! Simple Text Scroller for micro:bit 5*5 LED display


#![deny(
    dead_code,
    missing_crate_level_docs,
    missing_doc_code_examples,
    missing_docs,
    rust_2018_idioms,
    unsafe_code,
    unused_imports,
)]
#![no_main]
#![no_std]

mod font;
pub mod scroller;
pub use scroller::{Animate, ScrollMessage};

