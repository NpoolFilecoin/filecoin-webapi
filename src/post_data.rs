use filecoin_proofs_api::{ChallengeSeed, ProverId, RegisteredPoStProof, SectorId};
use serde::Deserialize;

use crate::types::*;

#[derive(Deserialize, Clone, Debug)]
pub struct GenerateWinningPostSectorChallengeData {
    pub proof_type: RegisteredPoStProof,
    pub randomness: ChallengeSeed,
    pub sector_set_len: u64,
    pub prover_id: ProverId,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GenerateWinningPostData {
    pub randomness: ChallengeSeed,
    pub replicas: WebPrivateReplicas,
    pub prover_id: ProverId,
}

#[derive(Deserialize, Clone, Debug)]
pub struct VerifyWinningPostData {
    pub randomness: ChallengeSeed,
    pub proof: String,
    pub replicas: WebPublicReplicas,
    pub prover_id: ProverId,
}

pub type GenerateWindowPostData = GenerateWinningPostData;

pub type VerifyWindowPostData = VerifyWinningPostData;

#[derive(Deserialize, Clone, Debug)]
pub struct ClearCacheData {
    pub sector_size: u64,
    pub cache_path: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SealPreCommitPhase1 {
    pub registered_proof: RegisteredPoStProof,
    pub cache_path: String,
    pub in_path: String,
    pub out_path: String,
    pub prover_id: ProverId,
    pub sector_id: SectorId,
    pub ticket: WebTicket,
    pub piece_infos: Vec<WebPieceInfo>,
}
