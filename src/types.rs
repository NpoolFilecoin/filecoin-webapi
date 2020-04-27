use std::collections::BTreeMap;
use std::path::PathBuf;

use filecoin_proofs_api::{ChallengeSeed, Commitment, PrivateReplicaInfo, RegisteredPoStProof, SectorId};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct WebRegisteredPoStProof(u32);

impl WebRegisteredPoStProof {
    pub fn as_object(&self) -> RegisteredPoStProof {
        match self.0 {
            0 => RegisteredPoStProof::StackedDrgWinning2KiBV1,
            1 => RegisteredPoStProof::StackedDrgWinning8MiBV1,
            2 => RegisteredPoStProof::StackedDrgWinning512MiBV1,
            3 => RegisteredPoStProof::StackedDrgWinning32GiBV1,
            4 => RegisteredPoStProof::StackedDrgWindow2KiBV1,
            5 => RegisteredPoStProof::StackedDrgWindow8MiBV1,
            6 => RegisteredPoStProof::StackedDrgWindow512MiBV1,
            7 => RegisteredPoStProof::StackedDrgWinning32GiBV1,
            _ => unreachable!(),
        }
    }
}

pub type WebProverId = [u8; 32];
pub type WebTicket = [u8; 32];
pub type WebSnarkProof = Vec<u8>;
pub type WebUnpaddedBytesAmount = u64;

#[derive(Deserialize, Debug, Clone)]
pub struct WebReplicas(Vec<WebReplica>);

impl WebReplicas {
    pub fn as_object(&self) -> BTreeMap<SectorId, PrivateReplicaInfo> {
        self.0
            .iter()
            .map(|x| (x.sector_id, x.private_replica_info.as_object()))
            .collect()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct WebPieceInfo {
    pub commitment: Commitment,
    pub size: WebUnpaddedBytesAmount,
}

#[derive(Deserialize, Debug, Clone)]
pub struct WebPrivateReplicaInfo {
    pub registered_proof: WebRegisteredPoStProof,
    pub comm_r: Commitment,
    pub cache_dir: String,
    pub replica_path: String,
}

impl WebPrivateReplicaInfo {
    pub fn as_object(&self) -> PrivateReplicaInfo {
        PrivateReplicaInfo::new(
            self.registered_proof.as_object(),
            self.comm_r,
            PathBuf::from(&self.cache_dir),
            PathBuf::from(&self.replica_path),
        )
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct WebReplica {
    pub sector_id: SectorId,
    pub private_replica_info: WebPrivateReplicaInfo,
}
