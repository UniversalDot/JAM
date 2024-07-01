pub struct Config {
    pub block_production_interval: u64,
    pub difficulty: usize,
    pub block_reward: f64,
    pub genesis_block_producer: String,
}

impl Config {
    pub fn new(block_production_interval: u64, difficulty: usize, block_reward: f64, genesis_block_producer: String) -> Self {
        Config {
            block_production_interval,
            difficulty,
            block_reward,
            genesis_block_producer,
        }
    }
}
