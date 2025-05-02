//! Convenience abstraction layer for working with Boosts.

use screeps::constants::ResourceType;
use screeps::constants::Boost;
use screeps::constants::Part;
use crate::boost::reaction::Reaction;
use crate::boost::reaction_chains;

/// Encapsulates the concept of a Creep boost, regardless of the actual numbers involved.
#[derive(Debug, PartialEq, Hash, Copy, Clone)]
pub enum AbstractBoost {
    // Work
    T1Harvest,
    T1BuildRepair,
    T1Dismantle,
    T1UpgradeController,
    T2Harvest,
    T2BuildRepair,
    T2Dismantle,
    T2UpgradeController,
    T3Harvest,
    T3BuildRepair,
    T3Dismantle,
    T3UpgradeController,

    // Attack
    T1Attack,
    T2Attack,
    T3Attack,

    // Ranged Attack
    T1RangedAttack,
    T2RangedAttack,
    T3RangedAttack,

    // Heal
    T1Heal,
    T2Heal,
    T3Heal,

    // Carry
    T1Carry,
    T2Carry,
    T3Carry,

    // Move
    T1Move, 
    T2Move, 
    T3Move, 

    // Tough
    T1Tough,
    T2Tough,
    T3Tough,
}

/// The 12 boosts that can be applied to [Work](Part::Work) parts.
///
/// ## Tier 1 Boosts
/// [T1Harvest](AbstractBoost::T1Harvest)
///
/// [T1BuildRepair](AbstractBoost::T1BuildRepair)
///
/// [T1Dismantle](AbstractBoost::T1Dismantle)
///
/// [T1UpgradeController](AbstractBoost::T1UpgradeController)
///
/// ## Tier 2 Boosts
/// [T2Harvest](AbstractBoost::T2Harvest)
///
/// [T2BuildRepair](AbstractBoost::T2BuildRepair)
///
/// [T2Dismantle](AbstractBoost::T2Dismantle)
///
/// [T2UpgradeController](AbstractBoost::T2UpgradeController)
///
/// ## Tier 3 Boosts
/// [T3Harvest](AbstractBoost::T3Harvest)
///
/// [T3BuildRepair](AbstractBoost::T3BuildRepair)
///
/// [T3Dismantle](AbstractBoost::T3Dismantle)
///
/// [T3UpgradeController](AbstractBoost::T3UpgradeController)
pub const WORK_BOOSTS: [AbstractBoost; 12] = [
    AbstractBoost::T1Harvest,
    AbstractBoost::T1BuildRepair,
    AbstractBoost::T1Dismantle,
    AbstractBoost::T1UpgradeController,
    AbstractBoost::T2Harvest,
    AbstractBoost::T2BuildRepair,
    AbstractBoost::T2Dismantle,
    AbstractBoost::T2UpgradeController,
    AbstractBoost::T3Harvest,
    AbstractBoost::T3BuildRepair,
    AbstractBoost::T3Dismantle,
    AbstractBoost::T3UpgradeController,
];

/// The 3 boosts that can be applied to [Work](Part::Work) parts to enhance
/// [Harvest](screeps::Creep::harvest) actions.
///
/// [T1Harvest](AbstractBoost::T1Harvest)
///
/// [T2Harvest](AbstractBoost::T2Harvest)
///
/// [T3Harvest](AbstractBoost::T3Harvest)
pub const HARVEST_BOOSTS: [AbstractBoost; 3] = [
    AbstractBoost::T1Harvest,
    AbstractBoost::T2Harvest,
    AbstractBoost::T3Harvest,
];

/// The 3 boosts that can be applied to [Work](Part::Work) parts to enhance
/// [Build](screeps::Creep::build) and [Repair](screeps::Creep::repair) actions.
///
/// [T1BuildRepair](AbstractBoost::T1BuildRepair)
///
/// [T2BuildRepair](AbstractBoost::T2BuildRepair)
///
/// [T3BuildRepair](AbstractBoost::T3BuildRepair)
pub const BUILD_REPAIR_BOOSTS: [AbstractBoost; 3] = [
    AbstractBoost::T1BuildRepair,
    AbstractBoost::T2BuildRepair,
    AbstractBoost::T3BuildRepair,
];

/// The 3 boosts that can be applied to [Work](Part::Work) parts to enhance
/// [Dismantle](screeps::Creep::dismantle) actions.
///
/// [T1Dismantle](AbstractBoost::T1Dismantle)
///
/// [T2Dismantle](AbstractBoost::T2Dismantle)
///
/// [T3Dismantle](AbstractBoost::T3Dismantle)
pub const DISMANTLE_BOOSTS: [AbstractBoost; 3] = [
    AbstractBoost::T1Dismantle,
    AbstractBoost::T2Dismantle,
    AbstractBoost::T3Dismantle,
];

/// The 3 boosts that can be applied to [Work](Part::Work) parts to enhance
/// [Upgrade Controller](screeps::Creep::upgrade_controller) actions.
///
/// [T1UpgradeController](AbstractBoost::T1UpgradeController)
///
/// [T2UpgradeController](AbstractBoost::T2UpgradeController)
///
/// [T3UpgradeController](AbstractBoost::T3UpgradeController)
pub const UPGRADE_CONTROLLER_BOOSTS: [AbstractBoost; 3] = [
    AbstractBoost::T1UpgradeController,
    AbstractBoost::T2UpgradeController,
    AbstractBoost::T3UpgradeController,
];

/// The 3 boosts that can be applied to [Attack](Part::Attack) parts to enhance
/// [Attack](screeps::Creep::attack) actions.
///
/// [T1Attack](AbstractBoost::T1Attack)
///
/// [T2Attack](AbstractBoost::T2Attack)
///
/// [T3Attack](AbstractBoost::T3Attack)
pub const ATTACK_BOOSTS: [AbstractBoost; 3] = [
    AbstractBoost::T1Attack,
    AbstractBoost::T2Attack,
    AbstractBoost::T3Attack,
];

/// The 3 boosts that can be applied to [Ranged Attack](Part::RangedAttack) parts to enhance
/// [Ranged Attack](screeps::Creep::ranged_attack) and [Ranged Mass Attack](screeps::Creep::ranged_mass_attack) actions.
///
/// [T1RangedAttack](AbstractBoost::T1RangedAttack)
///
/// [T2RangedAttack](AbstractBoost::T2RangedAttack)
///
/// [T3RangedAttack](AbstractBoost::T3RangedAttack)
pub const RANGED_ATTACK_BOOSTS: [AbstractBoost; 3] = [
    AbstractBoost::T1RangedAttack,
    AbstractBoost::T2RangedAttack,
    AbstractBoost::T3RangedAttack,
];

/// The 3 boosts that can be applied to [Heal](Part::Heal) parts to enhance
/// [Heal](screeps::Creep::heal) and [Ranged Heal](screeps::Creep::ranged_heal) actions.
///
/// [T1Heal](AbstractBoost::T1Heal)
///
/// [T2Heal](AbstractBoost::T2Heal)
///
/// [T3Heal](AbstractBoost::T3Heal)
pub const HEAL_BOOSTS: [AbstractBoost; 3] = [
    AbstractBoost::T1Heal,
    AbstractBoost::T2Heal,
    AbstractBoost::T3Heal,
];

/// The 3 boosts that can be applied to [Carry](Part::Carry) parts to enhance
/// the store capacity of a creep.
///
/// [T1Carry](AbstractBoost::T1Carry)
///
/// [T2Carry](AbstractBoost::T2Carry)
///
/// [T3Carry](AbstractBoost::T3Carry)
pub const CARRY_BOOSTS: [AbstractBoost; 3] = [
    AbstractBoost::T1Carry,
    AbstractBoost::T2Carry,
    AbstractBoost::T3Carry,
];

/// The 3 boosts that can be applied to [Move](Part::Move) parts to enhance
/// the fatigue reduction they provide.
///
/// [T1Move](AbstractBoost::T1Move)
///
/// [T2Move](AbstractBoost::T2Move)
///
/// [T3Move](AbstractBoost::T3Move)
pub const MOVE_BOOSTS: [AbstractBoost; 3] = [
    AbstractBoost::T1Move, 
    AbstractBoost::T2Move, 
    AbstractBoost::T3Move, 
];

/// The 3 boosts that can be applied to [Tough](Part::Tough) parts to enhance
/// the damage reduction they provide.
///
/// [T1Tough](AbstractBoost::T1Tough)
///
/// [T2Tough](AbstractBoost::T2Tough)
///
/// [T3Tough](AbstractBoost::T3Tough)
pub const TOUGH_BOOSTS: [AbstractBoost; 3] = [
    AbstractBoost::T1Tough,
    AbstractBoost::T2Tough,
    AbstractBoost::T3Tough,
];

/// The 10 Tier 1 boosts.
///
/// [T1Harvest](AbstractBoost::T1Harvest)
///
/// [T1BuildRepair](AbstractBoost::T1BuildRepair)
///
/// [T1Dismantle](AbstractBoost::T1Dismantle)
///
/// [T1UpgradeController](AbstractBoost::T1UpgradeController)
///
/// [T1Attack](AbstractBoost::T1Attack)
///
/// [T1RangedAttack](AbstractBoost::T1RangedAttack)
///
/// [T1Heal](AbstractBoost::T1Heal)
///
/// [T1Carry](AbstractBoost::T1Carry)
///
/// [T1Move](AbstractBoost::T1Move)
///
/// [T1Tough](AbstractBoost::T1Tough)
pub const T1_BOOSTS: [AbstractBoost; 10] = [
    AbstractBoost::T1Harvest,
    AbstractBoost::T1BuildRepair,
    AbstractBoost::T1Dismantle,
    AbstractBoost::T1UpgradeController,
    AbstractBoost::T1Attack,
    AbstractBoost::T1RangedAttack,
    AbstractBoost::T1Heal,
    AbstractBoost::T1Carry,
    AbstractBoost::T1Move, 
    AbstractBoost::T1Tough,
];

/// The 10 Tier 2 boosts.
///
/// [T2Harvest](AbstractBoost::T2Harvest)
///
/// [T2BuildRepair](AbstractBoost::T2BuildRepair)
///
/// [T2Dismantle](AbstractBoost::T2Dismantle)
///
/// [T2UpgradeController](AbstractBoost::T2UpgradeController)
///
/// [T2Attack](AbstractBoost::T2Attack)
///
/// [T2RangedAttack](AbstractBoost::T2RangedAttack)
///
/// [T2Heal](AbstractBoost::T2Heal)
///
/// [T2Carry](AbstractBoost::T2Carry)
///
/// [T2Move](AbstractBoost::T2Move)
///
/// [T2Tough](AbstractBoost::T2Tough)
pub const T2_BOOSTS: [AbstractBoost; 10] = [
    AbstractBoost::T2Harvest,
    AbstractBoost::T2BuildRepair,
    AbstractBoost::T2Dismantle,
    AbstractBoost::T2UpgradeController,
    AbstractBoost::T2Attack,
    AbstractBoost::T2RangedAttack,
    AbstractBoost::T2Heal,
    AbstractBoost::T2Carry,
    AbstractBoost::T2Move, 
    AbstractBoost::T2Tough,
];

/// The 10 Tier 3 boosts.
///
/// [T3Harvest](AbstractBoost::T3Harvest)
///
/// [T3BuildRepair](AbstractBoost::T3BuildRepair)
///
/// [T3Dismantle](AbstractBoost::T3Dismantle)
///
/// [T3UpgradeController](AbstractBoost::T3UpgradeController)
///
/// [T3Attack](AbstractBoost::T3Attack)
///
/// [T3RangedAttack](AbstractBoost::T3RangedAttack)
///
/// [T3Heal](AbstractBoost::T3Heal)
///
/// [T3Carry](AbstractBoost::T3Carry)
///
/// [T3Move](AbstractBoost::T3Move)
///
/// [T3Tough](AbstractBoost::T3Tough)
pub const T3_BOOSTS: [AbstractBoost; 10] = [
    AbstractBoost::T3Harvest,
    AbstractBoost::T3BuildRepair,
    AbstractBoost::T3Dismantle,
    AbstractBoost::T3UpgradeController,
    AbstractBoost::T3Attack,
    AbstractBoost::T3RangedAttack,
    AbstractBoost::T3Heal,
    AbstractBoost::T3Carry,
    AbstractBoost::T3Move, 
    AbstractBoost::T3Tough,
];

/// Abstract representation of attributes that can be boosted.
#[derive(Debug, PartialEq, Hash, Copy, Clone)]
pub enum BoostCategory {
    HarvestEnergy,
    HarvestMineral,
    HarvestDeposit,
    Build,
    Repair,
    Dismantle,
    UpgradeController,
    Attack,
    RangedAttack,
    RangedMassAttack,
    Heal,
    RangedHeal,
    Carry,
    Move,
    Tough,
}

impl BoostCategory {
    pub const fn get_associated_part_for_category(value: &BoostCategory) -> Part {
        use BoostCategory::*;
        match value {
            HarvestEnergy => Part::Work,
            HarvestMineral => Part::Work,
            HarvestDeposit => Part::Work,
            Build => Part::Work,
            Repair => Part::Work,
            Dismantle => Part::Work,
            UpgradeController => Part::Work,
            Attack => Part::Attack,
            RangedAttack => Part::RangedAttack,
            RangedMassAttack => Part::RangedAttack,
            Heal => Part::Heal,
            RangedHeal => Part::Heal,
            Carry => Part::Carry,
            Move => Part::Move,
            Tough => Part::Tough,
        }
    }

    pub const fn get_associated_part(&self) -> Part {
        BoostCategory::get_associated_part_for_category(self)
    }

    pub const fn get_abstract_boosts_for_category(value: &BoostCategory) -> [AbstractBoost; 3] {
        use BoostCategory::*;
        match value {
            HarvestEnergy => HARVEST_BOOSTS,
            HarvestMineral => HARVEST_BOOSTS,
            HarvestDeposit => HARVEST_BOOSTS,
            Build => BUILD_REPAIR_BOOSTS,
            Repair => BUILD_REPAIR_BOOSTS,
            Dismantle => DISMANTLE_BOOSTS,
            UpgradeController => UPGRADE_CONTROLLER_BOOSTS,
            Attack => ATTACK_BOOSTS,
            RangedAttack => RANGED_ATTACK_BOOSTS,
            RangedMassAttack => RANGED_ATTACK_BOOSTS,
            Heal => HEAL_BOOSTS,
            RangedHeal => HEAL_BOOSTS,
            Carry => CARRY_BOOSTS,
            Move => MOVE_BOOSTS,
            Tough => TOUGH_BOOSTS,
        }
    }

    pub const fn get_abstract_boosts(&self) -> [AbstractBoost; 3] {
        BoostCategory::get_abstract_boosts_for_category(self)
    }
}

impl AbstractBoost {

    /// A const fn to check whether a specific boost is contained within a slice.
    const fn boost_in_slice(s: &[AbstractBoost], boost: &AbstractBoost) -> bool {
        let mut i = 0;
        while i < s.len() {
            if AbstractBoost::const_eq(&s[i], boost) {
                return true;
            }
            i += 1;
        }
        false
    }

    /// A const fn to check whether two boosts are the same boost.
    ///
    /// We need a const function version of this because `PartialEq` functions are not const, and thus we
    /// can't use `==` to check for equality inside other const functions.
    pub const fn const_eq(a: &AbstractBoost, b: &AbstractBoost) -> bool {
        use AbstractBoost::*;
        match (a, b) {
            (T1Harvest, T1Harvest) => true,
            (T1BuildRepair, T1BuildRepair) => true,
            (T1Dismantle, T1Dismantle) => true,
            (T1UpgradeController, T1UpgradeController) => true,
            (T2Harvest, T2Harvest) => true,
            (T2BuildRepair, T2BuildRepair) => true,
            (T2Dismantle, T2Dismantle) => true,
            (T2UpgradeController, T2UpgradeController) => true,
            (T3Harvest, T3Harvest) => true,
            (T3BuildRepair, T3BuildRepair) => true,
            (T3Dismantle, T3Dismantle) => true,
            (T3UpgradeController, T3UpgradeController) => true,
            (T1Attack, T1Attack) => true,
            (T2Attack, T2Attack) => true,
            (T3Attack, T3Attack) => true,
            (T1RangedAttack, T1RangedAttack) => true,
            (T2RangedAttack, T2RangedAttack) => true,
            (T3RangedAttack, T3RangedAttack) => true,
            (T1Heal, T1Heal) => true,
            (T2Heal, T2Heal) => true,
            (T3Heal, T3Heal) => true,
            (T1Carry, T1Carry) => true,
            (T2Carry, T2Carry) => true,
            (T3Carry, T3Carry) => true,
            (T1Move, T1Move) => true, 
            (T2Move, T2Move) => true, 
            (T3Move, T3Move) => true, 
            (T1Tough, T1Tough) => true,
            (T2Tough, T2Tough) => true,
            (T3Tough, T3Tough) => true,
            _ => false,
        }
    }

    /// Attempts to create the corresponding boost for a given resource, failing if the resource
    /// does not have a corresponding boost.
    pub const fn try_from_resource_type(value: &ResourceType) -> Result<AbstractBoost, &'static str> {
        use ResourceType::*;
        use AbstractBoost::*;
        match value {
            UtriumHydride => Ok(T1Attack),
            UtriumAcid => Ok(T2Attack),
            CatalyzedUtriumAcid => Ok(T3Attack),
            UtriumOxide => Ok(T1Harvest),
            UtriumAlkalide => Ok(T2Harvest),
            CatalyzedUtriumAlkalide => Ok(T3Harvest),
            KeaniumHydride => Ok(T1Carry),
            KeaniumAcid => Ok(T2Carry),
            CatalyzedKeaniumAcid => Ok(T3Carry),
            KeaniumOxide => Ok(T1RangedAttack),
            KeaniumAlkalide => Ok(T2RangedAttack),
            CatalyzedKeaniumAlkalide => Ok(T3RangedAttack),
            LemergiumHydride => Ok(T1BuildRepair),
            LemergiumAcid => Ok(T2BuildRepair),
            CatalyzedLemergiumAcid => Ok(T3BuildRepair),
            LemergiumOxide => Ok(T1Heal),
            LemergiumAlkalide => Ok(T2Heal),
            CatalyzedLemergiumAlkalide => Ok(T3Heal),
            ZynthiumHydride => Ok(T1Dismantle),
            ZynthiumAcid => Ok(T2Dismantle),
            CatalyzedZynthiumAcid => Ok(T3Dismantle),
            ZynthiumOxide => Ok(T1Move),
            ZynthiumAlkalide => Ok(T2Move),
            CatalyzedZynthiumAlkalide => Ok(T3Move),
            GhodiumHydride => Ok(T1UpgradeController),
            GhodiumAcid => Ok(T2UpgradeController),
            CatalyzedGhodiumAcid => Ok(T3UpgradeController),
            GhodiumOxide => Ok(T1Tough),
            GhodiumAlkalide => Ok(T2Tough),
            CatalyzedGhodiumAlkalide => Ok(T3Tough),
            _ => Err("resource is not a boost compound"),
        }
    }

    /// Returns the AbstractBoost variants that are valid boosts for a particular Part variant.
    pub const fn boosts_for_part(part: &Part) -> &'static [AbstractBoost] {
        match part {
            Part::Work => &WORK_BOOSTS,
            Part::Attack => &ATTACK_BOOSTS,
            Part::RangedAttack => &RANGED_ATTACK_BOOSTS,
            Part::Heal => &HEAL_BOOSTS,
            Part::Carry => &CARRY_BOOSTS,
            Part::Move => &MOVE_BOOSTS,
            Part::Tough => &TOUGH_BOOSTS,
            _ => &[],
        }
    }

    /// Returns the [Part] that can be boosted by a particular boost.
    pub const fn part_for_boost(boost: &AbstractBoost) -> Part {
        match boost {
            // Work
            AbstractBoost::T1Harvest => Part::Work,
            AbstractBoost::T1BuildRepair => Part::Work,
            AbstractBoost::T1Dismantle => Part::Work,
            AbstractBoost::T1UpgradeController => Part::Work,
            AbstractBoost::T2Harvest => Part::Work,
            AbstractBoost::T2BuildRepair => Part::Work,
            AbstractBoost::T2Dismantle => Part::Work,
            AbstractBoost::T2UpgradeController => Part::Work,
            AbstractBoost::T3Harvest => Part::Work,
            AbstractBoost::T3BuildRepair => Part::Work,
            AbstractBoost::T3Dismantle => Part::Work,
            AbstractBoost::T3UpgradeController => Part::Work,

            // Attack
            AbstractBoost::T1Attack => Part::Attack,
            AbstractBoost::T2Attack => Part::Attack,
            AbstractBoost::T3Attack => Part::Attack,

            // Ranged Attack
            AbstractBoost::T1RangedAttack => Part::RangedAttack,
            AbstractBoost::T2RangedAttack => Part::RangedAttack,
            AbstractBoost::T3RangedAttack => Part::RangedAttack,

            // Heal
            AbstractBoost::T1Heal => Part::Heal,
            AbstractBoost::T2Heal => Part::Heal,
            AbstractBoost::T3Heal => Part::Heal,

            // Carry
            AbstractBoost::T1Carry => Part::Carry,
            AbstractBoost::T2Carry => Part::Carry,
            AbstractBoost::T3Carry => Part::Carry,

            // Move
            AbstractBoost::T1Move => Part::Move, 
            AbstractBoost::T2Move => Part::Move, 
            AbstractBoost::T3Move => Part::Move, 

            // Tough
            AbstractBoost::T1Tough => Part::Tough,
            AbstractBoost::T2Tough => Part::Tough,
            AbstractBoost::T3Tough => Part::Tough,
        }
    }

    /// Determines the [Part] associated with this boost.
    pub const fn associated_part(&self) -> Part {
        AbstractBoost::part_for_boost(self)
    }

    /// Determines the [resource](screeps::ResourceType) associated with the given boost.
    pub const fn resource_for_boost(boost: &AbstractBoost) -> ResourceType {
        use ResourceType::*;
        use AbstractBoost::*;
        match boost {
            T1Attack => UtriumHydride,
            T2Attack => UtriumAcid,
            T3Attack => CatalyzedUtriumAcid,
            T1Harvest => UtriumOxide,
            T2Harvest => UtriumAlkalide,
            T3Harvest => CatalyzedUtriumAlkalide,
            T1Carry => KeaniumHydride,
            T2Carry => KeaniumAcid,
            T3Carry => CatalyzedKeaniumAcid,
            T1RangedAttack => KeaniumOxide,
            T2RangedAttack => KeaniumAlkalide,
            T3RangedAttack => CatalyzedKeaniumAlkalide,
            T1BuildRepair => LemergiumHydride,
            T2BuildRepair => LemergiumAcid,
            T3BuildRepair => CatalyzedLemergiumAcid,
            T1Heal => LemergiumOxide,
            T2Heal => LemergiumAlkalide,
            T3Heal => CatalyzedLemergiumAlkalide,
            T1Dismantle => ZynthiumHydride,
            T2Dismantle => ZynthiumAcid,
            T3Dismantle => CatalyzedZynthiumAcid,
            T1Move => ZynthiumOxide,
            T2Move => ZynthiumAlkalide,
            T3Move => CatalyzedZynthiumAlkalide,
            T1UpgradeController => GhodiumHydride,
            T2UpgradeController => GhodiumAcid,
            T3UpgradeController => CatalyzedGhodiumAcid,
            T1Tough => GhodiumOxide,
            T2Tough => GhodiumAlkalide,
            T3Tough => CatalyzedGhodiumAlkalide,
        }
    }

    /// Determines the [resource](screeps::ResourceType) associated with this boost.
    pub const fn associated_resource(&self) -> ResourceType {
        AbstractBoost::resource_for_boost(self)
    }

    /// Determines the tier of the given boost.
    pub const fn tier_for_boost(boost: &AbstractBoost) -> u8 {
        if AbstractBoost::boost_in_slice(&T3_BOOSTS, boost) {
            3
        } else {
            if AbstractBoost::boost_in_slice(&T2_BOOSTS, boost) {
                2
            } else {
                if AbstractBoost::boost_in_slice(&T1_BOOSTS, boost) {
                    1
                } else {
                    panic!("boost categories should always be exhaustive")
                }
            }
        }
    }

    /// Determines the tier of this boost.
    pub const fn tier(&self) -> u8 {
        AbstractBoost::tier_for_boost(self)
    }

    /// Get the immediate resources used to produce the specified boost.
    pub const fn reaction_components_for_boost(boost: &AbstractBoost) -> [ResourceType; 2] {
        // Unwrap here is safe, because the ResourceType we get from converting an AbstractBoost
        // will always be one with a set of reaction components.
        boost.associated_resource().reaction_components().unwrap()
    }

    /// Get the immediate resources used to produce this boost.
    pub const fn reaction_components(&self) -> [ResourceType; 2] {
        AbstractBoost::reaction_components_for_boost(self)
    }

    /// Determines the reaction chain for the given boost.
    ///
    /// A reaction chain is a slice of [Reactions](crate::boost::reaction::Reaction) that, if
    /// run in-order in your labs, will produce a desired boost resource.
    pub const fn reaction_chain_for_boost(boost: &AbstractBoost) -> &'static [Reaction] {
        let resource = boost.associated_resource();
        reaction_chains::get_reaction_chain_for_resource(&resource)
    }

    /// Determines the reaction chain for this boost.
    ///
    /// A reaction chain is a slice of [Reactions](crate::boost::reaction::Reaction) that, if
    /// run in-order in your labs, will produce a desired boost resource.
    pub const fn reaction_chain(&self) -> &'static [Reaction] {
        AbstractBoost::reaction_chain_for_boost(self)
    }

    /// Returns the amount of time needed to run the lab reaction that produces the specified
    /// boost.
    pub const fn reaction_time_for_boost(boost: &AbstractBoost) -> u32 {
        let resource = boost.associated_resource();

        // Unwrap is safe here because a boost is always associated with a resource that has a
        // reaction time constant
        resource.reaction_time().unwrap()
    }

    /// Returns the amount of time needed to run the lab reaction that produces this boost.
    pub const fn reaction_time(&self) -> u32 {
        AbstractBoost::reaction_time_for_boost(self)
    }

    /// Returns the amount of time needed to run the reaction chain that produces the specified
    /// boost.
    pub const fn reaction_chain_time_for_boost(boost: &AbstractBoost) -> u32 {
        Reaction::reaction_time_for_chain(boost.reaction_chain())
    }

    /// Returns the amount of time needed to run the reaction chain that produces this boost.
    pub const fn reaction_chain_time(&self) -> u32 {
        AbstractBoost::reaction_chain_time_for_boost(self)
    }

    /// Converts an AbstractBoost into a Boost within a const context.
    pub const fn const_to_boost(value: &AbstractBoost) -> Boost {
        use AbstractBoost::*;
        match value {
            T1Attack => Boost::Attack(2),
            T2Attack => Boost::Attack(3),
            T3Attack => Boost::Attack(4),
            T1Harvest => Boost::Harvest(3),
            T2Harvest => Boost::Harvest(5),
            T3Harvest => Boost::Harvest(7),
            T1Carry => Boost::Carry(2),
            T2Carry => Boost::Carry(3),
            T3Carry => Boost::Carry(4),
            T1RangedAttack => Boost::RangedAttack(2),
            T2RangedAttack => Boost::RangedAttack(3),
            T3RangedAttack => Boost::RangedAttack(4),
            T1BuildRepair => Boost::BuildAndRepair(1.5),
            T2BuildRepair => Boost::BuildAndRepair(1.8),
            T3BuildRepair => Boost::BuildAndRepair(2.0),
            T1Heal => Boost::Heal(2),
            T2Heal => Boost::Heal(3),
            T3Heal => Boost::Heal(4),
            T1Dismantle => Boost::Dismantle(2),
            T2Dismantle => Boost::Dismantle(3),
            T3Dismantle => Boost::Dismantle(4),
            T1Move => Boost::Move(2),
            T2Move => Boost::Move(3),
            T3Move => Boost::Move(4),
            T1UpgradeController => Boost::UpgradeController(1.5),
            T2UpgradeController => Boost::UpgradeController(1.8),
            T3UpgradeController => Boost::UpgradeController(2.0),
            T1Tough => Boost::Tough(0.7),
            T2Tough => Boost::Tough(0.5),
            T3Tough => Boost::Tough(0.3),
        }
    }

    /// Whether the boost has a u32 multiplier.
    ///
    /// This is the inverse of `has_f32_multiplier`.
    ///
    /// true if the multiplier is a u32, false if it's a f32.
    pub const fn has_u32_multiplier(value: &AbstractBoost) -> bool {
        use AbstractBoost::*;
        match value {
            T1Attack => true,
            T2Attack => true,
            T3Attack => true,
            T1Harvest => true,
            T2Harvest => true,
            T3Harvest => true,
            T1Carry => true,
            T2Carry => true,
            T3Carry => true,
            T1RangedAttack => true,
            T2RangedAttack => true,
            T3RangedAttack => true,
            T1BuildRepair => false,
            T2BuildRepair => false,
            T3BuildRepair => false,
            T1Heal => true,
            T2Heal => true,
            T3Heal => true,
            T1Dismantle => true,
            T2Dismantle => true,
            T3Dismantle => true,
            T1Move => true,
            T2Move => true,
            T3Move => true,
            T1UpgradeController => false,
            T2UpgradeController => false,
            T3UpgradeController => false,
            T1Tough => false,
            T2Tough => false,
            T3Tough => false,
        }
    }

    /// Whether the boost has an f32 multiplier.
    ///
    /// This is the inverse of `has_u32_multiplier`.
    ///
    /// true if the multiplier is a f32, false if it's a u32.
    pub const fn has_f32_multiplier(value: &AbstractBoost) -> bool {
        !AbstractBoost::has_u32_multiplier(value)
    }

    /// Gets the boost multiplier for this boost.
    ///
    /// Returns Some(u32) if the multiplier is a u32, None otherwise.
    pub const fn get_u32_multiplier(value: &AbstractBoost) -> Option<u32> {
        use AbstractBoost::*;
        match value {
            T1Attack => Some(2),
            T2Attack => Some(3),
            T3Attack => Some(4),
            T1Harvest => Some(3),
            T2Harvest => Some(5),
            T3Harvest => Some(7),
            T1Carry => Some(2),
            T2Carry => Some(3),
            T3Carry => Some(4),
            T1RangedAttack => Some(2),
            T2RangedAttack => Some(3),
            T3RangedAttack => Some(4),
            T1BuildRepair => None,
            T2BuildRepair => None,
            T3BuildRepair => None,
            T1Heal => Some(2),
            T2Heal => Some(3),
            T3Heal => Some(4),
            T1Dismantle => Some(2),
            T2Dismantle => Some(3),
            T3Dismantle => Some(4),
            T1Move => Some(2),
            T2Move => Some(3),
            T3Move => Some(4),
            T1UpgradeController => None,
            T2UpgradeController => None,
            T3UpgradeController => None,
            T1Tough => None,
            T2Tough => None,
            T3Tough => None,
        }
    }

    /// Gets the boost multiplier for this boost.
    ///
    /// Returns Some(f32) if the multiplier is an f32, None otherwise.
    pub const fn get_f32_multiplier(value: &AbstractBoost) -> Option<f32> {
        use AbstractBoost::*;
        match value {
            T1Attack => None,
            T2Attack => None,
            T3Attack => None,
            T1Harvest => None,
            T2Harvest => None,
            T3Harvest => None,
            T1Carry => None,
            T2Carry => None,
            T3Carry => None,
            T1RangedAttack => None,
            T2RangedAttack => None,
            T3RangedAttack => None,
            T1BuildRepair => Some(1.5),
            T2BuildRepair => Some(1.8),
            T3BuildRepair => Some(2.0),
            T1Heal => None,
            T2Heal => None,
            T3Heal => None,
            T1Dismantle => None,
            T2Dismantle => None,
            T3Dismantle => None,
            T1Move => None,
            T2Move => None,
            T3Move => None,
            T1UpgradeController => Some(1.5),
            T2UpgradeController => Some(1.8),
            T3UpgradeController => Some(2.0),
            T1Tough => Some(0.7),
            T2Tough => Some(0.5),
            T3Tough => Some(0.3),
        }
    }
}

// TryFrom impl for Resource (since only some resources map to a boost)
impl TryFrom<ResourceType> for AbstractBoost {
    type Error = &'static str;

    fn try_from(value: ResourceType) -> Result<Self, Self::Error> {
        AbstractBoost::try_from_resource_type(&value)
    }
}

// From impl for ResourceType (since each boost maps to a resource)
impl From<AbstractBoost> for ResourceType {
    fn from(val: AbstractBoost) -> Self {
        ResourceType::from(&val)
    }
}

impl From<&AbstractBoost> for ResourceType {
    fn from(val: &AbstractBoost) -> Self {
        val.associated_resource()
    }
}

// TryFrom impl for Boost (since the boosts have internal numbers, so technically they might fail
// to match)
impl TryFrom<Boost> for AbstractBoost {
    type Error = &'static str;

    fn try_from(value: Boost) -> Result<Self, Self::Error> {
        use AbstractBoost::*;
        match value {
            Boost::Attack(val) => {
                match val {
                    2 => Ok(T1Attack),
                    3 => Ok(T2Attack),
                    4 => Ok(T3Attack),
                    _ => Err("invalid boost value"),
                }
            },
            Boost::Harvest(val) => {
                match val {
                    3 => Ok(T1Harvest),
                    5 => Ok(T2Harvest),
                    7 => Ok(T3Harvest),
                    _ => Err("invalid boost value"),
                }
            },
            Boost::Carry(val) => {
                match val {
                    2 => Ok(T1Carry),
                    3 => Ok(T2Carry),
                    4 => Ok(T3Carry),
                    _ => Err("invalid boost value"),
                }
            },
            Boost::RangedAttack(val) => {
                match val {
                    2 => Ok(T1RangedAttack),
                    3 => Ok(T2RangedAttack),
                    4 => Ok(T3RangedAttack),
                    _ => Err("invalid boost value"),
                }
            },
            Boost::BuildAndRepair(val) => {
                match val {
                    1.5 => Ok(T1BuildRepair),
                    1.8 => Ok(T2BuildRepair),
                    2.0 => Ok(T3BuildRepair),
                    _ => Err("invalid boost value"),
                }
            },
            Boost::Heal(val) => {
                match val {
                    2 => Ok(T1Heal),
                    3 => Ok(T2Heal),
                    4 => Ok(T3Heal),
                    _ => Err("invalid boost value"),
                }
            },
            Boost::Dismantle(val) => {
                match val {
                    2 => Ok(T1Dismantle),
                    3 => Ok(T2Dismantle),
                    4 => Ok(T3Dismantle),
                    _ => Err("invalid boost value"),
                }
            },
            Boost::Move(val) => {
                match val {
                    2 => Ok(T1Move),
                    3 => Ok(T2Move),
                    4 => Ok(T3Move),
                    _ => Err("invalid boost value"),
                }
            },
            Boost::UpgradeController(val) => {
                match val {
                    1.5 => Ok(T1UpgradeController),
                    1.8 => Ok(T2UpgradeController),
                    2.0 => Ok(T3UpgradeController),
                    _ => Err("invalid boost value"),
                }
            },
            Boost::Tough(val) => {
                match val {
                    0.7 => Ok(T1Tough),
                    0.5 => Ok(T2Tough),
                    0.3 => Ok(T3Tough),
                    _ => Err("invalid boost value"),
                }
            },
        }
    }
}

// From impl for Boost, because our abstract boosts always map to the Boost enum values
impl From<AbstractBoost> for Boost {
    fn from(value: AbstractBoost) -> Self {
        Boost::from(&value)
    }
}

impl From<&AbstractBoost> for Boost {
    fn from(value: &AbstractBoost) -> Self {
        AbstractBoost::const_to_boost(value)
    }
}
