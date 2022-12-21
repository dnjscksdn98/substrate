use std::{sync::Arc, marker::PhantomData};
use crate::communication::gossip::{topic, GossipValidator};
use sp_runtime::traits::Block;
use sc_network_common::{
	protocol::event::Event as NetEvent,
	service::{NetworkEventStream, NetworkRequest},
};
use sc_network::ProtocolName;
use sc_network_gossip::{GossipEngine, Network};

pub struct VRFWorkerParams<B: Block, N> {
	pub network: Arc<N>,
	pub gossip_protocol_name: ProtocolName,
	pub gossip_engine: GossipEngine<B>,
}

pub struct VRFWorker<B: Block, N> {

	pub gossip_engine: GossipEngine<B>,
	_marker: PhantomData<N>
}

impl<B, N> VRFWorker<B, N>
where
	B: Block,
	N: Network<B>
{

	pub fn new(worker_params: VRFWorkerParams<B, N>) {
		let VRFWorkerParams {
			gossip_engine,
			gossip_protocol_name,
			network
		} = worker_params;
	}
}
