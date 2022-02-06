#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::unsafe_derive_deserialize)]
#![allow(clippy::unused_unit)]

pub mod bindings;
pub mod matcher;
pub mod reader;
pub mod transaction;
pub mod ui;
pub mod writer;
