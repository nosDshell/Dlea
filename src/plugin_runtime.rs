//! The executable plugin protocol and supervised, bounded sessions shared by Dlea hosts.
mod process;
pub mod protocol;
mod session;
pub use session::{Config, Reply, Runtime};

#[cfg(test)]
mod tests;
