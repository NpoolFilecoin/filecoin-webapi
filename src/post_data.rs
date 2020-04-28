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

type GenerateWindowPostData = GenerateWinningPostData;

type VerifyWindowPostData = VerifyWinningPostData;

#[derive(Deserialize, Clone, Debug)]
pub struct ClearCacheData {
    sector_size: u64,
    cache_path: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SealPreCommitPhase1 {
    registered_proof: WebRegisteredPoStProof,
    cache_path: String,
    in_path: String,
    out_path: String,
    prover_id: ProverId,
    sector_id: SectorId,
    ticket: WebTicket,
    piece_infos: Vec<WebPieceInfo>,
}
