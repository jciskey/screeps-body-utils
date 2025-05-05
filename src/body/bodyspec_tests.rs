use itertools::Itertools;
use assert_float_eq;

use screeps::Part;
use screeps::constants::extra::{
    MOVE_COST_SWAMP,
    MOVE_COST_PLAIN,
    MOVE_COST_ROAD,
    MOVE_POWER,
    RANGED_MASS_ATTACK_POWER_RANGE_1,
    RANGED_MASS_ATTACK_POWER_RANGE_2,
    RANGED_MASS_ATTACK_POWER_RANGE_3,
};

use super::bodyspec;
use super::body_generation;
use crate::body::{BodySpec, PartSpec, BodySpecValidationError};
use crate::boost;
use boost::AbstractBoost;


static KNOWN_PARTS: [Part; 8] = [
    Part::Move,
    Part::Work,
    Part::Carry,
    Part::Attack,
    Part::RangedAttack,
    Part::Tough,
    Part::Heal,
    Part::Claim,
];

static KNOWN_PARTS_WITH_BODYGEN_CHARS: [(&str, Part); 8] = [
    ("M", Part::Move),
    ("W", Part::Work),
    ("C", Part::Carry),
    ("A", Part::Attack),
    ("R", Part::RangedAttack),
    ("T", Part::Tough),
    ("H", Part::Heal),
    ("L", Part::Claim),
];

// No boost, T1, T2, T3
static ATTACK_DATA: [u32; 4] = [30, 60, 90, 120];
static RANGED_ATTACK_DATA: [u32; 4] = [10, 20, 30, 40];
static RANGED_MASS_ATTACK_D1_DATA: [u32; 4] = [
    RANGED_MASS_ATTACK_POWER_RANGE_1,
    RANGED_MASS_ATTACK_POWER_RANGE_1 * 2,
    RANGED_MASS_ATTACK_POWER_RANGE_1 * 3,
    RANGED_MASS_ATTACK_POWER_RANGE_1 * 4,
];
static RANGED_MASS_ATTACK_D2_DATA: [u32; 4] = [
    RANGED_MASS_ATTACK_POWER_RANGE_2,
    RANGED_MASS_ATTACK_POWER_RANGE_2 * 2,
    RANGED_MASS_ATTACK_POWER_RANGE_2 * 3,
    RANGED_MASS_ATTACK_POWER_RANGE_2 * 4,
];
static RANGED_MASS_ATTACK_D3_DATA: [u32; 4] = [
    RANGED_MASS_ATTACK_POWER_RANGE_3,
    RANGED_MASS_ATTACK_POWER_RANGE_3 * 2,
    RANGED_MASS_ATTACK_POWER_RANGE_3 * 3,
    RANGED_MASS_ATTACK_POWER_RANGE_3 * 4,
];
static HEAL_DATA: [u32; 4] = [12, 24, 36, 48];
static RANGED_HEAL_DATA: [u32; 4] = [4, 8, 12, 16];
static BUILD_DATA: [f32; 4] = [5.0, 7.5, 9.0, 10.0];
static REPAIR_DATA: [f32; 4] = [100.0, 150.0, 180.0, 200.0];
static UPGRADE_CONTROLLER_DATA: [f32; 4] = [1.0, 1.5, 1.8, 2.0];
static DISMANTLE_DATA: [u32; 4] = [50, 100, 150, 200];
static HARVEST_SOURCE_DATA: [u32; 4] = [2, 6, 10, 14];
static HARVEST_MINERAL_DATA: [u32; 4] = [1, 3, 5, 7];
static HARVEST_DEPOSIT_DATA: [u32; 4] = [1, 3, 5, 7];
static CARRY_DATA: [u32; 4] = [50, 100, 150, 200];
static DAMAGE_CAPACITY_DATA: [f32; 4] = [100.0, 142.85715, 200.0, 333.3333];
static FATIGUE_REDUCTION_DATA: [u32; 4] = [2, 4, 6, 8];

////////////////////
// PartSpec Tests //
////////////////////

#[test]
fn partspec_attack_calculates_correctly() {
    // Verify non-Attack parts don't contribute
    for part in KNOWN_PARTS {
        if part == Part::Attack {
            continue;
        }
        let partspec = bodyspec::PartSpec::new_unboosted_part(part);
        assert_eq!(partspec.get_attack_damage(), 0);
    }

    // Verify Attack parts do contribute
    let opts: Vec<_> = std::iter::once(None).chain(boost::ATTACK_BOOSTS.iter().map(|b| Some(b))).collect();
    for (boost_opt, expected) in std::iter::zip(opts, ATTACK_DATA) {
        let partspec = if let Some(boost) = boost_opt {
            bodyspec::PartSpec::new_boosted_part(Part::Attack, *boost)
        } else {
            bodyspec::PartSpec::new_unboosted_part(Part::Attack)
        };
        assert_eq!(partspec.get_attack_damage(), expected);
    }
}

#[test]
fn partspec_ranged_attack_calculates_correctly() {
    // Verify non-RangedAttack parts don't contribute
    for part in KNOWN_PARTS {
        if part == Part::RangedAttack {
            continue;
        }
        let partspec = bodyspec::PartSpec::new_unboosted_part(part);
        assert_eq!(partspec.get_ranged_attack_damage(), 0);
    }

    // Verify RangedAttack parts do contribute
    let opts: Vec<_> = std::iter::once(None).chain(boost::RANGED_ATTACK_BOOSTS.iter().map(|b| Some(b))).collect();
    for (boost_opt, expected) in std::iter::zip(opts, RANGED_ATTACK_DATA) {
        let partspec = if let Some(boost) = boost_opt {
            bodyspec::PartSpec::new_boosted_part(Part::RangedAttack, *boost)
        } else {
            bodyspec::PartSpec::new_unboosted_part(Part::RangedAttack)
        };
        assert_eq!(partspec.get_ranged_attack_damage(), expected);
    }
}

#[test]
fn partspec_ranged_mass_attack_calculates_correctly() {
    let range_data = std::iter::zip([1,2,3], [RANGED_MASS_ATTACK_D1_DATA, RANGED_MASS_ATTACK_D2_DATA, RANGED_MASS_ATTACK_D3_DATA]);

    for (distance, damage_data) in range_data {
        // Verify non-RangedAttack parts don't contribute
        for part in KNOWN_PARTS {
            if part == Part::RangedAttack {
                continue;
            }
            let partspec = bodyspec::PartSpec::new_unboosted_part(part);
            assert_eq!(partspec.get_ranged_mass_attack_damage_at_distance_single_target(distance), 0);
        }

        // Verify RangedAttack parts do contribute
        let opts: Vec<_> = std::iter::once(None).chain(boost::RANGED_ATTACK_BOOSTS.iter().map(|b| Some(b))).collect();
        for (boost_opt, expected) in std::iter::zip(opts, damage_data) {
            let partspec = if let Some(boost) = boost_opt {
                bodyspec::PartSpec::new_boosted_part(Part::RangedAttack, *boost)
            } else {
                bodyspec::PartSpec::new_unboosted_part(Part::RangedAttack)
            };
            assert_eq!(partspec.get_ranged_mass_attack_damage_at_distance_single_target(distance), expected);
        }
    }
}

#[test]
fn partspec_heal_calculates_correctly() {
    // Verify non-Heal parts don't contribute
    for part in KNOWN_PARTS {
        if part == Part::Heal {
            continue;
        }
        let partspec = bodyspec::PartSpec::new_unboosted_part(part);
        assert_eq!(partspec.get_heal_amount(), 0);
    }

    // Verify Heal parts do contribute
    let opts: Vec<_> = std::iter::once(None).chain(boost::HEAL_BOOSTS.iter().map(|b| Some(b))).collect();
    for (boost_opt, expected) in std::iter::zip(opts, HEAL_DATA) {
        let partspec = if let Some(boost) = boost_opt {
            bodyspec::PartSpec::new_boosted_part(Part::Heal, *boost)
        } else {
            bodyspec::PartSpec::new_unboosted_part(Part::Heal)
        };
        assert_eq!(partspec.get_heal_amount(), expected);
    }
}

#[test]
fn partspec_ranged_heal_calculates_correctly() {
    // Verify non-Heal parts don't contribute
    for part in KNOWN_PARTS {
        if part == Part::Heal {
            continue;
        }
        let partspec = bodyspec::PartSpec::new_unboosted_part(part);
        assert_eq!(partspec.get_ranged_heal_amount(), 0);
    }

    // Verify Heal parts do contribute
    let opts: Vec<_> = std::iter::once(None).chain(boost::HEAL_BOOSTS.iter().map(|b| Some(b))).collect();
    for (boost_opt, expected) in std::iter::zip(opts, RANGED_HEAL_DATA) {
        let partspec = if let Some(boost) = boost_opt {
            bodyspec::PartSpec::new_boosted_part(Part::Heal, *boost)
        } else {
            bodyspec::PartSpec::new_unboosted_part(Part::Heal)
        };
        assert_eq!(partspec.get_ranged_heal_amount(), expected);
    }
}

#[test]
fn partspec_build_calculates_correctly() {
    // Verify non-Work parts don't contribute
    for part in KNOWN_PARTS {
        if part == Part::Work {
            continue;
        }
        let partspec = bodyspec::PartSpec::new_unboosted_part(part);
        assert_eq!(partspec.get_build_amount(), 0.0);
    }

    // Verify Work parts do contribute
    let opts: Vec<_> = std::iter::once(None).chain(boost::BUILD_REPAIR_BOOSTS.iter().map(|b| Some(b))).collect();
    for (boost_opt, expected) in std::iter::zip(opts, BUILD_DATA) {
        let partspec = if let Some(boost) = boost_opt {
            bodyspec::PartSpec::new_boosted_part(Part::Work, *boost)
        } else {
            bodyspec::PartSpec::new_unboosted_part(Part::Work)
        };
        assert_eq!(partspec.get_build_amount(), expected);
    }
}

#[test]
fn partspec_repair_calculates_correctly() {
    // Verify non-Work parts don't contribute
    for part in KNOWN_PARTS {
        if part == Part::Work {
            continue;
        }
        let partspec = bodyspec::PartSpec::new_unboosted_part(part);
        assert_eq!(partspec.get_repair_amount(), 0.0);
    }

    // Verify Work parts do contribute
    let opts: Vec<_> = std::iter::once(None).chain(boost::BUILD_REPAIR_BOOSTS.iter().map(|b| Some(b))).collect();
    for (boost_opt, expected) in std::iter::zip(opts, REPAIR_DATA) {
        let partspec = if let Some(boost) = boost_opt {
            bodyspec::PartSpec::new_boosted_part(Part::Work, *boost)
        } else {
            bodyspec::PartSpec::new_unboosted_part(Part::Work)
        };
        assert_eq!(partspec.get_repair_amount(), expected);
    }
}

#[test]
fn partspec_harvest_energy_calculates_correctly() {
    // Verify non-Work parts don't contribute
    for part in KNOWN_PARTS {
        if part == Part::Work {
            continue;
        }
        let partspec = bodyspec::PartSpec::new_unboosted_part(part);
        assert_eq!(partspec.get_harvest_energy_amount(), 0);
    }

    // Verify Work parts do contribute
    let opts: Vec<_> = std::iter::once(None).chain(boost::HARVEST_BOOSTS.iter().map(|b| Some(b))).collect();
    for (boost_opt, expected) in std::iter::zip(opts, HARVEST_SOURCE_DATA) {
        let partspec = if let Some(boost) = boost_opt {
            bodyspec::PartSpec::new_boosted_part(Part::Work, *boost)
        } else {
            bodyspec::PartSpec::new_unboosted_part(Part::Work)
        };
        assert_eq!(partspec.get_harvest_energy_amount(), expected);
    }
}

#[test]
fn partspec_harvest_mineral_calculates_correctly() {
    // Verify non-Work parts don't contribute
    for part in KNOWN_PARTS {
        if part == Part::Work {
            continue;
        }
        let partspec = bodyspec::PartSpec::new_unboosted_part(part);
        assert_eq!(partspec.get_harvest_mineral_amount(), 0);
    }

    // Verify Work parts do contribute
    let opts: Vec<_> = std::iter::once(None).chain(boost::HARVEST_BOOSTS.iter().map(|b| Some(b))).collect();
    for (boost_opt, expected) in std::iter::zip(opts, HARVEST_MINERAL_DATA) {
        let partspec = if let Some(boost) = boost_opt {
            bodyspec::PartSpec::new_boosted_part(Part::Work, *boost)
        } else {
            bodyspec::PartSpec::new_unboosted_part(Part::Work)
        };
        assert_eq!(partspec.get_harvest_mineral_amount(), expected);
    }
}

#[test]
fn partspec_harvest_deposit_calculates_correctly() {
    // Verify non-Work parts don't contribute
    for part in KNOWN_PARTS {
        if part == Part::Work {
            continue;
        }
        let partspec = bodyspec::PartSpec::new_unboosted_part(part);
        assert_eq!(partspec.get_harvest_deposit_amount(), 0);
    }

    // Verify Work parts do contribute
    let opts: Vec<_> = std::iter::once(None).chain(boost::HARVEST_BOOSTS.iter().map(|b| Some(b))).collect();
    for (boost_opt, expected) in std::iter::zip(opts, HARVEST_DEPOSIT_DATA) {
        let partspec = if let Some(boost) = boost_opt {
            bodyspec::PartSpec::new_boosted_part(Part::Work, *boost)
        } else {
            bodyspec::PartSpec::new_unboosted_part(Part::Work)
        };
        assert_eq!(partspec.get_harvest_deposit_amount(), expected);
    }
}

#[test]
fn partspec_upgrade_controller_calculates_correctly() {
    // Verify non-Work parts don't contribute
    for part in KNOWN_PARTS {
        if part == Part::Work {
            continue;
        }
        let partspec = bodyspec::PartSpec::new_unboosted_part(part);
        assert_eq!(partspec.get_upgrade_controller_amount(), 0.0);
    }

    // Verify Work parts do contribute
    let opts: Vec<_> = std::iter::once(None).chain(boost::UPGRADE_CONTROLLER_BOOSTS.iter().map(|b| Some(b))).collect();
    for (boost_opt, expected) in std::iter::zip(opts, UPGRADE_CONTROLLER_DATA) {
        let partspec = if let Some(boost) = boost_opt {
            bodyspec::PartSpec::new_boosted_part(Part::Work, *boost)
        } else {
            bodyspec::PartSpec::new_unboosted_part(Part::Work)
        };
        assert_eq!(partspec.get_upgrade_controller_amount(), expected);
    }
}

#[test]
fn partspec_dismantle_calculates_correctly() {
    // Verify non-Work parts don't contribute
    for part in KNOWN_PARTS {
        if part == Part::Work {
            continue;
        }
        let partspec = bodyspec::PartSpec::new_unboosted_part(part);
        assert_eq!(partspec.get_dismantle_damage(), 0);
    }

    // Verify Work parts do contribute
    let opts: Vec<_> = std::iter::once(None).chain(boost::DISMANTLE_BOOSTS.iter().map(|b| Some(b))).collect();
    for (boost_opt, expected) in std::iter::zip(opts, DISMANTLE_DATA) {
        let partspec = if let Some(boost) = boost_opt {
            bodyspec::PartSpec::new_boosted_part(Part::Work, *boost)
        } else {
            bodyspec::PartSpec::new_unboosted_part(Part::Work)
        };
        assert_eq!(partspec.get_dismantle_damage(), expected);
    }
}

#[test]
fn partspec_carry_calculates_correctly() {
    // Verify non-Carry parts don't contribute
    for part in KNOWN_PARTS {
        if part == Part::Carry {
            continue;
        }
        let partspec = bodyspec::PartSpec::new_unboosted_part(part);
        assert_eq!(partspec.get_carry_capacity(), 0);
    }

    // Verify Carry parts do contribute
    let opts: Vec<_> = std::iter::once(None).chain(boost::CARRY_BOOSTS.iter().map(|b| Some(b))).collect();
    for (boost_opt, expected) in std::iter::zip(opts, CARRY_DATA) {
        let partspec = if let Some(boost) = boost_opt {
            bodyspec::PartSpec::new_boosted_part(Part::Carry, *boost)
        } else {
            bodyspec::PartSpec::new_unboosted_part(Part::Carry)
        };
        assert_eq!(partspec.get_carry_capacity(), expected);
    }
}

#[test]
fn partspec_damage_capacity_calculates_correctly() {
    // Verify non-Tough parts don't contribute any additional damage capacity
    for part in KNOWN_PARTS {
        if part == Part::Tough {
            continue;
        }
        let partspec = bodyspec::PartSpec::new_unboosted_part(part);
        assert_eq!(partspec.get_damage_capacity(), 100.0);
    }

    // Verify Tough parts do contribute
    let opts: Vec<_> = std::iter::once(None).chain(boost::TOUGH_BOOSTS.iter().map(|b| Some(b))).collect();
    for (boost_opt, expected) in std::iter::zip(opts, DAMAGE_CAPACITY_DATA) {
        let partspec = if let Some(boost) = boost_opt {
            bodyspec::PartSpec::new_boosted_part(Part::Tough, *boost)
        } else {
            bodyspec::PartSpec::new_unboosted_part(Part::Tough)
        };
        assert_eq!(partspec.get_damage_capacity(), expected);
    }
}

#[test]
fn partspec_fatigue_reduction_calculates_correctly() {
    // Verify non-Move parts don't contribute
    for part in KNOWN_PARTS {
        if part == Part::Move {
            continue;
        }
        let partspec = bodyspec::PartSpec::new_unboosted_part(part);
        assert_eq!(partspec.get_fatigue_reduction(), 0);
    }

    // Verify Move parts do contribute
    let opts: Vec<_> = std::iter::once(None).chain(boost::MOVE_BOOSTS.iter().map(|b| Some(b))).collect();
    for (boost_opt, expected) in std::iter::zip(opts, FATIGUE_REDUCTION_DATA) {
        let partspec = if let Some(boost) = boost_opt {
            bodyspec::PartSpec::new_boosted_part(Part::Move, *boost)
        } else {
            bodyspec::PartSpec::new_unboosted_part(Part::Move)
        };
        assert_eq!(partspec.get_fatigue_reduction(), expected);
    }
}

#[test]
fn partspec_get_fatigue_generation_calculates_correctly() {
    // Fatigue generation doesn't care about boosts, it only cares about part type and hits
    // Move parts generate no fatigue
    // Carry parts with no hits generate no fatigue
    for tile_cost in [MOVE_COST_ROAD, MOVE_COST_PLAIN, MOVE_COST_SWAMP] {
        for hits in [100, 0] {
            for part in enum_iterator::all::<Part>() {
                let expected_fatigue = match part {
                    Part::Move => 0,
                    Part::Carry => {
                        if hits > 0 {
                            tile_cost
                        } else {
                            0
                        }
                    },
                    _ => tile_cost,
                };

                let partspec = PartSpec::new(part, hits, None);
                assert_eq!(expected_fatigue, partspec.get_fatigue_generation(tile_cost), "PartSpec: {:?}", partspec);

                for boost in AbstractBoost::boosts_for_part(&part) {
                    let partspec = PartSpec::new(part, hits, Some(*boost));
                    assert_eq!(expected_fatigue, partspec.get_fatigue_generation(tile_cost), "PartSpec: {:?}", partspec);
                }
            }
        }
    }
}

////////////////////
// BodySpec Tests //
////////////////////

#[test]
fn bodyspec_net_exhaustion_calculates_properly_all_moves() {
    // Verify an all Move part body has no net exhaustion, irrespective of boosting
    for i in 1..=50 {
        let s = format!("{}M", i);
        let result = body_generation::generate_body_from_string(&s);
        assert!(result.is_ok());

        let body = result.unwrap();
        let partspec_bodies_tuple: (Vec<_>, Vec<_>, Vec<_>, Vec<_>) = body.into_iter()
                                   .map(|p| {
                                       let boost_map = std::iter::once(None).chain(boost::MOVE_BOOSTS.map(|b| Some(b)));
                                       let p_iter = std::iter::repeat(p);
                                       let zipped_iter = p_iter.zip(boost_map);
                                       let partspecs_iter = zipped_iter.map(|(p, b)| PartSpec::new(p, 100, b));
                                       partspecs_iter.collect_tuple::<(PartSpec, PartSpec, PartSpec, PartSpec)>().unwrap()
                                   })
                                   .multiunzip();
        let partspec_bodies = [partspec_bodies_tuple.0, partspec_bodies_tuple.1, partspec_bodies_tuple.2, partspec_bodies_tuple.3];
        for partspec_vec in partspec_bodies {
            let bodyspec = BodySpec::new(&partspec_vec);
            assert_eq!(0, bodyspec.plains_move_net_exhaustion(), "BodySpec: {:?}", bodyspec);
            assert_eq!(0, bodyspec.swamp_move_net_exhaustion(), "BodySpec: {:?}", bodyspec);
            assert_eq!(0, bodyspec.road_move_net_exhaustion(), "BodySpec: {:?}", bodyspec);
        }
    }
}

#[test]
fn bodyspec_net_exhaustion_calculates_properly_no_moves() {
    // Verify a body with no Move parts generates exhaustion scaled to the number of body parts
    for (c, _) in KNOWN_PARTS_WITH_BODYGEN_CHARS {
        if c == "M" {
            continue;
        }

        for i in 1..=50 {
            let s = format!("{}{}", i, c);
            let result = body_generation::generate_body_from_string(&s);
            assert!(result.is_ok());

            let body = result.unwrap();

            let partspec_vec: Vec<PartSpec> = body.into_iter()
                                                  .map(|p| PartSpec::new_unboosted_part(p))
                                                  .collect();

            let bodyspec = BodySpec::new(&partspec_vec);
            assert_eq!(i * MOVE_COST_PLAIN, bodyspec.plains_move_net_exhaustion(), "BodySpec: {:?}", bodyspec);
            assert_eq!(i * MOVE_COST_SWAMP, bodyspec.swamp_move_net_exhaustion(), "BodySpec: {:?}", bodyspec);
            assert_eq!(i * MOVE_COST_ROAD, bodyspec.road_move_net_exhaustion(), "BodySpec: {:?}", bodyspec);
        }
    }
}

#[test]
fn bodyspec_net_exhaustion_calculates_properly_mixed_parts() {
    // Verify that Move offsets Non-Move parts appropriately

    let unboosted_move = PartSpec::new_unboosted_part(Part::Move);
    let t1_move = PartSpec::new_boosted_part(Part::Move, AbstractBoost::T1Move);
    let t2_move = PartSpec::new_boosted_part(Part::Move, AbstractBoost::T2Move);
    let t3_move = PartSpec::new_boosted_part(Part::Move, AbstractBoost::T3Move);
    let move_partspecs_arr = [unboosted_move, t1_move, t2_move, t3_move];

    for (c, part) in KNOWN_PARTS_WITH_BODYGEN_CHARS {
        if c == "M" {
            continue;
        }

        let unboosted_other = PartSpec::new_unboosted_part(part);
        let boosted_other_partspecs_vec: Vec<PartSpec> = AbstractBoost::boosts_for_part(&part).into_iter().map(|b| PartSpec::new_boosted_part(part, *b)).collect();
        let other_boost_vec: Vec<PartSpec> = std::iter::once(unboosted_other).chain(boosted_other_partspecs_vec).collect();

        for move_partspec in move_partspecs_arr {
            for other_partspec in &other_boost_vec {
                for num_parts_total in 1..=50 as usize {
                    for num_move_parts in 0..num_parts_total as usize {
                        let num_other_parts: usize = num_parts_total.saturating_sub(num_move_parts);

                        let mut move_parts_vec: Vec<_> = std::iter::repeat(move_partspec).take(num_move_parts).collect();
                        let mut other_parts_vec: Vec<_> = std::iter::repeat(*other_partspec).take(num_other_parts).collect();

                        let mut partspec_vec = Vec::with_capacity(num_parts_total);
                        partspec_vec.append(&mut other_parts_vec);
                        partspec_vec.append(&mut move_parts_vec);

                        let bodyspec = BodySpec::new(&partspec_vec);

                        let expected_plains_fatigue_generation = num_other_parts as u32 * MOVE_COST_PLAIN;
                        let expected_swamp_fatigue_generation = num_other_parts as u32 * MOVE_COST_SWAMP;
                        let expected_road_fatigue_generation = num_other_parts as u32 * MOVE_COST_ROAD;

                        let expected_plains_fatigue_reduction = num_move_parts as u32 * move_partspec.get_fatigue_reduction();
                        let expected_swamp_fatigue_reduction = num_move_parts as u32 * move_partspec.get_fatigue_reduction();
                        let expected_road_fatigue_reduction = num_move_parts as u32 * move_partspec.get_fatigue_reduction();

                        let expected_plains_exhaustion = expected_plains_fatigue_generation.saturating_sub(expected_plains_fatigue_reduction);
                        let expected_swamp_exhaustion = expected_swamp_fatigue_generation.saturating_sub(expected_swamp_fatigue_reduction);
                        let expected_road_exhaustion = expected_road_fatigue_generation.saturating_sub(expected_road_fatigue_reduction);

                        assert_eq!(expected_plains_exhaustion, bodyspec.plains_move_net_exhaustion(), "BodySpec: {:?}", bodyspec);
                        assert_eq!(expected_swamp_exhaustion, bodyspec.swamp_move_net_exhaustion(), "BodySpec: {:?}", bodyspec);
                        assert_eq!(expected_road_exhaustion, bodyspec.road_move_net_exhaustion(), "BodySpec: {:?}", bodyspec);
                    }
                }
            }
        }
    }
}



#[test]
fn bodyspec_validated_new_returns_error_for_too_many_parts() {
    let parts = [PartSpec::new_unboosted_part(Part::Move); 51];
    let bodyspec_res = BodySpec::validated_new(&parts);
    assert!(bodyspec_res.is_err());
    let is_too_many_parts_error = match bodyspec_res.unwrap_err() {
        BodySpecValidationError::TooManyParts => true,
    };
    assert!(is_too_many_parts_error);
}

#[test]
fn bodyspec_validated_new_returns_ok_for_lte_50_parts() {
    let parts = [PartSpec::new_unboosted_part(Part::Move); 50];
    for i in 1..=50 {
        let bodyspec_res = BodySpec::validated_new(&parts[..i]);
        assert!(bodyspec_res.is_ok());
    }
}


#[test]
fn bodyspec_ticks_to_spawn_calculates_correctly() {
    // All parts, regardless of type or boost, should increase the amount of ticks to spawn
    for (_, part) in KNOWN_PARTS_WITH_BODYGEN_CHARS {
        let unboosted = PartSpec::new_unboosted_part(part);
        let boosted_partspecs_vec: Vec<PartSpec> = AbstractBoost::boosts_for_part(&part).into_iter().map(|b| PartSpec::new_boosted_part(part, *b)).collect();
        let boost_vec: Vec<PartSpec> = std::iter::once(unboosted).chain(boosted_partspecs_vec).collect();

        for num_parts in 1..=50 as usize {
            for other_partspec in &boost_vec {
                let partspec_vec: Vec<_> = std::iter::repeat(*other_partspec).take(num_parts).collect();
                let bodyspec = BodySpec::new(&partspec_vec);

                let expected_ticks_to_spawn = num_parts as u32 * screeps::constants::creep::CREEP_SPAWN_TIME;
                assert_eq!(expected_ticks_to_spawn, bodyspec.ticks_to_spawn(), "BodySpec: {:?}", bodyspec);
            }
        }
    }
}

#[test]
fn bodyspec_energy_to_spawn_calculates_correctly() {
    // All parts, regardless of type or boost, should increase the amount of energy to spawn
    for (_, part) in KNOWN_PARTS_WITH_BODYGEN_CHARS {
        let unboosted = PartSpec::new_unboosted_part(part);
        let boosted_partspecs_vec: Vec<PartSpec> = AbstractBoost::boosts_for_part(&part).into_iter().map(|b| PartSpec::new_boosted_part(part, *b)).collect();
        let boost_vec: Vec<PartSpec> = std::iter::once(unboosted).chain(boosted_partspecs_vec).collect();

        for num_parts in 1..=50 as usize {
            for other_partspec in &boost_vec {
                let partspec_vec: Vec<_> = std::iter::repeat(*other_partspec).take(num_parts).collect();
                let bodyspec = BodySpec::new(&partspec_vec);

                let expected_energy_to_spawn = num_parts as u32 * part.cost();
                assert_eq!(expected_energy_to_spawn, bodyspec.energy_to_spawn(), "BodySpec: {:?}", bodyspec);
            }
        }
    }
}

#[test]
fn bodyspec_hits_calculates_correctly() {
    // All parts, regardless of type or boost, should increase the hits of a creep
    for (_, part) in KNOWN_PARTS_WITH_BODYGEN_CHARS {
        let unboosted = PartSpec::new_unboosted_part(part);
        let boosted_partspecs_vec: Vec<PartSpec> = AbstractBoost::boosts_for_part(&part).into_iter().map(|b| PartSpec::new_boosted_part(part, *b)).collect();
        let boost_vec: Vec<PartSpec> = std::iter::once(unboosted).chain(boosted_partspecs_vec).collect();

        for num_parts in 1..=50 as usize {
            for other_partspec in &boost_vec {
                let partspec_vec: Vec<_> = std::iter::repeat(*other_partspec).take(num_parts).collect();
                let bodyspec = BodySpec::new(&partspec_vec);

                let expected_hits = num_parts as u32 * screeps::constants::extra::CREEP_HITS_PER_PART;
                assert_eq!(expected_hits, bodyspec.hits(), "BodySpec: {:?}", bodyspec);
            }
        }
    }
}

#[test]
fn bodyspec_effective_hits_calculates_correctly() {
    // All parts should increase the effective hits of a creep,
    // Boosted tough parts should increase effective hits more
    for (_, part) in KNOWN_PARTS_WITH_BODYGEN_CHARS {
        let unboosted = PartSpec::new_unboosted_part(part);
        let boosted_partspecs_vec: Vec<PartSpec> = AbstractBoost::boosts_for_part(&part).into_iter().map(|b| PartSpec::new_boosted_part(part, *b)).collect();
        let boost_vec: Vec<PartSpec> = std::iter::once(unboosted).chain(boosted_partspecs_vec).collect();

        for num_parts in 1..=50 as usize {
            for other_partspec in &boost_vec {
                let partspec_vec: Vec<_> = std::iter::repeat(*other_partspec).take(num_parts).collect();
                let bodyspec = BodySpec::new(&partspec_vec);

                let current_boost = partspec_vec[0].boost;
                let effective_hits_divisor: f32 = match current_boost {
                    Some(AbstractBoost::T1Tough) |
                    Some(AbstractBoost::T2Tough) |
                    Some(AbstractBoost::T3Tough) => {
                        AbstractBoost::get_f32_multiplier(&current_boost.unwrap()).unwrap()
                    },
                    _ => 1.0,
                };

                // We have to iteratively calculate this, or we get off-by-one issues due to
                // floating point imprecision; the game engine calculates things iteratively, so we
                // do the same for consistency
                let mut expected_effective_hits: f32 = 0.0;
                for _ in 0..num_parts {
                    expected_effective_hits += screeps::constants::extra::CREEP_HITS_PER_PART as f32 / effective_hits_divisor
                }
                let expected_effective_hits = expected_effective_hits.floor() as u32;
                assert_eq!(expected_effective_hits, bodyspec.effective_hits(), "BodySpec: {:?}", bodyspec);
            }
        }
    }
}

#[test]
fn bodyspec_harvest_energy_minerals_deposits_calculates_correctly() {
    // Harvest energy, minerals, and deposits are only influenced by Work parts
    // Only the T{1,2,3}Harvest boosts affect harvesting
    let unboosted_work = PartSpec::new_unboosted_part(Part::Work);
    let t1_work = PartSpec::new_boosted_part(Part::Work, AbstractBoost::T1Harvest);
    let t2_work = PartSpec::new_boosted_part(Part::Work, AbstractBoost::T2Harvest);
    let t3_work = PartSpec::new_boosted_part(Part::Work, AbstractBoost::T3Harvest);
    let work_partspecs_arr = [unboosted_work, t1_work, t2_work, t3_work];

    for (c, part) in KNOWN_PARTS_WITH_BODYGEN_CHARS {
        let unboosted_other = PartSpec::new_unboosted_part(part);
        let boosted_other_partspecs_vec: Vec<PartSpec> = AbstractBoost::boosts_for_part(&part).into_iter()
                                                                                              .filter(|b| !boost::HARVEST_BOOSTS.contains(&b))
                                                                                              .map(|b| PartSpec::new_boosted_part(part, *b))
                                                                                              .collect();
        let other_boost_vec: Vec<PartSpec> = std::iter::once(unboosted_other).chain(boosted_other_partspecs_vec).collect();

        for work_partspec in work_partspecs_arr {
            for other_partspec in &other_boost_vec {
                for num_parts_total in 1..=50 as usize {
                    for num_work_parts in 0..num_parts_total as usize {
                        let num_other_parts: usize = num_parts_total.saturating_sub(num_work_parts);

                        let mut work_parts_vec: Vec<_> = std::iter::repeat(work_partspec).take(num_work_parts).collect();
                        let mut other_parts_vec: Vec<_> = std::iter::repeat(*other_partspec).take(num_other_parts).collect();

                        let mut partspec_vec = Vec::with_capacity(num_parts_total);
                        partspec_vec.append(&mut other_parts_vec);
                        partspec_vec.append(&mut work_parts_vec);

                        let bodyspec = BodySpec::new(&partspec_vec);

                        let num_other_work_parts = if c == "W" { num_other_parts } else { 0 };

                        let expected_other_harvest_energy = num_other_work_parts as u32 * unboosted_work.get_harvest_energy_amount();
                        let expected_other_harvest_mineral = num_other_work_parts as u32 * unboosted_work.get_harvest_mineral_amount();
                        let expected_other_harvest_deposit = num_other_work_parts as u32 * unboosted_work.get_harvest_deposit_amount();

                        let expected_work_harvest_energy = num_work_parts as u32 * work_partspec.get_harvest_energy_amount();
                        let expected_work_harvest_mineral = num_work_parts as u32 * work_partspec.get_harvest_mineral_amount();
                        let expected_work_harvest_deposit = num_work_parts as u32 * work_partspec.get_harvest_deposit_amount();

                        let expected_harvest_energy = expected_other_harvest_energy.saturating_add(expected_work_harvest_energy);
                        let expected_harvest_mineral = expected_other_harvest_mineral.saturating_add(expected_work_harvest_mineral);
                        let expected_harvest_deposit = expected_other_harvest_deposit.saturating_add(expected_work_harvest_deposit);

                        assert_eq!(expected_harvest_energy, bodyspec.harvest_energy_amount(), "BodySpec: {:?}", bodyspec);
                        assert_eq!(expected_harvest_mineral, bodyspec.harvest_mineral_amount(), "BodySpec: {:?}", bodyspec);
                        assert_eq!(expected_harvest_deposit, bodyspec.harvest_deposit_amount(), "BodySpec: {:?}", bodyspec);
                    }
                }
            }
        }
    }
}

#[test]
fn bodyspec_dismantle_calculates_correctly() {
    // Dismantle is only influenced by Work parts
    // Only the T{1,2,3}Dismantle boosts affect harvesting
    let unboosted_work = PartSpec::new_unboosted_part(Part::Work);
    let [t1_work, t2_work, t3_work] = boost::DISMANTLE_BOOSTS.map(|b| PartSpec::new_boosted_part(Part::Work, b));
    let work_partspecs_arr = [unboosted_work, t1_work, t2_work, t3_work];

    for (c, part) in KNOWN_PARTS_WITH_BODYGEN_CHARS {
        let unboosted_other = PartSpec::new_unboosted_part(part);
        let boosted_other_partspecs_vec: Vec<PartSpec> = AbstractBoost::boosts_for_part(&part).into_iter()
                                                                                              .filter(|b| !boost::DISMANTLE_BOOSTS.contains(&b))
                                                                                              .map(|b| PartSpec::new_boosted_part(part, *b))
                                                                                              .collect();
        let other_boost_vec: Vec<PartSpec> = std::iter::once(unboosted_other).chain(boosted_other_partspecs_vec).collect();

        for work_partspec in work_partspecs_arr {
            for other_partspec in &other_boost_vec {
                for num_parts_total in 1..=50 as usize {
                    for num_work_parts in 0..num_parts_total as usize {
                        let num_other_parts: usize = num_parts_total.saturating_sub(num_work_parts);

                        let mut work_parts_vec: Vec<_> = std::iter::repeat(work_partspec).take(num_work_parts).collect();
                        let mut other_parts_vec: Vec<_> = std::iter::repeat(*other_partspec).take(num_other_parts).collect();

                        let mut partspec_vec = Vec::with_capacity(num_parts_total);
                        partspec_vec.append(&mut other_parts_vec);
                        partspec_vec.append(&mut work_parts_vec);

                        let bodyspec = BodySpec::new(&partspec_vec);

                        let num_other_work_parts = if c == "W" { num_other_parts } else { 0 };

                        let expected_other_dismantle = num_other_work_parts as u32 * unboosted_work.get_dismantle_damage();
                        let expected_work_dismantle = num_work_parts as u32 * work_partspec.get_dismantle_damage();
                        let expected_dismantle = expected_other_dismantle.saturating_add(expected_work_dismantle);

                        assert_eq!(expected_dismantle, bodyspec.dismantle_damage(), "BodySpec: {:?}", bodyspec);
                    }
                }
            }
        }
    }
}

#[test]
fn bodyspec_build_repair_calculates_correctly() {
    // Build and Repair are only influenced by Work parts
    // Only the T{1,2,3}BuildRepair boosts affect building and repairing
    let unboosted_work = PartSpec::new_unboosted_part(Part::Work);
    let [t1_work, t2_work, t3_work] = boost::BUILD_REPAIR_BOOSTS.map(|b| PartSpec::new_boosted_part(Part::Work, b));
    let work_partspecs_arr = [unboosted_work, t1_work, t2_work, t3_work];

    for (c, part) in KNOWN_PARTS_WITH_BODYGEN_CHARS {
        let unboosted_other = PartSpec::new_unboosted_part(part);
        let boosted_other_partspecs_vec: Vec<PartSpec> = AbstractBoost::boosts_for_part(&part).into_iter()
                                                                                              .filter(|b| !boost::BUILD_REPAIR_BOOSTS.contains(&b))
                                                                                              .map(|b| PartSpec::new_boosted_part(part, *b))
                                                                                              .collect();
        let other_boost_vec: Vec<PartSpec> = std::iter::once(unboosted_other).chain(boosted_other_partspecs_vec).collect();

        for work_partspec in work_partspecs_arr {
            for other_partspec in &other_boost_vec {
                for num_parts_total in 1..=50 as usize {
                    for num_work_parts in 0..num_parts_total as usize {
                        let num_other_parts: usize = num_parts_total.saturating_sub(num_work_parts);

                        let mut work_parts_vec: Vec<_> = std::iter::repeat(work_partspec).take(num_work_parts).collect();
                        let mut other_parts_vec: Vec<_> = std::iter::repeat(*other_partspec).take(num_other_parts).collect();

                        let mut partspec_vec = Vec::with_capacity(num_parts_total);
                        partspec_vec.append(&mut other_parts_vec);
                        partspec_vec.append(&mut work_parts_vec);

                        let bodyspec = BodySpec::new(&partspec_vec);

                        let num_other_work_parts = if c == "W" { num_other_parts } else { 0 };

                        let expected_other_build_amount = unboosted_work.get_build_amount();
                        let expected_other_repair_amount = unboosted_work.get_repair_amount();

                        let expected_work_build_amount = work_partspec.get_build_amount();
                        let expected_work_repair_amount = work_partspec.get_repair_amount();

                        // We have to iteratively calculate this, or we get off-by-one issues due to
                        // floating point imprecision; the game engine calculates things iteratively, so we
                        // do the same for consistency
                        let mut expected_build_amount: f32 = 0.0;
                        let mut expected_repair_amount: f32 = 0.0;
                        for _ in 0..num_work_parts {
                            expected_build_amount += expected_work_build_amount;
                            expected_repair_amount += expected_work_repair_amount;
                        }
                        for _ in 0..num_other_work_parts {
                            expected_build_amount += expected_other_build_amount;
                            expected_repair_amount += expected_other_repair_amount;
                        }
                        assert_eq!(expected_build_amount, bodyspec.build_amount(), "BodySpec: {:?}", bodyspec);
                        assert_eq!(expected_repair_amount, bodyspec.repair_amount(), "BodySpec: {:?}", bodyspec);
                    }
                }
            }
        }
    }
}

#[test]
fn bodyspec_upgrade_controller_calculates_correctly() {
    // Upgrade controller is only influenced by Work parts
    // Only the T{1,2,3}UpgradeController boosts affect building and repairing
    let unboosted_work = PartSpec::new_unboosted_part(Part::Work);
    let [t1_work, t2_work, t3_work] = boost::UPGRADE_CONTROLLER_BOOSTS.map(|b| PartSpec::new_boosted_part(Part::Work, b));
    let work_partspecs_arr = [unboosted_work, t1_work, t2_work, t3_work];

    for (c, part) in KNOWN_PARTS_WITH_BODYGEN_CHARS {
        let unboosted_other = PartSpec::new_unboosted_part(part);
        let boosted_other_partspecs_vec: Vec<PartSpec> = AbstractBoost::boosts_for_part(&part).into_iter()
                                                                                              .filter(|b| !boost::UPGRADE_CONTROLLER_BOOSTS.contains(&b))
                                                                                              .map(|b| PartSpec::new_boosted_part(part, *b))
                                                                                              .collect();
        let other_boost_vec: Vec<PartSpec> = std::iter::once(unboosted_other).chain(boosted_other_partspecs_vec).collect();

        for work_partspec in work_partspecs_arr {
            for other_partspec in &other_boost_vec {
                for num_parts_total in 1..=50 as usize {
                    for num_work_parts in 0..num_parts_total as usize {
                        let num_other_parts: usize = num_parts_total.saturating_sub(num_work_parts);

                        let mut work_parts_vec: Vec<_> = std::iter::repeat(work_partspec).take(num_work_parts).collect();
                        let mut other_parts_vec: Vec<_> = std::iter::repeat(*other_partspec).take(num_other_parts).collect();

                        let mut partspec_vec = Vec::with_capacity(num_parts_total);
                        partspec_vec.append(&mut other_parts_vec);
                        partspec_vec.append(&mut work_parts_vec);

                        let bodyspec = BodySpec::new(&partspec_vec);

                        let num_other_work_parts = if c == "W" { num_other_parts } else { 0 };

                        let expected_other_upgrade_controller_amount = unboosted_work.get_upgrade_controller_amount();

                        let expected_work_upgrade_controller_amount = work_partspec.get_upgrade_controller_amount();

                        // We have to iteratively calculate this, or we get off-by-one issues due to
                        // floating point imprecision; the game engine calculates things iteratively, so we
                        // do the same for consistency
                        let mut expected_upgrade_controller_amount: f32 = 0.0;
                        for _ in 0..num_work_parts {
                            expected_upgrade_controller_amount += expected_work_upgrade_controller_amount;
                        }
                        for _ in 0..num_other_work_parts {
                            expected_upgrade_controller_amount += expected_other_upgrade_controller_amount;
                        }
                        assert_float_eq::assert_f32_near!(expected_upgrade_controller_amount, bodyspec.upgrade_controller_amount());
                    }
                }
            }
        }
    }
}

#[test]
fn bodyspec_attack_calculates_correctly() {
    // Attack is only influenced by Attack parts
    // Only the T{1,2,3}Attack boosts affect harvesting
    let unboosted_attack = PartSpec::new_unboosted_part(Part::Attack);
    let [t1_attack, t2_attack, t3_attack] = boost::ATTACK_BOOSTS.map(|b| PartSpec::new_boosted_part(Part::Attack, b));
    let attack_partspecs_arr = [unboosted_attack, t1_attack, t2_attack, t3_attack];

    for (c, part) in KNOWN_PARTS_WITH_BODYGEN_CHARS {
        if part == Part::Attack {
            continue;
        }

        let unboosted_other = PartSpec::new_unboosted_part(part);
        let boosted_other_partspecs_vec: Vec<PartSpec> = AbstractBoost::boosts_for_part(&part).into_iter()
                                                                                              .map(|b| PartSpec::new_boosted_part(part, *b))
                                                                                              .collect();
        let other_boost_vec: Vec<PartSpec> = std::iter::once(unboosted_other).chain(boosted_other_partspecs_vec).collect();

        for attack_partspec in attack_partspecs_arr {
            for other_partspec in &other_boost_vec {
                for num_parts_total in 1..=50 as usize {
                    for num_attack_parts in 0..num_parts_total as usize {
                        let num_other_parts: usize = num_parts_total.saturating_sub(num_attack_parts);

                        let mut attack_parts_vec: Vec<_> = std::iter::repeat(attack_partspec).take(num_attack_parts).collect();
                        let mut other_parts_vec: Vec<_> = std::iter::repeat(*other_partspec).take(num_other_parts).collect();

                        let mut partspec_vec = Vec::with_capacity(num_parts_total);
                        partspec_vec.append(&mut other_parts_vec);
                        partspec_vec.append(&mut attack_parts_vec);

                        let bodyspec = BodySpec::new(&partspec_vec);

                        let expected_attack = num_attack_parts as u32 * attack_partspec.get_attack_damage();

                        assert_eq!(expected_attack, bodyspec.attack_damage(), "BodySpec: {:?}", bodyspec);
                    }
                }
            }
        }
    }
}

#[test]
fn bodyspec_ranged_attack_calculates_correctly() {
    // Ranged attack and Ranged mass attack are only influenced by RangedAttack parts
    // Only the T{1,2,3}RangedAttack boosts affect these
    let unboosted_attack = PartSpec::new_unboosted_part(Part::RangedAttack);
    let [t1_attack, t2_attack, t3_attack] = boost::RANGED_ATTACK_BOOSTS.map(|b| PartSpec::new_boosted_part(Part::RangedAttack, b));
    let attack_partspecs_arr = [unboosted_attack, t1_attack, t2_attack, t3_attack];

    for (c, part) in KNOWN_PARTS_WITH_BODYGEN_CHARS {
        if part == Part::RangedAttack {
            continue;
        }

        let unboosted_other = PartSpec::new_unboosted_part(part);
        let boosted_other_partspecs_vec: Vec<PartSpec> = AbstractBoost::boosts_for_part(&part).into_iter()
                                                                                              .map(|b| PartSpec::new_boosted_part(part, *b))
                                                                                              .collect();
        let other_boost_vec: Vec<PartSpec> = std::iter::once(unboosted_other).chain(boosted_other_partspecs_vec).collect();

        for attack_partspec in attack_partspecs_arr {
            for other_partspec in &other_boost_vec {
                for num_parts_total in 1..=50 as usize {
                    for num_attack_parts in 0..num_parts_total as usize {
                        let num_other_parts: usize = num_parts_total.saturating_sub(num_attack_parts);

                        let mut attack_parts_vec: Vec<_> = std::iter::repeat(attack_partspec).take(num_attack_parts).collect();
                        let mut other_parts_vec: Vec<_> = std::iter::repeat(*other_partspec).take(num_other_parts).collect();

                        let mut partspec_vec = Vec::with_capacity(num_parts_total);
                        partspec_vec.append(&mut other_parts_vec);
                        partspec_vec.append(&mut attack_parts_vec);

                        let bodyspec = BodySpec::new(&partspec_vec);

                        let expected_ranged_attack = num_attack_parts as u32 * attack_partspec.get_ranged_attack_damage();
                        let expected_ranged_mass_attack = num_attack_parts as u32 * attack_partspec.get_ranged_mass_attack_damage_at_distance_single_target(1);

                        assert_eq!(expected_ranged_attack, bodyspec.ranged_attack_damage(), "BodySpec: {:?}", bodyspec);
                        assert_eq!(expected_ranged_mass_attack, bodyspec.ranged_mass_attack_damage_at_distance_single_target(1), "BodySpec: {:?}", bodyspec);
                    }
                }
            }
        }
    }
}

#[test]
fn bodyspec_heal_calculates_correctly() {
    // Heal and Ranged Heal are only influenced by Heal parts
    // Only the T{1,2,3}Heal boosts affect these
    let unboosted_heal = PartSpec::new_unboosted_part(Part::Heal);
    let [t1_heal, t2_heal, t3_heal] = boost::HEAL_BOOSTS.map(|b| PartSpec::new_boosted_part(Part::Heal, b));
    let heal_partspecs_arr = [unboosted_heal, t1_heal, t2_heal, t3_heal];

    for (_, part) in KNOWN_PARTS_WITH_BODYGEN_CHARS {
        if part == Part::Heal {
            continue;
        }

        let unboosted_other = PartSpec::new_unboosted_part(part);
        let boosted_other_partspecs_vec: Vec<PartSpec> = AbstractBoost::boosts_for_part(&part).into_iter()
                                                                                              .map(|b| PartSpec::new_boosted_part(part, *b))
                                                                                              .collect();
        let other_boost_vec: Vec<PartSpec> = std::iter::once(unboosted_other).chain(boosted_other_partspecs_vec).collect();

        for heal_partspec in heal_partspecs_arr {
            for other_partspec in &other_boost_vec {
                for num_parts_total in 1..=50 as usize {
                    for num_heal_parts in 0..num_parts_total as usize {
                        let num_other_parts: usize = num_parts_total.saturating_sub(num_heal_parts);

                        let mut heal_parts_vec: Vec<_> = std::iter::repeat(heal_partspec).take(num_heal_parts).collect();
                        let mut other_parts_vec: Vec<_> = std::iter::repeat(*other_partspec).take(num_other_parts).collect();

                        let mut partspec_vec = Vec::with_capacity(num_parts_total);
                        partspec_vec.append(&mut other_parts_vec);
                        partspec_vec.append(&mut heal_parts_vec);

                        let bodyspec = BodySpec::new(&partspec_vec);

                        let expected_heal = num_heal_parts as u32 * heal_partspec.get_heal_amount();
                        let expected_ranged_heal = num_heal_parts as u32 * heal_partspec.get_ranged_heal_amount();

                        assert_eq!(expected_heal, bodyspec.heal_amount(), "BodySpec: {:?}", bodyspec);
                        assert_eq!(expected_ranged_heal, bodyspec.ranged_heal_amount(), "BodySpec: {:?}", bodyspec);
                    }
                }
            }
        }
    }
}

#[test]
fn bodyspec_carry_calculates_correctly() {
    // Carry capacity is only influenced by Carry parts
    // Only the T{1,2,3}Carry boosts affect this
    let unboosted_carry = PartSpec::new_unboosted_part(Part::Carry);
    let [t1_carry, t2_carry, t3_carry] = boost::CARRY_BOOSTS.map(|b| PartSpec::new_boosted_part(Part::Carry, b));
    let carry_partspecs_arr = [unboosted_carry, t1_carry, t2_carry, t3_carry];

    for (_, part) in KNOWN_PARTS_WITH_BODYGEN_CHARS {
        if part == Part::Carry {
            continue;
        }

        let unboosted_other = PartSpec::new_unboosted_part(part);
        let boosted_other_partspecs_vec: Vec<PartSpec> = AbstractBoost::boosts_for_part(&part).into_iter()
                                                                                              .map(|b| PartSpec::new_boosted_part(part, *b))
                                                                                              .collect();
        let other_boost_vec: Vec<PartSpec> = std::iter::once(unboosted_other).chain(boosted_other_partspecs_vec).collect();

        for carry_partspec in carry_partspecs_arr {
            for other_partspec in &other_boost_vec {
                for num_parts_total in 1..=50 as usize {
                    for num_carry_parts in 0..num_parts_total as usize {
                        let num_other_parts: usize = num_parts_total.saturating_sub(num_carry_parts);

                        let mut carry_parts_vec: Vec<_> = std::iter::repeat(carry_partspec).take(num_carry_parts).collect();
                        let mut other_parts_vec: Vec<_> = std::iter::repeat(*other_partspec).take(num_other_parts).collect();

                        let mut partspec_vec = Vec::with_capacity(num_parts_total);
                        partspec_vec.append(&mut other_parts_vec);
                        partspec_vec.append(&mut carry_parts_vec);

                        let bodyspec = BodySpec::new(&partspec_vec);

                        let expected_carry = num_carry_parts as u32 * carry_partspec.get_carry_capacity();

                        assert_eq!(expected_carry, bodyspec.carry_capacity(), "BodySpec: {:?}", bodyspec);
                    }
                }
            }
        }
    }
}

