pub struct Config {
    pub block_production_interval: u64,
}

impl Config {
    pub fn new(block_production_interval: u64) -> Self {
        Config {
            block_production_interval,
        }
    }
}
