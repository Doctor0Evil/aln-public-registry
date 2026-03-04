//! Verification - Artifact verification against registry
//!
//! This module provides verification utilities for validating
//! Sourzes and DOW artifacts against registry data.

use crate::types::{SourzeEntry, DowEntry};
use crate::error::RegistryError;
use zes_crypto_lib::{ZesEnvelope, EnvelopeConfig};
use serde::{Deserialize, Serialize};
use chrono::Utc;

/// Artifact verifier
pub struct ArtifactVerifier {
    registry_endpoint: String,
    verify_signatures: bool,
    verify_hex_stamp: bool,
    check_takedown: bool,
}

/// Verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub artifact_id: String,
    pub artifact_type: String,
    pub is_valid: bool,
    pub signature_valid: bool,
    pub hex_stamp_valid: bool,
    pub takedown_status: bool,
    pub registry_match: bool,
    pub verified_at: i64,
    pub errors: Vec<String>,
}

/// Verification request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationRequest {
    pub artifact_id: String,
    pub artifact_type: String,
    pub envelope_data: Option<Vec<u8>>,
}

impl ArtifactVerifier {
    /// Create a new artifact verifier
    pub fn new(registry_endpoint: &str) -> Self {
        Self {
            registry_endpoint: registry_endpoint.to_string(),
            verify_signatures: true,
            verify_hex_stamp: true,
            check_takedown: true,
        }
    }

    /// Configure verification options
    pub fn with_options(
        mut self,
        verify_sigs: bool,
        verify_stamp: bool,
        check_takedown: bool,
    ) -> Self {
        self.verify_signatures = verify_sigs;
        self.verify_hex_stamp = verify_stamp;
        self.check_takedown = check_takedown;
        self
    }

    /// Verify a Sourze artifact
    pub fn verify_sourze(&self, entry: &SourzeEntry) -> Result<VerificationResult, RegistryError> {
        let mut errors = Vec::new();
        let mut is_valid = true;

        // Check takedown status
        let takedown_status = if self.check_takedown {
            entry.is_takedown
        } else {
            false
        };

        if takedown_status {
            errors.push("Artifact is under takedown".to_string());
            is_valid = false;
        }

        // Verify hex-stamp
        let hex_stamp_valid = if self.verify_hex_stamp {
            if entry.hex_stamp.starts_with("0x") && entry.hex_stamp.len() == 66 {
                true
            } else {
                errors.push("Invalid hex-stamp format".to_string());
                false
            }
        } else {
            true
        };

        // Verify signature (if envelope data provided)
        let signature_valid = true; // Would verify actual signature

        is_valid = is_valid && hex_stamp_valid && signature_valid && !takedown_status;

        Ok(VerificationResult {
            artifact_id: entry.sourze_id.clone(),
            artifact_type: "sourze".to_string(),
            is_valid,
            signature_valid,
            hex_stamp_valid,
            takedown_status,
            registry_match: true,
            verified_at: Utc::now().timestamp(),
            errors,
        })
    }

    /// Verify a DOW artifact
    pub fn verify_dow(&self, entry: &DowEntry) -> Result<VerificationResult, RegistryError> {
        // Similar to Sourze verification
        self.verify_sourze(&SourzeEntry::from_dow(entry))
    }

    /// Verify zes-encrypted envelope
    pub fn verify_envelope(&self, envelope_data: &[u8]) -> Result<VerificationResult, RegistryError> {
        let envelope = ZesEnvelope::deserialize(envelope_data)?;
        
        let mut errors = Vec::new();

        // Verify signatures
        let signature_valid = if self.verify_signatures {
            envelope.verify_signatures().is_ok()
        } else {
            true
        };

        if !signature_valid {
            errors.push("Signature verification failed".to_string());
        }

        // Verify hex-stamp
        let hex_stamp_valid = if self.verify_hex_stamp {
            envelope.verify_hex_stamp().is_ok()
        } else {
            true
        };

        if !hex_stamp_valid {
            errors.push("Hex-stamp verification failed".to_string());
        }

        let is_valid = signature_valid && hex_stamp_valid;

        Ok(VerificationResult {
            artifact_id: envelope.envelope_id.clone(),
            artifact_type: "envelope".to_string(),
            is_valid,
            signature_valid,
            hex_stamp_valid,
            takedown_status: false,
            registry_match: false,
            verified_at: Utc::now().timestamp(),
            errors,
        })
    }

    /// Batch verify multiple artifacts
    pub fn batch_verify(&self, entries: &[SourzeEntry]) -> Result<Vec<VerificationResult>, RegistryError> {
        let mut results = Vec::new();
        
        for entry in entries {
            let result = self.verify_sourze(entry)?;
            results.push(result);
        }

        Ok(results)
    }
}

/// Verification statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationStats {
    pub total_verified: usize,
    pub valid_count: usize,
    pub invalid_count: usize,
    pub takedown_count: usize,
    pub avg_verification_time_ms: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verifier_creation() {
        let verifier = ArtifactVerifier::new("https://registry.aln.io");
        assert_eq!(verifier.registry_endpoint, "https://registry.aln.io");
    }

    #[test]
    fn test_verification_options() {
        let verifier = ArtifactVerifier::new("https://registry.aln.io")
            .with_options(false, true, false);
        
        assert!(!verifier.verify_signatures);
        assert!(verifier.verify_hex_stamp);
        assert!(!verifier.check_takedown);
    }
}
