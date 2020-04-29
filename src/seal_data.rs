use filecoin_proofs_api::{ProverId, RegisteredPoStProof, SectorId, Ticket};
use serde::Deserialize;
use std::path::Path;

use crate::types::*;

#[derive(Deserialize, Clone, Debug)]
pub struct ClearCacheData {
    pub sector_size: u64,
    pub cache_path: Path,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SealPreCommitPhase1Data {
    pub registered_proof: RegisteredPoStProof,
    pub cache_path: String,
    pub in_path: String,
    pub out_path: String,
    pub prover_id: ProverId,
    pub sector_id: SectorId,
    pub ticket: Ticket,
    pub piece_infos: Vec<WebPieceInfo>,
}
