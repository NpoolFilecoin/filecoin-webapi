use filecoin_proofs_api::{ChallengeSeed, SectorId};
use serde::Deserialize;

use crate::types::*;

#[derive(Deserialize, Clone, Debug)]
pub struct GenerateWinningPostData {
    pub randomness: ChallengeSeed,
    pub replicas: WebReplicas,
    pub prover_id: WebProverId,
}

#[derive(Deserialize, Clone, Debug)]
pub struct VerifyWinningPostData {
    randomness: ChallengeSeed,
    proof: String,
    replicas: WebReplicas,
    prover_id: WebProverId,
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
    prover_id: WebProverId,
    sector_id: SectorId,
    ticket: WebTicket,
    piece_infos: Vec<WebPieceInfo>,
}

// #[derive(Deserialize, Clone, Debug)]
// pub struct SealPreCommitPhase2 {
//     phase1_output:
// }
