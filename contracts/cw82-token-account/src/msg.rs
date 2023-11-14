use cosmwasm_std::{Binary, Empty, CosmosMsg, Coin};
use cosmwasm_schema::{cw_serde, QueryResponses, serde::Serialize};
use cw721::Cw721ReceiveMsg;
pub use cw82::{
    smart_account_query, 
    CanExecuteResponse, 
    ValidSignatureResponse, 
    ValidSignaturesResponse
};
use cw_ownable::cw_ownable_query;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub pubkey: Binary,
    pub token_contract: String,
    pub token_id: String
}

#[cw_serde]
pub struct TokenInfo {
    pub token_contract: String,
    pub token_id: String
}


#[cw_serde]
pub struct PayloadInfo {
    pub account: String,
    pub algo: String
}


#[cw_serde]
pub struct Status {
    pub frozen: bool,
}

#[cw_serde]
pub struct AssetsResponse {
    pub balances: Vec<Coin>,
    pub tokens: Vec<TokenInfo>
}

pub type KnownTokensResponse = Vec<TokenInfo>;

#[smart_account_query]
#[cw_ownable_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsgBase <T = Empty> {
    #[returns(Binary)]
    Pubkey {},

    #[returns(KnownTokensResponse)]
    KnownTokens {
        skip: Option<u32>,
        limit: Option<u32>
    },

    #[returns(PayloadInfo)]
    Assets {
        skip: Option<u32>,
        limit: Option<u32>
    },

    #[returns(Status)]
    Status {},

    #[returns(TokenInfo)]
    Token {},
}

pub type QueryMsg = QueryMsgBase<Empty>;


#[cw_serde]
pub enum ExecuteMsg {
    Execute { msgs: Vec<CosmosMsg<Empty>> },
    SendToken { collection: String, token_id: String, contract: String, msg: Binary },
    TransferToken { collection: String, token_id: String, recipient: String  },
    ForgetTokens { collection: String, token_ids: Vec<String> },
    UpdateKnownTokens { collection: String, start_after: Option<String>, limit: Option<u32> },
    UpdateOwnership { new_owner: String, new_pubkey: Binary },
    UpdatePubkey { new_pubkey: Binary },
    ReceiveNft(Cw721ReceiveMsg),
    Freeze {},
    Unfreeze {},
}


#[cw_serde]
pub struct MigrateMsg<T = Empty> {
    pub params: Option<Box<T>>
}