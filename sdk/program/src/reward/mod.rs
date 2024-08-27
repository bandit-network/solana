//! The [reward native program][np].
//!
//! [np]: TODO: ADD DOCS URL

pub mod authorized_voters;
pub mod error;
pub mod instruction;
pub mod state;

pub mod program {
    crate::declare_id!("Reward111111111111111111111111111111111111111");
}
