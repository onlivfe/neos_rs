//! Models of the responses of Neos' API.

#![allow(clippy::struct_excessive_bools)]

mod credit_transaction;
mod friend;
mod friend_status;
mod group;
mod message;
mod neos_db_asset;
mod online_status;
mod output_device;
mod public_ban_type;
mod record;
mod record_id;
mod rsa_parameters_data;
mod session_access_level;
mod session_user;
mod sessions_info;
mod submission;
mod transaction_type;
mod user;
mod user_patreon_data;
mod user_profile;
mod user_session;
mod user_status;

pub use credit_transaction::*;
pub use friend::*;
pub use friend_status::*;
pub use group::*;
pub use message::*;
pub use neos_db_asset::*;
pub use online_status::*;
pub use output_device::*;
pub use public_ban_type::*;
pub use record::*;
pub use record_id::*;
pub use rsa_parameters_data::*;
pub use session_access_level::*;
pub use session_user::*;
pub use sessions_info::*;
pub use submission::*;
pub use transaction_type::*;
pub use user::*;
pub use user_patreon_data::*;
pub use user_profile::*;
pub use user_session::*;
pub use user_status::*;
