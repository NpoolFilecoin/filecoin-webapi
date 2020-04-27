use serde::Deserialize;

use crate::types::*;

#[derive(Deserialize, Clone, Debug)]
pub struct GenerateWinningPostData {
    randomness: WebChallengeSeed,
    replicas: WebReplicas,
    prover_id: WebProverId,
}