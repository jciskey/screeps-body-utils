
use itertools::Itertools;

use screeps::Part;

use super::body_generation;


static KNOWN_PARTS: [(&str, Part); 8] = [
        ("M", Part::Move),
        ("W", Part::Work),
        ("C", Part::Carry),
        ("A", Part::Attack),
        ("R", Part::RangedAttack),
        ("T", Part::Tough),
        ("H", Part::Heal),
        ("L", Part::Claim),
    ];

#[test]
fn part_conversion_works_for_known_parts() {
    for (char_slice, part) in KNOWN_PARTS {
        let res = body_generation::convert_character_to_part(char_slice);
        assert!(res.is_some());
        assert_eq!(res.unwrap(), part);
    }
}

#[test]
fn part_conversion_returns_none_for_unknown_characters() {
    let unknown_parts = vec!("Z", "Cl", "N", "Q");
    let known_slices: Vec<&str> = KNOWN_PARTS.iter().map(|tpl| tpl.0).collect();
    for char_slice in unknown_parts {
        assert!(!known_slices.iter().contains(&char_slice));
        let res = body_generation::convert_character_to_part(char_slice);
        assert!(res.is_none());
    }
}


#[test]
fn empty_body_string_produces_empty_body() {
    let s = "";
    let result = body_generation::generate_body_from_string(&s);

    assert!(result.is_ok());

    let body = result.unwrap();
    assert_eq!(body.len(), 0);
}

#[test]
fn one_part_body_string_produces_one_part_body_single_part() {
    let s = "1M";
    let result = body_generation::generate_body_from_string(&s);

    assert!(result.is_ok());

    let body = result.unwrap();
    assert_eq!(body.len(), 1);
    assert_eq!(body[0], Part::Move);
}

#[test]
fn two_part_body_string_produces_two_part_body_single_part() {
    let s = "2M";
    let result = body_generation::generate_body_from_string(&s);

    assert!(result.is_ok());

    let body = result.unwrap();
    assert_eq!(body.len(), 2);
    assert_eq!(body[0], Part::Move);
    assert_eq!(body[1], Part::Move);
}

#[test]
fn two_part_body_string_produces_two_part_body_multiple_parts() {
    let s = "1M1C";
    let result = body_generation::generate_body_from_string(&s);

    assert!(result.is_ok());

    let body = result.unwrap();
    assert_eq!(body.len(), 2);
    assert_eq!(body[0], Part::Move);
    assert_eq!(body[1], Part::Carry);
}

#[test]
fn count_prefix_applies_to_only_subsequent_parts_group() {
    let s = "2M3C";
    let result = body_generation::generate_body_from_string(&s);

    assert!(result.is_ok());

    let body = result.unwrap();
    assert_eq!(body.len(), 5);
    assert_eq!(body[0], Part::Move);
    assert_eq!(body[1], Part::Move);
    assert_eq!(body[2], Part::Carry);
    assert_eq!(body[3], Part::Carry);
    assert_eq!(body[4], Part::Carry);
}

#[test]
fn count_prefix_applies_to_entire_parts_group() {
    let s = "2MCR";
    let result = body_generation::generate_body_from_string(&s);

    assert!(result.is_ok());

    let body = result.unwrap();
    assert_eq!(body.len(), 6);
    assert_eq!(body[0], Part::Move);
    assert_eq!(body[1], Part::Carry);
    assert_eq!(body[2], Part::RangedAttack);
    assert_eq!(body[3], Part::Move);
    assert_eq!(body[4], Part::Carry);
    assert_eq!(body[5], Part::RangedAttack);
}

#[test]
fn unmerged_part_groups_produce_parts_ordered_correctly() {
    let s = "2MC1H1CM";
    let result = body_generation::generate_body_from_string(&s);

    assert!(result.is_ok());

    let body = result.unwrap();
    assert_eq!(body.len(), 7);
    assert_eq!(body[0], Part::Move);
    assert_eq!(body[1], Part::Carry);
    assert_eq!(body[2], Part::Move);
    assert_eq!(body[3], Part::Carry);
    assert_eq!(body[4], Part::Heal);
    assert_eq!(body[5], Part::Carry);
    assert_eq!(body[6], Part::Move);
}

#[test]
fn multidigit_count_prefixes_work_correctly() {
    let s = "11T4H15M";
    let result = body_generation::generate_body_from_string(&s);

    assert!(result.is_ok());

    let body = result.unwrap();
    assert_eq!(body.len(), 30);
    for idx in 0..11 {
        assert_eq!(body[idx], Part::Tough);
    }
    for idx in 11..14 {
        assert_eq!(body[idx], Part::Heal);
    }
    for idx in 15..30 {
        assert_eq!(body[idx], Part::Move);
    }
}

#[test]
fn too_many_parts_returns_error() {
    let s = "51W";
    let result = body_generation::generate_body_from_string(&s);

    assert!(result.is_err());

    let error = result.unwrap_err();
    assert_eq!(error, body_generation::GenerateBodyError::TooManyBodyParts);
}

