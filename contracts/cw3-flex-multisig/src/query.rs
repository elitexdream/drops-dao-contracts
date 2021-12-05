use cosmwasm_std::{Decimal, Uint128};
use cw3::{Status, ThresholdResponse};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::Votes;

/// Information about the current status of a proposal.
///
/// NOTE: this response type is not defined in the cw3 spec so we
/// define it ourselves.
/// Information about the current status of a proposal.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct VoteTallyResponse {
    /// Current proposal status
    pub status: Status,
    /// Required passing criteria
    pub threshold: ThresholdResponse,
    /// Current percentage turnout
    pub quorum: Decimal,
    /// Total number of votes for the proposal
    pub total_votes: Uint128,
    /// Total number of votes possible for the proposal
    pub total_weight: Uint128,
    /// Tally of the different votes
    pub votes: Votes,
}
