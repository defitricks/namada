//! Governance library code

use namada_core::types::address::{self, Address};

/// governance CLI structures
pub mod cli;
/// governance parameters
pub mod parameters;
pub mod pgf;
/// governance storage
pub mod storage;
/// Governance utility functions/structs
pub mod utils;

pub use storage::proposal::{InitProposalData, ProposalType, VoteProposalData};
pub use storage::vote::{StorageProposalVote, VoteType};
pub use storage::{init_proposal, vote_proposal};

/// The governance internal address
pub const ADDRESS: Address = address::GOV;
