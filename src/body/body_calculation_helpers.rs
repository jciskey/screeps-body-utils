use screeps::constants::extra::{
    MOVE_COST_PLAIN,
    MOVE_COST_ROAD,
};
use screeps::Part;
use crate::helpers::functions::{const_ceil_f32, const_floor_f32};
use crate::boost::boost::{AbstractBoost, BoostCategory};
use super::body_calculations::{BoostSelectionConfig, BoostTierChoice};
use super::PartSpec;


/// Errors that can occur when validating the input to construct a PartsSummary.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PartsSummaryValidationError {
    /// The number of boosts cannot be more than the number of parts
    TooManyBoosts,

    /// The number of parts cannot be more than 50 (the limit for a creep body)
    TooManyParts,
}

/// Encapsulates a number of parts, as well as a number of boosts of each tier.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PartsSummary {
    num_parts: usize,
    num_t1_boosts: usize,
    num_t2_boosts: usize,
    num_t3_boosts: usize,
}

impl PartsSummary {
    /// Create a new PartsSummary while validating the input.
    pub const fn checked_new(num_parts: usize, num_t1_boosts: usize, num_t2_boosts: usize, num_t3_boosts: usize) -> Result<PartsSummary, PartsSummaryValidationError> {
        if num_parts > 50 {
            return Err(PartsSummaryValidationError::TooManyParts);
        }

        let total_boosts = num_t1_boosts + num_t2_boosts + num_t3_boosts;

        if total_boosts > num_parts {
            Err(PartsSummaryValidationError::TooManyBoosts)
        } else {
            Ok(PartsSummary {
                num_parts,
                num_t1_boosts,
                num_t2_boosts,
                num_t3_boosts,
            })
        }
    }

    /// Create a new PartsSummary without validating the input.
    ///
    /// Method results on an object with invalid input are not guaranteed to be correct, and could
    /// result in panics. Use this at your own discretion.
    pub const fn unchecked_new(num_parts: usize, num_t1_boosts: usize, num_t2_boosts: usize, num_t3_boosts: usize) -> PartsSummary {
        PartsSummary {
            num_parts,
            num_t1_boosts,
            num_t2_boosts,
            num_t3_boosts,
        }
    }

    /// Returns the number of parts total
    pub const fn num_parts(&self) -> usize {
        self.num_parts
    }

    /// Returns the number of unboosted parts
    pub const fn num_unboosted_parts(&self) -> usize {
        self.num_parts - (self.num_t1_boosts + self.num_t2_boosts + self.num_t3_boosts)
    }

    /// Returns the number of Tier 1 boosted parts
    pub const fn num_t1_parts(&self) -> usize {
        self.num_t1_boosts
    }

    /// Returns the number of Tier 2 boosted parts
    pub const fn num_t2_parts(&self) -> usize {
        self.num_t2_boosts
    }

    /// Returns the number of Tier 3 boosted parts
    pub const fn num_t3_parts(&self) -> usize {
        self.num_t3_boosts
    }
}

/// Errors resulting from part amount calculations.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PartsNeededCalculationError {
    /// The target amount requires more parts than can be put onto a creep body
    TooManyNeededParts,
}

/// Internal struct to abstract and encapsulate parts-needed calculations, avoiding the need for
/// generics or monomorphization.
#[derive(Debug, PartialEq, Copy, Clone)]
enum IterativeCalculationParams {
    U32 {
        target_amount: u32,
        unboosted_power: u32,
        t1_power: u32,
        t2_power: u32,
        t3_power: u32,
        current_power: u32,
    },
    F32 {
        target_amount: f32,
        unboosted_power: f32,
        t1_power: f32,
        t2_power: f32,
        t3_power: f32,
        current_power: f32,
    },
}

impl IterativeCalculationParams {
    const fn new_u32(target_amount: u32, unboosted_power: u32, t1_power: u32, t2_power: u32, t3_power: u32) -> IterativeCalculationParams {
        IterativeCalculationParams::U32 {
            target_amount,
            unboosted_power,
            t1_power,
            t2_power,
            t3_power,
            current_power: 0,
        }
    }

    const fn new_f32(target_amount: f32, unboosted_power: f32, t1_power: f32, t2_power: f32, t3_power: f32) -> IterativeCalculationParams {
        IterativeCalculationParams::F32 {
            target_amount,
            unboosted_power,
            t1_power,
            t2_power,
            t3_power,
            current_power: 0.0,
        }
    }

    const fn current_power_less_than_target_amount(&self) -> bool {
        match self {
            IterativeCalculationParams::U32 { target_amount, current_power, .. } => crate::helpers::functions::const_lt_u32(*current_power, *target_amount),
            IterativeCalculationParams::F32 { target_amount, current_power, .. } => crate::helpers::functions::const_lt_f32(*current_power, *target_amount),
        }
    }

    const fn update_current_power_from_boost_counts(&mut self, unboosted: usize, t1_boosts: usize, t2_boosts: usize, t3_boosts: usize) {
        match self {
            IterativeCalculationParams::U32 { current_power, unboosted_power, t1_power, t2_power, t3_power, .. } => {
                        *current_power = *unboosted_power * (unboosted as u32) +
                                          *t1_power * (t1_boosts as u32) +
                                          *t2_power * (t2_boosts as u32) +
                                          *t3_power * (t3_boosts as u32);
            },
            IterativeCalculationParams::F32 { current_power, unboosted_power, t1_power, t2_power, t3_power, .. } => {
                        *current_power = *unboosted_power * (unboosted as f32) +
                                          *t1_power * (t1_boosts as f32) +
                                          *t2_power * (t2_boosts as f32) +
                                          *t3_power * (t3_boosts as f32);
            },
        }
    }

    const fn num_parts_needed_unboosted(&self) -> usize {
        match self {
            IterativeCalculationParams::U32 { target_amount, unboosted_power, .. } => {
                num_parts_needed_u32(*target_amount, *unboosted_power)
            },
            IterativeCalculationParams::F32 { target_amount, unboosted_power, .. } => {
                num_parts_needed_f32(*target_amount, *unboosted_power)
            },
        }
    }

    const fn num_parts_needed_t1(&self) -> usize {
        match self {
            IterativeCalculationParams::U32 { target_amount, t1_power, .. } => {
                num_parts_needed_u32(*target_amount, *t1_power)
            },
            IterativeCalculationParams::F32 { target_amount, t1_power, .. } => {
                num_parts_needed_f32(*target_amount, *t1_power)
            },
        }
    }

    const fn num_parts_needed_t2(&self) -> usize {
        match self {
            IterativeCalculationParams::U32 { target_amount, t2_power, .. } => {
                num_parts_needed_u32(*target_amount, *t2_power)
            },
            IterativeCalculationParams::F32 { target_amount, t2_power, .. } => {
                num_parts_needed_f32(*target_amount, *t2_power)
            },
        }
    }

    const fn num_parts_needed_t3(&self) -> usize {
        match self {
            IterativeCalculationParams::U32 { target_amount, t3_power, .. } => {
                num_parts_needed_u32(*target_amount, *t3_power)
            },
            IterativeCalculationParams::F32 { target_amount, t3_power, .. } => {
                num_parts_needed_f32(*target_amount, *t3_power)
            },
        }
    }
}

/// Helper function that calculates the number of parts needed to have a power greater than the
/// specified amount.
const fn num_parts_needed_u32(amount: u32, power: u32) -> usize {
    amount.div_ceil(power) as usize
}

/// Helper function that calculates the number of parts needed to have a power greater than the
/// specified amount.
///
/// This will panic if amount or power are NaN, or if power is 0.
const fn num_parts_needed_f32(amount: f32, power: f32) -> usize {
    let raw_division = amount / power;
    let calc_floor = const_floor_f32(raw_division) as usize;
    let calc_ceiling = const_ceil_f32(raw_division) as usize;

    // Determine if we're slightly off because of floating point representation issues in the math
    let calc_floor_amount = calc_floor as f32 * power;

    if calc_floor_amount < amount {
        calc_ceiling
    } else {
        calc_floor
    }
}

/// Generates an array of single-part power amounts for each of the 4 boost tiers applied to the
/// harvest energy action.
///
/// Return array structure is [unboosted_power, t1_power, t2_power, t3_power].
const fn power_array_for_harvest_energy_amount(part_specs: &[PartSpec; 4]) -> [u32; 4] {
    let mut power_array = [0; 4];
    let mut i = 0;
    while i < part_specs.len() {
        power_array[i] = part_specs[i].get_harvest_energy_amount();
        i += 1;
    }
    power_array
}

/// Generates an array of single-part power amounts for each of the 4 boost tiers applied to the
/// harvest mineral action.
///
/// Return array structure is [unboosted_power, t1_power, t2_power, t3_power].
const fn power_array_for_harvest_mineral_amount(part_specs: &[PartSpec; 4]) -> [u32; 4] {
    let mut power_array = [0; 4];
    let mut i = 0;
    while i < part_specs.len() {
        power_array[i] = part_specs[i].get_harvest_mineral_amount();
        i += 1;
    }
    power_array
}

/// Generates an array of single-part power amounts for each of the 4 boost tiers applied to the
/// harvest deposit action.
///
/// Return array structure is [unboosted_power, t1_power, t2_power, t3_power].
const fn power_array_for_harvest_deposit_amount(part_specs: &[PartSpec; 4]) -> [u32; 4] {
    let mut power_array = [0; 4];
    let mut i = 0;
    while i < part_specs.len() {
        power_array[i] = part_specs[i].get_harvest_deposit_amount();
        i += 1;
    }
    power_array
}

/// Generates an array of single-part power amounts for each of the 4 boost tiers applied to the
/// build action.
///
/// Return array structure is [unboosted_power, t1_power, t2_power, t3_power].
const fn power_array_for_build_amount(part_specs: &[PartSpec; 4]) -> [f32; 4] {
    let mut power_array = [0.0; 4];
    let mut i = 0;
    while i < part_specs.len() {
        power_array[i] = part_specs[i].get_build_amount();
        i += 1;
    }
    power_array
}

/// Generates an array of single-part power amounts for each of the 4 boost tiers applied to the
/// repair action.
///
/// Return array structure is [unboosted_power, t1_power, t2_power, t3_power].
const fn power_array_for_repair_amount(part_specs: &[PartSpec; 4]) -> [f32; 4] {
    let mut power_array = [0.0; 4];
    let mut i = 0;
    while i < part_specs.len() {
        power_array[i] = part_specs[i].get_repair_amount();
        i += 1;
    }
    power_array
}

/// Generates an array of single-part power amounts for each of the 4 boost tiers applied to the
/// dismantle action.
///
/// Return array structure is [unboosted_power, t1_power, t2_power, t3_power].
const fn power_array_for_dismantle_damage(part_specs: &[PartSpec; 4]) -> [u32; 4] {
    let mut power_array = [0; 4];
    let mut i = 0;
    while i < part_specs.len() {
        power_array[i] = part_specs[i].get_dismantle_damage();
        i += 1;
    }
    power_array
}

/// Generates an array of single-part power amounts for each of the 4 boost tiers applied to the
/// upgrade controller action.
///
/// Return array structure is [unboosted_power, t1_power, t2_power, t3_power].
const fn power_array_for_upgrade_controller_amount(part_specs: &[PartSpec; 4]) -> [f32; 4] {
    let mut power_array = [0.0; 4];
    let mut i = 0;
    while i < part_specs.len() {
        power_array[i] = part_specs[i].get_upgrade_controller_amount();
        i += 1;
    }
    power_array
}

/// Generates an array of single-part power amounts for each of the 4 boost tiers applied to the
/// attack action.
///
/// Return array structure is [unboosted_power, t1_power, t2_power, t3_power].
const fn power_array_for_attack_damage(part_specs: &[PartSpec; 4]) -> [u32; 4] {
    let mut power_array = [0; 4];
    let mut i = 0;
    while i < part_specs.len() {
        power_array[i] = part_specs[i].get_attack_damage();
        i += 1;
    }
    power_array
}

/// Generates an array of single-part power amounts for each of the 4 boost tiers applied to the
/// ranged attack action.
///
/// Return array structure is [unboosted_power, t1_power, t2_power, t3_power].
const fn power_array_for_ranged_attack_damage(part_specs: &[PartSpec; 4]) -> [u32; 4] {
    let mut power_array = [0; 4];
    let mut i = 0;
    while i < part_specs.len() {
        power_array[i] = part_specs[i].get_ranged_attack_damage();
        i += 1;
    }
    power_array
}

/// Generates an array of single-part power amounts for each of the 4 boost tiers applied to the
/// ranged mass attack action at range 1 to a single target.
///
/// Return array structure is [unboosted_power, t1_power, t2_power, t3_power].
const fn power_array_for_ranged_mass_attack_damage(part_specs: &[PartSpec; 4]) -> [u32; 4] {
    let mut power_array = [0; 4];
    let mut i = 0;
    while i < part_specs.len() {
        power_array[i] = part_specs[i].get_ranged_mass_attack_damage_at_distance_single_target(1);
        i += 1;
    }
    power_array
}

/// Generates an array of single-part power amounts for each of the 4 boost tiers applied to the
/// heal action.
///
/// Return array structure is [unboosted_power, t1_power, t2_power, t3_power].
const fn power_array_for_heal_amount(part_specs: &[PartSpec; 4]) -> [u32; 4] {
    let mut power_array = [0; 4];
    let mut i = 0;
    while i < part_specs.len() {
        power_array[i] = part_specs[i].get_heal_amount();
        i += 1;
    }
    power_array
}

/// Generates an array of single-part power amounts for each of the 4 boost tiers applied to the
/// ranged heal action.
///
/// Return array structure is [unboosted_power, t1_power, t2_power, t3_power].
const fn power_array_for_ranged_heal_amount(part_specs: &[PartSpec; 4]) -> [u32; 4] {
    let mut power_array = [0; 4];
    let mut i = 0;
    while i < part_specs.len() {
        power_array[i] = part_specs[i].get_ranged_heal_amount();
        i += 1;
    }
    power_array
}

/// Generates an array of single-part power amounts for each of the 4 boost tiers applied to
/// carrying capacity.
///
/// Return array structure is [unboosted_power, t1_power, t2_power, t3_power].
const fn power_array_for_carry_capacity(part_specs: &[PartSpec; 4]) -> [u32; 4] {
    let mut power_array = [0; 4];
    let mut i = 0;
    while i < part_specs.len() {
        power_array[i] = part_specs[i].get_carry_capacity();
        i += 1;
    }
    power_array
}

/// Generates an array of single-part power amounts for each of the 4 boost tiers applied to the
/// fatigue reduction of move parts.
///
/// Return array structure is [unboosted_power, t1_power, t2_power, t3_power].
const fn power_array_for_fatigue_reduction(part_specs: &[PartSpec; 4]) -> [u32; 4] {
    let mut power_array = [0; 4];
    let mut i = 0;
    while i < part_specs.len() {
        power_array[i] = part_specs[i].get_fatigue_reduction();
        i += 1;
    }
    power_array
}

/// Generates an array of single-part power amounts for each of the 4 boost tiers applied to damage
/// capacity.
///
/// Return array structure is [unboosted_power, t1_power, t2_power, t3_power].
const fn power_array_for_damage_capacity(part_specs: &[PartSpec; 4]) -> [f32; 4] {
    let mut power_array = [0.0; 4];
    let mut i = 0;
    while i < part_specs.len() {
        power_array[i] = part_specs[i].get_damage_capacity();
        i += 1;
    }
    power_array
}


/// Returns an array of upgrade controller powers for individual parts at each of the 4 boost
/// levels.
///
/// Return array structure is [unboosted_power, t1_power, t2_power, t3_power];
///
/// Returns Some if the boost category has a u32 multiplier, None otherwise;
const fn u32_parts_power_for_boost_category(category: &BoostCategory) -> Option<[u32; 4]> {
    let boosts_array = category.get_abstract_boosts();
    if AbstractBoost::has_f32_multiplier(&boosts_array[0]) {
        None
    } else {
        let part = category.get_associated_part();

        let unboosted_part = PartSpec::new_unboosted_part(part);
        let t1_part = PartSpec::new_boosted_part(part, boosts_array[0]);
        let t2_part = PartSpec::new_boosted_part(part, boosts_array[1]);
        let t3_part = PartSpec::new_boosted_part(part, boosts_array[2]);

        let parts_array = [unboosted_part, t1_part, t2_part, t3_part];

        let power_array = match category {
            BoostCategory::HarvestEnergy => power_array_for_harvest_energy_amount(&parts_array),
            BoostCategory::HarvestMineral => power_array_for_harvest_mineral_amount(&parts_array),
            BoostCategory::HarvestDeposit => power_array_for_harvest_deposit_amount(&parts_array),
            BoostCategory::Build => return None,
            BoostCategory::Repair => return None,
            BoostCategory::Dismantle => power_array_for_dismantle_damage(&parts_array),
            BoostCategory::UpgradeController => return None,
            BoostCategory::Attack => power_array_for_attack_damage(&parts_array),
            BoostCategory::RangedAttack => power_array_for_ranged_attack_damage(&parts_array),
            BoostCategory::RangedMassAttack => power_array_for_ranged_mass_attack_damage(&parts_array),
            BoostCategory::Heal => power_array_for_heal_amount(&parts_array),
            BoostCategory::RangedHeal => power_array_for_ranged_heal_amount(&parts_array),
            BoostCategory::Carry => power_array_for_carry_capacity(&parts_array),
            BoostCategory::Move => power_array_for_fatigue_reduction(&parts_array),
            BoostCategory::Tough => return None,
        };

        Some(power_array)
    }
}

/// Returns an array of upgrade controller powers for individual parts at each of the 4 boost
/// levels.
///
/// Return array structure is [unboosted_power, t1_power, t2_power, t3_power];
///
/// Returns Some if the boost category has a f32 multiplier, None otherwise;
const fn f32_parts_power_for_boost_category(category: &BoostCategory) -> Option<[f32; 4]> {
    let boosts_array = category.get_abstract_boosts();
    if AbstractBoost::has_u32_multiplier(&boosts_array[0]) {
        None
    } else {
        let part = category.get_associated_part();

        let unboosted_part = PartSpec::new_unboosted_part(part);
        let t1_part = PartSpec::new_boosted_part(part, boosts_array[0]);
        let t2_part = PartSpec::new_boosted_part(part, boosts_array[1]);
        let t3_part = PartSpec::new_boosted_part(part, boosts_array[2]);

        let parts_array = [unboosted_part, t1_part, t2_part, t3_part];

        let power_array = match category {
            BoostCategory::HarvestEnergy => return None,
            BoostCategory::HarvestMineral => return None,
            BoostCategory::HarvestDeposit => return None,
            BoostCategory::Build => power_array_for_build_amount(&parts_array),
            BoostCategory::Repair => power_array_for_repair_amount(&parts_array),
            BoostCategory::Dismantle => return None,
            BoostCategory::UpgradeController => power_array_for_upgrade_controller_amount(&parts_array),
            BoostCategory::Attack => return None,
            BoostCategory::RangedAttack => return None,
            BoostCategory::RangedMassAttack => return None,
            BoostCategory::Heal => return None,
            BoostCategory::RangedHeal => return None,
            BoostCategory::Carry => return None,
            BoostCategory::Move => return None,
            BoostCategory::Tough => power_array_for_damage_capacity(&parts_array),
        };

        Some(power_array)
    }
}

/// Calculates the Work part configuration necessary to harvest a specified amount of energy per tick.
///
/// If the boost configuration mandates a specific tier of boost, the number of body parts will be
/// minimized.
///
/// If the boost configuration allows for dynamic tier selection, it will maximize the number of
/// parts unless partial boosts is false, in which case it will maximize the boost tier of each
/// part before adding an additional part.
///
/// The returned number of body parts will total to 50 or less.
pub const fn parts_to_harvest_energy(amount: u32, boost_config: &BoostSelectionConfig) -> Result<PartsSummary, PartsNeededCalculationError> {
    parts_to_action_inner_wrapper_u32(BoostCategory::HarvestEnergy, amount, boost_config)
}

/// Calculates the Work part configuration necessary to harvest a specified amount of minerals per tick.
///
/// If the boost configuration mandates a specific tier of boost, the number of body parts will be
/// minimized.
///
/// If the boost configuration allows for dynamic tier selection, it will maximize the number of
/// parts unless partial boosts is false, in which case it will maximize the boost tier of each
/// part before adding an additional part.
///
/// The returned number of body parts will total to 50 or less.
pub const fn parts_to_harvest_mineral(amount: u32, boost_config: &BoostSelectionConfig) -> Result<PartsSummary, PartsNeededCalculationError> {
    parts_to_action_inner_wrapper_u32(BoostCategory::HarvestMineral, amount, boost_config)
}

/// Calculates the Work part configuration necessary to harvest a specified amount of deposit
/// resources per tick.
///
/// If the boost configuration mandates a specific tier of boost, the number of body parts will be
/// minimized.
///
/// If the boost configuration allows for dynamic tier selection, it will maximize the number of
/// parts unless partial boosts is false, in which case it will maximize the boost tier of each
/// part before adding an additional part.
///
/// The returned number of body parts will total to 50 or less.
pub const fn parts_to_harvest_deposit(amount: u32, boost_config: &BoostSelectionConfig) -> Result<PartsSummary, PartsNeededCalculationError> {
    parts_to_action_inner_wrapper_u32(BoostCategory::HarvestDeposit, amount, boost_config)
}

/// Calculates the Work part configuration necessary to build a specified amount per tick.
///
/// If the boost configuration mandates a specific tier of boost, the number of body parts will be
/// minimized.
///
/// If the boost configuration allows for dynamic tier selection, it will maximize the number of
/// parts unless partial boosts is false, in which case it will maximize the boost tier of each
/// part before adding an additional part.
///
/// The returned number of body parts will total to 50 or less.
pub const fn parts_to_build(amount: f32, boost_config: &BoostSelectionConfig) -> Result<PartsSummary, PartsNeededCalculationError> {
    parts_to_action_inner_wrapper_f32(BoostCategory::Build, amount, boost_config)
}

/// Calculates the Work part configuration necessary to repair a specified amount per tick.
///
/// If the boost configuration mandates a specific tier of boost, the number of body parts will be
/// minimized.
///
/// If the boost configuration allows for dynamic tier selection, it will maximize the number of
/// parts unless partial boosts is false, in which case it will maximize the boost tier of each
/// part before adding an additional part.
///
/// The returned number of body parts will total to 50 or less.
pub const fn parts_to_repair(amount: f32, boost_config: &BoostSelectionConfig) -> Result<PartsSummary, PartsNeededCalculationError> {
    parts_to_action_inner_wrapper_f32(BoostCategory::Repair, amount, boost_config)
}

/// Calculates the Work part configuration necessary to deal a specified amount of dismantle damage
/// per tick.
///
/// If the boost configuration mandates a specific tier of boost, the number of body parts will be
/// minimized.
///
/// If the boost configuration allows for dynamic tier selection, it will maximize the number of
/// parts unless partial boosts is false, in which case it will maximize the boost tier of each
/// part before adding an additional part.
///
/// The returned number of body parts will total to 50 or less.
pub const fn parts_to_dismantle(amount: f32, boost_config: &BoostSelectionConfig) -> Result<PartsSummary, PartsNeededCalculationError> {
    parts_to_action_inner_wrapper_f32(BoostCategory::Dismantle, amount, boost_config)
}

/// Calculates the Work part configuration necessary to increase progress a specified amount with
/// the upgrade controller action per tick.
///
/// If the boost configuration mandates a specific tier of boost, the number of body parts will be
/// minimized.
///
/// If the boost configuration allows for dynamic tier selection, it will maximize the number of
/// parts unless partial boosts is false, in which case it will maximize the boost tier of each
/// part before adding an additional part.
///
/// The returned number of body parts will total to 50 or less.
pub const fn parts_to_upgrade_controller(amount: f32, boost_config: &BoostSelectionConfig) -> Result<PartsSummary, PartsNeededCalculationError> {
    parts_to_action_inner_wrapper_f32(BoostCategory::UpgradeController, amount, boost_config)
}

/// Calculates the Attack part configuration necessary to deal a specified amount of melee damage
/// per tick.
///
/// If the boost configuration mandates a specific tier of boost, the number of body parts will be
/// minimized.
///
/// If the boost configuration allows for dynamic tier selection, it will maximize the number of
/// parts unless partial boosts is false, in which case it will maximize the boost tier of each
/// part before adding an additional part.
///
/// The returned number of body parts will total to 50 or less.
pub const fn parts_to_attack(amount: u32, boost_config: &BoostSelectionConfig) -> Result<PartsSummary, PartsNeededCalculationError> {
    parts_to_action_inner_wrapper_u32(BoostCategory::Attack, amount, boost_config)
}

/// Calculates the RangedAttack part configuration necessary to deal a specified amount of ranged
/// damage per tick.
///
/// If the boost configuration mandates a specific tier of boost, the number of body parts will be
/// minimized.
///
/// If the boost configuration allows for dynamic tier selection, it will maximize the number of
/// parts unless partial boosts is false, in which case it will maximize the boost tier of each
/// part before adding an additional part.
///
/// The returned number of body parts will total to 50 or less.
pub const fn parts_to_ranged_attack(amount: u32, boost_config: &BoostSelectionConfig) -> Result<PartsSummary, PartsNeededCalculationError> {
    parts_to_action_inner_wrapper_u32(BoostCategory::RangedAttack, amount, boost_config)
}

/// Calculates the RangedAttack part configuration necessary to deal a specified amount of ranged
/// mass attack damage to a single target at range 1 per tick.
///
/// If the boost configuration mandates a specific tier of boost, the number of body parts will be
/// minimized.
///
/// If the boost configuration allows for dynamic tier selection, it will maximize the number of
/// parts unless partial boosts is false, in which case it will maximize the boost tier of each
/// part before adding an additional part.
///
/// The returned number of body parts will total to 50 or less.
pub const fn parts_to_ranged_mass_attack(amount: u32, boost_config: &BoostSelectionConfig) -> Result<PartsSummary, PartsNeededCalculationError> {
    parts_to_action_inner_wrapper_u32(BoostCategory::RangedMassAttack, amount, boost_config)
}
            
/// Calculates the Heal part configuration necessary to heal a specified amount per tick.
///
/// If the boost configuration mandates a specific tier of boost, the number of body parts will be
/// minimized.
///
/// If the boost configuration allows for dynamic tier selection, it will maximize the number of
/// parts unless partial boosts is false, in which case it will maximize the boost tier of each
/// part before adding an additional part.
///
/// The returned number of body parts will total to 50 or less.
pub const fn parts_to_heal(amount: u32, boost_config: &BoostSelectionConfig) -> Result<PartsSummary, PartsNeededCalculationError> {
    parts_to_action_inner_wrapper_u32(BoostCategory::Heal, amount, boost_config)
}

/// Calculates the Heal part configuration necessary to heal a specified amount at range > 1 per tick.
///
/// If the boost configuration mandates a specific tier of boost, the number of body parts will be
/// minimized.
///
/// If the boost configuration allows for dynamic tier selection, it will maximize the number of
/// parts unless partial boosts is false, in which case it will maximize the boost tier of each
/// part before adding an additional part.
///
/// The returned number of body parts will total to 50 or less.
pub const fn parts_to_ranged_heal(amount: u32, boost_config: &BoostSelectionConfig) -> Result<PartsSummary, PartsNeededCalculationError> {
    parts_to_action_inner_wrapper_u32(BoostCategory::RangedHeal, amount, boost_config)
}

/// Calculates the Carry part configuration necessary to carry a specified amount of resources.
///
/// If the boost configuration mandates a specific tier of boost, the number of body parts will be
/// minimized.
///
/// If the boost configuration allows for dynamic tier selection, it will maximize the number of
/// parts unless partial boosts is false, in which case it will maximize the boost tier of each
/// part before adding an additional part.
///
/// The returned number of body parts will total to 50 or less.
pub const fn parts_to_carry(amount: u32, boost_config: &BoostSelectionConfig) -> Result<PartsSummary, PartsNeededCalculationError> {
    parts_to_action_inner_wrapper_u32(BoostCategory::Carry, amount, boost_config)
}

/// Calculates the Move part configuration necessary to reduce fatigue a specified amount per tick.
///
/// If the boost configuration mandates a specific tier of boost, the number of body parts will be
/// minimized.
///
/// If the boost configuration allows for dynamic tier selection, it will maximize the number of
/// parts unless partial boosts is false, in which case it will maximize the boost tier of each
/// part before adding an additional part.
///
/// The returned number of body parts will total to 50 or less.
pub const fn parts_to_reduce_fatigue(amount: u32, boost_config: &BoostSelectionConfig) -> Result<PartsSummary, PartsNeededCalculationError> {
    parts_to_action_inner_wrapper_u32(BoostCategory::Move, amount, boost_config)
}

/// Calculates the Tough part configuration necessary to absorb a specified amount of damage.
///
/// If the boost configuration mandates a specific tier of boost, the number of body parts will be
/// minimized.
///
/// If the boost configuration allows for dynamic tier selection, it will maximize the number of
/// parts unless partial boosts is false, in which case it will maximize the boost tier of each
/// part before adding an additional part.
///
/// The returned number of body parts will total to 50 or less.
pub const fn parts_to_absorb_damage(amount: f32, boost_config: &BoostSelectionConfig) -> Result<PartsSummary, PartsNeededCalculationError> {
    parts_to_action_inner_wrapper_f32(BoostCategory::Tough, amount, boost_config)
}

/// Calculates the Move part configuration necessary to allow the provided body to move "off-road"
/// without generating fatigue every tick.
///
/// Off-road in this context means on plain tiles that do not have a road.
///
/// If the boost configuration mandates a specific tier of boost, the number of body parts will be
/// minimized.
///
/// If the boost configuration allows for dynamic tier selection, it will maximize the number of
/// parts unless partial boosts is false, in which case it will maximize the boost tier of each
/// part before adding an additional part.
///
/// The returned number of body parts will total to 50 or less.
pub const fn parts_to_move_offroad(body: &[Part], boost_config: &BoostSelectionConfig) -> Result<PartsSummary, PartsNeededCalculationError> {
    let mut non_move_parts_count = 0;
    let mut i = 0;
    while i < body.len() {
        match body[i] {
            Part::Move => {}, // Do nothing
            _ => non_move_parts_count += 1,
        };
        i += 1;
    }
    parts_to_move_offroad_by_parts_count(non_move_parts_count, boost_config)
}

/// Calculates the Move part configuration necessary to allow a particular number of non-Move parts to move "off-road"
/// without generating fatigue every tick.
///
/// Off-road in this context means on plain tiles that do not have a road.
///
/// This method is useful for when you have a count of non-move parts, and don't need/want to
/// provide a slice of Parts.
///
/// If the boost configuration mandates a specific tier of boost, the number of body parts will be
/// minimized.
///
/// If the boost configuration allows for dynamic tier selection, it will maximize the number of
/// parts unless partial boosts is false, in which case it will maximize the boost tier of each
/// part before adding an additional part.
///
/// The returned number of body parts will total to 50 or less.
pub const fn parts_to_move_offroad_by_parts_count(num_non_move_parts: u32, boost_config: &BoostSelectionConfig) -> Result<PartsSummary, PartsNeededCalculationError> {
    let fatigue_generated = num_non_move_parts * MOVE_COST_PLAIN;
    parts_to_reduce_fatigue(fatigue_generated, boost_config)
}

/// Calculates the Move part configuration necessary to allow the provided body to move on-road
/// without generating fatigue every tick.
///
/// If the boost configuration mandates a specific tier of boost, the number of body parts will be
/// minimized.
///
/// If the boost configuration allows for dynamic tier selection, it will maximize the number of
/// parts unless partial boosts is false, in which case it will maximize the boost tier of each
/// part before adding an additional part.
///
/// The returned number of body parts will total to 50 or less.
pub const fn parts_to_move_onroad(body: &[Part], boost_config: &BoostSelectionConfig) -> Result<PartsSummary, PartsNeededCalculationError> {
    let mut non_move_parts_count = 0;
    let mut i = 0;
    while i < body.len() {
        match body[i] {
            Part::Move => {}, // Do nothing
            _ => non_move_parts_count += 1,
        };
        i += 1;
    }
    parts_to_move_onroad_by_parts_count(non_move_parts_count, boost_config)
}

/// Calculates the Move part configuration necessary to allow a particular number of non-Move parts to move on-road
/// without generating fatigue every tick.
///
/// This method is useful for when you have a count of non-move parts, and don't need/want to
/// provide a slice of Parts.
///
/// If the boost configuration mandates a specific tier of boost, the number of body parts will be
/// minimized.
///
/// If the boost configuration allows for dynamic tier selection, it will maximize the number of
/// parts unless partial boosts is false, in which case it will maximize the boost tier of each
/// part before adding an additional part.
///
/// The returned number of body parts will total to 50 or less.
pub const fn parts_to_move_onroad_by_parts_count(num_non_move_parts: u32, boost_config: &BoostSelectionConfig) -> Result<PartsSummary, PartsNeededCalculationError> {
    let fatigue_generated = num_non_move_parts * MOVE_COST_ROAD;
    parts_to_reduce_fatigue(fatigue_generated, boost_config)
}
/// Internal helper method for generically calculating parts needed for a boost category and a
/// target amount that operate in u32 space.
const fn parts_to_action_inner_wrapper_u32(category: BoostCategory, amount: u32, boost_config: &BoostSelectionConfig) -> Result<PartsSummary, PartsNeededCalculationError> {
    let [unboosted_power, t1_power, t2_power, t3_power] = u32_parts_power_for_boost_category(&category).unwrap();
    let params = IterativeCalculationParams::new_u32(amount, unboosted_power, t1_power, t2_power, t3_power);
    generic_get_parts_needed(params, boost_config)
}

/// Internal helper method for generically calculating parts needed for a boost category and a
/// target amount that operate in f32 space.
const fn parts_to_action_inner_wrapper_f32(category: BoostCategory, amount: f32, boost_config: &BoostSelectionConfig) -> Result<PartsSummary, PartsNeededCalculationError> {
    let [unboosted_power, t1_power, t2_power, t3_power] = f32_parts_power_for_boost_category(&category).unwrap();
    let params = IterativeCalculationParams::new_f32(amount, unboosted_power, t1_power, t2_power, t3_power);
    generic_get_parts_needed(params, boost_config)
}

/// Internal method for generically calculating the parts needed to meet or exceed a particular
/// amount.
///
/// Operates generically across both u32 and f32 by using the IterativeCalculationParams struct
/// to abstract away the type differences, since the overall math and algorithm are the same
/// between both types.
const fn generic_get_parts_needed(params: IterativeCalculationParams, boost_config: &BoostSelectionConfig) -> Result<PartsSummary, PartsNeededCalculationError> {
    let (num_parts, num_t1_boosts, num_t2_boosts, num_t3_boosts) = match boost_config.boost_tier_choice {
        // If the boost config mandates a certain tier of boost, we can do direct calculations to
        // calculate the number of parts necessary
        BoostTierChoice::NoBoosts => {
            let num_parts_needed = params.num_parts_needed_unboosted();
            (num_parts_needed, 0, 0, 0)
        },

        BoostTierChoice::T1Only => {
            let num_parts_needed = params.num_parts_needed_t1();
            (num_parts_needed, num_parts_needed, 0, 0)
        },

        BoostTierChoice::T2Only => {
            let num_parts_needed = params.num_parts_needed_t2();
            (num_parts_needed, 0, num_parts_needed, 0)
        },

        BoostTierChoice::T3Only => {
            let num_parts_needed = params.num_parts_needed_t3();
            (num_parts_needed, 0, 0, num_parts_needed)
        },

        // If the boost config doesn't allow for partial boosts, then we can do direct calculations
        // to determine the minimum tier required and calculate part count directly

        // If the boost config allows for partial boosts then we need to iteratively add parts and
        // modify boosts until we get an amount that works.
        BoostTierChoice::UpToT1 |
        BoostTierChoice::UpToT2 |
        BoostTierChoice::UpToT3 => {
            let unboosted_needed_parts = params.num_parts_needed_unboosted();
            let t1_needed_parts = params.num_parts_needed_t1();
            let t2_needed_parts = params.num_parts_needed_t2();
            let t3_needed_parts = params.num_parts_needed_t3();

            let max_tier: u8 = match boost_config.boost_tier_choice {
                BoostTierChoice::UpToT1 => {
                    1
                },
                BoostTierChoice::UpToT2 => {
                    2
                },
                BoostTierChoice::UpToT3 => {
                    3
                },
                _ => {
                    0 // We can't actually get here, but match isn't quite smart enough to recognize that, so set it to something valid
                },
            };

            // No partial boosts = determine tier, then do direct calculation
            if !boost_config.allow_partial_boosts {
                // Determine tier needed
                // - Assume 50 parts max; don't go over 50 parts unless already at the highest
                // allowed tier
                
                // - Check for unboosted maximum, use that if it's less than 50 parts
                // - Check for T1 needed value, use that if it's less than 50 parts
                //   - If T1 is max tier, use this value even if it's more than 50 parts
                // - Check for T2 needed value, use that if it's less than 50 parts
                //   - If T2 is max tier, use this value even if it's more than 50 parts
                // - Check for T3 needed value, use that (fallback, no other higher tiers to check,
                // so it'll always get used)
                if unboosted_needed_parts <= 50 {
                    (unboosted_needed_parts, 0, 0, 0)
                } else {
                    if max_tier < 1 {
                        // If the max tier is 0, then we can't escalate from here and we
                        // need too many parts
                        return Err(PartsNeededCalculationError::TooManyNeededParts);
                    }

                    if t1_needed_parts <= 50 {
                        (t1_needed_parts, t1_needed_parts, 0, 0)
                    } else {
                        if max_tier < 2 {
                            // If the max tier is 1 or 0, then we can't escalate from here and we
                            // need too many parts
                            return Err(PartsNeededCalculationError::TooManyNeededParts);
                        }

                        if t2_needed_parts <= 50 {
                            (t2_needed_parts, 0, t2_needed_parts, 0)
                        } else {
                            if max_tier < 3 {
                                // If the max tier is 2, 1, or 0, then we can't escalate from here and we
                                // need too many parts
                                return Err(PartsNeededCalculationError::TooManyNeededParts);
                            }

                            (t3_needed_parts, 0, 0, t3_needed_parts)
                        }
                    }
                }
            } else {
                let res = iteratively_calculate_boosted_parts(params, max_tier);
                match res {
                    Some(arr) => (arr[0], arr[1], arr[2], arr[3]),
                    None => return Err(PartsNeededCalculationError::TooManyNeededParts),
                }
            }
        },
    };

    if num_parts > 50 {
        return Err(PartsNeededCalculationError::TooManyNeededParts);
    }

    // We use unchecked new here because we know the number of boosts won't be over the number of
    // parts
    Ok(PartsSummary::unchecked_new(num_parts, num_t1_boosts, num_t2_boosts, num_t3_boosts))
}

/// This function generically calculates how many parts are needed, plus how many boosts of each
/// tier are needed, to minimally-beat a target amount.
///
/// Return contained array is [total_parts, t1_boosts, t2_boosts, t3_boosts].
/// Returns None if more than 50 parts were needed.
const fn iteratively_calculate_boosted_parts(mut params: IterativeCalculationParams, max_boost_tier: u8) -> Option<[usize; 4]> {
    // Iteratively add parts and modify boosts up to the max tier, then repeat
    let mut num_unboosted_parts: usize = 0;
    let mut num_t1_parts: usize = 0;
    let mut num_t2_parts: usize = 0;
    let mut num_t3_parts: usize = 0;
    let mut total_parts: usize = 0;

    while params.current_power_less_than_target_amount() {
        if total_parts > 50 {
            return None;
        }

        if max_boost_tier > 0 {
            // Upgrade any unmaxed parts
            if num_unboosted_parts > 0 {
                // - Unboosted -> T1
                increment_part_tier_u32(&mut num_unboosted_parts, &mut num_t1_parts);
                params.update_current_power_from_boost_counts(num_unboosted_parts, num_t1_parts, num_t2_parts, num_t3_parts);
                continue;
            }

            if max_boost_tier > 1 {
                if num_t1_parts > 0 {
                    // - T1 -> T2
                    increment_part_tier_u32(&mut num_t1_parts, &mut num_t2_parts);
                    params.update_current_power_from_boost_counts(num_unboosted_parts, num_t1_parts, num_t2_parts, num_t3_parts);
                    continue;
                }

                if max_boost_tier > 2 {
                    if num_t2_parts > 0 {
                        // - T2 -> T3
                        increment_part_tier_u32(&mut num_t2_parts, &mut num_t3_parts);
                        params.update_current_power_from_boost_counts(num_unboosted_parts, num_t1_parts, num_t2_parts, num_t3_parts);
                        continue;
                    }
                }
            }
        }

        // Add another unboosted part
        num_unboosted_parts += 1;
        total_parts += 1;
        params.update_current_power_from_boost_counts(num_unboosted_parts, num_t1_parts, num_t2_parts, num_t3_parts);
    }

    if total_parts > 50 {
        return None;
    }

    Some([total_parts, num_t1_parts, num_t2_parts, num_t3_parts])
}

const fn increment_part_tier_u32(lower_part_num: &mut usize, higher_part_num: &mut usize) {
    *lower_part_num -= 1;
    *higher_part_num += 1;
}


#[cfg(test)]
mod tests {
    use crate::body::body_calculations::{BoostSelectionConfig, BoostTierChoice};

    use super::*;

    const U32_BOOST_CATEGORIES: [BoostCategory; 11] = [
        BoostCategory::HarvestEnergy,
        BoostCategory::HarvestMineral,
        BoostCategory::HarvestDeposit,
        BoostCategory::Dismantle,
        BoostCategory::Attack,
        BoostCategory::RangedAttack,
        BoostCategory::RangedMassAttack,
        BoostCategory::Heal,
        BoostCategory::RangedHeal,
        BoostCategory::Carry,
        BoostCategory::Move,
    ];

    const F32_BOOST_CATEGORIES: [BoostCategory; 4] = [
        BoostCategory::Build,
        BoostCategory::Repair,
        BoostCategory::UpgradeController,
        BoostCategory::Tough,
    ];


    fn u32_target_amount_calc(power_arr: &[u32; 4], num_parts: u32) -> [u32; 4] {
        let unboosted_target_amount = power_arr[0] * num_parts;
        let t1_target_amount = power_arr[1] * num_parts;
        let t2_target_amount = power_arr[2] * num_parts;
        let t3_target_amount = power_arr[3] * num_parts;

        [unboosted_target_amount, t1_target_amount, t2_target_amount, t3_target_amount]
    }

    fn f32_target_amount_calc(power_arr: &[f32; 4], num_parts: f32) -> [f32; 4] {
        let unboosted_target_amount = power_arr[0] * num_parts;
        let t1_target_amount = power_arr[1] * num_parts;
        let t2_target_amount = power_arr[2] * num_parts;
        let t3_target_amount = power_arr[3] * num_parts;

        [unboosted_target_amount, t1_target_amount, t2_target_amount, t3_target_amount]
    }

    fn u32_boost_tier_data(target_amounts_arr: &[u32; 4]) -> [(BoostTierChoice, u32); 7] {
        let [unboosted_target_amount, t1_target_amount, t2_target_amount, t3_target_amount] = target_amounts_arr;

        [
            (BoostTierChoice::NoBoosts, *unboosted_target_amount),
            (BoostTierChoice::T1Only, *t1_target_amount),
            (BoostTierChoice::T2Only, *t2_target_amount),
            (BoostTierChoice::T3Only, *t3_target_amount),
            (BoostTierChoice::UpToT1, *t1_target_amount),
            (BoostTierChoice::UpToT2, *t2_target_amount),
            (BoostTierChoice::UpToT3, *t3_target_amount),
        ]
    }

    fn f32_boost_tier_data(target_amounts_arr: &[f32; 4]) -> [(BoostTierChoice, f32); 7] {
        let [unboosted_target_amount, t1_target_amount, t2_target_amount, t3_target_amount] = target_amounts_arr;

        [
            (BoostTierChoice::NoBoosts, *unboosted_target_amount),
            (BoostTierChoice::T1Only, *t1_target_amount),
            (BoostTierChoice::T2Only, *t2_target_amount),
            (BoostTierChoice::T3Only, *t3_target_amount),
            (BoostTierChoice::UpToT1, *t1_target_amount),
            (BoostTierChoice::UpToT2, *t2_target_amount),
            (BoostTierChoice::UpToT3, *t3_target_amount),
        ]
    }

    #[test]
    fn body_calculation_generic_needed_parts_returns_error_if_needed_parts_greater_than_50_u32_power() {
        for category in U32_BOOST_CATEGORIES {
            let power_arr = u32_parts_power_for_boost_category(&category).unwrap();
            let [unboosted_power, t1_power, t2_power, t3_power] = power_arr;

            // Calculate a target amount that requires a minimum of 51 parts with the maximum boost tier to
            // achieve
            let num_parts_for_power: u32 = 51;

            let target_amounts_arr = u32_target_amount_calc(&power_arr, num_parts_for_power);

            let boost_data = u32_boost_tier_data(&target_amounts_arr);

            for (tier_choice, target_amount) in boost_data {
                for allow_partial_boosts in [true, false] {
                    // Run the parts needed calculation fn
                    let boost_config = BoostSelectionConfig::new(tier_choice, allow_partial_boosts);
                    let params = IterativeCalculationParams::new_u32(target_amount, unboosted_power, t1_power, t2_power, t3_power);
                    let parts_needed_res = generic_get_parts_needed(params, &boost_config);

                    // Verify the output is an error, and the appropriate variant
                    assert!(parts_needed_res.is_err(), "Result is not an error:\nTier Choice: {:?}\nAllow Partial Boosts {:?}\nTarget Amount: {:?}\nResult: {:?}", tier_choice, allow_partial_boosts, target_amount, parts_needed_res);
                    assert!(match parts_needed_res {
                        Err(PartsNeededCalculationError::TooManyNeededParts) => true,
                        _ => false,
                    });
                }
            }
        }
    }

    #[test]
    fn body_calculation_generic_needed_parts_returns_error_if_needed_parts_greater_than_50_f32_power() {
        for category in F32_BOOST_CATEGORIES {
            let power_arr = f32_parts_power_for_boost_category(&category).unwrap();
            let [unboosted_power, t1_power, t2_power, t3_power] = power_arr;

            // Calculate a target amount that requires a minimum of 51 parts with the maximum boost tier to
            // achieve
            let num_parts_for_power: f32 = 51.0;

            let target_amounts_arr = f32_target_amount_calc(&power_arr, num_parts_for_power);

            let boost_data = f32_boost_tier_data(&target_amounts_arr);

            for (tier_choice, target_amount) in boost_data {
                for allow_partial_boosts in [true, false] {
                    // Run the parts needed calculation fn
                    let boost_config = BoostSelectionConfig::new(tier_choice, allow_partial_boosts);
                    let params = IterativeCalculationParams::new_f32(target_amount, unboosted_power, t1_power, t2_power, t3_power);
                    let parts_needed_res = generic_get_parts_needed(params, &boost_config);

                    // Verify the output is an error, and the appropriate variant
                    assert!(parts_needed_res.is_err(), "Result is not an error:\nTier Choice: {:?}\nAllow Partial Boosts {:?}\nTarget Amount: {:?}\nResult: {:?}", tier_choice, allow_partial_boosts, target_amount, parts_needed_res);
                    assert!(match parts_needed_res {
                        Err(PartsNeededCalculationError::TooManyNeededParts) => true,
                        _ => false,
                    });
                }
            }
        }
    }

    #[test]
    fn body_calculation_generic_needed_parts_returns_ok_if_needed_parts_lte_50_u32_power() {
        for category in U32_BOOST_CATEGORIES {
            let power_arr = u32_parts_power_for_boost_category(&category).unwrap();
            let [unboosted_power, t1_power, t2_power, t3_power] = power_arr;

            // Calculate a target amount that requires no more than 50 parts with the maximum boost tier to
            // achieve
            for num_parts_for_power in 1..=50 {
                let expected_needed_parts = num_parts_for_power as usize;

                let target_amounts_arr = u32_target_amount_calc(&power_arr, num_parts_for_power);

                let boost_data = u32_boost_tier_data(&target_amounts_arr);

                for (tier_choice, target_amount) in boost_data {
                    for allow_partial_boosts in [true, false] {
                        // Run the parts needed calculation fn
                        let boost_config = BoostSelectionConfig::new(tier_choice, allow_partial_boosts);
                        let params = IterativeCalculationParams::new_u32(target_amount, unboosted_power, t1_power, t2_power, t3_power);
                        let parts_needed_res = generic_get_parts_needed(params, &boost_config);

                        // Verify the output is not an error
                        assert!(parts_needed_res.is_ok(), "Result is not ok:\nTier Choice: {:?}\nAllow Partial Boosts {:?}\nTarget Amount: {:?}\nResult: {:?}", tier_choice, allow_partial_boosts, target_amount, parts_needed_res);

                        let parts_summary = parts_needed_res.unwrap(); // Safe because we asserted this is ok already

                        // Verify the total number of parts is <= 50
                        assert!(parts_summary.num_parts() <= 50);

                        // Verify that the power from the calculated parts exceeds the target amount
                        let calculated_total_power = unboosted_power * (parts_summary.num_unboosted_parts() as u32) +
                                                      t1_power * (parts_summary.num_t1_parts() as u32) +
                                                      t2_power * (parts_summary.num_t2_parts() as u32) +
                                                      t3_power * (parts_summary.num_t3_parts() as u32);
                        assert!(calculated_total_power >= target_amount, "Calculated total power < Target Amount\nTier Choice: {:?}\nAllow Partial Boosts {:?}\nTarget Amount: {:?}\nSummary: {:?}", tier_choice, allow_partial_boosts, target_amount, parts_summary);

                        // Verify the boost choices are valid
                        match tier_choice {
                            BoostTierChoice::NoBoosts => {
                                // No T1 boosts
                                assert_eq!(0, parts_summary.num_t1_parts());

                                // No T2 boosts
                                assert_eq!(0, parts_summary.num_t2_parts());

                                // No T3 boosts
                                assert_eq!(0, parts_summary.num_t3_parts());

                                // Unboosted parts equals total expected parts
                                let calculated_total_parts = parts_summary.num_unboosted_parts();
                                assert_eq!(expected_needed_parts, calculated_total_parts);
                            },
                            BoostTierChoice::T1Only => {
                                // No unboosted parts
                                assert_eq!(0, parts_summary.num_unboosted_parts());

                                // No T2 boosts
                                assert_eq!(0, parts_summary.num_t2_parts());

                                // No T3 boosts
                                assert_eq!(0, parts_summary.num_t3_parts());

                                // T1 boosted parts equals total expected parts
                                let calculated_total_parts = parts_summary.num_t1_parts();
                                assert_eq!(expected_needed_parts, calculated_total_parts);
                            },
                            BoostTierChoice::T2Only => {
                                // No unboosted parts
                                assert_eq!(0, parts_summary.num_unboosted_parts());

                                // No T1 boosts
                                assert_eq!(0, parts_summary.num_t1_parts());

                                // No T3 boosts
                                assert_eq!(0, parts_summary.num_t3_parts());

                                // T2 boosted parts equals total expected parts
                                let calculated_total_parts = parts_summary.num_t2_parts();
                                assert_eq!(expected_needed_parts, calculated_total_parts);
                            },
                            BoostTierChoice::T3Only => {
                                // No unboosted parts
                                assert_eq!(0, parts_summary.num_unboosted_parts());

                                // No T2 boosts
                                assert_eq!(0, parts_summary.num_t2_parts());

                                // No T1 boosts
                                assert_eq!(0, parts_summary.num_t1_parts());

                                // T3 boosted parts equals total expected parts
                                let calculated_total_parts = parts_summary.num_t3_parts();
                                assert_eq!(expected_needed_parts, calculated_total_parts);
                            },
                            BoostTierChoice::UpToT1 => {
                                // No T2 boosts
                                assert_eq!(0, parts_summary.num_t2_parts());

                                // No T3 boosts
                                assert_eq!(0, parts_summary.num_t3_parts());
                            },
                            BoostTierChoice::UpToT2 => {
                                // No T3 boosts
                                assert_eq!(0, parts_summary.num_t3_parts());
                            },
                            BoostTierChoice::UpToT3 => {
                                // Nothing to check here
                            },
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn body_calculation_generic_needed_parts_returns_ok_if_needed_parts_lte_50_f32_power() {
        for category in F32_BOOST_CATEGORIES {
            let power_arr = f32_parts_power_for_boost_category(&category).unwrap();
            let [unboosted_power, t1_power, t2_power, t3_power] = power_arr;

            // Calculate a target amount that requires no more than 50 parts with the maximum boost tier to
            // achieve
            for num_parts_for_power in 1..=50 {
                let expected_needed_parts = num_parts_for_power as usize;

                let target_amounts_arr = f32_target_amount_calc(&power_arr, num_parts_for_power as f32);

                let boost_data = f32_boost_tier_data(&target_amounts_arr);

                for (tier_choice, target_amount) in boost_data {
                    for allow_partial_boosts in [true, false] {
                        // Run the parts needed calculation fn
                        let boost_config = BoostSelectionConfig::new(tier_choice, allow_partial_boosts);
                        let params = IterativeCalculationParams::new_f32(target_amount, unboosted_power, t1_power, t2_power, t3_power);
                        let parts_needed_res = generic_get_parts_needed(params, &boost_config);

                        // Debugging
                        if parts_needed_res.is_err() {
                            println!("Expected Needed Parts: {:?}", expected_needed_parts);
                            println!("Params: {:?}", params);
                            let unboosted_needed_parts = params.num_parts_needed_unboosted();
                            let t1_needed_parts = params.num_parts_needed_t1();
                            let t2_needed_parts = params.num_parts_needed_t2();
                            let t3_needed_parts = params.num_parts_needed_t3();
                            println!("Unboosted Needed Parts: {:?}", unboosted_needed_parts);
                            println!("T1 Needed Parts: {:?}", t1_needed_parts);
                            println!("T2 Needed Parts: {:?}", t2_needed_parts);
                            println!("T3 Needed Parts: {:?}", t3_needed_parts);
                        }

                        // Verify the output is not an error
                        assert!(parts_needed_res.is_ok(), "Result is not ok:\nTier Choice: {:?}\nAllow Partial Boosts {:?}\nTarget Amount: {:?}\nCategory: {:?}\nResult: {:?}", tier_choice, allow_partial_boosts, target_amount, category, parts_needed_res);

                        let parts_summary = parts_needed_res.unwrap(); // Safe because we asserted this is ok already

                        // Verify the total number of parts is <= 50
                        assert!(parts_summary.num_parts() <= 50);

                        // Verify that the power from the calculated parts exceeds the target amount
                        let calculated_total_power = unboosted_power * (parts_summary.num_unboosted_parts() as f32) +
                                                      t1_power * (parts_summary.num_t1_parts() as f32) +
                                                      t2_power * (parts_summary.num_t2_parts() as f32) +
                                                      t3_power * (parts_summary.num_t3_parts() as f32);
                        assert!(calculated_total_power >= target_amount, "Calculated total power < Target Amount\nTier Choice: {:?}\nAllow Partial Boosts {:?}\nTarget Amount: {:?}\nCalculated power: {:?}\nSummary: {:?}", tier_choice, allow_partial_boosts, target_amount, calculated_total_power, parts_summary);

                        // Verify the boost choices are valid
                        match tier_choice {
                            BoostTierChoice::NoBoosts => {
                                // No T1 boosts
                                assert_eq!(0, parts_summary.num_t1_parts(), "Number of T1 parts > 0\nTier Choice: {:?}\nAllow Partial Boosts {:?}\nTarget Amount: {:?}\nCalculated power: {:?}\nSummary: {:?}", tier_choice, allow_partial_boosts, target_amount, calculated_total_power, parts_summary);

                                // No T2 boosts
                                assert_eq!(0, parts_summary.num_t2_parts(), "Number of T2 parts > 0\nTier Choice: {:?}\nAllow Partial Boosts {:?}\nTarget Amount: {:?}\nCalculated power: {:?}\nSummary: {:?}", tier_choice, allow_partial_boosts, target_amount, calculated_total_power, parts_summary);

                                // No T3 boosts
                                assert_eq!(0, parts_summary.num_t3_parts(), "Number of T3 parts > 0\nTier Choice: {:?}\nAllow Partial Boosts {:?}\nTarget Amount: {:?}\nCalculated power: {:?}\nSummary: {:?}", tier_choice, allow_partial_boosts, target_amount, calculated_total_power, parts_summary);

                                // Unboosted parts equals total expected parts
                                let calculated_total_parts = parts_summary.num_unboosted_parts();
                                assert_eq!(expected_needed_parts, calculated_total_parts, "Calculated total parts != Expected total parts\nTier Choice: {:?}\nAllow Partial Boosts {:?}\nTarget Amount: {:?}\nCalculated power: {:?}\nSummary: {:?}", tier_choice, allow_partial_boosts, target_amount, calculated_total_power, parts_summary);
                            },
                            BoostTierChoice::T1Only => {
                                // No unboosted parts
                                assert_eq!(0, parts_summary.num_unboosted_parts(), "Number of unboosted parts > 0\nTier Choice: {:?}\nAllow Partial Boosts {:?}\nTarget Amount: {:?}\nCalculated power: {:?}\nSummary: {:?}", tier_choice, allow_partial_boosts, target_amount, calculated_total_power, parts_summary);

                                // No T2 boosts
                                assert_eq!(0, parts_summary.num_t2_parts(), "Number of T2 parts > 0\nTier Choice: {:?}\nAllow Partial Boosts {:?}\nTarget Amount: {:?}\nCalculated power: {:?}\nSummary: {:?}", tier_choice, allow_partial_boosts, target_amount, calculated_total_power, parts_summary);

                                // No T3 boosts
                                assert_eq!(0, parts_summary.num_t3_parts(), "Number of T3 parts > 0\nTier Choice: {:?}\nAllow Partial Boosts {:?}\nTarget Amount: {:?}\nCalculated power: {:?}\nSummary: {:?}", tier_choice, allow_partial_boosts, target_amount, calculated_total_power, parts_summary);

                                // T1 boosted parts equals total expected parts
                                let calculated_total_parts = parts_summary.num_t1_parts();
                                assert_eq!(expected_needed_parts, calculated_total_parts, "Calculated total parts != Expected total parts\nTier Choice: {:?}\nAllow Partial Boosts {:?}\nTarget Amount: {:?}\nCalculated power: {:?}\nSummary: {:?}", tier_choice, allow_partial_boosts, target_amount, calculated_total_power, parts_summary);
                            },
                            BoostTierChoice::T2Only => {
                                // No unboosted parts
                                assert_eq!(0, parts_summary.num_unboosted_parts(), "Number of unboosted parts > 0\nTier Choice: {:?}\nAllow Partial Boosts {:?}\nTarget Amount: {:?}\nCalculated power: {:?}\nSummary: {:?}", tier_choice, allow_partial_boosts, target_amount, calculated_total_power, parts_summary);

                                // No T1 boosts
                                assert_eq!(0, parts_summary.num_t1_parts(), "Number of T1 parts > 0\nTier Choice: {:?}\nAllow Partial Boosts {:?}\nTarget Amount: {:?}\nCalculated power: {:?}\nSummary: {:?}", tier_choice, allow_partial_boosts, target_amount, calculated_total_power, parts_summary);

                                // No T3 boosts
                                assert_eq!(0, parts_summary.num_t3_parts(), "Number of T3 parts > 0\nTier Choice: {:?}\nAllow Partial Boosts {:?}\nTarget Amount: {:?}\nCalculated power: {:?}\nSummary: {:?}", tier_choice, allow_partial_boosts, target_amount, calculated_total_power, parts_summary);

                                // T2 boosted parts equals total expected parts
                                let calculated_total_parts = parts_summary.num_t2_parts();
                                assert_eq!(expected_needed_parts, calculated_total_parts, "Calculated total parts != Expected total parts\nTier Choice: {:?}\nAllow Partial Boosts {:?}\nTarget Amount: {:?}\nCalculated power: {:?}\nSummary: {:?}", tier_choice, allow_partial_boosts, target_amount, calculated_total_power, parts_summary);
                            },
                            BoostTierChoice::T3Only => {
                                // No unboosted parts
                                assert_eq!(0, parts_summary.num_unboosted_parts(), "Number of unboosted parts > 0\nTier Choice: {:?}\nAllow Partial Boosts {:?}\nTarget Amount: {:?}\nCalculated power: {:?}\nSummary: {:?}", tier_choice, allow_partial_boosts, target_amount, calculated_total_power, parts_summary);

                                // No T1 boosts
                                assert_eq!(0, parts_summary.num_t1_parts(), "Number of T1 parts > 0\nTier Choice: {:?}\nAllow Partial Boosts {:?}\nTarget Amount: {:?}\nCalculated power: {:?}\nSummary: {:?}", tier_choice, allow_partial_boosts, target_amount, calculated_total_power, parts_summary);

                                // No T2 boosts
                                assert_eq!(0, parts_summary.num_t2_parts(), "Number of T2 parts > 0\nTier Choice: {:?}\nAllow Partial Boosts {:?}\nTarget Amount: {:?}\nCalculated power: {:?}\nSummary: {:?}", tier_choice, allow_partial_boosts, target_amount, calculated_total_power, parts_summary);

                                // T3 boosted parts equals total expected parts
                                let calculated_total_parts = parts_summary.num_t3_parts();
                                assert_eq!(expected_needed_parts, calculated_total_parts, "Calculated total parts != Expected total parts\nTier Choice: {:?}\nAllow Partial Boosts {:?}\nTarget Amount: {:?}\nCalculated power: {:?}\nSummary: {:?}", tier_choice, allow_partial_boosts, target_amount, calculated_total_power, parts_summary);
                            },
                            BoostTierChoice::UpToT1 => {
                                // No T2 boosts
                                assert_eq!(0, parts_summary.num_t2_parts(), "Number of T2 parts > 0\nTier Choice: {:?}\nAllow Partial Boosts {:?}\nTarget Amount: {:?}\nCalculated power: {:?}\nSummary: {:?}", tier_choice, allow_partial_boosts, target_amount, calculated_total_power, parts_summary);

                                // No T3 boosts
                                assert_eq!(0, parts_summary.num_t3_parts(), "Number of T3 parts > 0\nTier Choice: {:?}\nAllow Partial Boosts {:?}\nTarget Amount: {:?}\nCalculated power: {:?}\nSummary: {:?}", tier_choice, allow_partial_boosts, target_amount, calculated_total_power, parts_summary);
                            },
                            BoostTierChoice::UpToT2 => {
                                // No T3 boosts
                                assert_eq!(0, parts_summary.num_t3_parts(), "Number of T3 parts > 0\nTier Choice: {:?}\nAllow Partial Boosts {:?}\nTarget Amount: {:?}\nCalculated power: {:?}\nSummary: {:?}", tier_choice, allow_partial_boosts, target_amount, calculated_total_power, parts_summary);
                            },
                            BoostTierChoice::UpToT3 => {
                                // Nothing to check here
                            },
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn body_calculation_iteratively_calculate_boosted_parts_returns_none_if_needed_parts_gt_50_u32_power() {
        for category in U32_BOOST_CATEGORIES {
            let power_arr = u32_parts_power_for_boost_category(&category).unwrap();
            let [unboosted_power, t1_power, t2_power, t3_power] = power_arr;

            // Calculate a target amount that requires 51 parts with the maximum boost tier to achieve
            let num_parts_for_power = 51;

            let target_amounts_arr = u32_target_amount_calc(&power_arr, num_parts_for_power);
            let boost_data = [
                (0, target_amounts_arr[0]),
                (1, target_amounts_arr[1]),
                (2, target_amounts_arr[2]),
                (3, target_amounts_arr[3]),
            ];

            for (max_boost_tier, target_amount) in boost_data {
                // Run the parts needed calculation fn
                let params = IterativeCalculationParams::new_u32(target_amount, unboosted_power, t1_power, t2_power, t3_power);
                let parts_needed_res = iteratively_calculate_boosted_parts(params, max_boost_tier);

                // Verify the output is None
                assert!(parts_needed_res.is_none(), "Result is not None:\nMax Tier: {:?}\nTarget Amount: {:?}\nCategory: {:?}\nResult: {:?}", max_boost_tier, target_amount, category, parts_needed_res);
            }
        }
    }

    #[test]
    fn body_calculation_iteratively_calculate_boosted_parts_returns_none_if_needed_parts_gt_50_f32_power() {
        for category in F32_BOOST_CATEGORIES {
            let power_arr = f32_parts_power_for_boost_category(&category).unwrap();
            let [unboosted_power, t1_power, t2_power, t3_power] = power_arr;

            // Calculate a target amount that requires 51 parts with the maximum boost tier to achieve
            let num_parts_for_power = 51.0;

            let target_amounts_arr = f32_target_amount_calc(&power_arr, num_parts_for_power);
            let boost_data = [
                (0, target_amounts_arr[0]),
                (1, target_amounts_arr[1]),
                (2, target_amounts_arr[2]),
                (3, target_amounts_arr[3]),
            ];

            for (max_boost_tier, target_amount) in boost_data {
                // Run the parts needed calculation fn
                let params = IterativeCalculationParams::new_f32(target_amount, unboosted_power, t1_power, t2_power, t3_power);
                let parts_needed_res = iteratively_calculate_boosted_parts(params, max_boost_tier);

                // Verify the output is None
                assert!(parts_needed_res.is_none(), "Result is not None:\nMax Tier: {:?}\nTarget Amount: {:?}\nCategory: {:?}\nResult: {:?}", max_boost_tier, target_amount, category, parts_needed_res);
            }
        }
    }

    #[test]
    fn body_calculation_iteratively_calculate_boosted_parts_returns_some_if_needed_parts_lte_50_u32_power() {
        for category in U32_BOOST_CATEGORIES {
            let power_arr = u32_parts_power_for_boost_category(&category).unwrap();
            let [unboosted_power, t1_power, t2_power, t3_power] = power_arr;

            // Calculate a target amount that requires no more than 50 parts with the maximum boost tier to
            // achieve
            for num_parts_for_power in 1..=50 {
                let expected_needed_parts = num_parts_for_power as usize;

                let target_amounts_arr = u32_target_amount_calc(&power_arr, num_parts_for_power);
                let boost_data = [
                    (0, target_amounts_arr[0]),
                    (1, target_amounts_arr[1]),
                    (2, target_amounts_arr[2]),
                    (3, target_amounts_arr[3]),
                ];

                for (max_boost_tier, target_amount) in boost_data {
                    // Run the parts needed calculation fn
                    let params = IterativeCalculationParams::new_u32(target_amount, unboosted_power, t1_power, t2_power, t3_power);
                    let parts_needed_res = iteratively_calculate_boosted_parts(params, max_boost_tier);

                    // Debugging
                    if parts_needed_res.is_none() {
                        println!("Expected Needed Parts: {:?}", expected_needed_parts);
                        println!("Params: {:?}", params);
                        let unboosted_needed_parts = params.num_parts_needed_unboosted();
                        let t1_needed_parts = params.num_parts_needed_t1();
                        let t2_needed_parts = params.num_parts_needed_t2();
                        let t3_needed_parts = params.num_parts_needed_t3();
                        println!("Unboosted Needed Parts: {:?}", unboosted_needed_parts);
                        println!("T1 Needed Parts: {:?}", t1_needed_parts);
                        println!("T2 Needed Parts: {:?}", t2_needed_parts);
                        println!("T3 Needed Parts: {:?}", t3_needed_parts);
                    }

                    // Verify the output is not an error
                    assert!(parts_needed_res.is_some(), "Result is not Some:\nMax Tier: {:?}\nTarget Amount: {:?}\nCategory: {:?}\nResult: {:?}", max_boost_tier, target_amount, category, parts_needed_res);

                    let [total_parts, t1_boosts, t2_boosts, t3_boosts] = parts_needed_res.unwrap(); // Safe because we asserted this is Some already
                    let parts_summary = PartsSummary::unchecked_new(total_parts, t1_boosts, t2_boosts, t3_boosts);

                    // Verify the total number of parts is <= 50
                    assert!(parts_summary.num_parts() <= 50);

                    // Verify that the power from the calculated parts exceeds the target amount
                    let calculated_total_power = unboosted_power * (parts_summary.num_unboosted_parts() as u32) +
                                                  t1_power * (parts_summary.num_t1_parts() as u32) +
                                                  t2_power * (parts_summary.num_t2_parts() as u32) +
                                                  t3_power * (parts_summary.num_t3_parts() as u32);
                    assert!(calculated_total_power >= target_amount, "Calculated total power < Target Amount\nMax Tier: {:?}\nTarget Amount: {:?}\nCategory: {:?}\nCalculated power: {:?}\nSummary: {:?}", max_boost_tier, target_amount, category, calculated_total_power, parts_summary);

                    // Verify the boost choices are valid
                    match max_boost_tier {
                        0 => {
                            // No T1 boosts
                            assert_eq!(0, parts_summary.num_t1_parts(), "Number of T1 parts > 0\nMax Tier: {:?}\nTarget Amount: {:?}\nCategory: {:?}\nCalculated power: {:?}\nSummary: {:?}", max_boost_tier, target_amount, category, calculated_total_power, parts_summary);

                            // No T2 boosts
                            assert_eq!(0, parts_summary.num_t2_parts(), "Number of T2 parts > 0\nMax Tier: {:?}\nTarget Amount: {:?}\nCategory: {:?}\nCalculated power: {:?}\nSummary: {:?}", max_boost_tier, target_amount, category, calculated_total_power, parts_summary);

                            // No T3 boosts
                            assert_eq!(0, parts_summary.num_t3_parts(), "Number of T3 parts > 0\nMax Tier: {:?}\nTarget Amount: {:?}\nCategory: {:?}\nCalculated power: {:?}\nSummary: {:?}", max_boost_tier, target_amount, category, calculated_total_power, parts_summary);

                            // Unboosted parts equals total expected parts
                            let calculated_total_parts = parts_summary.num_unboosted_parts();
                            assert_eq!(expected_needed_parts, calculated_total_parts, "Calculated total parts != Expected total parts\nMax Tier: {:?}\nTarget Amount: {:?}\nCategory: {:?}\nCalculated power: {:?}\nSummary: {:?}", max_boost_tier, target_amount, category, calculated_total_power, parts_summary);
                        },
                        1 => {
                            // No T2 boosts
                            assert_eq!(0, parts_summary.num_t2_parts(), "Number of T2 parts > 0\nMax Tier: {:?}\nTarget Amount: {:?}\nCategory: {:?}\nCalculated power: {:?}\nSummary: {:?}", max_boost_tier, target_amount, category, calculated_total_power, parts_summary);

                            // No T3 boosts
                            assert_eq!(0, parts_summary.num_t3_parts(), "Number of T3 parts > 0\nMax Tier: {:?}\nTarget Amount: {:?}\nCategory: {:?}\nCalculated power: {:?}\nSummary: {:?}", max_boost_tier, target_amount, category, calculated_total_power, parts_summary);

                            // Unboosted + T1 boosted parts equals total expected parts
                            let calculated_total_parts = parts_summary.num_unboosted_parts() + parts_summary.num_t1_parts();
                            assert_eq!(expected_needed_parts, calculated_total_parts, "Calculated total parts != Expected total parts\nMax Tier: {:?}\nTarget Amount: {:?}\nCategory: {:?}\nCalculated power: {:?}\nSummary: {:?}", max_boost_tier, target_amount, category, calculated_total_power, parts_summary);
                        },
                        2 => {
                            // No T3 boosts
                            assert_eq!(0, parts_summary.num_t3_parts(), "Number of T3 parts > 0\nMax Tier: {:?}\nTarget Amount: {:?}\nCategory: {:?}\nCalculated power: {:?}\nSummary: {:?}", max_boost_tier, target_amount, category, calculated_total_power, parts_summary);

                            // Unboosted + T1 + T2 boosted parts equals total expected parts
                            let calculated_total_parts = parts_summary.num_unboosted_parts() + parts_summary.num_t1_parts() + parts_summary.num_t2_parts();
                            
                            if expected_needed_parts != calculated_total_parts {
                                println!("Unboosted Power: {:?}", unboosted_power);
                                println!("T1 Power: {:?}", t1_power);
                                println!("T2 Power: {:?}", t2_power);
                                println!("T3 Power: {:?}", t3_power);
                                println!("Total Unboosted Power: {:?}", unboosted_power * (parts_summary.num_unboosted_parts() as u32));
                                println!("Total T1 Power: {:?}", t1_power * (parts_summary.num_t1_parts() as u32));
                                println!("Total T2 Power: {:?}", t2_power * (parts_summary.num_t2_parts() as u32));
                                println!("Total T3 Power: {:?}", t3_power * (parts_summary.num_t3_parts() as u32));
                            }

                            assert_eq!(expected_needed_parts, calculated_total_parts, "Calculated total parts != Expected total parts\nMax Tier: {:?}\nTarget Amount: {:?}\nCategory: {:?}\nCalculated power: {:?}\nSummary: {:?}", max_boost_tier, target_amount, category, calculated_total_power, parts_summary);
                        },
                        3 => {
                            // Unboosted + T1 + T2 + T3 boosted parts equals total expected parts
                            let calculated_total_parts = parts_summary.num_unboosted_parts() + parts_summary.num_t1_parts() + parts_summary.num_t2_parts() + parts_summary.num_t3_parts();
                            assert_eq!(expected_needed_parts, calculated_total_parts, "Calculated total parts != Expected total parts\nMax Tier: {:?}\nTarget Amount: {:?}\nCategory: {:?}\nCalculated power: {:?}\nSummary: {:?}", max_boost_tier, target_amount, category, calculated_total_power, parts_summary);
                        },
                        _ => {} // Should never get here
                    }
                }
            }
        }
    }

    #[test]
    fn body_calculation_iteratively_calculate_boosted_parts_returns_some_if_needed_parts_lte_50_f32_power() {
        for category in F32_BOOST_CATEGORIES {
            let power_arr = f32_parts_power_for_boost_category(&category).unwrap();
            let [unboosted_power, t1_power, t2_power, t3_power] = power_arr;

            // Calculate a target amount that requires no more than 50 parts with the maximum boost tier to
            // achieve
            for num_parts_for_power in 1..=50 {
                let expected_needed_parts = num_parts_for_power as usize;

                let target_amounts_arr = f32_target_amount_calc(&power_arr, num_parts_for_power as f32);
                let boost_data = [
                    (0, target_amounts_arr[0]),
                    (1, target_amounts_arr[1]),
                    (2, target_amounts_arr[2]),
                    (3, target_amounts_arr[3]),
                ];

                for (max_boost_tier, target_amount) in boost_data {
                    // Run the parts needed calculation fn
                    let params = IterativeCalculationParams::new_f32(target_amount, unboosted_power, t1_power, t2_power, t3_power);
                    let parts_needed_res = iteratively_calculate_boosted_parts(params, max_boost_tier);

                    // Debugging
                    if parts_needed_res.is_none() {
                        println!("Expected Needed Parts: {:?}", expected_needed_parts);
                        println!("Params: {:?}", params);
                        let unboosted_needed_parts = params.num_parts_needed_unboosted();
                        let t1_needed_parts = params.num_parts_needed_t1();
                        let t2_needed_parts = params.num_parts_needed_t2();
                        let t3_needed_parts = params.num_parts_needed_t3();
                        println!("Unboosted Needed Parts: {:?}", unboosted_needed_parts);
                        println!("T1 Needed Parts: {:?}", t1_needed_parts);
                        println!("T2 Needed Parts: {:?}", t2_needed_parts);
                        println!("T3 Needed Parts: {:?}", t3_needed_parts);
                    }

                    // Verify the output is not an error
                    assert!(parts_needed_res.is_some(), "Result is not Some:\nMax Tier: {:?}\nTarget Amount: {:?}\nCategory: {:?}\nResult: {:?}", max_boost_tier, target_amount, category, parts_needed_res);

                    let [total_parts, t1_boosts, t2_boosts, t3_boosts] = parts_needed_res.unwrap(); // Safe because we asserted this is Some already
                    let parts_summary = PartsSummary::unchecked_new(total_parts, t1_boosts, t2_boosts, t3_boosts);

                    // Verify the total number of parts is <= 50
                    assert!(parts_summary.num_parts() <= 50);

                    // Verify that the power from the calculated parts exceeds the target amount
                    let calculated_total_power = unboosted_power * (parts_summary.num_unboosted_parts() as f32) +
                                                  t1_power * (parts_summary.num_t1_parts() as f32) +
                                                  t2_power * (parts_summary.num_t2_parts() as f32) +
                                                  t3_power * (parts_summary.num_t3_parts() as f32);
                    assert!(calculated_total_power >= target_amount, "Calculated total power < Target Amount\nMax Tier: {:?}\nTarget Amount: {:?}\nCategory: {:?}\nCalculated power: {:?}\nSummary: {:?}", max_boost_tier, target_amount, category, calculated_total_power, parts_summary);

                    // Verify the boost choices are valid
                    match max_boost_tier {
                        0 => {
                            // No T1 boosts
                            assert_eq!(0, parts_summary.num_t1_parts(), "Number of T1 parts > 0\nMax Tier: {:?}\nTarget Amount: {:?}\nCategory: {:?}\nCalculated power: {:?}\nSummary: {:?}", max_boost_tier, target_amount, category, calculated_total_power, parts_summary);

                            // No T2 boosts
                            assert_eq!(0, parts_summary.num_t2_parts(), "Number of T2 parts > 0\nMax Tier: {:?}\nTarget Amount: {:?}\nCategory: {:?}\nCalculated power: {:?}\nSummary: {:?}", max_boost_tier, target_amount, category, calculated_total_power, parts_summary);

                            // No T3 boosts
                            assert_eq!(0, parts_summary.num_t3_parts(), "Number of T3 parts > 0\nMax Tier: {:?}\nTarget Amount: {:?}\nCategory: {:?}\nCalculated power: {:?}\nSummary: {:?}", max_boost_tier, target_amount, category, calculated_total_power, parts_summary);

                            // Unboosted parts equals total expected parts
                            let calculated_total_parts = parts_summary.num_unboosted_parts();
                            assert_eq!(expected_needed_parts, calculated_total_parts, "Calculated total parts != Expected total parts\nMax Tier: {:?}\nTarget Amount: {:?}\nCategory: {:?}\nCalculated power: {:?}\nSummary: {:?}", max_boost_tier, target_amount, category, calculated_total_power, parts_summary);
                        },
                        1 => {
                            // No T2 boosts
                            assert_eq!(0, parts_summary.num_t2_parts(), "Number of T2 parts > 0\nMax Tier: {:?}\nTarget Amount: {:?}\nCategory: {:?}\nCalculated power: {:?}\nSummary: {:?}", max_boost_tier, target_amount, category, calculated_total_power, parts_summary);

                            // No T3 boosts
                            assert_eq!(0, parts_summary.num_t3_parts(), "Number of T3 parts > 0\nMax Tier: {:?}\nTarget Amount: {:?}\nCategory: {:?}\nCalculated power: {:?}\nSummary: {:?}", max_boost_tier, target_amount, category, calculated_total_power, parts_summary);

                            // Unboosted + T1 boosted parts equals total expected parts
                            let calculated_total_parts = parts_summary.num_unboosted_parts() + parts_summary.num_t1_parts();
                            assert_eq!(expected_needed_parts, calculated_total_parts, "Calculated total parts != Expected total parts\nMax Tier: {:?}\nTarget Amount: {:?}\nCategory: {:?}\nCalculated power: {:?}\nSummary: {:?}", max_boost_tier, target_amount, category, calculated_total_power, parts_summary);
                        },
                        2 => {
                            // No T3 boosts
                            assert_eq!(0, parts_summary.num_t3_parts(), "Number of T3 parts > 0\nMax Tier: {:?}\nTarget Amount: {:?}\nCategory: {:?}\nCalculated power: {:?}\nSummary: {:?}", max_boost_tier, target_amount, category, calculated_total_power, parts_summary);

                            // Unboosted + T1 + T2 boosted parts equals total expected parts
                            let calculated_total_parts = parts_summary.num_unboosted_parts() + parts_summary.num_t1_parts() + parts_summary.num_t2_parts();
                            
                            if expected_needed_parts != calculated_total_parts {
                                println!("Unboosted Power: {:?}", unboosted_power);
                                println!("T1 Power: {:?}", t1_power);
                                println!("T2 Power: {:?}", t2_power);
                                println!("T3 Power: {:?}", t3_power);
                                println!("Total Unboosted Power: {:?}", unboosted_power * (parts_summary.num_unboosted_parts() as f32));
                                println!("Total T1 Power: {:?}", t1_power * (parts_summary.num_t1_parts() as f32));
                                println!("Total T2 Power: {:?}", t2_power * (parts_summary.num_t2_parts() as f32));
                                println!("Total T3 Power: {:?}", t3_power * (parts_summary.num_t3_parts() as f32));
                            }

                            assert_eq!(expected_needed_parts, calculated_total_parts, "Calculated total parts != Expected total parts\nMax Tier: {:?}\nTarget Amount: {:?}\nCategory: {:?}\nCalculated power: {:?}\nSummary: {:?}", max_boost_tier, target_amount, category, calculated_total_power, parts_summary);
                        },
                        3 => {
                            // Unboosted + T1 + T2 + T3 boosted parts equals total expected parts
                            let calculated_total_parts = parts_summary.num_unboosted_parts() + parts_summary.num_t1_parts() + parts_summary.num_t2_parts() + parts_summary.num_t3_parts();
                            assert_eq!(expected_needed_parts, calculated_total_parts, "Calculated total parts != Expected total parts\nMax Tier: {:?}\nTarget Amount: {:?}\nCategory: {:?}\nCalculated power: {:?}\nSummary: {:?}", max_boost_tier, target_amount, category, calculated_total_power, parts_summary);
                        },
                        _ => {} // Should never get here
                    }
                }
            }
        }
    }

    #[test]
    fn num_parts_needed_u32_calculates_correctly() {
        for category in U32_BOOST_CATEGORIES {
            let power_arr = u32_parts_power_for_boost_category(&category).unwrap();
            for num_parts in 1..=50 {
                for power in power_arr {
                    let amount = num_parts as u32 * power;
                    let calc = num_parts_needed_u32(amount, power);
                    let calc_amount = calc as u32 * power;
                    assert_eq!(num_parts, calc, "Expected != Calculated\nPower: {:?}\nAmount: {:?}\nCalculated Amount: {:?}\nCategory: {:?}", power, amount, calc_amount, category);
                }
            }
        }
    }

    #[test]
    fn num_parts_needed_f32_calculates_correctly() {
        for category in F32_BOOST_CATEGORIES {
            let power_arr = f32_parts_power_for_boost_category(&category).unwrap();
            for num_parts in 1..=50 {
                for power in power_arr {
                    let amount = num_parts as f32 * power;
                    let calc = num_parts_needed_f32(amount, power);
                    let direct_div = amount / power;
                    let calc_amount = calc as f32 * power;
                    let diff_amount = calc_amount - amount;
                    assert_eq!(num_parts, calc, "Expected != Calculated\nPower: {:?}\nAmount: {:?}\nCalculated Amount: {:?}\nDiff Amount: {:?}\nDirect Division: {:?}\nCategory: {:?}", power, amount, calc_amount, diff_amount, direct_div, category);
                }
            }
        }
    }
}
