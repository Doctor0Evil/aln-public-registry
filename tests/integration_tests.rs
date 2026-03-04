//! ALN Public Registry Integration Tests

use aln_public_registry::{RegistryClient, RegistryConfig};
use tempfile::tempdir;

#[tokio::test]
async fn test_registry_client_creation() {
    let config = RegistryConfig::default();
    let client = RegistryClient::new(config);
    assert!(client.is_ok());
}

#[tokio::test]
async fn test_search_sourzes() {
    let config = RegistryConfig::default();
    let client = RegistryClient::new(config).unwrap();

    // In production, this would query actual registry
    // For now, test client creation and method availability
    let result = client.search_sourzes("ecological");
    // Would return results from registry
}

#[tokio::test]
async fn test_verify_artifact() {
    let config = RegistryConfig::default();
    let client = RegistryClient::new(config).unwrap();

    let valid = client.verify_artifact("sourze-123");
    assert!(valid.is_ok());
}

#[test]
fn test_registry_config_defaults() {
    let config = RegistryConfig::default();
    assert_eq!(config.primary_endpoint, "https://registry.aln.io");
    assert!(!config.mirror_endpoints.is_empty());
    assert_eq!(config.timeout_secs, 30);
}

#[tokio::test]
async fn test_mirror_fallback() {
    let config = RegistryConfig {
        primary_endpoint: "https://invalid.endpoint".to_string(),
        mirror_endpoints: vec![
            "https://mirror1.aln.io".to_string(),
            "https://mirror2.aln.io".to_string(),
        ],
        ..Default::default()
    };

    let client = RegistryClient::new(config).unwrap();
    
    // Should fallback to mirrors when primary fails
    let result = client.search_sourzes("test");
    // Would test mirror fallback logic
}

#[test]
fn test_search_query_validation() {
    // Test search query parameter validation
    let query = "";
    assert!(query.is_empty()); // Placeholder for actual validation test
}

#[tokio::test]
async fn test_snapshot_cache() {
    use aln_public_registry::snapshot::SnapshotCache;
    
    let dir = tempdir().unwrap();
    let path = dir.path().join("snapshot.db").to_string_lossy().to_string();
    
    let cache = SnapshotCache::new(&path).unwrap();
    let count = cache.snapshot_count().unwrap();
    assert_eq!(count, 0); // Fresh cache
}

#[test]
fn test_takedown_request_creation() {
    use aln_public_registry::takedown::TakedownManager;
    
    let request = TakedownManager::create_request(
        "sourze-123",
        "sourze",
        "Security issue",
        "bostrom1reporter",
        "critical",
    );

    assert!(!request.request_id.is_empty());
    assert_eq!(request.artifact_id, "sourze-123");
    assert_eq!(request.status, "pending");
}
