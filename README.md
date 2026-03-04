# ALN Public Registry

**Public, mirrored registry of approved Sourzes and DOW artifacts with distributed mirror synchronization**

[![License: CC0](https://img.shields.io/badge/License-CC0-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/aln-public-registry.svg)](https://crates.io/crates/aln-public-registry)
[![Docs](https://docs.rs/aln-public-registry/badge.svg)](https://docs.rs/aln-public-registry)
[![Hex-Stamp](https://img.shields.io/badge/hex--stamp-0xff1f7e0d9c6b2a4f3e8d7c6b5a4f3e2d1c0b9a89-green.svg)](docs/security/hex-stamp-attestation.md)
[![Audit Status](https://img.shields.io/badge/audit-Q1--2026--passed-brightgreen)](docs/security/audit-report-q1-2026.md)

## Purpose

`aln-public-registry` is the **public access layer** for the ALN Sovereign Stack. It provides a transparent, distributed registry of approved Sourzes and DOW artifacts with mirror synchronization, offline verification, and takedown protocols for compromised artifacts.

This guarantees:
- **Public Transparency** - All approved artifacts publicly discoverable
- **Distributed Mirrors** - Resilient access via multiple mirror nodes
- **Offline Verification** - Pinned snapshots for air-gapped verification
- **Takedown Protocol** - Rapid response to compromised artifacts
- **No Private Keys** - Only public metadata and verification keys exposed

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    PUBLIC ACCESS                                 │
│         (Developers / Citizens / Auditors / Researchers)         │
└────────────────────────────┬────────────────────────────────────┘
                             │ Registry Queries
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                    aln-public-registry                           │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │  Registry API (REST + GraphQL + IPFS)                     │  │
│  └───────────────────────────────────────────────────────────┘  │
│          │                  │                  │                │
│          ▼                  ▼                  ▼                │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │MirrorSync    │  │SnapshotCache │  │TakedownMgr   │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
│          │                  │                  │                │
│          └──────────────────┼──────────────────┘                │
│                             ▼                                    │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │  Distributed Mirror Network (IPFS + HTTP + Torrent)       │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                    SOURCE REGISTRY                               │
│         (sovereigntycore → ROW/RPM → Googolswarm)                │
└─────────────────────────────────────────────────────────────────┘
```

## Key Components

| Component | Description |
|-----------|-------------|
| `RegistryAPI` | REST + GraphQL API for artifact discovery |
| `MirrorSync` | Distributed mirror synchronization protocol |
| `SnapshotCache` | Pinned snapshot storage for offline verification |
| `TakedownManager` | Compromised artifact takedown protocol |
| `MetadataIndex` | Searchable metadata for Sourzes and DOWs |
| `KeyServer` | Public DID key distribution (no private keys) |

## Registry Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/v1/sourzes` | GET | List approved Sourzes |
| `/api/v1/sourzes/{id}` | GET | Get Sourze metadata |
| `/api/v1/dows` | GET | List approved DOW artifacts |
| `/api/v1/dows/{id}` | GET | Get DOW metadata |
| `/api/v1/snapshots` | GET | List pinned snapshots |
| `/api/v1/keys/{did}` | GET | Get public DID key |
| `/api/v1/takedown` | POST | Report compromised artifact |
| `/api/v1/mirrors` | GET | List active mirror nodes |

## Quick Start

```bash
# Clone the repository
git clone https://github.com/aln-sovereign/aln-public-registry.git
cd aln-public-registry

# Build registry server
cargo build --release

# Run registry server
cargo run --bin registry-server -- --port 8080

# Run mirror sync daemon
cargo run --bin mirror-sync -- --source https://registry.aln.io

# Query registry via CLI
cargo run --bin registry-cli -- search --type sourze --query ecological

# Verify artifact offline
cargo run --bin registry-cli -- verify --artifact sourze-123.zes
```

## Mirror Network

| Mirror Type | Protocol | Purpose |
|-------------|----------|---------|
| Primary | HTTPS | Official registry source |
| Secondary | IPFS | Decentralized distribution |
| Tertiary | Torrent | High-bandwidth distribution |
| Offline | Local Cache | Air-gapped verification |

## Security Properties

- **Public Metadata** - All artifact metadata publicly auditable
- **Private Keys Protected** - Only public keys distributed
- **Mirror Resilience** - Multiple mirrors prevent single-point failure
- **Takedown Capability** - Compromised artifacts can be revoked
- **Offline Verification** - Pinned snapshots enable air-gapped use

## Governance

All registry operations require:
1. **ROW/RPM Anchoring** - All additions logged to immutable ledger
2. **Multi-Sig Approval** - New artifacts require multi-DID approval
3. **Public Audit Trail** - All takedowns publicly documented
4. **Mirror Verification** - Mirrors must verify against source

**Hex-Stamp Attestation:** `0xff1f7e0d9c6b2a4f3e8d7c6b5a4f3e2d1c0b9a89f8e7d6c5b4a3928170f6e5d4`  
**Ledger Reference:** `row:aln-public-registry:v1.0.0:2026-03-04`  
**Organichain Anchor:** `org:pending`

## License

CC0 (Public Domain) for metadata, individual Sourze licenses vary - See LICENSE for details.

---

**⚠️ Registry Notice:** This registry distributes public metadata only. Private keys and signing materials are never exposed. Verify all artifacts against pinned snapshots before use.
```
