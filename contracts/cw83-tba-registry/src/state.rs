use cw_storage_plus::{Map, Item};
use crate::msg::TokenInfo;

//
// pub unbond_requests: IndexedMap<'a, (u64, &'a Addr), UnbondRequest, UnbondRequestsIndexes<'a>>,


pub static TOKEN_ADDRESSES   : Map<(&str, &str), String>    = Map::new("t");
pub static LAST_ATTEMPTING   : Item<TokenInfo>              = Item::new("l");
pub static ALLOWED_IDS       : Item<Vec<u64>>               = Item::new("i");
pub static ADMIN             : Item<String>                 = Item::new("a");