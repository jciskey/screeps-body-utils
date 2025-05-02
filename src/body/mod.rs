///! Provides helper methods for interacting with creep bodies.
///!
///! This includes generating bodies, calculating body effectiveness,
///! and similar calculations.

mod bodyspec;
mod body_generation;
mod body_calculation_helpers;
mod boost_selection_config;

pub use bodyspec::*;

/// Provides helpers for generating bodies from strings.
pub mod body_specification {
    pub use super::body_generation::{
        GenerateBodyError,
        generate_body_from_string,
        convert_character_to_part,
    };
}

/// Provides helpers for calculating the body needed for a desired level of creep capabilities.
pub mod body_calculations {
    pub use super::boost_selection_config::*;
    pub use super::body_calculation_helpers::*;
}

#[cfg(test)]
mod body_generation_tests;

#[cfg(test)]
mod bodyspec_tests;

