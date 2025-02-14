//! Recon is a network protocol for set reconciliation
#![warn(missing_docs, missing_debug_implementations, clippy::all)]

pub use crate::recon::{AssociativeHash, Message, Recon, Response};
pub use ahash::Sha256a;
pub use eventid::{EventId, Network};
#[cfg(test)]
pub use recon::tests;

mod ahash;
mod eventid;
pub mod libp2p;
mod recon;

#[cfg(test)]
#[macro_use]
extern crate lalrpop_util;
