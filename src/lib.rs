mod pb;

use substreams::prelude::*;
use substreams_solana::pb::sf::solana::r#type::v1::{Block, CompiledInstruction, InnerInstructions, InnerInstruction, TransactionStatusMeta};
use crate::pb::meteora::v1::{Output, MeteoraEvent};
use bs58;

// Program IDs for Meteora contracts
const DYNAMIC_VAULT_PROGRAM: &str = "24Uqj9JCLxUeoC3hGfh5W3s9FM9uCHDS2SG3LYwBpyTi";
const FARM_PROGRAM: &str = "FarmuwXPWXvefWUeqFAa5w6rifKkq5X6E8bimYvrhCB1";
const ZAP_PROGRAM: &str = "zapvX9M3uf5pvy4wRPAbQgdQsM1xmuiFnkfHKPvwMiz";
const DAMM_V1_PROGRAM: &str = "Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB"; // Added DAMM v1

#[substreams::handlers::map]
pub fn map_block(block: Block) -> Result<Output, substreams::errors::Error> {
    let mut data: Vec<MeteoraEvent> = vec![];
    let slot = block.slot;
    let timestamp = block.block_time.as_ref().map_or(0, |t| t.timestamp);

    for trx in block.transactions_owned() {
        if let Some(ref transaction) = trx.transaction {
            let accounts = trx.resolved_accounts();
            let default_meta = TransactionStatusMeta::default();
            let meta = trx.meta.as_ref().unwrap_or(&default_meta);
            let tx_id = bs58::encode(&transaction.signatures[0]).into_string();
            let signer = bs58::encode(accounts.get(0).map_or(&vec![], |v| v)).into_string();
            let txn_fee = meta.fee;
            let signer_sol_change = get_signer_balance_change(&meta.pre_balances, &meta.post_balances);

            for (idx, inst) in transaction.message.as_ref().unwrap_or(&Default::default()).instructions.iter().enumerate() {
                let program_id = bs58::encode(&accounts[inst.program_id_index as usize]).into_string();

                if is_meteora_program(&program_id) {
                    data.push(get_meteora_event(
                        slot,
                        timestamp,
                        &tx_id,
                        &signer,
                        &program_id,
                        inst,
                        &accounts,
                        false,
                        idx as u32,
                        0,
                        &"".to_string(),
                        &"".to_string(),
                        txn_fee,
                        signer_sol_change,
                    ));
                }

                // Process inner instructions
                let inner_instructions: Vec<InnerInstructions> = filter_inner_instructions(&meta.inner_instructions, idx as u32);
                for (inner_idx, inner_inst) in inner_instructions.into_iter().flat_map(|i| i.instructions).enumerate() {
                    let inner_program_id = bs58::encode(&accounts[inner_inst.program_id_index as usize]).into_string();

                    if is_meteora_program(&inner_program_id) {
                        data.push(get_meteora_event_inner(
                            slot,
                            timestamp,
                            &tx_id,
                            &signer,
                            &inner_program_id,
                            &inner_inst,
                            &accounts,
                            true,
                            idx as u32,
                            inner_idx as u32,
                            &program_id,
                            &inner_program_id,
                            txn_fee,
                            signer_sol_change,
                        ));
                    }
                }
            }
        }
    }
    Ok(Output { data })
}

fn get_meteora_event(
    block_slot: u64,
    block_time: i64,
    tx_id: &String,
    signer: &String,
    program_id: &String,
    instruction: &CompiledInstruction,
    accounts: &Vec<&Vec<u8>>,
    is_inner: bool,
    instruction_index: u32,
    inner_instruction_index: u32,
    outer_program: &String,
    inner_program: &String,
    txn_fee: u64,
    signer_sol_change: i64,
) -> MeteoraEvent {
    let accounts_str = instruction.accounts.iter()
        .filter_map(|&idx| accounts.get(idx as usize))
        .map(|account| bs58::encode(account).into_string())
        .collect::<Vec<String>>()
        .join(",");

    MeteoraEvent {
        block_date: convert_to_date(block_time),
        block_time,
        block_slot,
        tx_id: tx_id.clone(),
        signer: signer.clone(),
        program_id: program_id.clone(),
        instruction_type: "unknown".to_string(), // Placeholder for actual instruction type
        accounts: accounts_str,
        instruction_data: instruction.data.clone(),
        is_inner_instruction: is_inner,
        instruction_index,
        inner_instruction_index,
        outer_program: outer_program.clone(),
        inner_program: inner_program.clone(),
        txn_fee,
        signer_sol_change,
    }
}

fn get_meteora_event_inner(
    block_slot: u64,
    block_time: i64,
    tx_id: &String,
    signer: &String,
    program_id: &String,
    instruction: &InnerInstruction,
    accounts: &Vec<&Vec<u8>>,
    is_inner: bool,
    instruction_index: u32,
    inner_instruction_index: u32,
    outer_program: &String,
    inner_program: &String,
    txn_fee: u64,
    signer_sol_change: i64,
) -> MeteoraEvent {
    let accounts_str = instruction.accounts.iter()
        .filter_map(|&idx| accounts.get(idx as usize))
        .map(|account| bs58::encode(account).into_string())
        .collect::<Vec<String>>()
        .join(",");

    MeteoraEvent {
        block_date: convert_to_date(block_time),
        block_time,
        block_slot,
        tx_id: tx_id.clone(),
        signer: signer.clone(),
        program_id: program_id.clone(),
        instruction_type: "unknown".to_string(), // Placeholder for actual instruction type
        accounts: accounts_str,
        instruction_data: instruction.data.clone(),
        is_inner_instruction: is_inner,
        instruction_index,
        inner_instruction_index,
        outer_program: outer_program.clone(),
        inner_program: inner_program.clone(),
        txn_fee,
        signer_sol_change,
    }
}

fn is_meteora_program(program_id: &String) -> bool {
    program_id == DYNAMIC_VAULT_PROGRAM || program_id == FARM_PROGRAM || program_id == ZAP_PROGRAM || program_id == DAMM_V1_PROGRAM
}

fn filter_inner_instructions(
    all_inner_instructions: &Vec<InnerInstructions>,
    instruction_index: u32,
) -> Vec<InnerInstructions> {
    all_inner_instructions
        .iter()
        .filter(|&inner_instruction| inner_instruction.index == instruction_index)
        .cloned()
        .collect()
}

fn get_signer_balance_change(pre_balances: &Vec<u64>, post_balances: &Vec<u64>) -> i64 {
    if pre_balances.is_empty() || post_balances.is_empty() {
        return 0;
    }
    (post_balances[0] as i64) - (pre_balances[0] as i64)
}

fn convert_to_date(timestamp: i64) -> String {
    use chrono::{TimeZone, Utc};
    Utc.timestamp_opt(timestamp, 0).unwrap().format("%Y-%m-%d").to_string()
}
