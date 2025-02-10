use hamcrest2::prelude::*;
use rstest::{fixture, rstest};

use rust_inventory::prelude::*;

#[fixture]
fn test_items() -> [Item; 3] {
    [
        Item::new(0, String::from("Diamond Boots")),
        Item::new(1, String::from("Tomato")),
        Item::new(2, String::from("Unbreaking Gold Shovel")),
    ]
}

#[fixture]
fn empty_inventory() -> Inventory {
    Inventory::default()
}

#[rstest]
fn test_default(empty_inventory: Inventory) {
    assert_that!(empty_inventory.utilized_slots(), equal_to(0));
    assert_that!(empty_inventory.empty_slots(), equal_to(10));
    assert_that!(empty_inventory.total_slots(), equal_to(10));
    assert_that!(empty_inventory.is_full(), is(not(true)));
    assert!(empty_inventory.is_empty());
}

#[rstest]
fn test_new() {
    let inv_with_8_slots = Inventory::new(8);

    assert_that!(inv_with_8_slots.utilized_slots(), equal_to(0));
    assert_that!(inv_with_8_slots.empty_slots(), equal_to(8));
    assert_that!(inv_with_8_slots.total_slots(), equal_to(8));
    assert_that!(inv_with_8_slots.is_full(), is(not(true)));
    assert!(inv_with_8_slots.is_empty());
}

///
/// Add ItemStacks to an Inventory without filling the Inventory or attempting
/// to add duplicate Items
///
#[rstest]
fn test_add_item_stack_no_check(test_items: [Item; 3]) {
    let stacks_to_add_0 = ItemStack::new(test_items[0].clone(), 1);
    let stacks_to_add_1 = ItemStack::new(test_items[1].clone(), 1);
    let stacks_to_add_2 = ItemStack::new(test_items[2].clone(), 1);

    let mut a_bag = Inventory::new(4);

    a_bag.add_items(stacks_to_add_0);
    a_bag.add_items(stacks_to_add_1);
    a_bag.add_items(stacks_to_add_2);

    assert_that!(a_bag.is_full(), is(false));
    assert_that!(a_bag.is_empty(), is(false));
    assert_that!(a_bag.utilized_slots(), equal_to(3));
    assert_that!(a_bag.empty_slots(), equal_to(1));
    assert_that!(a_bag.total_slots(), equal_to(4));
}

///
/// Add ItemStacks to an Inventory without filling the Inventory, but attempting
/// to add duplicate Items
///
#[rstest]
fn test_add_item_with_duplicate_items(test_items: [Item; 3]) {
    let stacks_to_add = [
        ItemStack::new(test_items[0].clone(), 1),
        ItemStack::new(test_items[1].clone(), 1),
        ItemStack::new(test_items[1].clone(), 1),
    ];

    let mut a_bag = Inventory::new(4);

    for stack in stacks_to_add.into_iter() {
        a_bag.add_items(stack);
    }

    assert_that!(a_bag.is_full(), is(false));
    assert_that!(a_bag.is_empty(), is(false));
    assert_that!(a_bag.utilized_slots(), equal_to(2));
    assert_that!(a_bag.empty_slots(), equal_to(2));
    assert_that!(a_bag.total_slots(), equal_to(4));
}
///
/// Add ItemStacks to an Inventory and fill it.
/// Then try to add one more ItemStack that is stackable.
///
#[rstest]
fn test_add_item_after_full_with_non_stackable(test_items: [Item; 3]) {
    let stacks_to_add = [
        ItemStack::new(test_items[0].clone(), 1),
        ItemStack::new(test_items[1].clone(), 1),
        ItemStack::new(test_items[2].clone(), 1),
    ];

    let mut a_bag = Inventory::new(2);

    a_bag.add_items(stacks_to_add[0].clone());
    a_bag.add_items(stacks_to_add[1].clone());

    assert_that!(a_bag.add_items(stacks_to_add[2].clone()), is(false));

    assert_that!(a_bag.is_full(), is(true));
    assert_that!(a_bag.utilized_slots(), equal_to(2));
    assert_that!(a_bag.empty_slots(), equal_to(0));
    assert_that!(a_bag.total_slots(), equal_to(2));
}

///
/// Add ItemStacks to an Inventory and fill it.
/// Then try to add one more ItemStack that is **not** stackable.
///
#[rstest]
fn test_add_item_after_full_with_stackable(test_items: [Item; 3]) {
    let stacks_to_add = [
        ItemStack::new(test_items[0].clone(), 1),
        ItemStack::new(test_items[1].clone(), 1),
    ];

    let mut a_bag = Inventory::new(2);

    a_bag.add_items(stacks_to_add[0].clone());
    a_bag.add_items(stacks_to_add[1].clone());
    a_bag.add_items(stacks_to_add[0].clone());

    assert_that!(a_bag.add_items(stacks_to_add[1].clone()), is(true));

    assert_that!(a_bag.is_full(), is(true));
    assert_that!(a_bag.is_empty(), is(false));
    assert_that!(a_bag.utilized_slots(), equal_to(2));
    assert_that!(a_bag.empty_slots(), equal_to(0));
    assert_that!(a_bag.total_slots(), equal_to(2));
}

#[rstest]
fn test_to_display(test_items: [Item; 3]) {
    let stacks_to_add = [
        ItemStack::new(test_items[0].clone(), 1),
        ItemStack::new(test_items[1].clone(), 1),
        ItemStack::new(test_items[2].clone(), 1),
    ];

    let mut a_bag = Inventory::new(4);
    for stack in stacks_to_add.iter().cloned() {
        a_bag.add_items(stack);
    }

    let items_as_strings: Vec<String> = stacks_to_add.iter().map(ItemStack::to_string).collect();

    let a_bag_as_str = a_bag.to_string();
    assert_that!(a_bag_as_str.find("3 of 4 slots"), is(some()));

    assert_that!(a_bag_as_str.find(&items_as_strings[0]), is(some()));
    assert_that!(a_bag_as_str.find(&items_as_strings[1]), is(some()));
    assert_that!(a_bag_as_str.find(&items_as_strings[2]), is(some()));

    let locations = items_as_strings
        .iter()
        .map(|item_as_str| a_bag_as_str.find(&*item_as_str))
        .flatten()
        .collect::<Vec<_>>();

    let mut sorted_locations = locations.clone();
    sorted_locations.sort();

    assert_that!(locations, is(equal_to(sorted_locations)));
}
