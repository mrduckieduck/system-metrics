extern crate serde_derive;

use serde::Serialize;

#[derive(Serialize)]
pub struct Metrics {
    pub total_memory: u64, 
    pub total_used_memory: u64,
    pub total_swap: u64,
    pub total_used_swap: u64
}