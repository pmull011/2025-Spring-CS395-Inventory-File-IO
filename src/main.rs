use eyre::WrapErr;

use rust_inventory::parser::ParsedLine;
use rust_inventory::prelude::*;

fn main() -> eyre::Result<()> {
    let argv: Vec<String> = std::env::args().collect();

    if argv.len() < 3 {
        eyre::bail!("Usage: {} items_filename inventories_filename", argv[0]);
    }

    let all_items = Parser::read_from_file(&argv[1], |ins| Parser::read_items(ins))?;
    let all_inventory_lines =
        Parser::read_from_file(&argv[2], |ins| Parser::read_inventory_lines(ins))?;

    let logged_inventories = process_inventory_requests(all_inventory_lines, &all_items);

    println!("Processing Log:");
    for (entries, _) in logged_inventories.iter() {
        for entry in entries.iter() {
            println!("{}", entry);
        }
    }
    println!();

    println!("Item List:");
    for item in all_items.iter() {
        println!("  {:>2} {}", item.get_id(), item.get_name());
    }
    println!();

    println!("Storage Summary:");
    for (_, inv) in logged_inventories.iter() {
        println!("{}", inv);
    }

    Ok(())
}

pub fn process_inventory_requests(
    all_inventory_lines: Vec<ParsedLine>,
    known_items: &[Item],
) -> Vec<(Vec<String>, Inventory)> {
    let lines = all_inventory_lines.split(|line| match line {
        ParsedLine::InventoryLine { .. } => true,
        _ => false,
    });

    let inventories: Vec<Inventory> = all_inventory_lines
        .iter()
        .flat_map(|line| match line {
            ParsedLine::InventoryLine { max_size } => Some(Inventory::new(*max_size)),
            _ => None,
        })
        .collect();

    let logged_inventories: Vec<(_, Inventory)> = inventories
        .into_iter()
        .zip(lines.skip(1))
        .map(|(mut inv, entries)| {
            let stacks_to_store: Vec<ItemStack> = entries
                .iter()
                .flat_map(|line| {
                    let possible_stack = match line {
                        ParsedLine::ItemStackLine { id, quantity } => {
                            match known_items
                                .iter()
                                .find(|known_item| known_item.get_id() == *id)
                            {
                                Some(&ref item) => Some(ItemStack::new(item.clone(), *quantity)),
                                None => None,
                            }
                        }
                        _ => None,
                    };
                    possible_stack
                })
                .collect();

            let entries: Vec<String> = stacks_to_store
                .into_iter()
                .map(|stack| {
                    format!(
                        "{:9} ({:>2}) {}",
                        if inv.add_items(stack.clone()) {
                            "Stored"
                        } else {
                            "Discarded"
                        },
                        stack.size(),
                        stack.get_item().get_name()
                    )
                })
                .collect();

            (entries, inv)
        })
        .collect();

    logged_inventories
}
