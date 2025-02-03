use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct FitnessConfig {
    pub name: String,
}
