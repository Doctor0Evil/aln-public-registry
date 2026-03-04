//! Snapshot Cache - Pinned snapshot storage for offline verification
//!
//! This module manages pinned registry snapshots for offline
//! verification and air-gapped environments.

use crate::types::{SourzeEntry, DowEntry};
use crate::error::RegistryError;
use serde::{Deserialize, Serialize};
use sled::Db;
use uuid::Uuid;
use chrono::Utc;

/// Snapshot cache manager
pub struct SnapshotCache {
    db: Db,
    snapshots_key: &'static [u8],
    artifacts_key: &'static [u8],
}

/// Pinned snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinnedSnapshot {
    pub snapshot_id: String,
    pub created_at: i64,
    pub snapshot_hash: String,
    pub sourze_count: usize,
    pub dow_count: usize,
    pub ledger_anchor: String,
    pub is_active: bool,
}

/// Snapshot metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotMetadata {
    pub version: String,
    pub created_at: i64,
    pub expires_at: Option<i64>,
    pub source_registry: String,
    pub hex_stamp: String,
}

impl SnapshotCache {
    /// Create a new snapshot cache
    pub fn new(cache_path: &str) -> Result<Self, RegistryError> {
        let db = sled::open(cache_path)?;
        Ok(Self {
            db,
            snapshots_key: b"snapshots",
            artifacts_key: b"artifacts",
        })
    }

    /// Create a new pinned snapshot
    pub fn create_snapshot(
        &self,
        sourzes: &[SourzeEntry],
        dows: &[DowEntry],
    ) -> Result<PinnedSnapshot, RegistryError> {
        let snapshot_id = Uuid::new_v4().to_string();
        let snapshot_hash = self.compute_snapshot_hash(sourzes, dows)?;

        let snapshot = PinnedSnapshot {
            snapshot_id,
            created_at: Utc::now().timestamp(),
            snapshot_hash,
            sourze_count: sourzes.len(),
            dow_count: dows.len(),
            ledger_anchor: String::new(),
            is_active: true,
        };

        // Store snapshot metadata
        self.store_snapshot(&snapshot)?;

        // Store artifacts
        self.store_artifacts(sourzes, dows, &snapshot_id)?;

        Ok(snapshot)
    }

    /// Get active snapshot
    pub fn get_active_snapshot(&self) -> Result<Option<PinnedSnapshot>, RegistryError> {
        let snapshots = self.get_snapshots()?;
        Ok(snapshots.into_iter().find(|s| s.is_active))
    }

    /// Verify artifact against snapshot
    pub fn verify_against_snapshot(
        &self,
        artifact_id: &str,
        snapshot_id: &str,
    ) -> Result<bool, RegistryError> {
        // Check if artifact exists in snapshot
        let exists = self.artifact_exists_in_snapshot(artifact_id, snapshot_id)?;
        Ok(exists)
    }

    /// Compute snapshot hash
    fn compute_snapshot_hash(
        &self,
        sourzes: &[SourzeEntry],
        dows: &[DowEntry],
    ) -> Result<String, RegistryError> {
        use sha3::{Digest, Sha3_256};
        
        let mut hasher = Sha3_256::new();
        
        for sourze in sourzes {
            hasher.update(serde_json::to_vec(sourze)?);
        }
        
        for dow in dows {
            hasher.update(serde_json::to_vec(dow)?);
        }

        Ok(format!("0x{}", hex::encode(hasher.finalize())))
    }

    /// Store snapshot metadata
    fn store_snapshot(&self, snapshot: &PinnedSnapshot) -> Result<(), RegistryError> {
        let mut snapshots = self.get_snapshots()?;
        snapshots.push(snapshot.clone());
        
        let data = bincode::serialize(&snapshots)?;
        self.db.insert(self.snapshots_key, data)?;
        
        Ok(())
    }

    /// Get all snapshots
    fn get_snapshots(&self) -> Result<Vec<PinnedSnapshot>, RegistryError> {
        match self.db.get(self.snapshots_key)? {
            Some(data) => Ok(bincode::deserialize(&data)?),
            None => Ok(Vec::new()),
        }
    }

    /// Store artifacts for snapshot
    fn store_artifacts(
        &self,
        sourzes: &[SourzeEntry],
        dows: &[DowEntry],
        snapshot_id: &str,
    ) -> Result<(), RegistryError> {
        let key = format!("artifacts:{}", snapshot_id);
        
        let mut artifacts = Vec::new();
        for sourze in sourzes {
            artifacts.push(RegistryArtifact::Sourze(sourze.clone()));
        }
        for dow in dows {
            artifacts.push(RegistryArtifact::Dow(dow.clone()));
        }

        let data = bincode::serialize(&artifacts)?;
        self.db.insert(key.as_bytes(), data)?;
        
        Ok(())
    }

    /// Check if artifact exists in snapshot
    fn artifact_exists_in_snapshot(
        &self,
        artifact_id: &str,
        snapshot_id: &str,
    ) -> Result<bool, RegistryError> {
        let key = format!("artifacts:{}", snapshot_id);
        
        match self.db.get(key.as_bytes())? {
            Some(data) => {
                let artifacts: Vec<RegistryArtifact> = bincode::deserialize(&data)?;
                Ok(artifacts.iter().any(|a| a.id() == artifact_id))
            }
            None => Ok(false),
        }
    }

    /// Deactivate old snapshot
    pub fn deactivate_snapshot(&self, snapshot_id: &str) -> Result<(), RegistryError> {
        let mut snapshots = self.get_snapshots()?;
        
        for snapshot in &mut snapshots {
            if snapshot.snapshot_id == snapshot_id {
                snapshot.is_active = false;
                break;
            }
        }

        let data = bincode::serialize(&snapshots)?;
        self.db.insert(self.snapshots_key, data)?;
        
        Ok(())
    }

    /// Get snapshot count
    pub fn snapshot_count(&self) -> Result<usize, RegistryError> {
        Ok(self.get_snapshots()?.len())
    }
}

/// Registry artifact (unified type)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegistryArtifact {
    Sourze(SourzeEntry),
    Dow(DowEntry),
}

impl RegistryArtifact {
    pub fn id(&self) -> &str {
        match self {
            RegistryArtifact::Sourze(s) => &s.sourze_id,
            RegistryArtifact::Dow(d) => &d.dow_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_snapshot_cache_creation() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("snapshot.db").to_string_lossy().to_string();
        
        let cache = SnapshotCache::new(&path);
        assert!(cache.is_ok());
    }
}
