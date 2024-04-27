use super::seed::Seed;

pub struct SeedEngine{
    rng:rand::rngs::StdRng,
}

impl SeedEngine {
    pub fn new() -> Self{
        SeedEngine{
            rng:rand::SeedableRng::seed_from_u64(5235235)
        }
    }

    
}