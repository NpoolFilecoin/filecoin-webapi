use std::collections::BTreeMap;
use std::path::PathBuf;

use filecoin_proofs_api::{
    Commitment, PieceInfo, PrivateReplicaInfo, PublicReplicaInfo, RegisteredPoStProof, SectorId, UnpaddedBytesAmount,
};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct WebPrivateReplicas(Vec<WebPrivateReplica>);

impl WebPrivateReplicas {
    pub fn as_object(&self) -> BTreeMap<SectorId, PrivateReplicaInfo> {
        self.0
            .iter()
            .map(|x| (x.sector_id, x.private_replica_info.as_object()))
            .collect()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WebPieceInfo {
    pub commitment: Commitment,
    pub size: UnpaddedBytesAmount,
}

impl WebPieceInfo {
    pub fn as_object(&self) -> PieceInfo {
        PieceInfo::new(self.commitment, self.size).unwrap()
    }

    pub fn from_object(piece_info: PieceInfo) -> Self {
        Self {
            commitment: piece_info.commitment,
            size: piece_info.size,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct WebPrivateReplicaInfo {
    pub registered_proof: RegisteredPoStProof,
    pub comm_r: Commitment,
    pub cache_dir: String,
    pub replica_path: String,
}

impl WebPrivateReplicaInfo {
    pub fn as_object(&self) -> PrivateReplicaInfo {
        PrivateReplicaInfo::new(
            self.registered_proof,
            self.comm_r,
            PathBuf::from(&self.cache_dir),
            PathBuf::from(&self.replica_path),
        )
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct WebPrivateReplica {
    pub sector_id: SectorId,
    pub private_replica_info: WebPrivateReplicaInfo,
}

#[derive(Deserialize, Debug, Clone)]
pub struct WebPublicReplica {
    pub sector_id: SectorId,
    pub public_replica_info: WebPublicReplicaInfo,
}

#[derive(Deserialize, Debug, Clone)]
pub struct WebPublicReplicaInfo {
    pub registered_proof: RegisteredPoStProof,
    pub comm_r: Commitment,
    pub sector_id: u64,
}

impl WebPublicReplicaInfo {
    pub fn as_object(&self) -> PublicReplicaInfo {
        PublicReplicaInfo::new(self.registered_proof, self.comm_r)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct WebPublicReplicas(Vec<WebPublicReplica>);

impl WebPublicReplicas {
    pub fn as_object(&self) -> BTreeMap<SectorId, PublicReplicaInfo> {
        self.0
            .iter()
            .map(|x| (x.sector_id, x.public_replica_info.as_object()))
            .collect()
    }
}
