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
    T1_ATTACK_MULTIPLIER, T2_ATTACK_MULTIPLIER, T3_ATTACK_MULTIPLIER,
    T1_HARVEST_MULTIPLIER, T2_HARVEST_MULTIPLIER, T3_HARVEST_MULTIPLIER,
    T1_CARRY_MULTIPLIER, T2_CARRY_MULTIPLIER, T3_CARRY_MULTIPLIER,
    T1_RANGED_ATTACK_MULTIPLIER, T2_RANGED_ATTACK_MULTIPLIER, T3_RANGED_ATTACK_MULTIPLIER,
    T1_BUILD_REPAIR_MULTIPLIER, T2_BUILD_REPAIR_MULTIPLIER, T3_BUILD_REPAIR_MULTIPLIER,
    T1_HEAL_MULTIPLIER, T2_HEAL_MULTIPLIER, T3_HEAL_MULTIPLIER,
    T1_DISMANTLE_MULTIPLIER, T2_DISMANTLE_MULTIPLIER, T3_DISMANTLE_MULTIPLIER,
    T1_MOVE_MULTIPLIER, T2_MOVE_MULTIPLIER, T3_MOVE_MULTIPLIER,
    T1_UPGRADE_CONTROLLER_MULTIPLIER, T2_UPGRADE_CONTROLLER_MULTIPLIER, T3_UPGRADE_CONTROLLER_MULTIPLIER,
    T1_TOUGH_MULTIPLIER, T2_TOUGH_MULTIPLIER, T3_TOUGH_MULTIPLIER,
};

pub use boost::AbstractBoost;

#[cfg(test)]
mod boost_tests;

#[cfg(test)]
mod bom_tests;

#[cfg(test)]
mod reaction_tests;
