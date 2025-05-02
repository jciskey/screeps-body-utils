
use screeps::constants::ResourceType;

use crate::boost::bom::ReactionBillOfMaterials;
use crate::constants::resources;

#[test]
fn bom_add_resource_increases_by_amount() {
    let mut bom = ReactionBillOfMaterials::new();
    assert_eq!(bom.resource_amount(&ResourceType::Hydrogen), 0); // Initial state: no hydrogen
    assert_eq!(bom.resource_amount(&ResourceType::Oxygen), 0); // Initial state: no oxygen
    bom.add_resource(&ResourceType::Hydrogen, 50);
    assert_eq!(bom.resource_amount(&ResourceType::Hydrogen), 50); // Updated state: 50 hydrogen added, total 50
    assert_eq!(bom.resource_amount(&ResourceType::Oxygen), 0); // Updated state: still no oxygen
    bom.add_resource(&ResourceType::Hydrogen, 50);
    assert_eq!(bom.resource_amount(&ResourceType::Hydrogen), 100); // Updated state: 50 hydrogen added, total 100
    assert_eq!(bom.resource_amount(&ResourceType::Oxygen), 0); // Updated state: still no oxygen
}

#[test]
fn bom_remove_resource_decreases_by_amount() {
    let mut bom = ReactionBillOfMaterials::new();
    bom.add_resource(&ResourceType::Hydrogen, 50);
    bom.add_resource(&ResourceType::Oxygen, 50);
    assert_eq!(bom.resource_amount(&ResourceType::Hydrogen), 50); // Initial state: 50 hydrogen
    assert_eq!(bom.resource_amount(&ResourceType::Oxygen), 50); // Initial state: 50 oxygen
    bom.remove_resource(&ResourceType::Hydrogen, 25);
    assert_eq!(bom.resource_amount(&ResourceType::Hydrogen), 25); // Updated state: 25 hydrogen removed, total 25
    assert_eq!(bom.resource_amount(&ResourceType::Oxygen), 50); // Updated state: still 50 oxygen
    bom.remove_resource(&ResourceType::Hydrogen, 25);
    assert_eq!(bom.resource_amount(&ResourceType::Hydrogen), 0); // Updated state: 25 hydrogen added, total 0
    assert_eq!(bom.resource_amount(&ResourceType::Oxygen), 50); // Updated state: still 50 oxygen
}

#[test]
fn bom_reduce_resource_does_nothing_for_minerals() {
    let mut bom = ReactionBillOfMaterials::new();
    bom.add_resource(&ResourceType::Hydrogen, 50);
    bom.add_resource(&ResourceType::Oxygen, 50);
    assert_eq!(bom.resource_amount(&ResourceType::Hydrogen), 50); // Initial state: 50 hydrogen
    assert_eq!(bom.resource_amount(&ResourceType::Oxygen), 50); // Initial state: 50 oxygen
    bom.reduce_resource(&ResourceType::Hydrogen, 25);
    assert_eq!(bom.resource_amount(&ResourceType::Hydrogen), 50); // Updated state: still 50 hydrogen
    assert_eq!(bom.resource_amount(&ResourceType::Oxygen), 50); // Updated state: still 50 oxygen
}

#[test]
fn bom_reduce_resource_lowers_one_tier_of_resources_by_correct_amount() {
    let mut bom = ReactionBillOfMaterials::new();
    bom.add_resource(&ResourceType::Hydroxide, 50);
    assert_eq!(bom.resource_amount(&ResourceType::Hydrogen), 0);
    assert_eq!(bom.resource_amount(&ResourceType::Oxygen), 0);
    assert_eq!(bom.resource_amount(&ResourceType::Hydroxide), 50);
    bom.reduce_resource(&ResourceType::Hydroxide, 25);
    assert_eq!(bom.resource_amount(&ResourceType::Hydrogen), 25);
    assert_eq!(bom.resource_amount(&ResourceType::Oxygen), 25);
    assert_eq!(bom.resource_amount(&ResourceType::Hydroxide), 25);
}

#[test]
fn bom_reduce_all_resources_to_base_materials() {
    let mut bom = ReactionBillOfMaterials::new();

    bom.add_resource(&ResourceType::Hydroxide, 50);
    bom.add_resource(&ResourceType::CatalyzedGhodiumAlkalide, 50);
    assert_eq!(bom.resource_amount(&ResourceType::Hydrogen), 0);
    assert_eq!(bom.resource_amount(&ResourceType::Oxygen), 0);
    assert_eq!(bom.resource_amount(&ResourceType::Hydroxide), 50);
    assert_eq!(bom.resource_amount(&ResourceType::CatalyzedGhodiumAlkalide), 50);

    bom.reduce_all_resources_to_base_materials();

    for resource_type in resources::BASE_COMPOUNDS {
        assert_eq!(bom.resource_amount(&resource_type), 0);
    }

    for resource_type in resources::T1_RESOURCES {
        assert_eq!(bom.resource_amount(&resource_type), 0);
    }

    for resource_type in resources::T2_RESOURCES {
        assert_eq!(bom.resource_amount(&resource_type), 0);
    }

    for resource_type in resources::T3_RESOURCES {
        assert_eq!(bom.resource_amount(&resource_type), 0);
    }

    let t3_hydrogen = 50;
    let t3_oxygen = 100;
    let hydroxide_hydrogen = 50;
    let hydroxide_oxygen = 50;
    let total_hydrogen = t3_hydrogen + hydroxide_hydrogen;
    let total_oxygen = t3_oxygen + hydroxide_oxygen;

    assert_eq!(bom.resource_amount(&ResourceType::Hydrogen), total_hydrogen);
    assert_eq!(bom.resource_amount(&ResourceType::Oxygen), total_oxygen);
    assert_eq!(bom.resource_amount(&ResourceType::Utrium), 50);
    assert_eq!(bom.resource_amount(&ResourceType::Lemergium), 50);
    assert_eq!(bom.resource_amount(&ResourceType::Keanium), 50);
    assert_eq!(bom.resource_amount(&ResourceType::Zynthium), 50);
    assert_eq!(bom.resource_amount(&ResourceType::Catalyst), 50);
}







