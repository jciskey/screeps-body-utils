//! Provides helper methods for interacting with creep bodies.
//!
//! This includes generating bodies, calculating body effectiveness,
//! and similar calculations.
//!
//! ```rust
//! use screeps::HARVEST_POWER;
//! use screeps_body_utils::body::body_specification::generate_bodyspec_from_string;
//!
//! let bodyspec = generate_bodyspec_from_string("6W3M").unwrap();
//! assert_eq!(HARVEST_POWER * 6, bodyspec.harvest_energy_amount());
//! ```

mod bodyspec;
mod body_generation;
mod body_calculation_helpers;
mod boost_selection_config;

pub use bodyspec::*;

/// Provides helpers for generating bodies from strings.
///
/// ```rust
/// use screeps::Part;
/// use screeps_body_utils::body::body_specification::generate_body_from_string;
///
/// let res = generate_body_from_string("6W3M");
/// let body = res.unwrap();
/// assert_eq!(9, body.len());
/// 
/// for i in 0..6 {
///   assert_eq!(Part::Work, body[i]);
/// }
///
/// for i in 6..9 {
///   assert_eq!(Part::Move, body[i]);
/// }
/// ```
///
/// The grammar used to convert strings to parts is:
///
/// | Character | Part |
/// | --------- | ---- |
/// | M | [Move](screeps::Part::Move) |
/// | W | [Work](screeps::Part::Work) |
/// | C | [Carry](screeps::Part::Carry) |
/// | A | [Attack](screeps::Part::Attack) |
/// | R | [RangedAttack](screeps::Part::RangedAttack) |
/// | T | [Tough](screeps::Part::Tough) |
/// | H | [Heal](screeps::Part::Heal) |
/// | L | [Claim](screeps::Part::Claim) |
///
pub mod body_specification {
    pub use super::body_generation::{
        GenerateBodyError,
        generate_body_from_string,
        generate_bodyspec_from_string,
        convert_character_to_part,
        parse_part_groups,
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

