#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Output {
    #[prost(message, repeated, tag = "1")]
    pub data: ::std::vec::Vec<MeteoraEvent>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MeteoraEvent {
    #[prost(string, tag = "1")]
    pub block_date: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub block_time: i64,
    #[prost(uint64, tag = "3")]
    pub block_slot: u64,
    #[prost(string, tag = "4")]
    pub tx_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub signer: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub program_id: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub instruction_type: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub accounts: ::prost::alloc::string::String,
    #[prost(bytes, tag = "9")]
    pub instruction_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "10")]
    pub is_inner_instruction: bool,
    #[prost(uint32, tag = "11")]
    pub instruction_index: u32,
    #[prost(uint32, tag = "12")]
    pub inner_instruction_index: u32,
    #[prost(string, tag = "13")]
    pub outer_program: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub inner_program: ::prost::alloc::string::String,
    #[prost(uint64, tag = "15")]
    pub txn_fee: u64,
    #[prost(int64, tag = "16")]
    pub signer_sol_change: i64,
}
