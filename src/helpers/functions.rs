use core::cmp::Ordering;

use const_soft_float::soft_f32::SoftF32;

use screeps::constants::ResourceType;

/// Returns the minimum of two u32 values.
pub const fn const_min_u32(a: u32, b: u32) -> u32 {
    if let Some(_) = a.checked_sub(b) {
        // a is larger than b, because it did not overflow, so we want to return b
        b
    } else {
        // b is larger than a, because it overflowed, so we want to return a
        a
    }
}

/// Returns the maximum of two u32 values.
pub const fn const_max_u32(a: u32, b: u32) -> u32 {
    if let Some(_) = a.checked_sub(b) {
        // a is larger than b, because it did not overflow, so we want to return a
        a
    } else {
        // b is larger than a, because it overflowed, so we want to return b
        b
    }
}

/// Returns true if a < b.
pub const fn const_lt_u32(a: u32, b: u32) -> bool {
    a < b
}

/// Returns true if a <= b.
pub const fn const_lte_u32(a: u32, b: u32) -> bool {
    a <= b
}

/// Returns true if a > b.
pub const fn const_gt_u32(a: u32, b: u32) -> bool {
    a > b
}

/// Returns true if a >= b.
pub const fn const_gte_u32(a: u32, b: u32) -> bool {
    a >= b
}

/// Returns true if a < b.
pub const fn const_lt_f32(a: f32, b: f32) -> bool {
    a < b
}

/// Returns true if a <= b.
pub const fn const_lte_f32(a: f32, b: f32) -> bool {
    a <= b
}

/// Returns true if a > b.
pub const fn const_gt_f32(a: f32, b: f32) -> bool {
    a > b
}

/// Returns true if a >= b.
pub const fn const_gte_f32(a: f32, b: f32) -> bool {
    a >= b
}

/// Returns the smallest integer (as a float) that is
/// greater than or equal to the provided float.
///
/// Will panic if the provided float is NaN.
pub const fn const_ceil_f32(a: f32) -> f32 {
    let v = SoftF32::from_f32(a);
    let floored = v.floor();
    let ordering = v.cmp(floored).unwrap();
    match ordering {
        Ordering::Equal => a, // If a == floor(a), then a is an integer and can be returned directly
        Ordering::Less => floored.to_f32(), // If a < floor(a), then something has gone very wrong, return floor(a)
        Ordering::Greater => floored.add(SoftF32::from_f32(1.0)).to_f32(), // If a > floor(a), then a had a decimal component and we should add 1 to the floor
    }
}

/// Returns the largest integer (as a float) that is
/// less than or equal to the provided float.
pub const fn const_floor_f32(a: f32) -> f32 {
    SoftF32::from_f32(a).floor().to_f32()
}

pub const fn resource_is_lab_compound(resource: &ResourceType) -> bool {
    use ResourceType::*;
    match resource {
        // Base compounds
        Hydroxide => true,
        ZynthiumKeanite => true,
        UtriumLemergite => true,
        Ghodium => true,

        // T1 Boost compounds
        UtriumHydride => true,
        UtriumOxide => true,
        KeaniumHydride => true,
        KeaniumOxide => true,
        LemergiumHydride => true,
        LemergiumOxide => true,
        ZynthiumHydride => true,
        ZynthiumOxide => true,
        GhodiumHydride => true,
        GhodiumOxide => true,

        // T2 Boost compounds
        UtriumAcid => true,
        UtriumAlkalide => true,
        KeaniumAcid => true,
        KeaniumAlkalide => true,
        LemergiumAcid => true,
        LemergiumAlkalide => true,
        ZynthiumAcid => true,
        ZynthiumAlkalide => true,
        GhodiumAcid => true,
        GhodiumAlkalide => true,

        // T3 Boost compounds
        CatalyzedUtriumAcid => true,
        CatalyzedUtriumAlkalide => true,
        CatalyzedKeaniumAcid => true,
        CatalyzedKeaniumAlkalide => true,
        CatalyzedLemergiumAcid => true,
        CatalyzedLemergiumAlkalide => true,
        CatalyzedZynthiumAcid => true,
        CatalyzedZynthiumAlkalide => true,
        CatalyzedGhodiumAcid => true,
        CatalyzedGhodiumAlkalide => true,

        _ => false,
    }
}

#[cfg(test)]
mod function_tests {
    use std::collections::HashSet;
    use crate::constants::resources;
    use super::*;

    #[test]
    fn const_min_u32_calculates_consistently() {
        let r1 = const_min_u32(50, 2);
        let r2 = const_min_u32(2, 50);
        assert_eq!(r1, r2);
    }

    #[test]
    fn const_min_u32_calculates_correctly() {
        let bigger = 50;
        let smaller = 10;
        assert_eq!(const_min_u32(bigger, smaller), smaller);
    }

    #[test]
    fn const_max_u32_calculates_consistently() {
        let r1 = const_max_u32(50, 2);
        let r2 = const_max_u32(2, 50);
        assert_eq!(r1, r2);
    }

    #[test]
    fn const_max_u32_calculates_correctly() {
        let bigger = 50;
        let smaller = 10;
        assert_eq!(const_max_u32(bigger, smaller), bigger);
    }

    #[test]
    fn const_lt_u32_calculates_correctly() {
        let bigger = 50;
        let smaller = 10;
        assert!(const_lt_u32(smaller, bigger));
        assert!(!const_lt_u32(bigger, smaller));
        assert!(!const_lt_u32(smaller, smaller));
    }

    #[test]
    fn const_lte_u32_calculates_correctly() {
        let bigger = 50;
        let smaller = 10;
        assert!(const_lte_u32(smaller, bigger));
        assert!(!const_lte_u32(bigger, smaller));
        assert!(const_lte_u32(smaller, smaller));
    }

    #[test]
    fn const_gt_u32_calculates_correctly() {
        let bigger = 50;
        let smaller = 10;
        assert!(const_gt_u32(bigger, smaller));
        assert!(!const_gt_u32(smaller, bigger));
        assert!(!const_gt_u32(smaller, smaller));
    }

    #[test]
    fn const_gte_u32_calculates_correctly() {
        let bigger = 50;
        let smaller = 10;
        assert!(const_gte_u32(bigger, smaller));
        assert!(!const_gte_u32(smaller, bigger));
        assert!(const_gte_u32(smaller, smaller));
    }

    #[test]
    fn const_lt_f32_calculates_correctly() {
        let bigger = 50.0;
        let smaller = 10.0;
        assert!(const_lt_f32(smaller, bigger));
        assert!(!const_lt_f32(bigger, smaller));
        assert!(!const_lt_f32(smaller, smaller));
    }

    #[test]
    fn const_lte_f32_calculates_correctly() {
        let bigger = 50.0;
        let smaller = 10.0;
        assert!(const_lte_f32(smaller, bigger));
        assert!(!const_lte_f32(bigger, smaller));
        assert!(const_lte_f32(smaller, smaller));
    }

    #[test]
    fn const_gt_f32_calculates_correctly() {
        let bigger = 50.0;
        let smaller = 10.0;
        assert!(const_gt_f32(bigger, smaller));
        assert!(!const_gt_f32(smaller, bigger));
        assert!(!const_gt_f32(smaller, smaller));
    }

    #[test]
    fn const_gte_f32_calculates_correctly() {
        let bigger = 50.0;
        let smaller = 10.0;
        assert!(const_gte_f32(bigger, smaller));
        assert!(!const_gte_f32(smaller, bigger));
        assert!(const_gte_f32(smaller, smaller));
    }

    #[test]
    fn const_ceil_f32_calculates_correctly() {
        assert_eq!(const_ceil_f32(50.0), 50.0);
        assert_eq!(const_ceil_f32(50.5), 51.0);
        assert_eq!(const_ceil_f32(-50.5), -50.0);
    }

    #[test]
    fn const_floor_f32_calculates_correctly() {
        assert_eq!(const_floor_f32(50.0), 50.0);
        assert_eq!(const_floor_f32(50.5), 50.0);
        assert_eq!(const_floor_f32(-50.5), -51.0);
    }

    #[test]
    fn resource_is_lab_compound_works_properly() {
        use enum_iterator::all;

        // Create a set of all the known lab compounds
        let mut lab_compounds = HashSet::new();
        for r in resources::BASE_COMPOUNDS {
            lab_compounds.insert(r);
        }
        for r in resources::T1_RESOURCES {
            lab_compounds.insert(r);
        }
        for r in resources::T2_RESOURCES {
            lab_compounds.insert(r);
        }
        for r in resources::T3_RESOURCES {
            lab_compounds.insert(r);
        }

        // Test that every single resource returns the correct value;
        // if the resource is one of the lab compounds, it will be in the set, and thus match the
        // return value
        // if the resource is not one of the lab compounds, it will not be in the set, and thus
        // match the return value
        for r in all::<ResourceType>().collect::<Vec<_>>() {
            assert_eq!(resource_is_lab_compound(&r), lab_compounds.contains(&r));
        }
    }

}
