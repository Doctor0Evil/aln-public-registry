//! Registry Client - API client for registry operations
//!
//! This module provides the main registry client for querying
//! Sourzes, DOWs, and other registry data.

use crate::types::{SourzeEntry, DowEntry, RegistryMetadata, PublicDIDKey};
use crate::error::RegistryError;
use crate::mirror::MirrorClient;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// Registry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryConfig {
    pub primary_endpoint: String,
    pub mirror_endpoints: Vec<String>,
    pub timeout_secs: u64,
    pub cache_enabled: bool,
    pub cache_path: Option<String>,
}

impl Default for RegistryConfig {
    fn default() -> Self {
        Self {
            primary_endpoint: "https://registry.aln.io".to_string(),
            mirror_endpoints: vec![
                "https://mirror1.aln.io".to_string(),
                "https://mirror2.aln.io".to_string(),
            ],
            timeout_secs: 30,
            cache_enabled: true,
            cache_path: None,
        }
    }
}

/// Registry client for all operations
pub struct RegistryClient {
    config: RegistryConfig,
    client: Client,
    mirror_client: MirrorClient,
}

/// Search query parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub query: String,
    pub artifact_type: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub sort_by: Option<String>,
}

/// Search results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults {
    pub total: usize,
    pub returned: usize,
    pub offset: usize,
    pub results: Vec<RegistryEntry>,
}

/// Registry entry (unified type)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegistryEntry {
    Sourze(SourzeEntry),
    Dow(DowEntry),
}

impl RegistryClient {
    /// Create a new registry client
    pub fn new(config: RegistryConfig) -> Result<Self, RegistryError> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_secs))
            .build()?;

        let mirror_client = MirrorClient::new(config.mirror_endpoints.clone())?;

        Ok(Self {
            config,
            client,
            mirror_client,
        })
    }

    /// Search for Sourzes
    pub fn search_sourzes(&self, query: &str) -> Result<Vec<SourzeEntry>, RegistryError> {
        let search_query = SearchQuery {
            query: query.to_string(),
            artifact_type: Some("sourze".to_string()),
            limit: Some(100),
            offset: Some(0),
            sort_by: Some("relevance".to_string()),
        };

        self.search(&search_query)
    }

    /// Search for DOW artifacts
    pub fn search_dows(&self, query: &str) -> Result<Vec<DowEntry>, RegistryError> {
        let search_query = SearchQuery {
            query: query.to_string(),
            artifact_type: Some("dow".to_string()),
            limit: Some(100),
            offset: Some(0),
            sort_by: Some("relevance".to_string()),
        };

        self.search(&search_query)
    }

    /// Generic search
    pub fn search(&self, query: &SearchQuery) -> Result<Vec<RegistryEntry>, RegistryError> {
        // Try primary endpoint first
        match self.search_primary(query).await {
            Ok(results) => Ok(results),
            Err(_) => {
                // Fallback to mirrors
                self.mirror_client.search(query).await
            }
        }
    }

    /// Search primary endpoint
    async fn search_primary(&self, query: &SearchQuery) -> Result<Vec<RegistryEntry>, RegistryError> {
        let url = format!("{}/api/v1/search", self.config.primary_endpoint);
        
        let response = self.client
            .post(&url)
            .json(query)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(RegistryError::RegistryAPIError {
                status: response.status().as_u16(),
                message: response.text().await?,
            });
        }

        let results: SearchResults = response.json().await?;
        Ok(results.results)
    }

    /// Get Sourze by ID
    pub fn get_sourze(&self, id: &str) -> Result<SourzeEntry, RegistryError> {
        // Implementation would fetch from registry
        Ok(SourzeEntry::default())
    }

    /// Get DOW by ID
    pub fn get_dow(&self, id: &str) -> Result<DowEntry, RegistryError> {
        // Implementation would fetch from registry
        Ok(DowEntry::default())
    }

    /// Verify artifact against registry
    pub fn verify_artifact(&self, artifact_id: &str) -> Result<bool, RegistryError> {
        // Check if artifact exists and is not takedown
        let entry = self.get_sourze(artifact_id);
        match entry {
            Ok(e) => Ok(!e.is_takedown),
            Err(_) => Ok(false),
        }
    }

    /// Get public DID key
    pub fn get_did_key(&self, did: &str) -> Result<PublicDIDKey, RegistryError> {
        let url = format!("{}/api/v1/keys/{}", self.config.primary_endpoint, did);
        
        // Implementation would fetch key
        Ok(PublicDIDKey::default())
    }

    /// Get registry metadata
    pub fn get_metadata(&self) -> Result<RegistryMetadata, RegistryError> {
        let url = format!("{}/api/v1/metadata", self.config.primary_endpoint);
        
        // Implementation would fetch metadata
        Ok(RegistryMetadata::default())
    }

    /// List active mirrors
    pub fn list_mirrors(&self) -> Result<Vec<MirrorInfo>, RegistryError> {
        let url = format!("{}/api/v1/mirrors", self.config.primary_endpoint);
        
        // Implementation would fetch mirror list
        Ok(vec![])
    }
}

/// Mirror information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirrorInfo {
    pub endpoint: String,
    pub status: String,
    pub last_sync: i64,
    pub artifact_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_client_creation() {
        let config = RegistryConfig::default();
        let client = RegistryClient::new(config);
        assert!(client.is_ok());
    }

    #[test]
    fn test_search_query_creation() {
        let query = SearchQuery {
            query: "ecological".to_string(),
            artifact_type: Some("sourze".to_string()),
            limit: Some(100),
            offset: Some(0),
            sort_by: Some("relevance".to_string()),
        };
        assert_eq!(query.query, "ecological");
    }
}
