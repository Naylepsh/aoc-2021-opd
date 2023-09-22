use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{AocInput, AocSolution, AocTask, traits::*};

pub struct ExtendedPolymerization;

impl AocTask for ExtendedPolymerization {
    fn directory(&self) -> PathBuf {
        "tasks/14_extended_polymerization".into()
    }

    fn solution(&self, input: AocInput, phase: usize) -> Result<AocSolution, BoxedError> {
        input.flatten().solved()
    }
}
