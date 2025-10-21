use substreams_solana::pb::sf::solana::r#type::v1::Block;
use substreams::store::FoundationalStore;
use bs58;

// Program IDs for Meteora contracts
const DYNAMIC_VAULT_PROGRAM: &str = "24Uqj9JCLxUeoC3hGfh5W3s9FM9uCHDS2SG3LYwBpyTi";
const FARM_PROGRAM: &str = "FarmuwXPWXvefWUeqFAa5w6rifKkq5X6E8bimYvrhCB1";
const ZAP_PROGRAM: &str = "zapvX9M3uf5pvy4wRPAbQgdQsM1xmuiFnkfHKPvwMiz";
const DAMM_V1_PROGRAM: &str = "Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB"; // Added DAMM v1

#[substreams::handlers::map]
pub fn map_block(block: Block) -> Result<Block, substreams::errors::Error> {
    let mut meteora_transaction_count = 0;
    let slot = block.slot;

    for trx in block.clone().transactions_owned() {
        if let Some(ref transaction) = trx.transaction {
            let accounts = trx.resolved_accounts();

            for inst in &transaction.message.as_ref().unwrap_or(&Default::default()).instructions {
                let program_id = bs58::encode(&accounts[inst.program_id_index as usize]).into_string();

                if is_meteora_program(&program_id) {
                    meteora_transaction_count += 1;
                    substreams::log::info!("Found Meteora transaction in slot {}: program_id={}", slot, program_id);
                }
            }
        }
    }

    if meteora_transaction_count > 0 {
        substreams::log::info!("Block {} contains {} Meteora transactions", slot, meteora_transaction_count);
    }

    Ok(block)
}

fn is_meteora_program(program_id: &String) -> bool {
    program_id == DYNAMIC_VAULT_PROGRAM || program_id == FARM_PROGRAM || program_id == ZAP_PROGRAM || program_id == DAMM_V1_PROGRAM
}

// New foundational store module - uses SPL initialized account foundational store
#[substreams::handlers::map]
pub fn map_spl_instructions(
    block: Block,
    account_owner_store: FoundationalStore,
) -> Result<Block, substreams::errors::Error> {
    let mut meteora_transaction_count = 0;
    let slot = block.slot;

    // Process transactions from the block
    for trx in block.clone().transactions_owned() {
        if let Some(ref transaction) = trx.transaction {
            let accounts = trx.resolved_accounts();

            for inst in &transaction.message.as_ref().unwrap_or(&Default::default()).instructions {
                let program_id = bs58::encode(&accounts[inst.program_id_index as usize]).into_string();

                if is_meteora_program(&program_id) {
                    meteora_transaction_count += 1;
                    
                    // Use foundational store to get account owner data
                    let account_key = accounts[inst.program_id_index as usize].clone();
                    let response = account_owner_store.get(&account_key);
                    
                    if let Some(account_data) = response.value {
                        substreams::log::info!("Found Meteora transaction in slot {}: program_id={}, account_owner_data_length={}", 
                            slot, program_id, account_data.value.len());
                    } else {
                        substreams::log::info!("Found Meteora transaction in slot {}: program_id={} (no account owner data)", 
                            slot, program_id);
                    }
                }
            }
        }
    }

    if meteora_transaction_count > 0 {
        substreams::log::info!("Processed {} Meteora transactions with foundational store in slot {}", meteora_transaction_count, slot);
    }

    Ok(block)
}
