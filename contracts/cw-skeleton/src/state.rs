use cw_storage_plus::Item;
// Remember the "pub" here, since it's used elsewhere.
pub use cw_skeleton_pkg::types::Config;

pub const CONFIG: Item<Config> = cw_skeleton_pkg::state::CONFIG;
