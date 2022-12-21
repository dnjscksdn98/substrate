use sc_network_gossip::{Validator, ValidationResult};
use sp_runtime::{
	traits::{Block, Header, Hash},
	scale_info::TypeInfo,
};
use sp_consensus_vrf::schnorrkel::{VRFOutput, VRFProof};
use codec::{Decode, Encode};
use sp_core::crypto::Pair;
use crate::AuthorityId;

/// Topic for gossip
pub fn topic<B: Block>() -> B::Hash {
	<<B::Header as Header>::Hashing as Hash>::hash(b"randomness")
}

/// Validator for VRF gossip
pub struct GossipValidator<B: Block>
{
	pub topic: B::Hash
}

impl<B: Block> Validator<B> for GossipValidator<B> {
	fn validate(
			&self,
			context: &mut dyn sc_network_gossip::ValidatorContext<B>,
			sender: &sc_network::PeerId,
			data: &[u8],
		) -> sc_network_gossip::ValidationResult<<B as Block>::Hash> {
		// ToDo!
		ValidationResult::Discard
	}
}

/// Message for gossip
#[derive(Decode, Encode, TypeInfo)]
pub struct GossipMessage<P: Pair> {
	pub id: AuthorityId<P>,
	pub vrf_output: VRFOutput,
	pub vrf_proof: VRFProof,
}
