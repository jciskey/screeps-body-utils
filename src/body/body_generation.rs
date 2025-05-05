
use std::error::Error;
use std::fmt;

use screeps::Part;
use screeps::constants::MAX_CREEP_SIZE;

use crate::body::{BodySpec, PartSpec};

/// Dedicated error describing what went wrong when parsing a body spec.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GenerateBodyError {
    InvalidString,
    TooManyBodyParts,
    InvalidMultiplier,
}

impl fmt::Display for GenerateBodyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::InvalidString => "Body string is invalid",
            Self::TooManyBodyParts => "Too many parts specified",
            Self::InvalidMultiplier => "Invalid multiplier",
        };
        write!(f, "{}", s)
    }
}

impl Error for GenerateBodyError {}

/// Converts a creep body spec string (i.e. "2MC") into a list of Parts that can be used for spawning.
///
/// ```rust
/// use screeps::Part;
/// use screeps_body_utils::body::body_specification::generate_body_from_string;
///
/// let rcl_1_worker = "WM";
/// let res = generate_body_from_string(rcl_1_worker);
/// assert!(res.is_ok());
///
/// let body = res.unwrap();
/// assert_eq!(2, body.len());
/// assert_eq!(Part::Work, body[0]);
/// assert_eq!(Part::Move, body[1]);
/// ```
pub fn generate_body_from_string(body_string: &str) -> Result<Vec<Part>, GenerateBodyError> {
    let mut res = Vec::new();

    let part_groups = parse_part_groups(body_string);

    for (multiplier_string, parts_string) in part_groups {
        let multiplier_parse_res = if multiplier_string == "" {
            Ok(1)
        } else {
            multiplier_string.parse::<usize>()
        };
        match multiplier_parse_res {
            Ok(multiplier) => {
                let parts_vec: Vec<Part> = parts_string.split("").filter_map(convert_character_to_part).collect();
                if parts_vec.len() > 0 {
                    let full_parts_vec: Vec<Part> = (&parts_vec).repeat(multiplier);
                    res.push(full_parts_vec);
                }
            },
            Err(_msg) => {
                return Err(GenerateBodyError::InvalidMultiplier);
            },
        };
    }

    let output_vec: Vec<_> = res.into_iter().flatten().collect();

    if output_vec.len() > MAX_CREEP_SIZE as usize {
        Err(GenerateBodyError::TooManyBodyParts)
    }
    else {
        Ok(output_vec)
    }
}

/// Converts a creep body spec string (i.e. "2MC") into a BodySpec that can be used for
/// calculations.
///
/// ```rust
/// use screeps::Part;
/// use screeps_body_utils::body::body_specification::generate_bodyspec_from_string;
///
/// let rcl_1_worker = "WM";
/// let res = generate_bodyspec_from_string(rcl_1_worker);
/// assert!(res.is_ok());
///
/// let bodyspec = res.unwrap();
/// let parts = bodyspec.get_parts();
/// assert_eq!(2, parts.len());
/// assert_eq!(Part::Work, parts[0]);
/// assert_eq!(Part::Move, parts[1]);
/// ```
pub fn generate_bodyspec_from_string(body_string: &str) -> Result<BodySpec, GenerateBodyError> {
    let parts_vec = generate_body_from_string(body_string)?;
    Ok(BodySpec::from(parts_vec))
}

/// Converts a string slice into a Part.
///
/// ```rust
/// use screeps::Part;
/// use screeps_body_utils::body::body_specification::convert_character_to_part;
///
/// let res = convert_character_to_part("W");
/// assert!(res.is_some());
///
/// let part = res.unwrap();
/// assert_eq!(Part::Work, part);
/// ```
pub fn convert_character_to_part(char_slice: &str) -> Option<Part> {
    match char_slice {
        "M" => Some(Part::Move),
        "W" => Some(Part::Work),
        "C" => Some(Part::Carry),
        "A" => Some(Part::Attack),
        "R" => Some(Part::RangedAttack),
        "T" => Some(Part::Tough),
        "H" => Some(Part::Heal),
        "L" => Some(Part::Claim),
        _ => None,
    }
}

/// Splits up a creep body spec string into individual part groups and multipliers.
///
/// ```rust
/// use screeps::Part;
/// use screeps_body_utils::body::body_specification::parse_part_groups;
///
/// let groups_vec = parse_part_groups("5T2MC");
/// assert_eq!(2, groups_vec.len());
///
/// assert_eq!(("5".to_string(), "T".to_string()), groups_vec[0]);
/// assert_eq!(("2".to_string(), "MC".to_string()), groups_vec[1]);
/// ```
pub fn parse_part_groups(body_string: &str) -> Vec<(String, String)> {
    let mut part_groups = Vec::new();

    let mut multiplier_accumulator_vec: Vec<char> = Vec::new();
    let mut parts_accumulator_vec: Vec<char> = Vec::new();
    for character in body_string.chars() {
        if character.is_ascii_digit() {
            if parts_accumulator_vec.len() > 0 {
                // If there are previously-parsed-parts, then we've moved to a
                // new group and need to store the old one before starting on the new group
                let multipler_string: String = multiplier_accumulator_vec.into_iter().collect();
                let parts_string: String = parts_accumulator_vec.into_iter().collect();
                let tpl = (multipler_string, parts_string);
                part_groups.push(tpl);

                multiplier_accumulator_vec = Vec::new();
                parts_accumulator_vec = Vec::new();
            }
            multiplier_accumulator_vec.push(character);
        }
        else {
            parts_accumulator_vec.push(character);
        }
    }

    // Aggregate the final group into the output
    if parts_accumulator_vec.len() > 0 {
        let multipler_string: String = multiplier_accumulator_vec.into_iter().collect();
        let parts_string: String = parts_accumulator_vec.into_iter().collect();
        let tpl = (multipler_string, parts_string);
        part_groups.push(tpl);
    }

    // Return the parsed part groups
    part_groups
}

