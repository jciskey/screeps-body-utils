use enum_iterator::all;

use screeps::Boost;

use super::*;


/// Hand-coded matching data for the AbstractBoost <-> Boost conversions.
const abstractboost_boost_conversion_data: [(Boost, AbstractBoost); 30] = [
    (Boost::Attack(T1_ATTACK_MULTIPLIER), AbstractBoost::T1Attack),
    (Boost::Attack(T2_ATTACK_MULTIPLIER), AbstractBoost::T2Attack),
    (Boost::Attack(T3_ATTACK_MULTIPLIER), AbstractBoost::T3Attack),
    (Boost::Harvest(T1_HARVEST_MULTIPLIER), AbstractBoost::T1Harvest),
    (Boost::Harvest(T2_HARVEST_MULTIPLIER), AbstractBoost::T2Harvest),
    (Boost::Harvest(T3_HARVEST_MULTIPLIER), AbstractBoost::T3Harvest),
    (Boost::Carry(T1_CARRY_MULTIPLIER), AbstractBoost::T1Carry),
    (Boost::Carry(T2_CARRY_MULTIPLIER), AbstractBoost::T2Carry),
    (Boost::Carry(T3_CARRY_MULTIPLIER), AbstractBoost::T3Carry),
    (Boost::RangedAttack(T1_RANGED_ATTACK_MULTIPLIER), AbstractBoost::T1RangedAttack),
    (Boost::RangedAttack(T2_RANGED_ATTACK_MULTIPLIER), AbstractBoost::T2RangedAttack),
    (Boost::RangedAttack(T3_RANGED_ATTACK_MULTIPLIER), AbstractBoost::T3RangedAttack),
    (Boost::BuildAndRepair(T1_BUILD_REPAIR_MULTIPLIER), AbstractBoost::T1BuildRepair),
    (Boost::BuildAndRepair(T2_BUILD_REPAIR_MULTIPLIER), AbstractBoost::T2BuildRepair),
    (Boost::BuildAndRepair(T3_BUILD_REPAIR_MULTIPLIER), AbstractBoost::T3BuildRepair),
    (Boost::Heal(T1_HEAL_MULTIPLIER), AbstractBoost::T1Heal),
    (Boost::Heal(T2_HEAL_MULTIPLIER), AbstractBoost::T2Heal),
    (Boost::Heal(T3_HEAL_MULTIPLIER), AbstractBoost::T3Heal),
    (Boost::Dismantle(T1_DISMANTLE_MULTIPLIER), AbstractBoost::T1Dismantle),
    (Boost::Dismantle(T2_DISMANTLE_MULTIPLIER), AbstractBoost::T2Dismantle),
    (Boost::Dismantle(T3_DISMANTLE_MULTIPLIER), AbstractBoost::T3Dismantle),
    (Boost::Move(T1_MOVE_MULTIPLIER), AbstractBoost::T1Move),
    (Boost::Move(T2_MOVE_MULTIPLIER), AbstractBoost::T2Move),
    (Boost::Move(T3_MOVE_MULTIPLIER), AbstractBoost::T3Move),
    (Boost::UpgradeController(T1_UPGRADE_CONTROLLER_MULTIPLIER), AbstractBoost::T1UpgradeController),
    (Boost::UpgradeController(T2_UPGRADE_CONTROLLER_MULTIPLIER), AbstractBoost::T2UpgradeController),
    (Boost::UpgradeController(T3_UPGRADE_CONTROLLER_MULTIPLIER), AbstractBoost::T3UpgradeController),
    (Boost::Tough(T1_TOUGH_MULTIPLIER), AbstractBoost::T1Tough),
    (Boost::Tough(T2_TOUGH_MULTIPLIER), AbstractBoost::T2Tough),
    (Boost::Tough(T3_TOUGH_MULTIPLIER), AbstractBoost::T3Tough),
];

const invalid_boost_multipliers: [Boost; 10] = [
    Boost::Attack(200),
    Boost::Harvest(200),
    Boost::Carry(200),
    Boost::RangedAttack(200),
    Boost::BuildAndRepair(200.0),
    Boost::Heal(200),
    Boost::Dismantle(200),
    Boost::Move(200),
    Boost::UpgradeController(200.0),
    Boost::Tough(200.0),
];

/// Boost does not implement Eq or PartialEq, so we need to do it ourselves.
///
/// A boost is equal to another boost if they are both the same variant and they both have the same
/// multiplier.
fn screeps_boosts_eq(a: &Boost, b: &Boost) -> bool {
    match (a, b) {
        (Boost::Attack(mult_a), Boost::Attack(mult_b)) => mult_a == mult_b,
        (Boost::Harvest(mult_a), Boost::Harvest(mult_b)) => mult_a == mult_b,
        (Boost::Carry(mult_a), Boost::Carry(mult_b)) => mult_a == mult_b,
        (Boost::RangedAttack(mult_a), Boost::RangedAttack(mult_b)) => mult_a == mult_b,
        (Boost::BuildAndRepair(mult_a), Boost::BuildAndRepair(mult_b)) => mult_a == mult_b,
        (Boost::Heal(mult_a), Boost::Heal(mult_b)) => mult_a == mult_b,
        (Boost::Dismantle(mult_a), Boost::Dismantle(mult_b)) => mult_a == mult_b,
        (Boost::Move(mult_a), Boost::Move(mult_b)) => mult_a == mult_b,
        (Boost::UpgradeController(mult_a), Boost::UpgradeController(mult_b)) => mult_a == mult_b,
        (Boost::Tough(mult_a), Boost::Tough(mult_b)) => mult_a == mult_b,
        _ => false,
    }
}

#[test]
fn abstractboost_try_from_boost_converts_correctly() {
    for (screeps_boost, expected) in abstractboost_boost_conversion_data {
        let res = AbstractBoost::try_from(screeps_boost);
        assert!(res.is_ok());
        let abstract_boost = res.unwrap();
        assert_eq!(expected, abstract_boost);
    }
}

#[test]
fn abstractboost_try_from_boost_errors_for_bad_boost_multipliers() {
    for invalid_boost in invalid_boost_multipliers {
        let res = AbstractBoost::try_from(invalid_boost);
        assert!(res.is_err());
    }
}

#[test]
fn boost_from_abstractboost_converts_correctly() {
    for (expected, abstract_boost) in abstractboost_boost_conversion_data {
        let res = Boost::from(abstract_boost);
        assert!(screeps_boosts_eq(&expected, &res));
    }
}


