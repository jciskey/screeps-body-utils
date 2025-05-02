//! Reaction chain constants, for returning slices from const functions (since we can't create
//! dynamic arrays at compile time).
//!
//! All reaction chains produce [LAB_REACTION_AMOUNT] of their output product, and are ordered from
//! least complex (base compounds) to most complex (T3 compounds).
//!
//! For more details about the individual compounds and their reaction components, see the Mineral
//! Compounds table in the [official documentation](https://docs.screeps.com/resources.html#Creep-boosts).

use screeps::ResourceType;
use screeps::constants::LAB_REACTION_AMOUNT;
use super::reaction::Reaction;

// Base compounds

/// Reaction chain to produce [Hydroxide](ResourceType::Hydroxide) from base minerals.
///
/// ### Steps
/// - [Oxygen](ResourceType::Oxygen) + [Hydrogen](ResourceType::Hydrogen) -> [Hydroxide](ResourceType::Hydroxide)
pub const HYDROXIDE_REACTION_CHAIN: [Reaction; 1] = [Reaction::unchecked_new(ResourceType::Hydroxide, LAB_REACTION_AMOUNT)];

/// Reaction chain to produce [ZynthiumKeanite](ResourceType::ZynthiumKeanite) from base minerals.
///
/// ### Steps
/// - [Zynthium](ResourceType::Zynthium) + [Keanium](ResourceType::Keanium) -> [ZynthiumKeanite](ResourceType::ZynthiumKeanite)
pub const ZYNTHIUM_KEANITE_REACTION_CHAIN: [Reaction; 1] = [Reaction::unchecked_new(ResourceType::ZynthiumKeanite, LAB_REACTION_AMOUNT)];

/// Reaction chain to produce [UtriumLemergite](ResourceType::UtriumLemergite) from base minerals.
///
/// ### Steps
/// - [Utrium](ResourceType::Utrium) + [Lemergium](ResourceType::Lemergium) -> [UtriumLemergite](ResourceType::UtriumLemergite)
pub const UTRIUM_LEMERGITE_REACTION_CHAIN: [Reaction; 1] = [Reaction::unchecked_new(ResourceType::UtriumLemergite, LAB_REACTION_AMOUNT)];

/// Reaction chain to produce [Ghodium](ResourceType::Ghodium) from base minerals.
/// 
/// ### Steps
/// - [Zynthium](ResourceType::Zynthium) + [Keanium](ResourceType::Keanium) -> [ZynthiumKeanite](ResourceType::ZynthiumKeanite)
/// - [Utrium](ResourceType::Utrium) + [Lemergium](ResourceType::Lemergium) -> [UtriumLemergite](ResourceType::UtriumLemergite)
/// - [ZynthiumKeanite](ResourceType::ZynthiumKeanite) +
/// [UtriumLemergite](ResourceType::UtriumLemergite) -> [Ghodium](ResourceType::Ghodium)
pub const GHODIUM_REACTION_CHAIN: [Reaction; 3] = [
    ZYNTHIUM_KEANITE_REACTION_CHAIN[0], // ZK
    UTRIUM_LEMERGITE_REACTION_CHAIN[0], // UL
    Reaction::unchecked_new(ResourceType::Ghodium, LAB_REACTION_AMOUNT), // G
];

// T1 boost compounds

/// Reaction chain to produce [UtriumHydride](ResourceType::UtriumHydride) ([T1 Attack](crate::boost::boost::AbstractBoost::T1Attack)) from base minerals.
/// 
/// ### Steps
/// - [Utrium](ResourceType::Utrium) + [Hydrogen](ResourceType::Hydrogen) -> [UtriumHydride](ResourceType::UtriumHydride)
pub const UTRIUM_HYDRIDE_REACTION_CHAIN: [Reaction; 1] = [Reaction::unchecked_new(ResourceType::UtriumHydride, LAB_REACTION_AMOUNT)];

/// Reaction chain to produce [UtriumOxide](ResourceType::UtriumOxide) ([T1 Harvest](crate::boost::boost::AbstractBoost::T1Harvest)) from base minerals.
/// 
/// ### Steps
/// - [Utrium](ResourceType::Utrium) + [Oxygen](ResourceType::Oxygen) -> [UtriumOxide](ResourceType::UtriumOxide)
pub const UTRIUM_OXIDE_REACTION_CHAIN: [Reaction; 1] = [Reaction::unchecked_new(ResourceType::UtriumOxide, LAB_REACTION_AMOUNT)];

/// Reaction chain to produce [KeaniumHydride](ResourceType::KeaniumHydride) ([T1 Carry](crate::boost::boost::AbstractBoost::T1Carry)) from base minerals.
/// 
/// ### Steps
/// - [Keanium](ResourceType::Keanium) + [Hydrogen](ResourceType::Hydrogen) -> [KeaniumHydride](ResourceType::KeaniumHydride)
pub const KEANIUM_HYDRIDE_REACTION_CHAIN: [Reaction; 1] = [Reaction::unchecked_new(ResourceType::KeaniumHydride, LAB_REACTION_AMOUNT)];

/// Reaction chain to produce [KeaniumOxide](ResourceType::KeaniumOxide) ([T1 Ranged Attack](crate::boost::boost::AbstractBoost::T1RangedAttack)) from base minerals.
/// 
/// ### Steps
/// - [Keanium](ResourceType::Keanium) + [Oxygen](ResourceType::Oxygen) -> [KeaniumOxide](ResourceType::KeaniumOxide)
pub const KEANIUM_OXIDE_REACTION_CHAIN: [Reaction; 1] = [Reaction::unchecked_new(ResourceType::KeaniumOxide, LAB_REACTION_AMOUNT)];

/// Reaction chain to produce [LemergiumHydride](ResourceType::LemergiumHydride) ([T1 Build/Repair](crate::boost::boost::AbstractBoost::T1BuildRepair)) from base minerals.
/// 
/// ### Steps
/// - [Lemergium](ResourceType::Lemergium) + [Hydrogen](ResourceType::Hydrogen) -> [LemergiumHydride](ResourceType::LemergiumHydride)
pub const LEMERGIUM_HYDRIDE_REACTION_CHAIN: [Reaction; 1] = [Reaction::unchecked_new(ResourceType::LemergiumHydride, LAB_REACTION_AMOUNT)];

/// Reaction chain to produce [LemergiumOxide](ResourceType::LemergiumOxide) ([T1 Heal](crate::boost::boost::AbstractBoost::T1Heal)) from base minerals.
/// 
/// ### Steps
/// - [Lemergium](ResourceType::Lemergium) + [Oxygen](ResourceType::Oxygen) -> [LemergiumOxide](ResourceType::LemergiumOxide)
pub const LEMERGIUM_OXIDE_REACTION_CHAIN: [Reaction; 1] = [Reaction::unchecked_new(ResourceType::LemergiumOxide, LAB_REACTION_AMOUNT)];

/// Reaction chain to produce [ZynthiumHydride](ResourceType::ZynthiumHydride) ([T1 Dismantle](crate::boost::boost::AbstractBoost::T1Dismantle)) from base minerals.
/// 
/// ### Steps
/// - [Zynthium](ResourceType::Zynthium) + [Hydrogen](ResourceType::Hydrogen) -> [ZynthiumHydride](ResourceType::ZynthiumHydride)
pub const ZYNTHIUM_HYDRIDE_REACTION_CHAIN: [Reaction; 1] = [Reaction::unchecked_new(ResourceType::ZynthiumHydride, LAB_REACTION_AMOUNT)];

/// Reaction chain to produce [ZynthiumOxide](ResourceType::ZynthiumOxide) ([T1 Move](crate::boost::boost::AbstractBoost::T1Move)) from base minerals.
/// 
/// ### Steps
/// - [Zynthium](ResourceType::Zynthium) + [Oxygen](ResourceType::Oxygen) -> [ZynthiumOxide](ResourceType::ZynthiumOxide)
pub const ZYNTHIUM_OXIDE_REACTION_CHAIN: [Reaction; 1] = [Reaction::unchecked_new(ResourceType::ZynthiumOxide, LAB_REACTION_AMOUNT)];

/// Reaction chain to produce [GhodiumHydride](ResourceType::GhodiumHydride) ([T1 Upgrade Controller](crate::boost::boost::AbstractBoost::T1UpgradeController)) from base minerals.
///
/// ### Steps
/// - [Zynthium](ResourceType::Zynthium) + [Keanium](ResourceType::Keanium) -> [ZynthiumKeanite](ResourceType::ZynthiumKeanite)
/// - [Utrium](ResourceType::Utrium) + [Lemergium](ResourceType::Lemergium) -> [UtriumLemergite](ResourceType::UtriumLemergite)
/// - [ZynthiumKeanite](ResourceType::ZynthiumKeanite) +
/// [UtriumLemergite](ResourceType::UtriumLemergite) -> [Ghodium](ResourceType::Ghodium)
/// - [Ghodium](ResourceType::Ghodium) + [Hydrogen](ResourceType::Hydrogen) ->
/// [GhodiumHydride](ResourceType::GhodiumHydride)
pub const GHODIUM_HYDRIDE_REACTION_CHAIN: [Reaction; 4] = [
    GHODIUM_REACTION_CHAIN[0], // ZK
    GHODIUM_REACTION_CHAIN[1], // UL
    GHODIUM_REACTION_CHAIN[2], // G
    Reaction::unchecked_new(ResourceType::GhodiumHydride, LAB_REACTION_AMOUNT), // GH
];

/// Reaction chain to produce [GhodiumOxide](ResourceType::GhodiumOxide) ([T1 Tough](crate::boost::boost::AbstractBoost::T1Tough)) from base minerals.
///
/// ### Steps
/// - [Zynthium](ResourceType::Zynthium) + [Keanium](ResourceType::Keanium) -> [ZynthiumKeanite](ResourceType::ZynthiumKeanite)
/// - [Utrium](ResourceType::Utrium) + [Lemergium](ResourceType::Lemergium) -> [UtriumLemergite](ResourceType::UtriumLemergite)
/// - [ZynthiumKeanite](ResourceType::ZynthiumKeanite) +
/// [UtriumLemergite](ResourceType::UtriumLemergite) -> [Ghodium](ResourceType::Ghodium)
/// - [Ghodium](ResourceType::Ghodium) + [Oxygen](ResourceType::Oxygen) ->
/// [GhodiumOxide](ResourceType::GhodiumOxide)
pub const GHODIUM_OXIDE_REACTION_CHAIN: [Reaction; 4] = [
    GHODIUM_REACTION_CHAIN[0], // ZK
    GHODIUM_REACTION_CHAIN[1], // UL
    GHODIUM_REACTION_CHAIN[2], // G
    Reaction::unchecked_new(ResourceType::GhodiumOxide, LAB_REACTION_AMOUNT), // GO
];

// T2 boost compounds

/// Reaction chain to produce [UtriumAcid](ResourceType::UtriumAcid) ([T2 Attack](crate::boost::boost::AbstractBoost::T2Attack)) from base minerals.
/// 
/// ### Steps
/// - [Oxygen](ResourceType::Oxygen) + [Hydrogen](ResourceType::Hydrogen) -> [Hydroxide](ResourceType::Hydroxide)
/// - [Utrium](ResourceType::Utrium) + [Hydrogen](ResourceType::Hydrogen) -> [UtriumHydride](ResourceType::UtriumHydride)
/// - [UtriumHydride](ResourceType::UtriumHydride) + [Hydroxide](ResourceType::Hydroxide) -> [UtriumAcid](ResourceType::UtriumAcid)
pub const UTRIUM_ACID_REACTION_CHAIN: [Reaction; 3] = [
    HYDROXIDE_REACTION_CHAIN[0],
    UTRIUM_HYDRIDE_REACTION_CHAIN[0],
    Reaction::unchecked_new(ResourceType::UtriumAcid, LAB_REACTION_AMOUNT),
];

/// Reaction chain to produce [UtriumAlkalide](ResourceType::UtriumAlkalide) ([T2 Harvest](crate::boost::boost::AbstractBoost::T2Harvest)) from base minerals.
/// 
/// ### Steps
/// - [Oxygen](ResourceType::Oxygen) + [Hydrogen](ResourceType::Hydrogen) -> [Hydroxide](ResourceType::Hydroxide)
/// - [Utrium](ResourceType::Utrium) + [Oxygen](ResourceType::Oxygen) -> [UtriumOxide](ResourceType::UtriumOxide)
/// - [UtriumOxide](ResourceType::UtriumOxide) + [Hydroxide](ResourceType::Hydroxide) -> [UtriumAlkalide](ResourceType::UtriumAlkalide)
pub const UTRIUM_ALKALIDE_REACTION_CHAIN: [Reaction; 3] = [
    HYDROXIDE_REACTION_CHAIN[0],
    UTRIUM_OXIDE_REACTION_CHAIN[0],
    Reaction::unchecked_new(ResourceType::UtriumAlkalide, LAB_REACTION_AMOUNT),
];

/// Reaction chain to produce [KeaniumAcid](ResourceType::KeaniumAcid) ([T2 Carry](crate::boost::boost::AbstractBoost::T2Carry)) from base minerals.
/// 
/// ### Steps
/// - [Oxygen](ResourceType::Oxygen) + [Hydrogen](ResourceType::Hydrogen) -> [Hydroxide](ResourceType::Hydroxide)
/// - [Keanium](ResourceType::Keanium) + [Hydrogen](ResourceType::Hydrogen) -> [KeaniumHydride](ResourceType::KeaniumHydride)
/// - [KeaniumHydride](ResourceType::KeaniumHydride) + [Hydroxide](ResourceType::Hydroxide) -> [KeaniumAcid](ResourceType::KeaniumAcid)
pub const KEANIUM_ACID_REACTION_CHAIN: [Reaction; 3] = [
    HYDROXIDE_REACTION_CHAIN[0],
    KEANIUM_HYDRIDE_REACTION_CHAIN[0],
    Reaction::unchecked_new(ResourceType::KeaniumAcid, LAB_REACTION_AMOUNT),
];

/// Reaction chain to produce [KeaniumAlkalide](ResourceType::KeaniumAlkalide) ([T2 Ranged Attack](crate::boost::boost::AbstractBoost::T2RangedAttack)) from base minerals.
/// 
/// ### Steps
/// - [Oxygen](ResourceType::Oxygen) + [Hydrogen](ResourceType::Hydrogen) -> [Hydroxide](ResourceType::Hydroxide)
/// - [Keanium](ResourceType::Keanium) + [Oxygen](ResourceType::Oxygen) -> [KeaniumOxide](ResourceType::KeaniumOxide)
/// - [KeaniumOxide](ResourceType::KeaniumOxide) + [Hydroxide](ResourceType::Hydroxide) -> [KeaniumAlkalide](ResourceType::KeaniumAlkalide)
pub const KEANIUM_ALKALIDE_REACTION_CHAIN: [Reaction; 3] = [
    HYDROXIDE_REACTION_CHAIN[0],
    KEANIUM_OXIDE_REACTION_CHAIN[0],
    Reaction::unchecked_new(ResourceType::KeaniumAlkalide, LAB_REACTION_AMOUNT),
];

/// Reaction chain to produce [LemergiumAcid](ResourceType::LemergiumAcid) ([T2 Build/Repair](crate::boost::boost::AbstractBoost::T2BuildRepair)) from base minerals.
/// 
/// ### Steps
/// - [Oxygen](ResourceType::Oxygen) + [Hydrogen](ResourceType::Hydrogen) -> [Hydroxide](ResourceType::Hydroxide)
/// - [Lemergium](ResourceType::Lemergium) + [Hydrogen](ResourceType::Hydrogen) -> [LemergiumHydride](ResourceType::LemergiumHydride)
/// - [LemergiumHydride](ResourceType::LemergiumHydride) + [Hydroxide](ResourceType::Hydroxide) -> [LemergiumAcid](ResourceType::LemergiumAcid)
pub const LEMERGIUM_ACID_REACTION_CHAIN: [Reaction; 3] = [
    HYDROXIDE_REACTION_CHAIN[0],
    LEMERGIUM_HYDRIDE_REACTION_CHAIN[0],
    Reaction::unchecked_new(ResourceType::LemergiumAcid, LAB_REACTION_AMOUNT),
];

/// Reaction chain to produce [LemergiumAlkalide](ResourceType::LemergiumAlkalide) ([T2 Heal](crate::boost::boost::AbstractBoost::T2Heal)) from base minerals.
/// 
/// ### Steps
/// - [Oxygen](ResourceType::Oxygen) + [Hydrogen](ResourceType::Hydrogen) -> [Hydroxide](ResourceType::Hydroxide)
/// - [Lemergium](ResourceType::Lemergium) + [Oxygen](ResourceType::Oxygen) -> [LemergiumOxide](ResourceType::LemergiumOxide)
/// - [LemergiumOxide](ResourceType::LemergiumOxide) + [Hydroxide](ResourceType::Hydroxide) -> [LemergiumAlkalide](ResourceType::LemergiumAlkalide)
pub const LEMERGIUM_ALKALIDE_REACTION_CHAIN: [Reaction; 3] = [
    HYDROXIDE_REACTION_CHAIN[0],
    LEMERGIUM_OXIDE_REACTION_CHAIN[0],
    Reaction::unchecked_new(ResourceType::LemergiumAlkalide, LAB_REACTION_AMOUNT),
];

/// Reaction chain to produce [ZynthiumAcid](ResourceType::ZynthiumAcid) ([T2 Dismantle](crate::boost::boost::AbstractBoost::T2Dismantle)) from base minerals.
/// 
/// ### Steps
/// - [Oxygen](ResourceType::Oxygen) + [Hydrogen](ResourceType::Hydrogen) -> [Hydroxide](ResourceType::Hydroxide)
/// - [Zynthium](ResourceType::Zynthium) + [Hydrogen](ResourceType::Hydrogen) -> [ZynthiumHydride](ResourceType::ZynthiumHydride)
/// - [ZynthiumHydride](ResourceType::ZynthiumHydride) + [Hydroxide](ResourceType::Hydroxide) -> [ZynthiumAcid](ResourceType::ZynthiumAcid)
pub const ZYNTHIUM_ACID_REACTION_CHAIN: [Reaction; 3] = [
    HYDROXIDE_REACTION_CHAIN[0],
    ZYNTHIUM_HYDRIDE_REACTION_CHAIN[0],
    Reaction::unchecked_new(ResourceType::ZynthiumAcid, LAB_REACTION_AMOUNT),
];

/// Reaction chain to produce [ZynthiumAlkalide](ResourceType::ZynthiumAlkalide) ([T2 Move](crate::boost::boost::AbstractBoost::T2Move)) from base minerals.
/// 
/// ### Steps
/// - [Oxygen](ResourceType::Oxygen) + [Hydrogen](ResourceType::Hydrogen) -> [Hydroxide](ResourceType::Hydroxide)
/// - [Zynthium](ResourceType::Zynthium) + [Oxygen](ResourceType::Oxygen) -> [ZynthiumOxide](ResourceType::ZynthiumOxide)
/// - [ZynthiumOxide](ResourceType::ZynthiumOxide) + [Hydroxide](ResourceType::Hydroxide) -> [ZynthiumAlkalide](ResourceType::ZynthiumAlkalide)
pub const ZYNTHIUM_ALKALIDE_REACTION_CHAIN: [Reaction; 3] = [
    HYDROXIDE_REACTION_CHAIN[0],
    ZYNTHIUM_OXIDE_REACTION_CHAIN[0],
    Reaction::unchecked_new(ResourceType::ZynthiumAlkalide, LAB_REACTION_AMOUNT),
];

/// Reaction chain to produce [GhodiumAcid](ResourceType::GhodiumAcid) ([T2 Upgrade Controller](crate::boost::boost::AbstractBoost::T2UpgradeController)) from base minerals.
///
/// ### Steps
/// - [Oxygen](ResourceType::Oxygen) + [Hydrogen](ResourceType::Hydrogen) -> [Hydroxide](ResourceType::Hydroxide)
/// - [Zynthium](ResourceType::Zynthium) + [Keanium](ResourceType::Keanium) -> [ZynthiumKeanite](ResourceType::ZynthiumKeanite)
/// - [Utrium](ResourceType::Utrium) + [Lemergium](ResourceType::Lemergium) -> [UtriumLemergite](ResourceType::UtriumLemergite)
/// - [ZynthiumKeanite](ResourceType::ZynthiumKeanite) +
/// [UtriumLemergite](ResourceType::UtriumLemergite) -> [Ghodium](ResourceType::Ghodium)
/// - [Ghodium](ResourceType::Ghodium) + [Hydrogen](ResourceType::Hydrogen) ->
/// [GhodiumHydride](ResourceType::GhodiumHydride)
/// - [GhodiumHydride](ResourceType::GhodiumHydride) + [Hydroxide](ResourceType::Hydroxide) -> [GhodiumAcid](ResourceType::GhodiumAcid)
pub const GHODIUM_ACID_REACTION_CHAIN: [Reaction; 6] = [
    HYDROXIDE_REACTION_CHAIN[0], // OH
    GHODIUM_REACTION_CHAIN[0],   // ZK
    GHODIUM_REACTION_CHAIN[1],   // UL
    GHODIUM_REACTION_CHAIN[2],   // G
    GHODIUM_HYDRIDE_REACTION_CHAIN[0], // GH
    Reaction::unchecked_new(ResourceType::GhodiumAcid, LAB_REACTION_AMOUNT), // GH2O
];

/// Reaction chain to produce [GhodiumAlkalide](ResourceType::GhodiumAlkalide) ([T2 Tough](crate::boost::boost::AbstractBoost::T2Tough)) from base minerals.
///
/// ### Steps
/// - [Oxygen](ResourceType::Oxygen) + [Hydrogen](ResourceType::Hydrogen) -> [Hydroxide](ResourceType::Hydroxide)
/// - [Zynthium](ResourceType::Zynthium) + [Keanium](ResourceType::Keanium) -> [ZynthiumKeanite](ResourceType::ZynthiumKeanite)
/// - [Utrium](ResourceType::Utrium) + [Lemergium](ResourceType::Lemergium) -> [UtriumLemergite](ResourceType::UtriumLemergite)
/// - [ZynthiumKeanite](ResourceType::ZynthiumKeanite) +
/// [UtriumLemergite](ResourceType::UtriumLemergite) -> [Ghodium](ResourceType::Ghodium)
/// - [Ghodium](ResourceType::Ghodium) + [Oxygen](ResourceType::Oxygen) ->
/// [GhodiumOxide](ResourceType::GhodiumOxide)
/// - [GhodiumOxide](ResourceType::GhodiumOxide) + [Hydroxide](ResourceType::Hydroxide) -> [GhodiumAlkalide](ResourceType::GhodiumAlkalide)
pub const GHODIUM_ALKALIDE_REACTION_CHAIN: [Reaction; 6] = [
    HYDROXIDE_REACTION_CHAIN[0], // OH
    GHODIUM_REACTION_CHAIN[0],   // ZK
    GHODIUM_REACTION_CHAIN[1],   // UL
    GHODIUM_REACTION_CHAIN[2],   // G
    GHODIUM_OXIDE_REACTION_CHAIN[0], // GO
    Reaction::unchecked_new(ResourceType::GhodiumAlkalide, LAB_REACTION_AMOUNT), // GHO2
];

// T3 boost compounds

/// Reaction chain to produce [CatalyzedUtriumAcid](ResourceType::CatalyzedUtriumAcid) ([T3 Attack](crate::boost::boost::AbstractBoost::T3Attack)) from base minerals.
/// 
/// ### Steps
/// - [Oxygen](ResourceType::Oxygen) + [Hydrogen](ResourceType::Hydrogen) -> [Hydroxide](ResourceType::Hydroxide)
/// - [Utrium](ResourceType::Utrium) + [Hydrogen](ResourceType::Hydrogen) -> [UtriumHydride](ResourceType::UtriumHydride)
/// - [UtriumHydride](ResourceType::UtriumHydride) + [Hydroxide](ResourceType::Hydroxide) -> [UtriumAcid](ResourceType::UtriumAcid)
/// - [UtriumAcid](ResourceType::UtriumAcid) + [Catalyst](ResourceType::Catalyst) ->
/// [CatalyzedUtriumAcid](ResourceType::CatalyzedUtriumAcid)
pub const CATALYZED_UTRIUM_ACID_REACTION_CHAIN: [Reaction; 4] = [
    UTRIUM_ACID_REACTION_CHAIN[0],
    UTRIUM_ACID_REACTION_CHAIN[1],
    UTRIUM_ACID_REACTION_CHAIN[2],
    Reaction::unchecked_new(ResourceType::CatalyzedUtriumAcid, LAB_REACTION_AMOUNT),
];

/// Reaction chain to produce [CatalyzedUtriumAlkalide](ResourceType::CatalyzedUtriumAlkalide) ([T3 Harvest](crate::boost::boost::AbstractBoost::T3Harvest)) from base minerals.
/// 
/// ### Steps
/// - [Oxygen](ResourceType::Oxygen) + [Hydrogen](ResourceType::Hydrogen) -> [Hydroxide](ResourceType::Hydroxide)
/// - [Utrium](ResourceType::Utrium) + [Oxygen](ResourceType::Oxygen) -> [UtriumOxide](ResourceType::UtriumOxide)
/// - [UtriumOxide](ResourceType::UtriumOxide) + [Hydroxide](ResourceType::Hydroxide) -> [UtriumAlkalide](ResourceType::UtriumAlkalide)
/// - [UtriumAlkalide](ResourceType::UtriumAlkalide) + [Catalyst](ResourceType::Catalyst) ->
/// [CatalyzedUtriumAlkalide](ResourceType::CatalyzedUtriumAlkalide)
pub const CATALYZED_UTRIUM_ALKALIDE_REACTION_CHAIN: [Reaction; 4] = [
    UTRIUM_ALKALIDE_REACTION_CHAIN[0],
    UTRIUM_ALKALIDE_REACTION_CHAIN[1],
    UTRIUM_ALKALIDE_REACTION_CHAIN[2],
    Reaction::unchecked_new(ResourceType::CatalyzedUtriumAlkalide, LAB_REACTION_AMOUNT),
];

/// Reaction chain to produce [CatalyzedKeaniumAcid](ResourceType::CatalyzedKeaniumAcid) ([T3 Carry](crate::boost::boost::AbstractBoost::T3Carry)) from base minerals.
/// 
/// ### Steps
/// - [Oxygen](ResourceType::Oxygen) + [Hydrogen](ResourceType::Hydrogen) -> [Hydroxide](ResourceType::Hydroxide)
/// - [Keanium](ResourceType::Keanium) + [Hydrogen](ResourceType::Hydrogen) -> [KeaniumHydride](ResourceType::KeaniumHydride)
/// - [KeaniumHydride](ResourceType::KeaniumHydride) + [Hydroxide](ResourceType::Hydroxide) -> [KeaniumAcid](ResourceType::KeaniumAcid)
/// - [KeaniumAcid](ResourceType::KeaniumAcid) + [Catalyst](ResourceType::Catalyst) ->
/// [CatalyzedKeaniumAcid](ResourceType::CatalyzedKeaniumAcid)
pub const CATALYZED_KEANIUM_ACID_REACTION_CHAIN: [Reaction; 4] = [
    KEANIUM_ACID_REACTION_CHAIN[0],
    KEANIUM_ACID_REACTION_CHAIN[1],
    KEANIUM_ACID_REACTION_CHAIN[2],
    Reaction::unchecked_new(ResourceType::CatalyzedKeaniumAcid, LAB_REACTION_AMOUNT),
];

/// Reaction chain to produce [CatalyzedKeaniumAlkalide](ResourceType::CatalyzedKeaniumAlkalide) ([T3 Ranged Attack](crate::boost::boost::AbstractBoost::T3RangedAttack)) from base minerals.
/// 
/// ### Steps
/// - [Oxygen](ResourceType::Oxygen) + [Hydrogen](ResourceType::Hydrogen) -> [Hydroxide](ResourceType::Hydroxide)
/// - [Keanium](ResourceType::Keanium) + [Oxygen](ResourceType::Oxygen) -> [KeaniumOxide](ResourceType::KeaniumOxide)
/// - [KeaniumOxide](ResourceType::KeaniumOxide) + [Hydroxide](ResourceType::Hydroxide) -> [KeaniumAlkalide](ResourceType::KeaniumAlkalide)
/// - [KeaniumAlkalide](ResourceType::KeaniumAlkalide) + [Catalyst](ResourceType::Catalyst) ->
/// [CatalyzedKeaniumAlkalide](ResourceType::CatalyzedKeaniumAlkalide)
pub const CATALYZED_KEANIUM_ALKALIDE_REACTION_CHAIN: [Reaction; 4] = [
    KEANIUM_ALKALIDE_REACTION_CHAIN[0],
    KEANIUM_ALKALIDE_REACTION_CHAIN[1],
    KEANIUM_ALKALIDE_REACTION_CHAIN[2],
    Reaction::unchecked_new(ResourceType::CatalyzedKeaniumAlkalide, LAB_REACTION_AMOUNT),
];

/// Reaction chain to produce [CatalyzedLemergiumAcid](ResourceType::CatalyzedLemergiumAcid) ([T3 Build/Repair](crate::boost::boost::AbstractBoost::T3BuildRepair)) from base minerals.
/// 
/// ### Steps
/// - [Oxygen](ResourceType::Oxygen) + [Hydrogen](ResourceType::Hydrogen) -> [Hydroxide](ResourceType::Hydroxide)
/// - [Lemergium](ResourceType::Lemergium) + [Hydrogen](ResourceType::Hydrogen) -> [LemergiumHydride](ResourceType::LemergiumHydride)
/// - [LemergiumHydride](ResourceType::LemergiumHydride) + [Hydroxide](ResourceType::Hydroxide) -> [LemergiumAcid](ResourceType::LemergiumAcid)
/// - [LemergiumAcid](ResourceType::LemergiumAcid) + [Catalyst](ResourceType::Catalyst) ->
/// [CatalyzedLemergiumAcid](ResourceType::CatalyzedLemergiumAcid)
pub const CATALYZED_LEMERGIUM_ACID_REACTION_CHAIN: [Reaction; 4] = [
    LEMERGIUM_ACID_REACTION_CHAIN[0],
    LEMERGIUM_ACID_REACTION_CHAIN[1],
    LEMERGIUM_ACID_REACTION_CHAIN[2],
    Reaction::unchecked_new(ResourceType::CatalyzedLemergiumAcid, LAB_REACTION_AMOUNT),
];

/// Reaction chain to produce [CatalyzedLemergiumAlkalide](ResourceType::CatalyzedLemergiumAlkalide) ([T3 Heal](crate::boost::boost::AbstractBoost::T3Heal)) from base minerals.
/// 
/// ### Steps
/// - [Oxygen](ResourceType::Oxygen) + [Hydrogen](ResourceType::Hydrogen) -> [Hydroxide](ResourceType::Hydroxide)
/// - [Lemergium](ResourceType::Lemergium) + [Oxygen](ResourceType::Oxygen) -> [LemergiumOxide](ResourceType::LemergiumOxide)
/// - [LemergiumOxide](ResourceType::LemergiumOxide) + [Hydroxide](ResourceType::Hydroxide) -> [LemergiumAlkalide](ResourceType::LemergiumAlkalide)
/// - [LemergiumAlkalide](ResourceType::LemergiumAlkalide) + [Catalyst](ResourceType::Catalyst) ->
/// [CatalyzedLemergiumAlkalide](ResourceType::CatalyzedLemergiumAlkalide)
pub const CATALYZED_LEMERGIUM_ALKALIDE_REACTION_CHAIN: [Reaction; 4] = [
    LEMERGIUM_ALKALIDE_REACTION_CHAIN[0],
    LEMERGIUM_ALKALIDE_REACTION_CHAIN[1],
    LEMERGIUM_ALKALIDE_REACTION_CHAIN[2],
    Reaction::unchecked_new(ResourceType::CatalyzedLemergiumAlkalide, LAB_REACTION_AMOUNT),
];

/// Reaction chain to produce [CatalyzedZynthiumAcid](ResourceType::CatalyzedZynthiumAcid) ([T3 Dismantle](crate::boost::boost::AbstractBoost::T3Dismantle)) from base minerals.
/// 
/// ### Steps
/// - [Oxygen](ResourceType::Oxygen) + [Hydrogen](ResourceType::Hydrogen) -> [Hydroxide](ResourceType::Hydroxide)
/// - [Zynthium](ResourceType::Zynthium) + [Hydrogen](ResourceType::Hydrogen) -> [ZynthiumHydride](ResourceType::ZynthiumHydride)
/// - [ZynthiumHydride](ResourceType::ZynthiumHydride) + [Hydroxide](ResourceType::Hydroxide) -> [ZynthiumAcid](ResourceType::ZynthiumAcid)
/// - [ZynthiumAcid](ResourceType::ZynthiumAcid) + [Catalyst](ResourceType::Catalyst) ->
/// [CatalyzedZynthiumAcid](ResourceType::CatalyzedZynthiumAcid)
pub const CATALYZED_ZYNTHIUM_ACID_REACTION_CHAIN: [Reaction; 4] = [
    ZYNTHIUM_ACID_REACTION_CHAIN[0],
    ZYNTHIUM_ACID_REACTION_CHAIN[1],
    ZYNTHIUM_ACID_REACTION_CHAIN[2],
    Reaction::unchecked_new(ResourceType::CatalyzedZynthiumAcid, LAB_REACTION_AMOUNT),
];

/// Reaction chain to produce [CatalyzedZynthiumAlkalide](ResourceType::CatalyzedZynthiumAlkalide) ([T3 Move](crate::boost::boost::AbstractBoost::T3Move)) from base minerals.
/// 
/// ### Steps
/// - [Oxygen](ResourceType::Oxygen) + [Hydrogen](ResourceType::Hydrogen) -> [Hydroxide](ResourceType::Hydroxide)
/// - [Zynthium](ResourceType::Zynthium) + [Oxygen](ResourceType::Oxygen) -> [ZynthiumOxide](ResourceType::ZynthiumOxide)
/// - [ZynthiumOxide](ResourceType::ZynthiumOxide) + [Hydroxide](ResourceType::Hydroxide) -> [ZynthiumAlkalide](ResourceType::ZynthiumAlkalide)
/// - [ZynthiumAlkalide](ResourceType::ZynthiumAlkalide) + [Catalyst](ResourceType::Catalyst) ->
/// [CatalyzedZynthiumAlkalide](ResourceType::CatalyzedZynthiumAlkalide)
pub const CATALYZED_ZYNTHIUM_ALKALIDE_REACTION_CHAIN: [Reaction; 4] = [
    ZYNTHIUM_ALKALIDE_REACTION_CHAIN[0],
    ZYNTHIUM_ALKALIDE_REACTION_CHAIN[1],
    ZYNTHIUM_ALKALIDE_REACTION_CHAIN[2],
    Reaction::unchecked_new(ResourceType::CatalyzedZynthiumAlkalide, LAB_REACTION_AMOUNT),
];

/// Reaction chain to produce [CatalyzedGhodiumAcid](ResourceType::CatalyzedGhodiumAcid) ([T3 Upgrade Controller](crate::boost::boost::AbstractBoost::T3UpgradeController)) from base minerals.
///
/// ### Steps
/// - [Oxygen](ResourceType::Oxygen) + [Hydrogen](ResourceType::Hydrogen) -> [Hydroxide](ResourceType::Hydroxide)
/// - [Zynthium](ResourceType::Zynthium) + [Keanium](ResourceType::Keanium) -> [ZynthiumKeanite](ResourceType::ZynthiumKeanite)
/// - [Utrium](ResourceType::Utrium) + [Lemergium](ResourceType::Lemergium) -> [UtriumLemergite](ResourceType::UtriumLemergite)
/// - [ZynthiumKeanite](ResourceType::ZynthiumKeanite) +
/// [UtriumLemergite](ResourceType::UtriumLemergite) -> [Ghodium](ResourceType::Ghodium)
/// - [Ghodium](ResourceType::Ghodium) + [Hydrogen](ResourceType::Hydrogen) ->
/// [GhodiumHydride](ResourceType::GhodiumHydride)
/// - [GhodiumHydride](ResourceType::GhodiumHydride) + [Hydroxide](ResourceType::Hydroxide) -> [GhodiumAcid](ResourceType::GhodiumAcid)
/// - [GhodiumAcid](ResourceType::GhodiumAcid) + [Catalyst](ResourceType::Catalyst) ->
/// [CatalyzedGhodiumAcid](ResourceType::CatalyzedGhodiumAcid)
pub const CATALYZED_GHODIUM_ACID_REACTION_CHAIN: [Reaction; 7] = [
    GHODIUM_ACID_REACTION_CHAIN[0], // OH
    GHODIUM_ACID_REACTION_CHAIN[1], // ZK
    GHODIUM_ACID_REACTION_CHAIN[2], // UL
    GHODIUM_ACID_REACTION_CHAIN[3], // G
    GHODIUM_ACID_REACTION_CHAIN[4], // GH
    GHODIUM_ACID_REACTION_CHAIN[5], // GH2O
    Reaction::unchecked_new(ResourceType::CatalyzedGhodiumAcid, LAB_REACTION_AMOUNT), // XGH2O
];

/// Reaction chain to produce [CatalyzedGhodiumAlkalide](ResourceType::CatalyzedGhodiumAlkalide) ([T3 Tough](crate::boost::boost::AbstractBoost::T3Tough)) from base minerals.
///
/// ### Steps
/// - [Oxygen](ResourceType::Oxygen) + [Hydrogen](ResourceType::Hydrogen) -> [Hydroxide](ResourceType::Hydroxide)
/// - [Zynthium](ResourceType::Zynthium) + [Keanium](ResourceType::Keanium) -> [ZynthiumKeanite](ResourceType::ZynthiumKeanite)
/// - [Utrium](ResourceType::Utrium) + [Lemergium](ResourceType::Lemergium) -> [UtriumLemergite](ResourceType::UtriumLemergite)
/// - [ZynthiumKeanite](ResourceType::ZynthiumKeanite) +
/// [UtriumLemergite](ResourceType::UtriumLemergite) -> [Ghodium](ResourceType::Ghodium)
/// - [Ghodium](ResourceType::Ghodium) + [Oxygen](ResourceType::Oxygen) ->
/// [GhodiumOxide](ResourceType::GhodiumOxide)
/// - [GhodiumOxide](ResourceType::GhodiumOxide) + [Hydroxide](ResourceType::Hydroxide) -> [GhodiumAlkalide](ResourceType::GhodiumAlkalide)
/// - [GhodiumAlkalide](ResourceType::GhodiumAlkalide) + [Catalyst](ResourceType::Catalyst) ->
/// [CatalyzedGhodiumAlkalide](ResourceType::CatalyzedGhodiumAlkalide)
pub const CATALYZED_GHODIUM_ALKALIDE_REACTION_CHAIN: [Reaction; 7] = [
    GHODIUM_ALKALIDE_REACTION_CHAIN[0], // OH
    GHODIUM_ALKALIDE_REACTION_CHAIN[1], // ZK
    GHODIUM_ALKALIDE_REACTION_CHAIN[2], // UL
    GHODIUM_ALKALIDE_REACTION_CHAIN[3], // G
    GHODIUM_ALKALIDE_REACTION_CHAIN[4], // GO
    GHODIUM_ALKALIDE_REACTION_CHAIN[5], // GHO2
    Reaction::unchecked_new(ResourceType::CatalyzedGhodiumAlkalide, LAB_REACTION_AMOUNT), // XGHO2
];

/// Helper method for getting the reaction chain slice for a particular resource type
pub const fn get_reaction_chain_for_resource(resource: &ResourceType) -> &'static [Reaction] {
    use ResourceType::*;
    match resource {
        // Base compounds
        Hydroxide => &HYDROXIDE_REACTION_CHAIN,
        ZynthiumKeanite => &ZYNTHIUM_KEANITE_REACTION_CHAIN,
        UtriumLemergite => &UTRIUM_LEMERGITE_REACTION_CHAIN,
        Ghodium => &GHODIUM_REACTION_CHAIN,

        // T1 compounds
        UtriumHydride => &UTRIUM_HYDRIDE_REACTION_CHAIN,
        UtriumOxide => &UTRIUM_OXIDE_REACTION_CHAIN,
        KeaniumHydride => &KEANIUM_HYDRIDE_REACTION_CHAIN,
        KeaniumOxide => &KEANIUM_OXIDE_REACTION_CHAIN,
        LemergiumHydride => &LEMERGIUM_HYDRIDE_REACTION_CHAIN,
        LemergiumOxide => &LEMERGIUM_OXIDE_REACTION_CHAIN,
        ZynthiumHydride => &ZYNTHIUM_HYDRIDE_REACTION_CHAIN,
        ZynthiumOxide => &ZYNTHIUM_OXIDE_REACTION_CHAIN,
        GhodiumHydride => &GHODIUM_HYDRIDE_REACTION_CHAIN,
        GhodiumOxide => &GHODIUM_OXIDE_REACTION_CHAIN,

        // T2 compounds
        UtriumAcid => &UTRIUM_ACID_REACTION_CHAIN,
        UtriumAlkalide => &UTRIUM_ALKALIDE_REACTION_CHAIN,
        KeaniumAcid => &KEANIUM_ACID_REACTION_CHAIN,
        KeaniumAlkalide => &KEANIUM_ALKALIDE_REACTION_CHAIN,
        LemergiumAcid => &LEMERGIUM_ACID_REACTION_CHAIN,
        LemergiumAlkalide => &LEMERGIUM_ALKALIDE_REACTION_CHAIN,
        ZynthiumAcid => &ZYNTHIUM_ACID_REACTION_CHAIN,
        ZynthiumAlkalide => &ZYNTHIUM_ALKALIDE_REACTION_CHAIN,
        GhodiumAcid => &GHODIUM_ACID_REACTION_CHAIN,
        GhodiumAlkalide => &GHODIUM_ALKALIDE_REACTION_CHAIN,

        // T3 compounds
        CatalyzedUtriumAcid => &CATALYZED_UTRIUM_ACID_REACTION_CHAIN,
        CatalyzedUtriumAlkalide => &CATALYZED_UTRIUM_ALKALIDE_REACTION_CHAIN,
        CatalyzedKeaniumAcid => &CATALYZED_KEANIUM_ACID_REACTION_CHAIN,
        CatalyzedKeaniumAlkalide => &CATALYZED_KEANIUM_ALKALIDE_REACTION_CHAIN,
        CatalyzedLemergiumAcid => &CATALYZED_LEMERGIUM_ACID_REACTION_CHAIN,
        CatalyzedLemergiumAlkalide => &CATALYZED_LEMERGIUM_ALKALIDE_REACTION_CHAIN,
        CatalyzedZynthiumAcid => &CATALYZED_ZYNTHIUM_ACID_REACTION_CHAIN,
        CatalyzedZynthiumAlkalide => &CATALYZED_ZYNTHIUM_ALKALIDE_REACTION_CHAIN,
        CatalyzedGhodiumAcid => &CATALYZED_GHODIUM_ACID_REACTION_CHAIN,
        CatalyzedGhodiumAlkalide => &CATALYZED_GHODIUM_ALKALIDE_REACTION_CHAIN,
        
        // Catch-all
        _ => &[],
    }
}
