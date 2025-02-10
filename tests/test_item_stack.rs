use hamcrest2::prelude::*;
use rstest::{fixture, rstest};

use rust_inventory::prelude::*;

#[fixture]
fn tomato() -> Item {
    Item::new(1, String::from("Tomato"))
}
#[fixture]
fn shovel() -> Item {
    Item::new(9001, String::from("Unbreaking Gold Shovel"))
}

#[rstest]
fn test_default() {
    let generic = ItemStack::default();

    let default_item = Item::default();
    assert_that!(generic.get_item(), is(equal_to(&default_item)));
    assert_that!(generic.size(), equal_to(0));
}

#[rstest]
fn test_constructor(tomato: Item) {
    let a_stack = ItemStack::new(tomato.clone(), 1);

    assert_that!(a_stack.get_item(), equal_to(&tomato));

    assert_that!(a_stack.size(), equal_to(1));
    assert_that!(a_stack.permits_stacking(), is(true));
}

#[rstest]
pub fn test_add_items_stackable(tomato: Item) {
    let mut original_stack = ItemStack::new(tomato.clone(), 1);
    original_stack.add_items(11);

    assert_that!(original_stack.get_item(), equal_to(&tomato));
    assert_that!(original_stack.size(), equal_to(12));
    assert_that!(original_stack.permits_stacking(), is(true));

    let another_stack = ItemStack::new(tomato.clone(), 1);
    assert_that!(&original_stack, is(not(equal_to(&another_stack))));
    // assert_that!(original_stack.hashCode(), equal_to(another_stack.hashCode()));

    let another_stack = ItemStack::new(tomato, 12);
    assert_that!(original_stack, is(equal_to(another_stack)));
}

#[rstest]
pub fn test_display(shovel: Item, tomato: Item) {
    let a_stack = ItemStack::new(shovel.clone(), 1);

    assert_that!(a_stack.to_string().find(&shovel.to_string()), is(some()));

    assert_that!(
        a_stack
            .to_string()
            .find(&format!("( 1) {}", shovel.get_name())),
        is(some())
    );

    // Tomatoes are delicious
    let a_stack = ItemStack::new(tomato.clone(), 337);

    assert_that!(a_stack.to_string().find(&tomato.to_string()), is(some()));
    assert_that!(
        a_stack
            .to_string()
            .find(&format!("(337) {}", tomato.get_name())),
        is(some())
    );
}
