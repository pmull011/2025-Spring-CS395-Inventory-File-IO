///
/// Item represents an individual Item in an inventory.
/// This includes items such as potions, building materials, and food.
///
/// Only one of each item can exist--i.e., no two items share the
/// same numeric id.
///
#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Item {
    /// Unique numeric id
    id: u64,

    /// Short title--e.g., HP Potion.
    name: String,
}

impl Default for Item {
    fn default() -> Self {
        Self::new(0, "Air".to_string())
    }
}

impl Item {
    /// Create an Item with a specified and name.
    ///
    /// # Arguments
    ///
    /// * `nme` - desired name
    ///
    pub fn new(id: u64, nme: String) -> Self {
        Self { id, name: nme }
    }

    ///
    /// Retrieve name
    ///
    pub fn get_id(&self) -> u64 {
        self.id
    }

    ///
    /// Update id.
    ///
    /// # Arguments
    ///
    /// * `nme` - replacement id
    ///
    pub fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    ///
    /// Retrieve name
    ///
    pub fn get_name(&self) -> &str {
        &self.name
    }

    ///
    /// Update name.
    ///
    /// # Arguments
    ///
    /// * `nme` - replacement name
    ///
    pub fn set_name(&mut self, nme: String) {
        self.name = nme
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)?;

        Ok(())
    }
}

///
/// A Homogeneous--i.e., uniform--stack of Items.
///
#[derive(Clone, Debug, Hash, PartialEq)]
pub struct ItemStack {
    ///
    /// The specific type of item out of which this stack is built.
    ///
    item: Item,

    ///
    /// Represents the number of items in this stack.
    ///
    quantity: usize,
}

impl Default for ItemStack {
    ///
    /// Create an empty stack composed of Air.
    ///
    fn default() -> Self {
        Self::new(Item::default(), 0)
    }
}

impl ItemStack {
    ///
    /// Create a stack of the desired type.
    ///
    /// # Arguments
    ///
    /// * `base` - Item out of which the stack is composed
    ///
    /// * `qty` - number of items to place in the stack
    ///
    pub fn new(base: Item, qty: usize) -> Self {
        Self {
            item: base,
            quantity: qty,
        }
    }

    ///
    /// Retrieve the Item out of which the stack is composed.
    ///
    /// # Returns
    ///
    /// the item that serves as the base
    ///
    pub fn get_item(&self) -> &Item {
        &self.item
    }

    ///
    /// Retrieve the size of the stack.
    ///
    /// # Returns
    ///
    /// the current number of items
    ///
    pub fn size(&self) -> usize {
        self.quantity
    }

    ///
    /// Increase the size of the stack.
    ///
    /// # Arguments
    ///
    /// * `qty` - number of items to add
    ///
    pub fn add_items(&mut self, qty: usize) {
        self.quantity += qty;
    }

    ///
    /// Does the Item contained in this stack permit stacking?
    ///
    /// This can be less formally phrased, is this a stackable ItemStack?
    ///
    /// # Returns
    ///
    /// true if the addition of items is permitted
    ///
    pub fn permits_stacking(&self) -> bool {
        // For now... all items are stackable
        return true;
    }
}

impl std::fmt::Display for ItemStack {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({:2}) {}", self.quantity, &self.item)?;

        Ok(())
    }
}
