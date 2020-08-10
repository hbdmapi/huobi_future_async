#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::missing_errors_doc)]
mod client;
pub mod error;
pub mod models;
mod transport;

pub use crate::models::*;
pub use crate::error::*;

pub use crate::client::{websocket::HuobiWebsocket, HuobiFuture};
