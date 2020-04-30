use crate::types::*;

use filecoin_proofs_api::seal::{SealCommitPhase1Output, SealPreCommitPhase1Output, SealPreCommitPhase2Output};
use filecoin_proofs_api::{
    Commitment, ProverId, RegisteredSealProof, SectorId, Ticket, UnpaddedByteIndex, UnpaddedBytesAmount,
};
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct ClearCacheData {
    pub sector_size: u64,
    pub cache_path: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SealPreCommitPhase1Data {
    pub registered_proof: RegisteredSealProof,
    pub cache_path: String,
    pub in_path: String,
    pub out_path: String,
    pub prover_id: ProverId,
    pub sector_id: SectorId,
    pub ticket: Ticket,
    pub piece_infos: Vec<WebPieceInfo>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SealPreCommitPhase2Data {
    pub phase1_output: SealPreCommitPhase1Output,
    pub cache_path: String,
    pub out_path: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ComputeCommDData {
    pub registered_proof: RegisteredSealProof,
    pub piece_infos: Vec<WebPieceInfo>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SealCommitPhase1Data {
    pub cache_path: String,
    pub replica_path: String,
    pub prover_id: ProverId,
    pub sector_id: SectorId,
    pub ticket: Ticket,
    pub seed: Ticket,
    pub pre_commit: SealPreCommitPhase2Output,
    pub piece_infos: Vec<WebPieceInfo>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SealCommitPhase2Data {
    pub phase1_output: SealCommitPhase1Output,
    pub prover_id: ProverId,
    pub sector_id: SectorId,
}

#[derive(Deserialize, Clone, Debug)]
pub struct VerifySealData {
    pub registered_proof: RegisteredSealProof,
    pub comm_r_in: Commitment,
    pub comm_d_in: Commitment,
    pub prover_id: ProverId,
    pub sector_id: SectorId,
    pub ticket: Ticket,
    pub seed: Ticket,
    pub proof_vec: Vec<u8>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct VerifyBatchSealData {
    pub registered_proof: RegisteredSealProof,
    pub comm_r_ins: Vec<Commitment>,
    pub comm_d_ins: Vec<Commitment>,
    pub prover_ids: Vec<ProverId>,
    pub sector_ids: Vec<SectorId>,
    pub tickets: Vec<Ticket>,
    pub seeds: Vec<Ticket>,
    pub proof_vecs: Vec<Vec<u8>>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GetUnsealedRangeData {
    pub registered_proof: RegisteredSealProof,
    pub cache_path: String,
    pub sealed_path: String,
    pub output_path: String,
    pub prover_id: ProverId,
    pub sector_id: SectorId,
    pub comm_d: Commitment,
    pub ticket: Ticket,
    pub offset: UnpaddedByteIndex,
    pub num_bytes: UnpaddedBytesAmount,
}

// #[derive(Deserialize, Clone, Debug)]
// pub struct GeneratePieceCommitmentData {
//     pub registered_proof: RegisteredSealProof,
//     pub source:
// }
