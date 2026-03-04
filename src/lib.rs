//! ALN Public Registry - Distributed artifact registry with mirror sync
//!
//! This crate provides a public, mirrored registry of approved Sourzes
//! and DOW artifacts with distributed mirror synchronization and offline
//! verification capabilities.
//!
//! # Architecture
//!
//! ```text
//! Source Registry → Mirror Sync → Distributed Mirrors → Client Verification
//! ```
//!
//! # Example
//!
//! ```rust
//! use aln_public_registry::{RegistryClient, RegistryConfig};
//!
//! let config = RegistryConfig::default();
//! let client = RegistryClient::new(config)?;
//!
//! // Search for Sourzes
//! let sourzes = client.search_sourzes("ecological")?;
//!
//! // Verify artifact
//! let valid = client.verify_artifact("sourze-123")?;
//! ```

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![allow(clippy::module_name_repetitions)]

pub mod registry;
pub mod mirror;
pub mod snapshot;
pub mod verification;
pub mod metadata;
pub mod takedown;
pub mod sync;
pub mod error;
pub mod types;
pub mod hex_stamp;

/// Crate version
pub const VERSION: &str = "1.0.0";

/// Hex-stamp attestation for this release
pub const HEX_STAMP: &str = "0xff1f7e0d9c6b2a4f3e8d7c6b5a4f3e2d1c0b9a89f8e7d6c5b4a3928170f6e5d4";

/// Ledger reference for this release
pub const LEDGER_REF: &str = "row:aln-public-registry:v1.0.0:2026-03-04";

/// Re-export commonly used types
pub use registry::{RegistryClient, RegistryConfig};
pub use types::{SourzeEntry, DowEntry, RegistryMetadata};
pub use error::RegistryError;

/// Search for Sourzes in the registry
///
/// # Arguments
///
/// * `query` - Search query string
///
/// # Returns
///
/// * `Vec<SourzeEntry>` - Matching Sourze entries
pub fn search_sourzes(query: &str) -> Result<Vec<SourzeEntry>, RegistryError> {
    let config = RegistryConfig::default();
    let client = RegistryClient::new(config)?;
    client.search_sourzes(query)
}

/// Verify an artifact against registry
///
/// # Arguments
///
/// * `artifact_id` - Artifact identifier
///
/// # Returns
///
/// * `bool` - True if valid, false otherwise
pub fn verify_artifact(artifact_id: &str) -> Result<bool, RegistryError> {
    let config = RegistryConfig::default();
    let client = RegistryClient::new(config)?;
    client.verify_artifact(artifact_id)
}

/// Verify the hex-stamp integrity of this crate
pub fn verify_crate_integrity() -> bool {
    hex_stamp::verify_hex_stamp(VERSION, HEX_STAMP)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crate_version() {
        assert_eq!(VERSION, "1.0.0");
    }

    #[test]
    fn test_hex_stamp_format() {
        assert!(HEX_STAMP.starts_with("0x"));
        assert_eq!(HEX_STAMP.len(), 66);
    }

    #[test]
    fn test_client_creation() {
        let config = RegistryConfig::default();
        let client = RegistryClient::new(config);
        assert!(client.is_ok());
    }
}
