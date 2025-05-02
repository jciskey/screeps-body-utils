//! Defines constants separating resources into categories.

use screeps::constants::ResourceType;

/// The 7 harvestable minerals that form the base of reactions.
/// 
/// [Hydrogen](ResourceType::Hydrogen)
/// 
/// [Oxygen](ResourceType::Oxygen)
/// 
/// [Utrium](ResourceType::Utrium)
/// 
/// [Lemergium](ResourceType::Lemergium)
/// 
/// [Keanium](ResourceType::Keanium)
/// 
/// [Zynthium](ResourceType::Zynthium)
/// 
/// [Catalyst](ResourceType::Catalyst)
pub const BASE_MINERALS: [ResourceType; 7] = [
    ResourceType::Hydrogen,
    ResourceType::Oxygen,
    ResourceType::Utrium,
    ResourceType::Lemergium,
    ResourceType::Keanium,
    ResourceType::Zynthium,
    ResourceType::Catalyst,
];

/// The 4 initial compounds used for boost reactions.
///
/// [Hydroxide](ResourceType::Hydroxide)
///
/// [ZynthiumKeanite](ResourceType::ZynthiumKeanite)
///
/// [UtriumLemergite](ResourceType::UtriumLemergite)
///
/// [Ghodium](ResourceType::Ghodium)
pub const BASE_COMPOUNDS: [ResourceType; 4] = [
    ResourceType::Hydroxide,
    ResourceType::ZynthiumKeanite,
    ResourceType::UtriumLemergite,
    ResourceType::Ghodium,
];

/// The 10 compounds that comprise Tier 1 boosts.
///
/// [UtriumHydride](ResourceType::UtriumHydride)
///
/// [UtriumOxide](ResourceType::UtriumOxide)
///
/// [KeaniumHydride](ResourceType::KeaniumHydride)
///
/// [KeaniumOxide](ResourceType::KeaniumOxide)
///
/// [LemergiumHydride](ResourceType::LemergiumHydride)
///
/// [LemergiumOxide](ResourceType::LemergiumOxide)
///
/// [ZynthiumHydride](ResourceType::ZynthiumHydride)
///
/// [ZynthiumOxide](ResourceType::ZynthiumOxide)
///
/// [GhodiumHydride](ResourceType::GhodiumHydride)
///
/// [GhodiumOxide](ResourceType::GhodiumOxide)
pub const T1_RESOURCES: [ResourceType; 10] = [
    ResourceType::UtriumHydride,
    ResourceType::UtriumOxide,
    ResourceType::KeaniumHydride,
    ResourceType::KeaniumOxide,
    ResourceType::LemergiumHydride,
    ResourceType::LemergiumOxide,
    ResourceType::ZynthiumHydride,
    ResourceType::ZynthiumOxide,
    ResourceType::GhodiumHydride,
    ResourceType::GhodiumOxide,
];

/// The 10 compounds that comprise Tier 2 boosts.
///
/// [UtriumAcid](ResourceType::UtriumAcid)
///
/// [UtriumAlkalide](ResourceType::UtriumAlkalide)
///
/// [KeaniumAcid](ResourceType::KeaniumAcid)
///
/// [KeaniumAlkalide](ResourceType::KeaniumAlkalide)
///
/// [LemergiumAcid](ResourceType::LemergiumAcid)
///
/// [LemergiumAlkalide](ResourceType::LemergiumAlkalide)
///
/// [ZynthiumAcid](ResourceType::ZynthiumAcid)
///
/// [ZynthiumAlkalide](ResourceType::ZynthiumAlkalide)
///
/// [GhodiumAcid](ResourceType::GhodiumAcid)
///
/// [GhodiumAlkalide](ResourceType::GhodiumAlkalide)
pub const T2_RESOURCES: [ResourceType; 10] = [
    ResourceType::UtriumAcid,
    ResourceType::UtriumAlkalide,
    ResourceType::KeaniumAcid,
    ResourceType::KeaniumAlkalide,
    ResourceType::LemergiumAcid,
    ResourceType::LemergiumAlkalide,
    ResourceType::ZynthiumAcid,
    ResourceType::ZynthiumAlkalide,
    ResourceType::GhodiumAcid,
    ResourceType::GhodiumAlkalide,
];

/// The 10 compounds that comprise Tier 3 boosts.
///
/// [CatalyzedUtriumAcid](ResourceType::CatalyzedUtriumAcid)
///
/// [CatalyzedUtriumAlkalide](ResourceType::CatalyzedUtriumAlkalide)
///
/// [CatalyzedKeaniumAcid](ResourceType::CatalyzedKeaniumAcid)
///
/// [CatalyzedKeaniumAlkalide](ResourceType::CatalyzedKeaniumAlkalide)
///
/// [CatalyzedLemergiumAcid](ResourceType::CatalyzedLemergiumAcid)
///
/// [CatalyzedLemergiumAlkalide](ResourceType::CatalyzedLemergiumAlkalide)
///
/// [CatalyzedZynthiumAcid](ResourceType::CatalyzedZynthiumAcid)
///
/// [CatalyzedZynthiumAlkalide](ResourceType::CatalyzedZynthiumAlkalide)
///
/// [CatalyzedGhodiumAcid](ResourceType::CatalyzedGhodiumAcid)
///
/// [CatalyzedGhodiumAlkalide](ResourceType::CatalyzedGhodiumAlkalide)
pub const T3_RESOURCES: [ResourceType; 10] = [
    ResourceType::CatalyzedUtriumAcid,
    ResourceType::CatalyzedUtriumAlkalide,
    ResourceType::CatalyzedKeaniumAcid,
    ResourceType::CatalyzedKeaniumAlkalide,
    ResourceType::CatalyzedLemergiumAcid,
    ResourceType::CatalyzedLemergiumAlkalide,
    ResourceType::CatalyzedZynthiumAcid,
    ResourceType::CatalyzedZynthiumAlkalide,
    ResourceType::CatalyzedGhodiumAcid,
    ResourceType::CatalyzedGhodiumAlkalide,
];

/// The 34 compounds that can be produced by a [run_reaction](screeps::objects::StructureLab::run_reaction).
///
/// ## Base Compounds
/// [Hydroxide](ResourceType::Hydroxide)
///
/// [ZynthiumKeanite](ResourceType::ZynthiumKeanite)
///
/// [UtriumLemergite](ResourceType::UtriumLemergite)
///
/// [Ghodium](ResourceType::Ghodium)
///
/// ## Tier 1 Boost Compounds
/// [UtriumHydride](ResourceType::UtriumHydride)
///
/// [UtriumOxide](ResourceType::UtriumOxide)
///
/// [KeaniumHydride](ResourceType::KeaniumHydride)
///
/// [KeaniumOxide](ResourceType::KeaniumOxide)
///
/// [LemergiumHydride](ResourceType::LemergiumHydride)
///
/// [LemergiumOxide](ResourceType::LemergiumOxide)
///
/// [ZynthiumHydride](ResourceType::ZynthiumHydride)
///
/// [ZynthiumOxide](ResourceType::ZynthiumOxide)
///
/// [GhodiumHydride](ResourceType::GhodiumHydride)
///
/// [GhodiumOxide](ResourceType::GhodiumOxide)
///
/// ## Tier 2 Boost Compounds
/// [UtriumAcid](ResourceType::UtriumAcid)
///
/// [UtriumAlkalide](ResourceType::UtriumAlkalide)
///
/// [KeaniumAcid](ResourceType::KeaniumAcid)
///
/// [KeaniumAlkalide](ResourceType::KeaniumAlkalide)
///
/// [LemergiumAcid](ResourceType::LemergiumAcid)
///
/// [LemergiumAlkalide](ResourceType::LemergiumAlkalide)
///
/// [ZynthiumAcid](ResourceType::ZynthiumAcid)
///
/// [ZynthiumAlkalide](ResourceType::ZynthiumAlkalide)
///
/// [GhodiumAcid](ResourceType::GhodiumAcid)
///
/// [GhodiumAlkalide](ResourceType::GhodiumAlkalide)
///
/// ## Tier 3 Boost Compounds
/// [CatalyzedUtriumAcid](ResourceType::CatalyzedUtriumAcid)
///
/// [CatalyzedUtriumAlkalide](ResourceType::CatalyzedUtriumAlkalide)
///
/// [CatalyzedKeaniumAcid](ResourceType::CatalyzedKeaniumAcid)
///
/// [CatalyzedKeaniumAlkalide](ResourceType::CatalyzedKeaniumAlkalide)
///
/// [CatalyzedLemergiumAcid](ResourceType::CatalyzedLemergiumAcid)
///
/// [CatalyzedLemergiumAlkalide](ResourceType::CatalyzedLemergiumAlkalide)
///
/// [CatalyzedZynthiumAcid](ResourceType::CatalyzedZynthiumAcid)
///
/// [CatalyzedZynthiumAlkalide](ResourceType::CatalyzedZynthiumAlkalide)
///
/// [CatalyzedGhodiumAcid](ResourceType::CatalyzedGhodiumAcid)
///
/// [CatalyzedGhodiumAlkalide](ResourceType::CatalyzedGhodiumAlkalide)
pub const LAB_OUTPUT_RESOURCES: [ResourceType; 34] = [
    ResourceType::Hydroxide,
    ResourceType::ZynthiumKeanite,
    ResourceType::UtriumLemergite,
    ResourceType::Ghodium,

    ResourceType::UtriumHydride,
    ResourceType::UtriumOxide,
    ResourceType::KeaniumHydride,
    ResourceType::KeaniumOxide,
    ResourceType::LemergiumHydride,
    ResourceType::LemergiumOxide,
    ResourceType::ZynthiumHydride,
    ResourceType::ZynthiumOxide,
    ResourceType::GhodiumHydride,
    ResourceType::GhodiumOxide,

    ResourceType::UtriumAcid,
    ResourceType::UtriumAlkalide,
    ResourceType::KeaniumAcid,
    ResourceType::KeaniumAlkalide,
    ResourceType::LemergiumAcid,
    ResourceType::LemergiumAlkalide,
    ResourceType::ZynthiumAcid,
    ResourceType::ZynthiumAlkalide,
    ResourceType::GhodiumAcid,
    ResourceType::GhodiumAlkalide,

    ResourceType::CatalyzedUtriumAcid,
    ResourceType::CatalyzedUtriumAlkalide,
    ResourceType::CatalyzedKeaniumAcid,
    ResourceType::CatalyzedKeaniumAlkalide,
    ResourceType::CatalyzedLemergiumAcid,
    ResourceType::CatalyzedLemergiumAlkalide,
    ResourceType::CatalyzedZynthiumAcid,
    ResourceType::CatalyzedZynthiumAlkalide,
    ResourceType::CatalyzedGhodiumAcid,
    ResourceType::CatalyzedGhodiumAlkalide,
];


/// The 41 compounds that can be used in or produced by a [run_reaction](screeps::objects::StructureLab::run_reaction).
///
/// ## Base Minerals
/// [Hydrogen](ResourceType::Hydrogen)
/// 
/// [Oxygen](ResourceType::Oxygen)
/// 
/// [Utrium](ResourceType::Utrium)
/// 
/// [Lemergium](ResourceType::Lemergium)
/// 
/// [Keanium](ResourceType::Keanium)
/// 
/// [Zynthium](ResourceType::Zynthium)
/// 
/// [Catalyst](ResourceType::Catalyst)
///
/// ## Base Compounds
/// [Hydroxide](ResourceType::Hydroxide)
///
/// [ZynthiumKeanite](ResourceType::ZynthiumKeanite)
///
/// [UtriumLemergite](ResourceType::UtriumLemergite)
///
/// [Ghodium](ResourceType::Ghodium)
///
/// ## Tier 1 Boost Compounds
/// [UtriumHydride](ResourceType::UtriumHydride)
///
/// [UtriumOxide](ResourceType::UtriumOxide)
///
/// [KeaniumHydride](ResourceType::KeaniumHydride)
///
/// [KeaniumOxide](ResourceType::KeaniumOxide)
///
/// [LemergiumHydride](ResourceType::LemergiumHydride)
///
/// [LemergiumOxide](ResourceType::LemergiumOxide)
///
/// [ZynthiumHydride](ResourceType::ZynthiumHydride)
///
/// [ZynthiumOxide](ResourceType::ZynthiumOxide)
///
/// [GhodiumHydride](ResourceType::GhodiumHydride)
///
/// [GhodiumOxide](ResourceType::GhodiumOxide)
///
/// ## Tier 2 Boost Compounds
/// [UtriumAcid](ResourceType::UtriumAcid)
///
/// [UtriumAlkalide](ResourceType::UtriumAlkalide)
///
/// [KeaniumAcid](ResourceType::KeaniumAcid)
///
/// [KeaniumAlkalide](ResourceType::KeaniumAlkalide)
///
/// [LemergiumAcid](ResourceType::LemergiumAcid)
///
/// [LemergiumAlkalide](ResourceType::LemergiumAlkalide)
///
/// [ZynthiumAcid](ResourceType::ZynthiumAcid)
///
/// [ZynthiumAlkalide](ResourceType::ZynthiumAlkalide)
///
/// [GhodiumAcid](ResourceType::GhodiumAcid)
///
/// [GhodiumAlkalide](ResourceType::GhodiumAlkalide)
///
/// ## Tier 3 Boost Compounds
/// [CatalyzedUtriumAcid](ResourceType::CatalyzedUtriumAcid)
///
/// [CatalyzedUtriumAlkalide](ResourceType::CatalyzedUtriumAlkalide)
///
/// [CatalyzedKeaniumAcid](ResourceType::CatalyzedKeaniumAcid)
///
/// [CatalyzedKeaniumAlkalide](ResourceType::CatalyzedKeaniumAlkalide)
///
/// [CatalyzedLemergiumAcid](ResourceType::CatalyzedLemergiumAcid)
///
/// [CatalyzedLemergiumAlkalide](ResourceType::CatalyzedLemergiumAlkalide)
///
/// [CatalyzedZynthiumAcid](ResourceType::CatalyzedZynthiumAcid)
///
/// [CatalyzedZynthiumAlkalide](ResourceType::CatalyzedZynthiumAlkalide)
///
/// [CatalyzedGhodiumAcid](ResourceType::CatalyzedGhodiumAcid)
///
/// [CatalyzedGhodiumAlkalide](ResourceType::CatalyzedGhodiumAlkalide)
pub const LAB_RESOURCES: [ResourceType; 41] = [
    ResourceType::Hydrogen,
    ResourceType::Oxygen,
    ResourceType::Utrium,
    ResourceType::Lemergium,
    ResourceType::Keanium,
    ResourceType::Zynthium,
    ResourceType::Catalyst,
    
    ResourceType::Hydroxide,
    ResourceType::ZynthiumKeanite,
    ResourceType::UtriumLemergite,
    ResourceType::Ghodium,

    ResourceType::UtriumHydride,
    ResourceType::UtriumOxide,
    ResourceType::KeaniumHydride,
    ResourceType::KeaniumOxide,
    ResourceType::LemergiumHydride,
    ResourceType::LemergiumOxide,
    ResourceType::ZynthiumHydride,
    ResourceType::ZynthiumOxide,
    ResourceType::GhodiumHydride,
    ResourceType::GhodiumOxide,

    ResourceType::UtriumAcid,
    ResourceType::UtriumAlkalide,
    ResourceType::KeaniumAcid,
    ResourceType::KeaniumAlkalide,
    ResourceType::LemergiumAcid,
    ResourceType::LemergiumAlkalide,
    ResourceType::ZynthiumAcid,
    ResourceType::ZynthiumAlkalide,
    ResourceType::GhodiumAcid,
    ResourceType::GhodiumAlkalide,

    ResourceType::CatalyzedUtriumAcid,
    ResourceType::CatalyzedUtriumAlkalide,
    ResourceType::CatalyzedKeaniumAcid,
    ResourceType::CatalyzedKeaniumAlkalide,
    ResourceType::CatalyzedLemergiumAcid,
    ResourceType::CatalyzedLemergiumAlkalide,
    ResourceType::CatalyzedZynthiumAcid,
    ResourceType::CatalyzedZynthiumAlkalide,
    ResourceType::CatalyzedGhodiumAcid,
    ResourceType::CatalyzedGhodiumAlkalide,
];
