use filecoin_proofs_api::{ChallengeSeed, ProverId, RegisteredPoStProof};
use serde::{Deserialize, Serialize};

use crate::types::*;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GenerateWinningPostSectorChallengeData {
    pub proof_type: RegisteredPoStProof,
    pub randomness: ChallengeSeed,
    pub sector_set_len: u64,
    pub prover_id: ProverId,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GenerateWinningPostData {
    pub randomness: ChallengeSeed,
    pub replicas: WebPrivateReplicas,
    pub prover_id: ProverId,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VerifyWinningPostData {
    pub randomness: ChallengeSeed,
    pub proof: String,
    pub replicas: WebPublicReplicas,
    pub prover_id: ProverId,
}

pub type GenerateWindowPostData = GenerateWinningPostData;

pub type VerifyWindowPostData = VerifyWinningPostData;
