
struct ConsistencyAlgorithm {
	/// The minimum number of consistent blocks.
	min_consistent_blocks: u64,
	/// The blocks currently in use.
	consistent_blocks: Vec<(u64, u128)>,
}

impl ConsistencyAlgorithm {
	/// Called when a block is received.
	pub fn on_block(&mut self, slot_number: u64, timestamp: u128)  {

	}
}