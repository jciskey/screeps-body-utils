//! Module for working with boosts, lab reactions, and material calculations.

#[allow(clippy::module_inception)]
pub mod boost;
pub mod bom;
pub mod reaction;
pub mod reaction_chains;

pub use boost::{
	ATTACK_BOOSTS, RANGED_ATTACK_BOOSTS, HEAL_BOOSTS,
	BUILD_REPAIR_BOOSTS, HARVEST_BOOSTS, DISMANTLE_BOOSTS,
	UPGRADE_CONTROLLER_BOOSTS, CARRY_BOOSTS, MOVE_BOOSTS,
	TOUGH_BOOSTS, T1_BOOSTS, T2_BOOSTS, T3_BOOSTS,
};

pub use boost::AbstractBoost;

#[cfg(test)]
mod bom_tests;

#[cfg(test)]
mod reaction_tests;
