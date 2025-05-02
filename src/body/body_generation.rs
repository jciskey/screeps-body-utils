
use std::error::Error;
use std::fmt;

use screeps::Part;
use screeps::constants::MAX_CREEP_SIZE;

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
pub fn generate_body_from_string(body_string: &str) -> Result<Vec<Part>, GenerateBodyError> {
    let mut res = Vec::new();

    let part_groups = parse_part_groups(body_string);

    for (multiplier_string, parts_string) in part_groups {
        match multiplier_string.parse::<usize>() {
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

/// Converts a string slice into a Part.
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
/// Example:
/// "5T2MC" produces two part groups:
/// - ("5", "T")
/// - ("2", "MC")
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

