use cosmwasm_std::{StdResult, Deps, Order};

use crate::{state::{TOKEN_ADDRESSES, KNOWN_COLLECTIONS}, msg::{AccountInfoResponse, TokenInfo, CollectionAccount, AccountsResponse, CollectionsResponse}};

const DEFAULT_BATCH_SIZE : u32 = 100;


pub fn account_info(
    deps: Deps,
    info: TokenInfo
) -> StdResult<AccountInfoResponse> {
            
    let address = TOKEN_ADDRESSES.load(
        deps.storage, 
        (info.contract.as_str(), info.id.as_str())
    )?;

    Ok(AccountInfoResponse {
        address, info: None
    })
}


pub fn collections(
    deps: Deps,
    skip: Option<u32>,
    limit: Option<u32>
) -> StdResult<CollectionsResponse> {
    
    let skip  = skip.unwrap_or(0) as usize;
    let limit = limit.unwrap_or(DEFAULT_BATCH_SIZE) as usize;

    let collections =  KNOWN_COLLECTIONS
        .keys(deps.storage, None, None, Order::Descending)
        .into_iter()
        .enumerate()
        .filter(|(i, _)| *i >= skip)
        .take(limit)
        .map(|(_, c) | c.unwrap())
        .collect::<Vec<String>>();

    Ok(CollectionsResponse { collections })
}


pub fn accounts(
    deps: Deps,
    collection: &str,
    skip: Option<u32>,
    limit: Option<u32>
) -> StdResult<AccountsResponse> {

    let skip  = skip.unwrap_or(0) as usize;
    let limit = limit.unwrap_or(DEFAULT_BATCH_SIZE) as usize;
            
    TOKEN_ADDRESSES
        .prefix(collection)
        .range(deps.storage, None, None, Order::Descending)
        .enumerate()
        .filter(|(i, _)| *i >= skip)
        .take(limit)
        .map(|(_,item)| {
            let (id, address) = item?;
            Ok(CollectionAccount { id, address })
        })
        .collect()

}