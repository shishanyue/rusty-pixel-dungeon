
pub struct SeedEngine{
    rng:rand::rngs::StdRng,
}

impl Default for SeedEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SeedEngine {
    pub fn new() -> Self{
        SeedEngine{
            rng:rand::SeedableRng::seed_from_u64(5235235)
        }
    }

    
}