use serde::{Serialize, Deserialize};

/// Represents the choice of what boosts, if any, are desired when constructing a creep body.
#[derive(Debug, Hash, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum BoostTierChoice {
    /// No boosts should be chosen
    NoBoosts,

    /// Only Tier 1 boosts should be chosen
    T1Only,

    /// Only Tier 2 boosts should be chosen
    T2Only,

    /// Only Tier 3 boosts should be chosen
    T3Only,

    /// Any boosts up to Tier 1 (including no boosts) may be chosen
    UpToT1,

    /// Any boosts up to Tier 2 (including no boosts) may be chosen
    UpToT2,

    /// Any boosts up to Tier 3 (including no boosts) may be chosen
    UpToT3,
}


/// Contains criteria for selecting how to choose boosts when generating a creep body.
///
/// If boost tier choice is [NoBoosts](BoostTierChoice::NoBoosts), [T1Only](BoostTierChoice::T1Only), [T2Only](BoostTierChoice::T2Only), or [T3Only](BoostTierChoice::T3Only), `allow_partial_boosts` will be ignored and all parts will have their boosts set to the designated tier.
#[derive(Debug, Hash, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct BoostSelectionConfig {
    /// How to select the tier of any particular boost
    pub boost_tier_choice: BoostTierChoice,

    /// Whether all parts in a body must be boosted.
    pub allow_partial_boosts: bool,
}

impl BoostSelectionConfig {
    pub fn new(boost_tier_choice: BoostTierChoice, allow_partial_boosts: bool) -> BoostSelectionConfig {
        BoostSelectionConfig {
            boost_tier_choice,
            allow_partial_boosts,
        }
    }
}

impl Default for BoostSelectionConfig {
    fn default() -> BoostSelectionConfig {
        BoostSelectionConfig::new(BoostTierChoice::NoBoosts, false)
    }
}
