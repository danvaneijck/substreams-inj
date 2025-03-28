// @generated
// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trade {
    #[prost(string, tag="1")]
    pub transaction_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub contract_address: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub action: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub ask_asset: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub burn_amount: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub commission_amount: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub fee_wallet_amount: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub offer_amount: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub offer_asset: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub pool_amount: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub receiver: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub return_amount: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub spread_amount: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub msg_index: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChoiceTrades {
    #[prost(message, repeated, tag="1")]
    pub trades: ::prost::alloc::vec::Vec<Trade>,
}
// @@protoc_insertion_point(module)
