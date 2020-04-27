use serde::Deserialize;

pub type WebRegisteredPoStProof = u32;
pub type WebChallengeSeed = [u8; 32];
pub type WebCommitment = [u8; 32];
pub type WebProverId = [u8; 32];
pub type WebTicket = [u8; 32];
pub type WebSnarkProof = Vec<u8>;
pub type WebSectorId = u64;
pub type WebUnpaddedBytesAmount = u64;

pub type WebReplicas = Vec<WebReplica>;

#[derive(Deserialize, Debug, Clone)]
pub struct WebPieceInfo {
    commitment: WebCommitment,
    size: WebUnpaddedBytesAmount,
}

#[derive(Deserialize, Debug, Clone)]
pub struct WebPrivateReplicaInfo {
    registered_proof: WebRegisteredPoStProof,
    comm_r: WebCommitment,
    cache_dir: String,
    replica_path: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct WebReplica {
    sector_id: WebSectorId,
    private_replica_info: WebPrivateReplicaInfo,
}
