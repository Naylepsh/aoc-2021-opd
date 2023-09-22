use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{AocInput, AocSolution, AocTask, traits::*};

pub struct GiantSquid;

impl AocTask for GiantSquid {
    fn directory(&self) -> PathBuf {
        "tasks/04_giant_squid".into()
    }

    fn solution(&self, input: AocInput, phase: usize) -> Result<AocSolution, BoxedError> {
        input.flatten().solved()
    }
}
