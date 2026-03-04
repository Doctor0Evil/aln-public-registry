//! Takedown Manager - Compromised artifact takedown protocol
//!
//! This module handles reporting and processing of compromised
//! artifacts with public audit trail.

use crate::error::RegistryError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Takedown manager
pub struct TakedownManager {
    registry_endpoint: String,
    require_multi_sig: bool,
}

/// Takedown request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TakedownRequest {
    pub request_id: String,
    pub artifact_id: String,
    pub artifact_type: String,
    pub reason: String,
    pub reporter_did: String,
    pub evidence: Vec<String>,
    pub severity: String,
    pub created_at: i64,
    pub status: String,
}

/// Takedown status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TakedownStatus {
    Pending,
    UnderReview,
    Approved,
    Rejected,
    Executed,
}

/// Takedown decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TakedownDecision {
    pub request_id: String,
    pub decision: TakedownStatus,
    pub decided_by: String,
    pub decided_at: i64,
    pub reason: String,
    pub row_id: Option<String>,
}

impl TakedownManager {
    /// Create a new takedown manager
    pub fn new(registry_endpoint: &str) -> Self {
        Self {
            registry_endpoint: registry_endpoint.to_string(),
            require_multi_sig: true,
        }
    }

    /// Submit takedown request
    pub fn submit_request(&self, request: TakedownRequest) -> Result<String, RegistryError> {
        // Validate request
        if request.artifact_id.is_empty() {
            return Err(RegistryError::InvalidTakedownRequest {
                reason: "Artifact ID required".to_string(),
            });
        }

        if request.reason.is_empty() {
            return Err(RegistryError::InvalidTakedownRequest {
                reason: "Reason required".to_string(),
            });
        }

        // In production, submit to registry
        Ok(request.request_id)
    }

    /// Create takedown request
    pub fn create_request(
        artifact_id: &str,
        artifact_type: &str,
        reason: &str,
        reporter_did: &str,
        severity: &str,
    ) -> TakedownRequest {
        TakedownRequest {
            request_id: Uuid::new_v4().to_string(),
            artifact_id: artifact_id.to_string(),
            artifact_type: artifact_type.to_string(),
            reason: reason.to_string(),
            reporter_did: reporter_did.to_string(),
            evidence: vec![],
            severity: severity.to_string(),
            created_at: Utc::now().timestamp(),
            status: "pending".to_string(),
        }
    }

    /// Review takedown request
    pub fn review_request(
        &self,
        request_id: &str,
        decision: TakedownStatus,
        reviewer_did: &str,
    ) -> Result<TakedownDecision, RegistryError> {
        let takedown_decision = TakedownDecision {
            request_id: request_id.to_string(),
            decision,
            decided_by: reviewer_did.to_string(),
            decided_at: Utc::now().timestamp(),
            reason: String::new(),
            row_id: Some(Uuid::new_v4().to_string()),
        };

        Ok(takedown_decision)
    }

    /// Execute takedown
    pub fn execute_takedown(&self, request_id: &str) -> Result<(), RegistryError> {
        // In production, mark artifact as takedown in registry
        log::info!("Executing takedown for {}", request_id);
        Ok(())
    }

    /// Get takedown status
    pub fn get_status(&self, request_id: &str) -> Result<TakedownStatus, RegistryError> {
        // In production, query registry
        Ok(TakedownStatus::Pending)
    }

    /// List pending takedowns
    pub fn list_pending(&self) -> Result<Vec<TakedownRequest>, RegistryError> {
        // In production, query registry
        Ok(vec![])
    }
}

/// Takedown reason taxonomy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TakedownReason {
    SecurityVulnerability,
    MalwareDetected,
    CapabilityViolation,
    AuthorshipDispute,
    LicenseViolation,
    NDMThresholdBreach,
    Other,
}

impl TakedownReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            TakedownReason::SecurityVulnerability => "security_vulnerability",
            TakedownReason::MalwareDetected => "malware_detected",
            TakedownReason::CapabilityViolation => "capability_violation",
            TakedownReason::AuthorshipDispute => "authorship_dispute",
            TakedownReason::LicenseViolation => "license_violation",
            TakedownReason::NDMThresholdBreach => "ndm_threshold_breach",
            TakedownReason::Other => "other",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_takedown_manager_creation() {
        let manager = TakedownManager::new("https://registry.aln.io");
        assert!(manager.require_multi_sig);
    }

    #[test]
    fn test_create_takedown_request() {
        let request = TakedownManager::create_request(
            "sourze-123",
            "sourze",
            "Security vulnerability detected",
            "bostrom1reporter",
            "critical",
        );

        assert!(!request.request_id.is_empty());
        assert_eq!(request.artifact_id, "sourze-123");
        assert_eq!(request.status, "pending");
    }

    #[test]
    fn test_invalid_request_rejected() {
        let manager = TakedownManager::new("https://registry.aln.io");
        
        let request = TakedownRequest {
            request_id: "test-123".to_string(),
            artifact_id: "".to_string(),
            artifact_type: "sourze".to_string(),
            reason: "".to_string(),
            reporter_did: "bostrom1test".to_string(),
            evidence: vec![],
            severity: "low".to_string(),
            created_at: Utc::now().timestamp(),
            status: "pending".to_string(),
        };

        let result = manager.submit_request(request);
        assert!(result.is_err());
    }
}
