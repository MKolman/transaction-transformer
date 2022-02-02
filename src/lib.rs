#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

pub mod bindings;
pub mod matcher;
pub mod reader;
pub mod transaction;
pub mod ui;
pub mod writer;
