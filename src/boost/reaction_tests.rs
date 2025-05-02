use enum_iterator::all;

use screeps::constants::ResourceType;

use crate::constants::resources;
use crate::boost::reaction::Reaction;
use crate::boost::reaction_chains::get_reaction_chain_for_resource;


#[test]
fn reaction_checked_new_works_for_reaction_output_resources() {
	for r in resources::LAB_OUTPUT_RESOURCES {
	    let res = Reaction::checked_new(r, 5);
		assert!(res.is_ok());
	}
}

#[test]
fn reaction_checked_new_fails_for_non_reaction_output_resources() {
	for r in all::<ResourceType>() {
	    let res = Reaction::checked_new(r, 5);
		assert_eq!(res.is_ok(), resources::LAB_OUTPUT_RESOURCES.contains(&r));
	}
}

#[test]
fn reaction_output_matches() {
	for r in resources::LAB_OUTPUT_RESOURCES {
	    let res = Reaction::unchecked_new(r, 5);
		assert_eq!(res.output(), r);
	}
}

#[test]
fn reaction_num_desired_matches() {
	for r in resources::LAB_OUTPUT_RESOURCES {
	    let res = Reaction::unchecked_new(r, 5);
		assert_eq!(res.num_desired(), 5);
	}
}

#[test]
fn reaction_reaction_time_matches() {
	for r in resources::LAB_OUTPUT_RESOURCES {
	    let res = Reaction::unchecked_new(r, 5);
		assert_eq!(res.reaction_time(), r.reaction_time().unwrap());
	}
}

#[test]
fn reaction_from_scratch_components_needed_returns_only_base_minerals() {
	for r in resources::LAB_OUTPUT_RESOURCES {
	    let res = Reaction::unchecked_new(r, 5);
		let bom = res.from_scratch_components_needed();
        for (r, _) in bom.iter() {
            assert!(resources::BASE_MINERALS.contains(&r));
        }
	}
}

#[test]
fn reaction_components_needed_matches() {
	for r in resources::LAB_OUTPUT_RESOURCES {
	    let res = Reaction::unchecked_new(r, 100);
		let bom = res.components_needed();
        let direct_reaction_components = r.reaction_components().unwrap();
        for (r, _) in bom.iter() {
            assert!(direct_reaction_components.contains(&r));
            assert_eq!(bom.resource_amount(&r), 100);
        }
	}
}

#[test]
fn reaction_reaction_chain_time_matches() {
	for r in resources::LAB_OUTPUT_RESOURCES {
		let reaction_chain = get_reaction_chain_for_resource(&r);
	    let res = Reaction::reaction_time_for_chain(reaction_chain);
		let sum_times: u32 = reaction_chain.iter().map(|r| r.reaction_time()).sum();
		assert_eq!(res, sum_times);
	}
}
