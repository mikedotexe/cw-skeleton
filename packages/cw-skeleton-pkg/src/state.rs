use crate::types::Config;
use cw_storage_plus::Item;

// We might as well have one-letter keys like "c" or "n" to save ones and zeroes.
// CONFIG will typically store values that might be updated later using a special function that checks if the sender is allowed to change it.
pub const CONFIG: Item<Config> = Item::new("c");
