//! Abstraction of individual lab reactions.

use screeps::ResourceType;
use screeps::LAB_REACTION_AMOUNT;

use crate::boost::bom::ReactionBillOfMaterials;
use crate::helpers::functions::resource_is_lab_compound;

// Ultimately, we want a way to capture and work with the reaction chains specifically, and then
// convert those reaction chains into a bill of materials at different levels of complexity so that
// a bot can evaluate rooms for mineral harvesting, purchase supplies from the market, or just know
// how much of each resource they need to stockpile in order to make the amount they want of a
// particular boost.
// 
// A reaction should be a single struct that stores the output resource and the desired number of
// that output. We can derive the input resources from the output resource reaction_components
// method.
//
// A BoM should ultimately be a struct that stores the base resources needed to build up the
// desired resource from scratch.
//
// Having a way to say, "I want N quantity of XYZ product, I have A, B, and C resources in T, U, V
// quantities, what else do I need?" and get an answer would be nice.
// 
// 

/// Encapsulates a specific lab reaction intent, with both the desired output and the desired
/// amount of that output.
#[derive(Debug, PartialEq, Hash, Copy, Clone)]
pub struct Reaction {
    output: ResourceType,
    num_desired: u32,
}

pub enum ReactionValidationError {
    NotALabCompound,
}

impl Reaction {
    /// Generates a new Reaction, validating that the desired output resource type is actually a
    /// lab compound.
    pub const fn checked_new(output: ResourceType, num_desired: u32) -> Result<Self, ReactionValidationError> {
        let res = resource_is_lab_compound(&output);
        if res {
            Ok(Self::unchecked_new(output, num_desired))
        } else {
            Err(ReactionValidationError::NotALabCompound)
        }
    }

    /// Generates a new Reaction without validating that the desired output resource type is a lab
    /// compound. This will not result in undefined behavior, but can absolutely result in panics
    /// later when using the Reaction.
    ///
    /// Manual validation can be done via the [resource_is_lab_compound] function.
    pub const fn unchecked_new(output: ResourceType, num_desired: u32) -> Self {
        Self {
            output,
            num_desired,
        }
    }

    /// The resource type that the Reaction will produce.
    pub const fn output(&self) -> ResourceType {
        self.output
    }

    /// The desired amount of the output resource.
    pub const fn num_desired(&self) -> u32 {
        self.num_desired
    }

    /// The amount of time needed to perform this reaction.
    ///
    /// This will be the number of ticks needed to run enough reactions to produce the desired
    /// amount of the desired output resource.
    pub const fn reaction_time(&self) -> u32 {
        // Unwrap is safe here, because the output is either explicitly validated on creation of
        // the Reaction or contractually validated by the user.
        let reaction_time = self.output.reaction_time().unwrap();
        let needed_ops = self.num_desired.div_ceil(LAB_REACTION_AMOUNT);
        reaction_time * needed_ops
    }

    /// Generates a bill of materials listing all of the immediate resources necessary to produce
    /// the desired output at the desired amount.
    pub const fn components_needed(&self) -> ReactionBillOfMaterials {
        // Unwrap is safe because the user should be using either checked_new to validate the
        // output is a lab compound, or they've used unchecked_new and assumed the responsibility
        // for validity themselves.
        let comps = self.output.reaction_components().unwrap();
        let mut bom = ReactionBillOfMaterials::new();
        let mut i = 0;
        while i < comps.len() {
            bom.add_resource(&comps[i], self.num_desired());
            i += 1;
        }
        bom
    }

    /// Generates a bill of materials listing all of the basic minerals necessary to produce the
    /// desired output at the desired amount.
    pub const fn from_scratch_components_needed(&self) -> ReactionBillOfMaterials {
        let mut bom = self.components_needed();
        bom.reduce_all_resources_to_base_materials();
        bom
    }

    /// Gets the total reaction time needed to run an entire reaction chain.
    pub const fn reaction_time_for_chain(chain: &[Reaction]) -> u32 {
        let mut acc: u32 = 0;
        let mut i = 0;

        while i < chain.len() {
            acc = acc.saturating_add(chain[i].reaction_time());
            i += 1;
        }

        acc
    }
}
