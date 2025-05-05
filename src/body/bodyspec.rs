//! Implements BodySpec as a way to perform calculations on a particular Creep body layout.

use screeps::constants::{
    Part,
    ATTACK_POWER,
    BUILD_POWER,
    CARRY_CAPACITY,
    CREEP_SPAWN_TIME,
    DISMANTLE_POWER,
    HARVEST_DEPOSIT_POWER,
    HARVEST_MINERAL_POWER,
    HARVEST_POWER,
    HEAL_POWER,
    MAX_CREEP_SIZE,
    RANGED_ATTACK_POWER,
    RANGED_HEAL_POWER,
    REPAIR_POWER,
    UPGRADE_CONTROLLER_POWER,
};
use screeps::constants::extra::{
    CREEP_HITS_PER_PART,
    MOVE_COST_SWAMP,
    MOVE_COST_PLAIN,
    MOVE_COST_ROAD,
    MOVE_POWER,
};
use screeps::constants::Boost;
use screeps::objects::output::BodyPart;
use crate::boost::boost::AbstractBoost;

const fn ranged_mass_attack_power_at_distance(distance: u8) -> u32 {
    match distance {
        1 => 10,
        2 => 4,
        3 => 1,
        _ => 0,
    }
}

/// Represents a part on a creep body.
///
/// # Examples
///
/// ```rust
/// use screeps::{Part, HARVEST_POWER};
/// use screeps_body_utils::body::PartSpec;
/// use screeps_body_utils::boost::AbstractBoost;
///
/// let unboosted_part = PartSpec::new_unboosted_part(Part::Work);
/// let t2_part = PartSpec::new_boosted_part(Part::Work, AbstractBoost::T2Harvest);
///
/// assert_eq!(HARVEST_POWER, unboosted_part.get_harvest_energy_amount());
/// assert_eq!(HARVEST_POWER * 5, t2_part.get_harvest_energy_amount());
/// ```
#[derive(Debug, PartialEq, Hash, Copy, Clone)]
pub struct PartSpec {
  pub part: Part,
  pub hits: u32,
  pub boost: Option<AbstractBoost>,
}

impl PartSpec {
    /// Create a new PartSpec from basic information.
    ///
    /// This provides the most customizability for detailed calculations, but for basic creep body
    /// sizing, you'll want to use [new_unboosted_part](PartSpec::new_unboosted_part) or [new_boosted_part](PartSpec::new_boosted_part).
    pub const fn new(part: Part, hits: u32, boost: Option<AbstractBoost>) -> Self {
        Self {
            part,
            hits,
            boost,
        }
    }

    /// Create a new PartSpec with full hits and no boost.
    pub const fn new_unboosted_part(part: Part) -> Self {
        Self::new(part, CREEP_HITS_PER_PART, None)
    }

    /// Create a new PartSpec with full hits and the specified boost.
    pub const fn new_boosted_part(part: Part, boost: AbstractBoost) -> Self {
        Self::new(part, CREEP_HITS_PER_PART, Some(boost))
    }

    /// Whether this part would be considered active (hits > 0).
    pub const fn is_active(&self) -> bool {
        self.hits > 0
    }

    /// Calculates how much fatigue this part reduces per tick.
    /// 
    /// Unlike most other methods, this *does* take into account whether the part has enough hits
    /// to be considered active.
    pub const fn get_fatigue_reduction(&self) -> u32 {
        match self.part {
            // Only Move parts contribute to fatigue reduction
            Part::Move => {
                if self.is_active() {
                    let boost_multiplier = match self.boost {
                        None => 1,
                        Some(abstract_boost) => match AbstractBoost::const_to_boost(&abstract_boost) {
                            Boost::Move(val) => val,
                            _ => 1,
                        },
                    };
                    boost_multiplier * MOVE_POWER
                } else {
                    // Inactive Move parts do not contribute to fatigue reduction
                    0
                }
            },
            _ => 0,
        }
    }

    /// Calculates how much fatigue this part generates per tick when moving onto a tile with the
    /// specified cost.
    ///
    /// For Carry parts, this assumes that the part is not empty, as long as its hits > 0. Carry
    /// parts with hits = 0 do not contribute to fatigue since they are not carrying anything. This
    /// has been verified in the engine code.
    pub const fn get_fatigue_generation(&self, tile_cost: u32) -> u32 {
        match self.part {
            // The docs phrase things so that Move parts should be contributing to fatigue as well
            // as reducing it, but they don't if you look at the actual engine code.
            //
            // Ref: https://github.com/screeps/engine/blob/master/src/processor/intents/movement.js#L237
            Part::Move => 0,
            Part::Carry => {
                // Carry parts only contribute to fatigue generation if they're carrying something.
                // But an inactive Carry part cannot be carrying anything by definition, and thus
                // it can't contribute to fatigue generation.
                if self.hits > 0 {
                    tile_cost
                } else {
                    0
                }
            },
            _ => tile_cost,
        }
    }

    /// Calculates how much energy this part can harvest from a [Source](screeps::Source) when
    /// using [harvest](screeps::Creep::harvest).
    /// 
    /// Note: This does not take into account whether the part has enough hits to be
    /// active. Filtering for inactive parts must be done by the caller.
    pub const fn get_harvest_energy_amount(&self) -> u32 {
        match self.part {
            Part::Work => {
                let boost_multiplier = match self.boost {
                    None => 1,
                    Some(abstract_boost) => match AbstractBoost::const_to_boost(&abstract_boost) {
                        Boost::Harvest(val) => val,
                        _ => 1,
                    },
                };
                boost_multiplier * HARVEST_POWER
            },
            _ => 0,
        }
    }

    /// Calculates how many resources this part can harvest from a [Mineral](screeps::Mineral) when
    /// using [harvest](screeps::Creep::harvest).
    /// 
    /// Note: This does not take into account whether the part has enough hits to be
    /// active. Filtering for inactive parts must be done by the caller.
    pub const fn get_harvest_mineral_amount(&self) -> u32 {
        match self.part {
            Part::Work => {
                let boost_multiplier = match self.boost {
                    None => 1,
                    Some(abstract_boost) => match AbstractBoost::const_to_boost(&abstract_boost) {
                        Boost::Harvest(val) => val,
                        _ => 1,
                    },
                };
                boost_multiplier * HARVEST_MINERAL_POWER
            },
            _ => 0,
        }
    }

    /// Calculates how many resources this part can harvest from a [Deposit](screeps::Deposit) when
    /// using [harvest](screeps::Creep::harvest).
    /// 
    /// Note: This does not take into account whether the part has enough hits to be
    /// active. Filtering for inactive parts must be done by the caller.
    pub const fn get_harvest_deposit_amount(&self) -> u32 {
        match self.part {
            Part::Work => {
                let boost_multiplier = match self.boost {
                    None => 1,
                    Some(abstract_boost) => match AbstractBoost::const_to_boost(&abstract_boost) {
                        Boost::Harvest(val) => val,
                        _ => 1,
                    },
                };
                boost_multiplier * HARVEST_DEPOSIT_POWER
            },
            _ => 0,
        }
    }

    /// Calculates how many resources this part can store.
    /// 
    /// Note: This does not take into account whether the part has enough hits to be
    /// active. Filtering for inactive parts must be done by the caller.
    pub const fn get_carry_capacity(&self) -> u32 {
        match self.part {
            Part::Carry => {
                let boost_multiplier = match self.boost {
                    None => 1,
                    Some(abstract_boost) => match AbstractBoost::const_to_boost(&abstract_boost) {
                        Boost::Carry(val) => val,
                        _ => 1,
                    },
                };
                boost_multiplier * CARRY_CAPACITY
            },
            _ => 0,
        }
    }

    /// Calculates how much damage this part can deal when using [attack](screeps::Creep::attack).
    /// 
    /// Note: This does not take into account whether the part has enough hits to be
    /// active. Filtering for inactive parts must be done by the caller.
    pub const fn get_attack_damage(&self) -> u32 {
        match self.part {
            Part::Attack => {
                let boost_multiplier = match self.boost {
                    None => 1,
                    Some(abstract_boost) => match AbstractBoost::const_to_boost(&abstract_boost) {
                        Boost::Attack(val) => val,
                        _ => 1,
                    },
                };
                boost_multiplier * ATTACK_POWER
            },
            _ => 0,
        }
    }

    /// Calculates how much damage this part can deal when using [ranged_attack](screeps::Creep::ranged_attack).
    /// 
    /// Note: This does not take into account whether the part has enough hits to be
    /// active. Filtering for inactive parts must be done by the caller.
    pub const fn get_ranged_attack_damage(&self) -> u32 {
        match self.part {
            Part::RangedAttack => {
                let boost_multiplier = match self.boost {
                    None => 1,
                    Some(abstract_boost) => match AbstractBoost::const_to_boost(&abstract_boost) {
                        Boost::RangedAttack(val) => val,
                        _ => 1,
                    },
                };
                boost_multiplier * RANGED_ATTACK_POWER
            },
            _ => 0,
        }
    }

    /// Calculates how much damage this part can deal to a single target at the specified
    /// distance when using [ranged_mass_attack](screeps::Creep::ranged_mass_attack).
    /// 
    /// Note: This does not take into account whether the part has enough hits to be
    /// active. Filtering for inactive parts must be done by the caller.
    pub const fn get_ranged_mass_attack_damage_at_distance_single_target(&self, distance: u8) -> u32 {
        match self.part {
            Part::RangedAttack => {
                let boost_multiplier = match self.boost {
                    None => 1,
                    Some(abstract_boost) => match AbstractBoost::const_to_boost(&abstract_boost) {
                        Boost::RangedAttack(val) => val,
                        _ => 1,
                    },
                };
                boost_multiplier * ranged_mass_attack_power_at_distance(distance)
            },
            _ => 0,
        }
    }

    /// Calculates how much damage this part can restore when using [heal](screeps::Creep::heal) on a creep with
    /// a distance of 1.
    /// 
    /// Note: This does not take into account whether the part has enough hits to be
    /// active. Filtering for inactive parts must be done by the caller.
    pub const fn get_heal_amount(&self) -> u32 {
        match self.part {
            Part::Heal => {
                let boost_multiplier = match self.boost {
                    None => 1,
                    Some(abstract_boost) => match AbstractBoost::const_to_boost(&abstract_boost) {
                        Boost::Heal(val) => val,
                        _ => 1,
                    },
                };
                boost_multiplier * HEAL_POWER
            },
            _ => 0,
        }
    }

    /// Calculates how much damage this part can restore when using [heal](screeps::Creep::heal) on a creep with
    /// a distance > 1.
    /// 
    /// Note: This does not take into account whether the part has enough hits to be
    /// active. Filtering for inactive parts must be done by the caller.
    pub const fn get_ranged_heal_amount(&self) -> u32 {
        match self.part {
            Part::Heal => {
                let boost_multiplier = match self.boost {
                    None => 1,
                    Some(abstract_boost) => match AbstractBoost::const_to_boost(&abstract_boost) {
                        Boost::Heal(val) => val,
                        _ => 1,
                    },
                };
                boost_multiplier * RANGED_HEAL_POWER
            },
            _ => 0,
        }
    }

    /// Calculates how much progress this part can add to the controller when
    /// using [upgrade_controller](screeps::Creep::upgrade_controller).
    /// 
    /// Note: This does not take into account whether the part has enough hits to be
    /// active. Filtering for inactive parts must be done by the caller.
    pub const fn get_upgrade_controller_amount(&self) -> f32 {
        match self.part {
            Part::Work => {
                let boost_multiplier = match self.boost {
                    None => 1.0,
                    Some(abstract_boost) => match AbstractBoost::const_to_boost(&abstract_boost) {
                        Boost::UpgradeController(val) => val,
                        _ => 1.0,
                    },
                };
                boost_multiplier * (UPGRADE_CONTROLLER_POWER as f32)
            },
            _ => 0.0,
        }
    }

    /// Calculates how much damage this part can restore when using [repair](screeps::Creep::repair).
    /// 
    /// Note: This does not take into account whether the part has enough hits to be
    /// active. Filtering for inactive parts must be done by the caller.
    pub const fn get_repair_amount(&self) -> f32 {
        match self.part {
            Part::Work => {
                let boost_multiplier = match self.boost {
                    None => 1.0,
                    Some(abstract_boost) => match AbstractBoost::const_to_boost(&abstract_boost) {
                        Boost::BuildAndRepair(val) => val,
                        _ => 1.0,
                    },
                };
                boost_multiplier * (REPAIR_POWER as f32)
            },
            _ => 0.0,
        }
    }

    /// Calculates how much build progress this part can add to a structure when
    /// using [build](screeps::Creep::build).
    /// 
    /// Note: This does not take into account whether the part has enough hits to be
    /// active. Filtering for inactive parts must be done by the caller.
    pub const fn get_build_amount(&self) -> f32 {
        match self.part {
            Part::Work => {
                let boost_multiplier = match self.boost {
                    None => 1.0,
                    Some(abstract_boost) => match AbstractBoost::const_to_boost(&abstract_boost) {
                        Boost::BuildAndRepair(val) => val,
                        _ => 1.0,
                    },
                };
                boost_multiplier * (BUILD_POWER as f32)
            },
            _ => 0.0,
        }
    }

    /// Calculates how much damage this part can deal when using [dismantle](screeps::Creep::dismantle).
    /// 
    /// Note: This does not take into account whether the part has enough hits to be
    /// active. Filtering for inactive parts must be done by the caller.
    pub const fn get_dismantle_damage(&self) -> u32 {
        match self.part {
            Part::Work => {
                let boost_multiplier = match self.boost {
                    None => 1,
                    Some(abstract_boost) => match AbstractBoost::const_to_boost(&abstract_boost) {
                        Boost::Dismantle(val) => val,
                        _ => 1,
                    },
                };
                boost_multiplier * DISMANTLE_POWER
            },
            _ => 0,
        }
    }

    /// Calculates how much damage this part can currently take; i.e. its effective hits.
    ///
    /// This takes into account Tough boosts, if this is a Tough part. Otherwise, this is equal to
    /// the current hits for the part.
    ///
    /// Note: This matches the `bodyPartHitsEffective` variable from the damage calculations in the
    /// [engine
    /// code](https://github.com/screeps/engine/blob/master/src/processor/intents/creeps/tick.js#L20).
    ///
    /// This is a f32 because if you're doing damage calculations like the engine does, fractional
    /// damage overflow matters up until the creep is dead, and doesn't get clamped to an integer
    /// except at the very end of the damage calculations for the creep as a whole.
    pub const fn get_damage_capacity(&self) -> f32 {
        match self.part {
            Part::Tough => {
                let boost_multiplier = match self.boost {
                    None => 1.0,
                    Some(abstract_boost) => match AbstractBoost::const_to_boost(&abstract_boost) {
                        Boost::Tough(val) => val,
                        _ => 1.0,
                    },
                };
                // The value of the Tough boost is the multiplier for the damage:
                // T1: 0.7
                // T2: 0.5
                // T3: 0.3
                // damage taken = raw damage * boost multiplier
                // The damage capacity of a tough part is when the damage taken value
                // is equal to the current hits of the tough part; i.e. the raw damage
                // needed to bring the part down to 0 hits. This can be calculated as:
                // capacity = hits / multiplier
                (self.hits as f32) / boost_multiplier
            },
            _ => self.hits as f32,
        }
    } 

}

impl From<Part> for PartSpec {
    fn from(val: Part) -> Self {
        Self::new(val, CREEP_HITS_PER_PART, None)
    }
}

impl From<BodyPart> for PartSpec {
    fn from(val: BodyPart) -> Self {
        let boost_opt: Option<AbstractBoost> = val.boost().and_then(|r| AbstractBoost::try_from(r).ok());
        Self::new(val.part(), val.hits(), boost_opt)
    }
}


/*
- BodySpec implementation -- stores a body specification and calculates based on that
  - compounds needed for boosts
*/

fn filter_is_work_part(p: &&PartSpec) -> bool {
    (p.part == Part::Work) & (p.hits > 0)
}

fn filter_is_attack_part(p: &&PartSpec) -> bool {
    (p.part == Part::Attack) & (p.hits > 0)
}

fn filter_is_ranged_attack_part(p: &&PartSpec) -> bool {
    (p.part == Part::RangedAttack) & (p.hits > 0)
}

fn filter_is_heal_part(p: &&PartSpec) -> bool {
    (p.part == Part::Heal) & (p.hits > 0)
}

fn filter_is_carry_part(p: &&PartSpec) -> bool {
    (p.part == Part::Carry) & (p.hits > 0)
}

fn filter_is_move_part(p: &&PartSpec) -> bool {
    (p.part == Part::Move) & (p.hits > 0)
}

fn filter_is_tough_part(p: &&PartSpec) -> bool {
    (p.part == Part::Tough) & (p.hits > 0)
}

fn filter_is_claim_part(p: &&PartSpec) -> bool {
    (p.part == Part::Claim) & (p.hits > 0)
}

/// Errors that can occur while validating a BodySpec.
pub enum BodySpecValidationError {
    /// Creeps are only allowed a maximum of [MAX_CREEP_SIZE](screeps::constants::MAX_CREEP_SIZE) parts.
    TooManyParts,
}

/// Represents a creep body.
///
/// # Examples
///
/// ```rust
/// use screeps::{Part, HARVEST_POWER};
/// use screeps_body_utils::body::{BodySpec, PartSpec};
/// use screeps_body_utils::boost::AbstractBoost;
///
/// // Set up some basic PartSpecs
/// let work = PartSpec::new_unboosted_part(Part::Work);
/// let unboosted_move = PartSpec::new_unboosted_part(Part::Move);
/// let t2_move = PartSpec::new_boosted_part(Part::Move, AbstractBoost::T2Move);
///
/// let work_parts = vec![work; 5];
/// let unboosted_move_parts = vec![unboosted_move; 5];
/// let t2_move_parts = vec![t2_move; 2];
///
/// // Create some body layouts
/// let mut unboosted_body = Vec::new();
/// unboosted_body.extend(work_parts.clone());
/// unboosted_body.extend(unboosted_move_parts);
/// let mut t2_body = Vec::new();
/// t2_body.extend(work_parts.clone());
/// t2_body.extend(t2_move_parts);
///
/// // Create actual BodySpecs for calculations
/// let unboosted_bodyspec = BodySpec::new(&unboosted_body);
/// let t2_bodyspec = BodySpec::new(&t2_body);
///
/// // Verify that the T2 Move boosts don't affect the amount of energy harvested by the body
/// assert_eq!(HARVEST_POWER * 5, unboosted_bodyspec.harvest_energy_amount());
/// assert_eq!(HARVEST_POWER * 5, t2_bodyspec.harvest_energy_amount());
///
/// // Verify that even though the T2 body has less move parts, it can still move offroad without
/// // incurring any net fatigue each tick
/// assert_eq!(0, unboosted_bodyspec.plains_move_net_exhaustion());
/// assert_eq!(0, t2_bodyspec.plains_move_net_exhaustion());
///
/// // Verify that the net fatigue generation on swamp tiles is lower with the T2 Move parts
/// assert_eq!(40, unboosted_bodyspec.swamp_move_net_exhaustion());
/// assert_eq!(38, t2_bodyspec.swamp_move_net_exhaustion());
/// ```
#[derive(Debug, PartialEq, Hash, Clone)]
pub struct BodySpec {
  body: Vec<PartSpec>,
}

impl BodySpec {
    /// Creates a new, unvalidated BodySpec object.
    ///
    /// If you're wanting to use this to build an actual Creep, you'll likely want to validate it
    /// first. You should use [validated_new](BodySpec::validated_new) in that case.
    ///
    /// However, if you just want to do arbitrary calculations to see what sort of Creep would be
    /// needed to do something, an unvalidated BodySpec object could be useful. This method is
    /// there for the latter case, or for if you've already done validation yourself and just want
    /// the BodySpec to work with.
    ///
    /// ```rust
    /// use screeps::{Part, HARVEST_POWER};
    /// use screeps_body_utils::body::{BodySpec, PartSpec};
    ///
    /// let work = PartSpec::new_unboosted_part(Part::Work);
    /// let unboosted_move = PartSpec::new_unboosted_part(Part::Move);
    ///
    /// let work_parts = vec![work; 5];
    /// let unboosted_move_parts = vec![unboosted_move; 5];
    ///
    /// let mut unboosted_body = Vec::new();
    /// unboosted_body.extend(work_parts);
    /// unboosted_body.extend(unboosted_move_parts);
    ///
    /// let unboosted_bodyspec = BodySpec::new(&unboosted_body);
    ///
    /// assert_eq!(HARVEST_POWER * 5, unboosted_bodyspec.harvest_energy_amount());
    /// ```
    pub fn new(body: &[PartSpec]) -> Self {
        Self {
            body: body.iter().copied().collect(),
        }
    }

    /// Does basic validation of a provided body spec before creating a BodySpec object.
    ///
    /// ```rust
    /// use screeps::Part;
    /// use screeps_body_utils::body::{BodySpec, PartSpec};
    /// 
    /// let m = PartSpec::new_unboosted_part(Part::Move);
    /// let body = vec!(m);
    /// let res = BodySpec::validated_new(&body);
    /// assert!(res.is_ok());
    ///
    /// let oversized_body = vec![m; 51];
    /// let res = BodySpec::validated_new(&oversized_body);
    /// assert!(res.is_err());
    /// ```
    pub fn validated_new(body: &[PartSpec]) -> Result<Self, BodySpecValidationError> {
        // Unwrap is safe here because this u32 is actually the constant 50
        if body.len() > MAX_CREEP_SIZE.try_into().unwrap() {
            Err(BodySpecValidationError::TooManyParts)
        } else {
            Ok(BodySpec::new(body))
        }
    }

    /// Returns the parts that make up this creep body.
    ///
    /// ```rust
    /// use screeps::Part;
    /// use screeps_body_utils::body::{BodySpec, PartSpec};
    /// 
    /// let m = PartSpec::new_unboosted_part(Part::Move);
    /// let w = PartSpec::new_unboosted_part(Part::Work);
    /// let body = vec!(m, w);
    /// let bodyspec = BodySpec::new(&body);
    /// let parts = bodyspec.get_parts();
    /// assert_eq!(Part::Move, parts[0]);
    /// assert_eq!(Part::Work, parts[1]);
    /// ```
    pub fn get_parts(&self) -> Vec<Part> {
        let mut v = Vec::with_capacity(self.body.len());
        v.extend(self.body.iter().map(|p| p.part));
        v
    }

    /// Calculates the current hits that a creep has.
    ///
    /// ```rust
    /// use screeps::Part;
    /// use screeps::constants::extra::CREEP_HITS_PER_PART;
    /// use screeps_body_utils::body::{BodySpec, PartSpec};
    /// 
    /// let m = PartSpec::new_unboosted_part(Part::Move);
    /// let w = PartSpec::new_unboosted_part(Part::Work);
    /// let body = vec!(m, w);
    /// let bodyspec = BodySpec::new(&body);
    /// assert_eq!(CREEP_HITS_PER_PART * 2, bodyspec.hits());
    /// ```
    pub fn hits(&self) -> u32 {
        self.body.iter().fold(0, |acc, p| acc + p.hits)
    }

    /// Calculates the effective damage that a creep can sustain.
    ///
    /// ```rust
    /// use screeps::Part;
    /// use screeps::constants::extra::CREEP_HITS_PER_PART;
    /// use screeps_body_utils::body::{BodySpec, PartSpec};
    /// use screeps_body_utils::boost::AbstractBoost;
    /// 
    /// // Setup a boosted-Tough body
    /// let m = PartSpec::new_unboosted_part(Part::Move);
    /// let t = PartSpec::new_boosted_part(Part::Tough, AbstractBoost::T3Tough);
    /// let body = vec!(m, t);
    /// let bodyspec = BodySpec::new(&body);
    /// assert_eq!(433, bodyspec.effective_hits());
    /// ```
    pub fn effective_hits(&self) -> u32 {
        self.body.iter().fold(0.0, |acc, p| acc + p.get_damage_capacity()).floor() as u32
    }

    fn get_u32_active_parts_of_type(&self, part: Part) -> std::iter::Filter<std::slice::Iter<'_, PartSpec>, fn(&&PartSpec) -> bool> {
        let f = match part {
            Part::Work => filter_is_work_part,
            Part::Attack => filter_is_attack_part,
            Part::RangedAttack => filter_is_ranged_attack_part,
            Part::Heal => filter_is_heal_part,
            Part::Tough => filter_is_tough_part,
            Part::Carry => filter_is_carry_part,
            Part::Move => filter_is_move_part,
            Part::Claim => filter_is_claim_part,
            _ => unimplemented!(),
        };
        self.body.iter().filter(f)
    }

    /// Calculates the amount of resources that a creep can store.
    ///
    /// ```rust
    /// use screeps::Part;
    /// use screeps::constants::CARRY_CAPACITY;
    /// use screeps_body_utils::body::{BodySpec, PartSpec};
    /// 
    /// let m = PartSpec::new_unboosted_part(Part::Move);
    /// let c = PartSpec::new_unboosted_part(Part::Carry);
    /// let body = vec!(c, m);
    /// let bodyspec = BodySpec::new(&body);
    /// assert_eq!(CARRY_CAPACITY, bodyspec.carry_capacity());
    /// ```
    pub fn carry_capacity(&self) -> u32 {
        self.get_u32_active_parts_of_type(Part::Carry).fold(0, |acc, p| acc + p.get_carry_capacity())
    }

    /// Calculates the melee attack damage that a creep can deal.
    ///
    /// ```rust
    /// use screeps::Part;
    /// use screeps::constants::ATTACK_POWER;
    /// use screeps_body_utils::body::{BodySpec, PartSpec};
    /// 
    /// let m = PartSpec::new_unboosted_part(Part::Move);
    /// let a = PartSpec::new_unboosted_part(Part::Attack);
    /// let body = vec!(a, m);
    /// let bodyspec = BodySpec::new(&body);
    /// assert_eq!(ATTACK_POWER, bodyspec.attack_damage());
    /// ```
    pub fn attack_damage(&self) -> u32 {
        self.get_u32_active_parts_of_type(Part::Attack).fold(0, |acc, p| acc + p.get_attack_damage())
    }

    /// Calculates the ranged attack damage that a creep can deal.
    ///
    /// ```rust
    /// use screeps::Part;
    /// use screeps::constants::RANGED_ATTACK_POWER;
    /// use screeps_body_utils::body::{BodySpec, PartSpec};
    /// 
    /// let m = PartSpec::new_unboosted_part(Part::Move);
    /// let r = PartSpec::new_unboosted_part(Part::RangedAttack);
    /// let body = vec!(r, m);
    /// let bodyspec = BodySpec::new(&body);
    /// assert_eq!(RANGED_ATTACK_POWER, bodyspec.ranged_attack_damage());
    /// ```
    pub fn ranged_attack_damage(&self) -> u32 {
        self.get_u32_active_parts_of_type(Part::RangedAttack).fold(0, |acc, p| acc + p.get_ranged_attack_damage())
    }

    /// Calculates the ranged mass attack damage that a creep can deal to a single target at a
    /// specific distance.
    ///
    /// ```rust
    /// use screeps::Part;
    /// use screeps::constants::extra::RANGED_MASS_ATTACK_POWER_RANGE_1;
    /// use screeps_body_utils::body::{BodySpec, PartSpec};
    /// 
    /// let m = PartSpec::new_unboosted_part(Part::Move);
    /// let r = PartSpec::new_unboosted_part(Part::RangedAttack);
    /// let body = vec!(r, m);
    /// let bodyspec = BodySpec::new(&body);
    /// assert_eq!(RANGED_MASS_ATTACK_POWER_RANGE_1, bodyspec.ranged_mass_attack_damage_at_distance_single_target(1));
    /// ```
    pub fn ranged_mass_attack_damage_at_distance_single_target(&self, distance: u8) -> u32 {
        self.get_u32_active_parts_of_type(Part::RangedAttack).fold(0, |acc, p| acc + p.get_ranged_mass_attack_damage_at_distance_single_target(distance))
    }

    /// Calculates the amount of hits that a creep can restore to a target creep at range 1.
    ///
    /// ```rust
    /// use screeps::Part;
    /// use screeps::constants::HEAL_POWER;
    /// use screeps_body_utils::body::{BodySpec, PartSpec};
    /// 
    /// let m = PartSpec::new_unboosted_part(Part::Move);
    /// let h = PartSpec::new_unboosted_part(Part::Heal);
    /// let body = vec!(h, m);
    /// let bodyspec = BodySpec::new(&body);
    /// assert_eq!(HEAL_POWER, bodyspec.heal_amount());
    /// ```
    pub fn heal_amount(&self) -> u32 {
        self.get_u32_active_parts_of_type(Part::Heal).fold(0, |acc, p| acc + p.get_heal_amount())
    }

    /// Calculates the amount of hits that a creep can restore to a target creep at range > 1.
    ///
    /// ```rust
    /// use screeps::Part;
    /// use screeps::constants::RANGED_HEAL_POWER;
    /// use screeps_body_utils::body::{BodySpec, PartSpec};
    /// 
    /// let m = PartSpec::new_unboosted_part(Part::Move);
    /// let h = PartSpec::new_unboosted_part(Part::Heal);
    /// let body = vec!(h, m);
    /// let bodyspec = BodySpec::new(&body);
    /// assert_eq!(RANGED_HEAL_POWER, bodyspec.ranged_heal_amount());
    /// ```
    pub fn ranged_heal_amount(&self) -> u32 {
        self.get_u32_active_parts_of_type(Part::Heal).fold(0, |acc, p| acc + p.get_ranged_heal_amount())
    }

    /// Calculates the amount of progress that a creep can add to a controller.
    ///
    /// ```rust
    /// use screeps::Part;
    /// use screeps::constants::UPGRADE_CONTROLLER_POWER;
    /// use screeps_body_utils::body::{BodySpec, PartSpec};
    /// 
    /// let m = PartSpec::new_unboosted_part(Part::Move);
    /// let w = PartSpec::new_unboosted_part(Part::Work);
    /// let body = vec!(w, m);
    /// let bodyspec = BodySpec::new(&body);
    /// assert_eq!(UPGRADE_CONTROLLER_POWER as f32, bodyspec.upgrade_controller_amount());
    /// ```
    pub fn upgrade_controller_amount(&self) -> f32 {
        self.get_u32_active_parts_of_type(Part::Work).fold(0.0, |acc, p| acc + p.get_upgrade_controller_amount())
    }

    /// Calculates the amount of hits that a creep can restore to a structure with repair.
    ///
    /// ```rust
    /// use screeps::Part;
    /// use screeps::constants::REPAIR_POWER;
    /// use screeps_body_utils::body::{BodySpec, PartSpec};
    /// 
    /// let m = PartSpec::new_unboosted_part(Part::Move);
    /// let w = PartSpec::new_unboosted_part(Part::Work);
    /// let body = vec!(w, m);
    /// let bodyspec = BodySpec::new(&body);
    /// assert_eq!(REPAIR_POWER as f32, bodyspec.repair_amount());
    /// ```
    pub fn repair_amount(&self) -> f32 {
        self.get_u32_active_parts_of_type(Part::Work).fold(0.0, |acc, p| acc + p.get_repair_amount())
    }

    /// Calculates the amount of progress that a creep can add to a construction site.
    ///
    /// ```rust
    /// use screeps::Part;
    /// use screeps::constants::BUILD_POWER;
    /// use screeps_body_utils::body::{BodySpec, PartSpec};
    /// 
    /// let m = PartSpec::new_unboosted_part(Part::Move);
    /// let w = PartSpec::new_unboosted_part(Part::Work);
    /// let body = vec!(w, m);
    /// let bodyspec = BodySpec::new(&body);
    /// assert_eq!(BUILD_POWER as f32, bodyspec.build_amount());
    /// ```
    pub fn build_amount(&self) -> f32 {
        self.get_u32_active_parts_of_type(Part::Work).fold(0.0, |acc, p| acc + p.get_build_amount())
    }

    /// Calculates the amount of damage that a creep can deal to a structure with dismantle.
    ///
    /// ```rust
    /// use screeps::Part;
    /// use screeps::constants::DISMANTLE_POWER;
    /// use screeps_body_utils::body::{BodySpec, PartSpec};
    /// 
    /// let m = PartSpec::new_unboosted_part(Part::Move);
    /// let w = PartSpec::new_unboosted_part(Part::Work);
    /// let body = vec!(w, m);
    /// let bodyspec = BodySpec::new(&body);
    /// assert_eq!(DISMANTLE_POWER, bodyspec.dismantle_damage());
    /// ```
    pub fn dismantle_damage(&self) -> u32 {
        self.get_u32_active_parts_of_type(Part::Work).fold(0, |acc, p| acc + p.get_dismantle_damage())
    }

    /// Calculates the amount of energy that a creep can harvest from a Source with harvest.
    ///
    /// ```rust
    /// use screeps::Part;
    /// use screeps::constants::HARVEST_POWER;
    /// use screeps_body_utils::body::{BodySpec, PartSpec};
    /// 
    /// let m = PartSpec::new_unboosted_part(Part::Move);
    /// let w = PartSpec::new_unboosted_part(Part::Work);
    /// let body = vec!(w, m);
    /// let bodyspec = BodySpec::new(&body);
    /// assert_eq!(HARVEST_POWER, bodyspec.harvest_energy_amount());
    /// ```
    pub fn harvest_energy_amount(&self) -> u32 {
        self.get_u32_active_parts_of_type(Part::Work).fold(0, |acc, p| acc + p.get_harvest_energy_amount())
    }

    /// Calculates the amount of minerals that a creep can harvest from a Mineral with harvest.
    ///
    /// ```rust
    /// use screeps::Part;
    /// use screeps::constants::HARVEST_MINERAL_POWER;
    /// use screeps_body_utils::body::{BodySpec, PartSpec};
    /// 
    /// let m = PartSpec::new_unboosted_part(Part::Move);
    /// let w = PartSpec::new_unboosted_part(Part::Work);
    /// let body = vec!(w, m);
    /// let bodyspec = BodySpec::new(&body);
    /// assert_eq!(HARVEST_MINERAL_POWER, bodyspec.harvest_mineral_amount());
    /// ```
    pub fn harvest_mineral_amount(&self) -> u32 {
        self.get_u32_active_parts_of_type(Part::Work).fold(0, |acc, p| acc + p.get_harvest_mineral_amount())
    }

    /// Calculates the amount of resources that a creep can harvest from a Deposit with harvest.
    ///
    /// ```rust
    /// use screeps::Part;
    /// use screeps::constants::HARVEST_DEPOSIT_POWER;
    /// use screeps_body_utils::body::{BodySpec, PartSpec};
    /// 
    /// let m = PartSpec::new_unboosted_part(Part::Move);
    /// let w = PartSpec::new_unboosted_part(Part::Work);
    /// let body = vec!(w, m);
    /// let bodyspec = BodySpec::new(&body);
    /// assert_eq!(HARVEST_DEPOSIT_POWER, bodyspec.harvest_deposit_amount());
    /// ```
    pub fn harvest_deposit_amount(&self) -> u32 {
        self.get_u32_active_parts_of_type(Part::Work).fold(0, |acc, p| acc + p.get_harvest_deposit_amount())
    }

    /// Calculates the amount of energy needed to spawn a creep with this body.
    ///
    /// ```rust
    /// use screeps::Part;
    /// use screeps_body_utils::body::{BodySpec, PartSpec};
    /// 
    /// let m = PartSpec::new_unboosted_part(Part::Move);
    /// let w = PartSpec::new_unboosted_part(Part::Work);
    /// let body = vec!(w, m);
    /// let bodyspec = BodySpec::new(&body);
    /// assert_eq!(Part::Move.cost() + Part::Work.cost(), bodyspec.energy_to_spawn());
    /// ```
    pub fn energy_to_spawn(&self) -> u32 {
        self.body.iter().fold(0, |acc, p| acc + p.part.cost())
    }

    /// Calculates the number of ticks needed to spawn a creep with this body.
    ///
    /// ```rust
    /// use screeps::Part;
    /// use screeps::constants::CREEP_SPAWN_TIME;
    /// use screeps_body_utils::body::{BodySpec, PartSpec};
    /// 
    /// let m = PartSpec::new_unboosted_part(Part::Move);
    /// let w = PartSpec::new_unboosted_part(Part::Work);
    /// let body = vec!(w, m);
    /// let bodyspec = BodySpec::new(&body);
    /// assert_eq!(CREEP_SPAWN_TIME * 2, bodyspec.ticks_to_spawn());
    /// ```
    pub fn ticks_to_spawn(&self) -> u32 {
        self.body.iter().fold(0, |acc, _| acc + CREEP_SPAWN_TIME)
    }

    /// Calculates the net exhaustion that results from moving this body onto a tile with a
    /// specific cost.
    fn tile_move_net_exhaustion(&self, tile_cost: u32) -> u32 {
        let (fatigue_generated, fatigue_reduced) = self.body.iter()
            .map(|p| (p.get_fatigue_generation(tile_cost), p.get_fatigue_reduction()))
            .fold((0,0), |acc, tpl| (acc.0 + tpl.0, acc.1 + tpl.1));
        fatigue_generated.saturating_sub(fatigue_reduced) 
    }

    /// Calculates the net exhaustion that results from moving this body onto a plains tile.
    ///
    /// ```rust
    /// use screeps::Part;
    /// use screeps::constants::extra::MOVE_COST_PLAIN;
    /// use screeps_body_utils::body::{BodySpec, PartSpec};
    /// 
    /// let m = PartSpec::new_unboosted_part(Part::Move);
    /// let w = PartSpec::new_unboosted_part(Part::Work);
    /// let body = vec!(w, m);
    /// let bodyspec = BodySpec::new(&body);
    /// assert_eq!(0, bodyspec.plains_move_net_exhaustion());
    ///
    /// let body = vec!(w, w, m);
    /// let bodyspec = BodySpec::new(&body);
    /// assert_eq!(MOVE_COST_PLAIN, bodyspec.plains_move_net_exhaustion());
    /// ```
    pub fn plains_move_net_exhaustion(&self) -> u32 {
        self.tile_move_net_exhaustion(MOVE_COST_PLAIN)
    }

    /// Calculates the net exhaustion that results from moving this body onto a swamp tile.
    ///
    /// ```rust
    /// use screeps::Part;
    /// use screeps::constants::MOVE_POWER;
    /// use screeps::constants::extra::MOVE_COST_SWAMP;
    /// use screeps_body_utils::body::{BodySpec, PartSpec};
    /// 
    /// let m = PartSpec::new_unboosted_part(Part::Move);
    /// let w = PartSpec::new_unboosted_part(Part::Work);
    /// let body = vec!(w, m);
    /// let bodyspec = BodySpec::new(&body);
    /// assert_eq!(MOVE_COST_SWAMP - MOVE_POWER, bodyspec.swamp_move_net_exhaustion());
    /// ```
    pub fn swamp_move_net_exhaustion(&self) -> u32 {
        self.tile_move_net_exhaustion(MOVE_COST_SWAMP)
    }

    /// Calculates the net exhaustion that results from moving this body onto a road tile.
    ///
    /// ```rust
    /// use screeps::Part;
    /// use screeps::constants::extra::MOVE_COST_ROAD;
    /// use screeps_body_utils::body::{BodySpec, PartSpec};
    /// 
    /// let m = PartSpec::new_unboosted_part(Part::Move);
    /// let w = PartSpec::new_unboosted_part(Part::Work);
    /// let body = vec!(w, w, m);
    /// let bodyspec = BodySpec::new(&body);
    /// assert_eq!(0, bodyspec.road_move_net_exhaustion());
    /// ```
    pub fn road_move_net_exhaustion(&self) -> u32 {
        self.tile_move_net_exhaustion(MOVE_COST_ROAD)
    }
}

