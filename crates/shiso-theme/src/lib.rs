//! Shiso design tokens for Rust.
//!
//! [`tokens`] is generated from `tokens/tokens.json` and has no dependencies.
//! Enable the `egui` feature for [`egui`](mod@egui): fonts, light and dark styles, and the few widgets
//! egui does not have (card, pill, meter).

pub mod tokens;

#[cfg(feature = "egui")]
pub mod egui;
