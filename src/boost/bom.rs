//! Utilities to help quantify needed materials for lab compound transmutation.

use std::iter::{FusedIterator, ExactSizeIterator};

use screeps::constants::ResourceType;
use screeps::constants::LAB_REACTION_AMOUNT;

use crate::constants::resources::{
    LAB_RESOURCES,
    BASE_MINERALS,
    BASE_COMPOUNDS,
    T1_RESOURCES,
    T2_RESOURCES,
    T3_RESOURCES,
};
use crate::boost::reaction::Reaction;
use crate::helpers::functions::const_min_u32;

/// Encapsulates a list of minerals and mineral compounds and the amounts of each.
///
/// This can be used to determine what resources are needed to get a certain quantity of boosts,
/// with helper methods for breaking down specific compounds into base components and amounts.
///
/// Most methods are const, so that the calculations can be done at compile time instead of runtime
/// if defining hardcoded bills of materials for specific boosts.
#[derive(Debug, PartialEq, Hash, Copy, Clone)]
pub struct ReactionBillOfMaterials {
    // Raw minerals
    pub hydrogen: u32,
    pub oxygen: u32,
    pub utrium: u32,
    pub keanium: u32,
    pub lemergium: u32,
    pub zynthium: u32,
    pub catalyst: u32,

    // Base compounds
    pub ghodium: u32,
    pub hydroxide: u32,
    pub zynthium_keanite: u32,
    pub utrium_lemergite: u32,

    // Tier 1 boosts
    pub utrium_hydride: u32,
    pub utrium_oxide: u32,
    pub keanium_hydride: u32,
    pub keanium_oxide: u32,
    pub lemergium_hydride: u32,
    pub lemergium_oxide: u32,
    pub zynthium_hydride: u32,
    pub zynthium_oxide: u32,
    pub ghodium_hydride: u32,
    pub ghodium_oxide: u32,

    // Tier 2 boosts
    pub utrium_acid: u32,
    pub utrium_alkalide: u32,
    pub keanium_acid: u32,
    pub keanium_alkalide: u32,
    pub lemergium_acid: u32,
    pub lemergium_alkalide: u32,
    pub zynthium_acid: u32,
    pub zynthium_alkalide: u32,
    pub ghodium_acid: u32,
    pub ghodium_alkalide: u32,

    // Tier 3 boosts
    pub catalyzed_utrium_acid: u32,
    pub catalyzed_utrium_alkalide: u32,
    pub catalyzed_keanium_acid: u32,
    pub catalyzed_keanium_alkalide: u32,
    pub catalyzed_lemergium_acid: u32,
    pub catalyzed_lemergium_alkalide: u32,
    pub catalyzed_zynthium_acid: u32,
    pub catalyzed_zynthium_alkalide: u32,
    pub catalyzed_ghodium_acid: u32,
    pub catalyzed_ghodium_alkalide: u32,

}

impl ReactionBillOfMaterials {
    /// Generates an empty BOM.
    pub const fn new() -> Self {
        Self::default()
    }

    /// Generates an empty BOM.
    pub const fn default() -> Self {
        Self {
            // Raw minerals
            hydrogen: 0,
            oxygen: 0,
            utrium: 0,
            keanium: 0,
            lemergium: 0,
            zynthium: 0,
            catalyst: 0,

            // Base compounds
            ghodium: 0,
            hydroxide: 0,
            zynthium_keanite: 0,
            utrium_lemergite: 0,

            // Tier 1 boosts
            utrium_hydride: 0,
            utrium_oxide: 0,
            keanium_hydride: 0,
            keanium_oxide: 0,
            lemergium_hydride: 0,
            lemergium_oxide: 0,
            zynthium_hydride: 0,
            zynthium_oxide: 0,
            ghodium_hydride: 0,
            ghodium_oxide: 0,

            // Tier 2 boosts
            utrium_acid: 0,
            utrium_alkalide: 0,
            keanium_acid: 0,
            keanium_alkalide: 0,
            lemergium_acid: 0,
            lemergium_alkalide: 0,
            zynthium_acid: 0,
            zynthium_alkalide: 0,
            ghodium_acid: 0,
            ghodium_alkalide: 0,

            // Tier 3 boosts
            catalyzed_utrium_acid: 0,
            catalyzed_utrium_alkalide: 0,
            catalyzed_keanium_acid: 0,
            catalyzed_keanium_alkalide: 0,
            catalyzed_lemergium_acid: 0,
            catalyzed_lemergium_alkalide: 0,
            catalyzed_zynthium_acid: 0,
            catalyzed_zynthium_alkalide: 0,
            catalyzed_ghodium_acid: 0,
            catalyzed_ghodium_alkalide: 0,
        }
    }

    /// Adds the specified amount of the specified resource to this BOM, saturating at [u32::MAX].
    pub const fn add_resource(&mut self, resource: &ResourceType, amount: u32) {
        use ResourceType::*;
        match resource {
            // Raw minerals
            Hydrogen => self.hydrogen = self.hydrogen.saturating_add(amount),
            Oxygen => self.oxygen = self.oxygen.saturating_add(amount),
            Utrium => self.utrium = self.utrium.saturating_add(amount),
            Keanium => self.keanium = self.keanium.saturating_add(amount),
            Lemergium => self.lemergium = self.lemergium.saturating_add(amount),
            Zynthium => self.zynthium = self.zynthium.saturating_add(amount),
            Catalyst => self.catalyst = self.catalyst.saturating_add(amount),

            // Base compounds
            Ghodium => self.ghodium = self.ghodium.saturating_add(amount),
            Hydroxide => self.hydroxide = self.hydroxide.saturating_add(amount),
            ZynthiumKeanite => self.zynthium_keanite = self.zynthium_keanite.saturating_add(amount),
            UtriumLemergite => self.utrium_lemergite = self.utrium_lemergite.saturating_add(amount),

            // T1 Boost Resources
            UtriumHydride => self.utrium_hydride = self.utrium_hydride.saturating_add(amount),
            UtriumOxide => self.utrium_oxide = self.utrium_oxide.saturating_add(amount),
            KeaniumHydride => self.keanium_hydride = self.keanium_hydride.saturating_add(amount),
            KeaniumOxide => self.keanium_oxide = self.keanium_oxide.saturating_add(amount),
            LemergiumHydride => self.lemergium_hydride = self.lemergium_hydride.saturating_add(amount),
            LemergiumOxide => self.lemergium_oxide = self.lemergium_oxide.saturating_add(amount),
            ZynthiumHydride => self.zynthium_hydride = self.zynthium_hydride.saturating_add(amount),
            ZynthiumOxide => self.zynthium_oxide = self.zynthium_oxide.saturating_add(amount),
            GhodiumHydride => self.ghodium_hydride = self.ghodium_hydride.saturating_add(amount),
            GhodiumOxide => self.ghodium_oxide = self.ghodium_oxide.saturating_add(amount),

            // T2 Boost Resources
            UtriumAcid => self.utrium_acid = self.utrium_acid.saturating_add(amount),
            UtriumAlkalide => self.utrium_alkalide = self.utrium_alkalide.saturating_add(amount),
            KeaniumAcid => self.keanium_acid = self.keanium_acid.saturating_add(amount),
            KeaniumAlkalide => self.keanium_alkalide = self.keanium_alkalide.saturating_add(amount),
            LemergiumAcid => self.lemergium_acid = self.lemergium_acid.saturating_add(amount),
            LemergiumAlkalide => self.lemergium_alkalide = self.lemergium_alkalide.saturating_add(amount),
            ZynthiumAcid => self.zynthium_acid = self.zynthium_acid.saturating_add(amount),
            ZynthiumAlkalide => self.zynthium_alkalide = self.zynthium_alkalide.saturating_add(amount),
            GhodiumAcid => self.ghodium_acid = self.ghodium_acid.saturating_add(amount),
            GhodiumAlkalide => self.ghodium_alkalide = self.ghodium_alkalide.saturating_add(amount),

            // T3 Boost Resources
            CatalyzedUtriumAcid => self.catalyzed_utrium_acid = self.catalyzed_utrium_acid.saturating_add(amount),
            CatalyzedUtriumAlkalide => self.catalyzed_utrium_alkalide = self.catalyzed_utrium_alkalide.saturating_add(amount),
            CatalyzedKeaniumAcid => self.catalyzed_keanium_acid = self.catalyzed_keanium_acid.saturating_add(amount),
            CatalyzedKeaniumAlkalide => self.catalyzed_keanium_alkalide = self.catalyzed_keanium_alkalide.saturating_add(amount),
            CatalyzedLemergiumAcid => self.catalyzed_lemergium_acid = self.catalyzed_lemergium_acid.saturating_add(amount),
            CatalyzedLemergiumAlkalide => self.catalyzed_lemergium_alkalide = self.catalyzed_lemergium_alkalide.saturating_add(amount),
            CatalyzedZynthiumAcid => self.catalyzed_zynthium_acid = self.catalyzed_zynthium_acid.saturating_add(amount),
            CatalyzedZynthiumAlkalide => self.catalyzed_zynthium_alkalide = self.catalyzed_zynthium_alkalide.saturating_add(amount),
            CatalyzedGhodiumAcid => self.catalyzed_ghodium_acid = self.catalyzed_ghodium_acid.saturating_add(amount),
            CatalyzedGhodiumAlkalide => self.catalyzed_ghodium_alkalide = self.catalyzed_ghodium_alkalide.saturating_add(amount),

            // Catch-all -- do nothing
            _ => {},
        };
    }

    /// Removes the specified amount of the specified resource from this BOM, saturating at 0.
    pub const fn remove_resource(&mut self, resource: &ResourceType, amount: u32) {
        use ResourceType::*;
        match resource {
            // Raw minerals
            Hydrogen => self.hydrogen = self.hydrogen.saturating_sub(amount),
            Oxygen => self.oxygen = self.oxygen.saturating_sub(amount),
            Utrium => self.utrium = self.utrium.saturating_sub(amount),
            Keanium => self.keanium = self.keanium.saturating_sub(amount),
            Lemergium => self.lemergium = self.lemergium.saturating_sub(amount),
            Zynthium => self.zynthium = self.zynthium.saturating_sub(amount),
            Catalyst => self.catalyst = self.catalyst.saturating_sub(amount),

            // Base compounds
            Ghodium => self.ghodium = self.ghodium.saturating_sub(amount),
            Hydroxide => self.hydroxide = self.hydroxide.saturating_sub(amount),
            ZynthiumKeanite => self.zynthium_keanite = self.zynthium_keanite.saturating_sub(amount),
            UtriumLemergite => self.utrium_lemergite = self.utrium_lemergite.saturating_sub(amount),

            // T1 Boost Resources
            UtriumHydride => self.utrium_hydride = self.utrium_hydride.saturating_sub(amount),
            UtriumOxide => self.utrium_oxide = self.utrium_oxide.saturating_sub(amount),
            KeaniumHydride => self.keanium_hydride = self.keanium_hydride.saturating_sub(amount),
            KeaniumOxide => self.keanium_oxide = self.keanium_oxide.saturating_sub(amount),
            LemergiumHydride => self.lemergium_hydride = self.lemergium_hydride.saturating_sub(amount),
            LemergiumOxide => self.lemergium_oxide = self.lemergium_oxide.saturating_sub(amount),
            ZynthiumHydride => self.zynthium_hydride = self.zynthium_hydride.saturating_sub(amount),
            ZynthiumOxide => self.zynthium_oxide = self.zynthium_oxide.saturating_sub(amount),
            GhodiumHydride => self.ghodium_hydride = self.ghodium_hydride.saturating_sub(amount),
            GhodiumOxide => self.ghodium_oxide = self.ghodium_oxide.saturating_sub(amount),

            // T2 Boost Resources
            UtriumAcid => self.utrium_acid = self.utrium_acid.saturating_sub(amount),
            UtriumAlkalide => self.utrium_alkalide = self.utrium_alkalide.saturating_sub(amount),
            KeaniumAcid => self.keanium_acid = self.keanium_acid.saturating_sub(amount),
            KeaniumAlkalide => self.keanium_alkalide = self.keanium_alkalide.saturating_sub(amount),
            LemergiumAcid => self.lemergium_acid = self.lemergium_acid.saturating_sub(amount),
            LemergiumAlkalide => self.lemergium_alkalide = self.lemergium_alkalide.saturating_sub(amount),
            ZynthiumAcid => self.zynthium_acid = self.zynthium_acid.saturating_sub(amount),
            ZynthiumAlkalide => self.zynthium_alkalide = self.zynthium_alkalide.saturating_sub(amount),
            GhodiumAcid => self.ghodium_acid = self.ghodium_acid.saturating_sub(amount),
            GhodiumAlkalide => self.ghodium_alkalide = self.ghodium_alkalide.saturating_sub(amount),

            // T3 Boost Resources
            CatalyzedUtriumAcid => self.catalyzed_utrium_acid = self.catalyzed_utrium_acid.saturating_sub(amount),
            CatalyzedUtriumAlkalide => self.catalyzed_utrium_alkalide = self.catalyzed_utrium_alkalide.saturating_sub(amount),
            CatalyzedKeaniumAcid => self.catalyzed_keanium_acid = self.catalyzed_keanium_acid.saturating_sub(amount),
            CatalyzedKeaniumAlkalide => self.catalyzed_keanium_alkalide = self.catalyzed_keanium_alkalide.saturating_sub(amount),
            CatalyzedLemergiumAcid => self.catalyzed_lemergium_acid = self.catalyzed_lemergium_acid.saturating_sub(amount),
            CatalyzedLemergiumAlkalide => self.catalyzed_lemergium_alkalide = self.catalyzed_lemergium_alkalide.saturating_sub(amount),
            CatalyzedZynthiumAcid => self.catalyzed_zynthium_acid = self.catalyzed_zynthium_acid.saturating_sub(amount),
            CatalyzedZynthiumAlkalide => self.catalyzed_zynthium_alkalide = self.catalyzed_zynthium_alkalide.saturating_sub(amount),
            CatalyzedGhodiumAcid => self.catalyzed_ghodium_acid = self.catalyzed_ghodium_acid.saturating_sub(amount),
            CatalyzedGhodiumAlkalide => self.catalyzed_ghodium_alkalide = self.catalyzed_ghodium_alkalide.saturating_sub(amount),

            // Catch-all -- do nothing
            _ => {},
        };
    }

    /// Returns the current amount of the specified resource in this BOM.
    pub const fn resource_amount(&self, resource: &ResourceType) -> u32 {
        use ResourceType::*;
        match resource {
            // Raw minerals
            Hydrogen => self.hydrogen,
            Oxygen => self.oxygen,
            Utrium => self.utrium,
            Keanium => self.keanium,
            Lemergium => self.lemergium,
            Zynthium => self.zynthium,
            Catalyst => self.catalyst,

            // Base compounds
            Ghodium => self.ghodium,
            Hydroxide => self.hydroxide,
            ZynthiumKeanite => self.zynthium_keanite,
            UtriumLemergite => self.utrium_lemergite,

            // T1 Boost Resources
            UtriumHydride => self.utrium_hydride,
            UtriumOxide => self.utrium_oxide,
            KeaniumHydride => self.keanium_hydride,
            KeaniumOxide => self.keanium_oxide,
            LemergiumHydride => self.lemergium_hydride,
            LemergiumOxide => self.lemergium_oxide,
            ZynthiumHydride => self.zynthium_hydride,
            ZynthiumOxide => self.zynthium_oxide,
            GhodiumHydride => self.ghodium_hydride,
            GhodiumOxide => self.ghodium_oxide,

            // T2 Boost Resources
            UtriumAcid => self.utrium_acid,
            UtriumAlkalide => self.utrium_alkalide,
            KeaniumAcid => self.keanium_acid,
            KeaniumAlkalide => self.keanium_alkalide,
            LemergiumAcid => self.lemergium_acid,
            LemergiumAlkalide => self.lemergium_alkalide,
            ZynthiumAcid => self.zynthium_acid,
            ZynthiumAlkalide => self.zynthium_alkalide,
            GhodiumAcid => self.ghodium_acid,
            GhodiumAlkalide => self.ghodium_alkalide,

            // T3 Boost Resources
            CatalyzedUtriumAcid => self.catalyzed_utrium_acid,
            CatalyzedUtriumAlkalide => self.catalyzed_utrium_alkalide,
            CatalyzedKeaniumAcid => self.catalyzed_keanium_acid,
            CatalyzedKeaniumAlkalide => self.catalyzed_keanium_alkalide,
            CatalyzedLemergiumAcid => self.catalyzed_lemergium_acid,
            CatalyzedLemergiumAlkalide => self.catalyzed_lemergium_alkalide,
            CatalyzedZynthiumAcid => self.catalyzed_zynthium_acid,
            CatalyzedZynthiumAlkalide => self.catalyzed_zynthium_alkalide,
            CatalyzedGhodiumAcid => self.catalyzed_ghodium_acid,
            CatalyzedGhodiumAlkalide => self.catalyzed_ghodium_alkalide,

            // Catch-all -- do nothing
            _ => 0,
        }
    }

    /// Whether this BOM contains a non-zero amount of the specified resource.
    pub const fn contains(&self, resource: &ResourceType) -> bool {
        self.resource_amount(resource) > 0
    }

    /// Converts an arbitrary amount of a particular resource in this BOM into its reaction
    /// components.
    ///
    /// Clamps to the maximum of the amount provided, or the amount listed in the BOM.
    ///
    /// Does nothing for resources that do not have reaction components to break down into.
    pub const fn reduce_resource(&mut self, resource: &ResourceType, amount: u32) {
        // Verify that we have enough resources to actually do a reduction, and clamp the amount to
        // what we have
        let available_amount = self.resource_amount(resource);
        let reduction_amount = const_min_u32(available_amount, amount);
        if reduction_amount > 0 {
            // Verify that it's a reaction compound
            if let Some([component_1, component_2]) = resource.reaction_components() {
                // Subtract the amount of the resource from the BOM
                self.remove_resource(resource, reduction_amount);

                // Add the amount of the reaction components to the BOM
                self.add_resource(&component_1, reduction_amount);
                self.add_resource(&component_2, reduction_amount);
            }
        }
    }

    /// Converts all non-mineral resources in this BOM into their base mineral components.
    ///
    /// Primarily useful if you have no access to pre-made boosts and have to react everything up
    /// from scratch.
    pub const fn reduce_all_resources_to_base_materials(&mut self) {
		// Iterate through all T3 resources and reduce them
        let mut i = 0;
        while i < T3_RESOURCES.len() {
            self.reduce_resource(&T3_RESOURCES[i], u32::MAX); // This will clamp to the contained amount, if any
            i += 1;
        }

        // Iterate through all T2 resources and reduce them
        let mut i = 0;
        while i < T2_RESOURCES.len() {
            self.reduce_resource(&T2_RESOURCES[i], u32::MAX); // This will clamp to the contained amount, if any
            i += 1;
        }

        // Iterate through all T1 resources and reduce them
        let mut i = 0;
        while i < T1_RESOURCES.len() {
            self.reduce_resource(&T1_RESOURCES[i], u32::MAX); // This will clamp to the contained amount, if any
            i += 1;
        }

        // Iterate through all base compounds and reduce them
        // Reduce Ghodium specifically first, since it itself breaks down into other base compounds
        self.reduce_resource(&ResourceType::Ghodium, u32::MAX);
        let mut i = 0;
        while i < BASE_COMPOUNDS.len() {
            self.reduce_resource(&BASE_COMPOUNDS[i], u32::MAX); // This will clamp to the contained amount, if any
            i += 1;
        }
    }

    /// Merges the provided bill of materials with this bill of materials.
    pub const fn merge(&mut self, other: ReactionBillOfMaterials) {
		// Iterate through all T3 resources and copy their amounts
        let mut i = 0;
        while i < T3_RESOURCES.len() {
            self.add_resource(&T3_RESOURCES[i], other.resource_amount(&T3_RESOURCES[i]));
            i += 1;
        }

        // Iterate through all T2 resources and copy them
        let mut i = 0;
        while i < T2_RESOURCES.len() {
            self.add_resource(&T2_RESOURCES[i], other.resource_amount(&T2_RESOURCES[i]));
            i += 1;
        }

        // Iterate through all T1 resources and copy them
        let mut i = 0;
        while i < T1_RESOURCES.len() {
            self.add_resource(&T1_RESOURCES[i], other.resource_amount(&T1_RESOURCES[i]));
            i += 1;
        }

        // Iterate through all base compounds and copy them
        let mut i = 0;
        while i < BASE_COMPOUNDS.len() {
            self.add_resource(&BASE_COMPOUNDS[i], other.resource_amount(&BASE_COMPOUNDS[i]));
            i += 1;
        }

        // Iterate through all minerals and copy them
        let mut i = 0;
        while i < BASE_MINERALS.len() {
            self.add_resource(&BASE_MINERALS[i], other.resource_amount(&BASE_MINERALS[i]));
            i += 1;
        }

        // Let the other BOM drop here
    }

    /// Determines whether the lab reaction to produce the given resource can be run with the
    /// materials available in this bill of materials.
    pub const fn can_run_reaction_for_resource(&self, resource: &ResourceType) -> bool {
        if let Some(reactants) = resource.reaction_components() {
            (self.resource_amount(&reactants[0]) >= LAB_REACTION_AMOUNT) & 
            (self.resource_amount(&reactants[1]) >= LAB_REACTION_AMOUNT)
        } else {
            false
        }
    }

    /// Determines whether the given lab reaction can be run with the materials available in
    /// this bill of materials.
    pub const fn can_run_reaction(&self, reaction: &Reaction) -> bool {
        if let Some(reactants) = reaction.output().reaction_components() {
            (self.resource_amount(&reactants[0]) >= reaction.num_desired()) & 
            (self.resource_amount(&reactants[1]) >= reaction.num_desired())
        } else {
            false
        }
    }

    /// Adjusts this bill of materials as if the lab reaction to produce the given resource was
    /// run, incrementing the output resource amount and decrementing the input resources by
    /// [LAB_REACTION_AMOUNT].
    ///
    /// Returns true if the reaction was successful, false if it wasn't (due to resource amounts or
    /// the specified resource not being a lab resource).
    pub const fn run_reaction_for_resource(&mut self, resource: &ResourceType) -> bool {
        if let Some(reactants) = resource.reaction_components() {
            let can_run_reaction = {
                (self.resource_amount(&reactants[0]) >= LAB_REACTION_AMOUNT) & 
                (self.resource_amount(&reactants[1]) >= LAB_REACTION_AMOUNT)
            };

            if can_run_reaction {
                self.add_resource(resource, LAB_REACTION_AMOUNT);
                self.remove_resource(&reactants[0], LAB_REACTION_AMOUNT);
                self.remove_resource(&reactants[1], LAB_REACTION_AMOUNT);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Adjusts this bill of materials as if the specified reaction was run, incrementing the output
    /// resource amount and decrementing the input resources by the desired amount in the specified
    /// reaction.
    ///
    /// Returns true if the reaction was successful, false if it wasn't (due to resource amounts or
    /// the specified resource not being a lab resource).
    pub const fn run_reaction(&mut self, reaction: &Reaction) -> bool {
        let resource = reaction.output();
        if let Some(reactants) = resource.reaction_components() {
            let can_run_reaction = {
                (self.resource_amount(&reactants[0]) >= reaction.num_desired()) & 
                (self.resource_amount(&reactants[1]) >= reaction.num_desired())
            };

            if can_run_reaction {
                self.add_resource(&resource, LAB_REACTION_AMOUNT);
                self.remove_resource(&reactants[0], reaction.num_desired());
                self.remove_resource(&reactants[1], reaction.num_desired());
                true
            } else {
                false
            }
        } else {
            // This is a fallback, but shouldn't ever actually happen, since reaction.output()
            // should always be a lab resource that has a reaction chain.
            false
        }
    }

    /// Returns an iterator over all the resource-amount pairs where amount is non-zero.
    ///
    /// The iterator element type is ([ResourceType], [u32]).
    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }
}

impl Default for ReactionBillOfMaterials {
    fn default() -> Self {
        Self::default()
    }
}

/// An iterator over the non-zero resources and their amounts in a `ReactionBillOfMaterials`.
///
/// This `struct` is created by the [iter](ReactionBillOfMaterials::iter) method on a
/// [ReactionBillOfMaterials]. See its documentation for more information.
pub struct Iter {
    resources: Vec<(ResourceType, u32)>,
    current_index: usize,
}

impl Iter {
    fn new(bom: &ReactionBillOfMaterials) -> Iter {
        let v_r = LAB_RESOURCES.into_iter()
            .map(|r| (r, bom.resource_amount(&r)))
            .filter(|(_, amount)| *amount > 0)
            .collect();

        Iter {
            resources: v_r,
            current_index: 0,
        }
    }
}

impl Iterator for Iter {
    type Item = (ResourceType, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.resources.len() {
            None
        } else {
            let current = self.resources[self.current_index];
            self.current_index += 1;

            Some(current)
        }
    }
}

// Our implementation of Iterator behaves as a FusedIterator, so we get this trait for free.
impl FusedIterator for Iter {}

// We know exactly how many elements are remaining, so we can implement this trait more effectively
// than the default implementation.
impl ExactSizeIterator for Iter {
    fn len(&self) -> usize {
        self.resources.len() - self.current_index
    }
}

