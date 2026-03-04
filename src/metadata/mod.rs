//! Metadata Index - Searchable metadata for registry artifacts
//!
//! This module provides metadata indexing and search capabilities
//! for Sourzes and DOW artifacts.

use crate::types::{SourzeEntry, DowEntry};
use crate::error::RegistryError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use sled::Db;

/// Metadata index manager
pub struct MetadataIndex {
    db: Db,
    sourze_index_key: &'static [u8],
    dow_index_key: &'static [u8],
    tag_index_key: &'static [u8],
}

/// Indexed metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexedMetadata {
    pub artifact_id: String,
    pub artifact_type: String,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub author_did: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub version: String,
    pub capabilities: Vec<String>,
    pub eco_score: f64,
    pub ndm_ceiling: f64,
}

/// Search index
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchIndex {
    pub by_id: HashMap<String, String>,
    pub by_name: HashMap<String, Vec<String>>,
    pub by_tag: HashMap<String, Vec<String>>,
    pub by_author: HashMap<String, Vec<String>>,
    pub by_capability: HashMap<String, Vec<String>>,
}

impl MetadataIndex {
    /// Create a new metadata index
    pub fn new(index_path: &str) -> Result<Self, RegistryError> {
        let db = sled::open(index_path)?;
        Ok(Self {
            db,
            sourze_index_key: b"sourze_index",
            dow_index_key: b"dow_index",
            tag_index_key: b"tag_index",
        })
    }

    /// Index a Sourze entry
    pub fn index_sourze(&self, entry: &SourzeEntry) -> Result<(), RegistryError> {
        let metadata = IndexedMetadata {
            artifact_id: entry.sourze_id.clone(),
            artifact_type: "sourze".to_string(),
            name: entry.name.clone(),
            description: entry.description.clone(),
            tags: entry.tags.clone(),
            author_did: entry.author_did.clone(),
            created_at: entry.created_at,
            updated_at: entry.updated_at,
            version: entry.version.clone(),
            capabilities: entry.capabilities.clone(),
            eco_score: entry.eco_vector.eco_impact_score,
            ndm_ceiling: entry.ndm_ceiling,
        };

        self.store_metadata(&metadata, self.sourze_index_key)
    }

    /// Index a DOW entry
    pub fn index_dow(&self, entry: &DowEntry) -> Result<(), RegistryError> {
        let metadata = IndexedMetadata {
            artifact_id: entry.dow_id.clone(),
            artifact_type: "dow".to_string(),
            name: entry.name.clone(),
            description: entry.description.clone(),
            tags: entry.tags.clone(),
            author_did: entry.author_did.clone(),
            created_at: entry.created_at,
            updated_at: entry.updated_at,
            version: entry.version.clone(),
            capabilities: vec![],
            eco_score: entry.eco_vector.eco_impact_score,
            ndm_ceiling: 0.0,
        };

        self.store_metadata(&metadata, self.dow_index_key)
    }

    /// Search by query
    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<IndexedMetadata>, RegistryError> {
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();

        // Search by name
        let sourzes = self.get_indexed_metadata(self.sourze_index_key)?;
        for metadata in sourzes {
            if metadata.name.to_lowercase().contains(&query_lower)
                || metadata.description.to_lowercase().contains(&query_lower)
                || metadata.tags.iter().any(|t| t.to_lowercase().contains(&query_lower))
            {
                results.push(metadata);
                if results.len() >= limit {
                    break;
                }
            }
        }

        Ok(results)
    }

    /// Search by tag
    pub fn search_by_tag(&self, tag: &str) -> Result<Vec<IndexedMetadata>, RegistryError> {
        // Implementation would query tag index
        Ok(vec![])
    }

    /// Search by author
    pub fn search_by_author(&self, author_did: &str) -> Result<Vec<IndexedMetadata>, RegistryError> {
        // Implementation would query author index
        Ok(vec![])
    }

    /// Store metadata
    fn store_metadata(
        &self,
        metadata: &IndexedMetadata,
        index_key: &[u8],
    ) -> Result<(), RegistryError> {
        let mut index = self.get_index(index_key)?;
        index.push(metadata.clone());
        
        let data = bincode::serialize(&index)?;
        self.db.insert(index_key, data)?;
        
        Ok(())
    }

    /// Get indexed metadata
    fn get_indexed_metadata(&self, index_key: &[u8]) -> Result<Vec<IndexedMetadata>, RegistryError> {
        match self.db.get(index_key)? {
            Some(data) => Ok(bincode::deserialize(&data)?),
            None => Ok(Vec::new()),
        }
    }

    /// Get index
    fn get_index(&self, index_key: &[u8]) -> Result<Vec<IndexedMetadata>, RegistryError> {
        self.get_indexed_metadata(index_key)
    }

    /// Remove metadata
    pub fn remove_metadata(&self, artifact_id: &str) -> Result<(), RegistryError> {
        // Implementation would remove from index
        Ok(())
    }

    /// Get index statistics
    pub fn get_stats(&self) -> Result<IndexStats, RegistryError> {
        let sourzes = self.get_indexed_metadata(self.sourze_index_key)?.len();
        let dows = self.get_indexed_metadata(self.dow_index_key)?.len();

        Ok(IndexStats {
            total_sourzes: sourzes,
            total_dows: dows,
            total_artifacts: sourzes + dows,
        })
    }
}

/// Index statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexStats {
    pub total_sourzes: usize,
    pub total_dows: usize,
    pub total_artifacts: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_metadata_index_creation() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("index.db").to_string_lossy().to_string();
        
        let index = MetadataIndex::new(&path);
        assert!(index.is_ok());
    }
}
