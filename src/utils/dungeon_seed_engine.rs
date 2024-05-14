
pub struct DungeonSeedEngine{
    rng:rand::rngs::StdRng,
}

impl Default for DungeonSeedEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl DungeonSeedEngine {
    pub fn new() -> Self{
        DungeonSeedEngine{
            rng:rand::SeedableRng::seed_from_u64(5235235)
        }
    }

    
}