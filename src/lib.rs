use substreams_solana::pb::sf::solana::r#type::v1::Block;
use substreams::store::FoundationalStore;
use bs58;
use std::collections::HashMap;

// Program IDs for Meteora contracts
const DYNAMIC_VAULT_PROGRAM: &str = "24Uqj9JCLxUeoC3hGfh5W3s9FM9uCHDS2SG3LYwBpyTi";
const FARM_PROGRAM: &str = "FarmuwXPWXvefWUeqFAa5w6rifKkq5X6E8bimYvrhCB1";
const ZAP_PROGRAM: &str = "zapvX9M3uf5pvy4wRPAbQgdQsM1xmuiFnkfHKPvwMiz";
const DAMM_V1_PROGRAM: &str = "Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB"; // Added DAMM v1

// Meteora instruction discriminators (common patterns)
const INSTRUCTION_INITIALIZE: u8 = 0;
const INSTRUCTION_DEPOSIT: u8 = 1;
const INSTRUCTION_WITHDRAW: u8 = 2;
const INSTRUCTION_REBALANCE: u8 = 3;
const INSTRUCTION_STAKE: u8 = 4;
const INSTRUCTION_UNSTAKE: u8 = 5;
const INSTRUCTION_CLAIM_REWARDS: u8 = 6;
const INSTRUCTION_ZAP_IN: u8 = 7;
const INSTRUCTION_ZAP_OUT: u8 = 8;

// Significant event thresholds
const SIGNIFICANT_AMOUNT_THRESHOLD: u64 = 10000; // $10k in lamports (adjust as needed)
const SIGNIFICANT_USER_THRESHOLD: u32 = 100; // Minimum user count for aggregation

#[substreams::handlers::map]
pub fn map_block(block: Block) -> Result<Block, substreams::errors::Error> {
    let mut meteora_transaction_count = 0;
    let mut significant_events = 0;
    let slot = block.slot;

    for trx in block.clone().transactions_owned() {
        if let Some(ref transaction) = trx.transaction {
            let accounts = trx.resolved_accounts();

            for inst in &transaction.message.as_ref().unwrap_or(&Default::default()).instructions {
                let program_id = bs58::encode(&accounts[inst.program_id_index as usize]).into_string();

                if is_meteora_program(&program_id) {
                    meteora_transaction_count += 1;
                    
                    // Parse instruction type and check if significant
                    if let Some(instruction_type) = parse_meteora_instruction(&inst.data) {
                        if is_significant_event(&instruction_type, &inst.data) {
                            significant_events += 1;
                            
                            // Extract amount if available
                            if let Some(amount) = extract_amount_from_instruction(&inst.data) {
                                substreams::log::info!(
                                    "Significant Meteora event in slot {}: {} on program {} with amount {}", 
                                    slot, instruction_type, program_id, amount
                                );
                            } else {
                                substreams::log::info!(
                                    "Significant Meteora event in slot {}: {} on program {}", 
                                    slot, instruction_type, program_id
                                );
                            }
                        } else {
                            substreams::log::info!(
                                "Meteora transaction in slot {}: {} on program {} (not significant)", 
                                slot, instruction_type, program_id
                            );
                        }
                    } else {
                        substreams::log::info!("Found Meteora transaction in slot {}: program_id={} (unknown instruction)", slot, program_id);
                    }
                }
            }
        }
    }

    if meteora_transaction_count > 0 {
        substreams::log::info!("Block {} contains {} Meteora transactions, {} significant events", slot, meteora_transaction_count, significant_events);
    }

    Ok(block)
}

fn is_meteora_program(program_id: &String) -> bool {
    program_id == DYNAMIC_VAULT_PROGRAM || program_id == FARM_PROGRAM || program_id == ZAP_PROGRAM || program_id == DAMM_V1_PROGRAM
}

// Parse Meteora instruction type from instruction data
fn parse_meteora_instruction(instruction_data: &[u8]) -> Option<String> {
    if instruction_data.is_empty() {
        return None;
    }
    
    match instruction_data[0] {
        INSTRUCTION_INITIALIZE => Some("initialize".to_string()),
        INSTRUCTION_DEPOSIT => Some("deposit".to_string()),
        INSTRUCTION_WITHDRAW => Some("withdraw".to_string()),
        INSTRUCTION_REBALANCE => Some("rebalance".to_string()),
        INSTRUCTION_STAKE => Some("stake".to_string()),
        INSTRUCTION_UNSTAKE => Some("unstake".to_string()),
        INSTRUCTION_CLAIM_REWARDS => Some("claim_rewards".to_string()),
        INSTRUCTION_ZAP_IN => Some("zap_in".to_string()),
        INSTRUCTION_ZAP_OUT => Some("zap_out".to_string()),
        _ => Some("unknown".to_string()),
    }
}

// Check if this is a significant event worth emitting
fn is_significant_event(instruction_type: &str, instruction_data: &[u8]) -> bool {
    // Always emit rebalancing events (vault optimization)
    if instruction_type == "rebalance" {
        return true;
    }
    
    // Always emit vault initialization
    if instruction_type == "initialize" {
        return true;
    }
    
    // For deposit/withdraw, check if amount is significant
    if instruction_type == "deposit" || instruction_type == "withdraw" {
        // Try to extract amount from instruction data (simplified)
        if instruction_data.len() >= 9 {
            // Assuming amount is stored as u64 in bytes 1-8
            let amount_bytes = &instruction_data[1..9];
            if amount_bytes.len() == 8 {
                let amount = u64::from_le_bytes([
                    amount_bytes[0], amount_bytes[1], amount_bytes[2], amount_bytes[3],
                    amount_bytes[4], amount_bytes[5], amount_bytes[6], amount_bytes[7],
                ]);
                return amount >= SIGNIFICANT_AMOUNT_THRESHOLD;
            }
        }
    }
    
    // Always emit stake/unstake events (user behavior tracking)
    if instruction_type == "stake" || instruction_type == "unstake" {
        return true;
    }
    
    // Always emit zap operations (complex DeFi operations)
    if instruction_type == "zap_in" || instruction_type == "zap_out" {
        return true;
    }
    
    false
}

// Extract amount from instruction data (simplified implementation)
fn extract_amount_from_instruction(instruction_data: &[u8]) -> Option<u64> {
    if instruction_data.len() >= 9 {
        let amount_bytes = &instruction_data[1..9];
        if amount_bytes.len() == 8 {
            return Some(u64::from_le_bytes([
                amount_bytes[0], amount_bytes[1], amount_bytes[2], amount_bytes[3],
                amount_bytes[4], amount_bytes[5], amount_bytes[6], amount_bytes[7],
            ]));
        }
    }
    None
}

// Enhanced foundational store module with instruction parsing and smart filtering
#[substreams::handlers::map]
pub fn map_spl_instructions(
    block: Block,
    account_owner_store: FoundationalStore,
) -> Result<Block, substreams::errors::Error> {
    let mut meteora_transaction_count = 0;
    let mut significant_events = 0;
    let mut user_activity_map: HashMap<String, u32> = HashMap::new();
    let slot = block.slot;

    // Process transactions from the block
    for trx in block.clone().transactions_owned() {
        if let Some(ref transaction) = trx.transaction {
            let accounts = trx.resolved_accounts();

            for inst in &transaction.message.as_ref().unwrap_or(&Default::default()).instructions {
                let program_id = bs58::encode(&accounts[inst.program_id_index as usize]).into_string();

                if is_meteora_program(&program_id) {
                    meteora_transaction_count += 1;
                    
                    // Parse instruction type and check if significant
                    if let Some(instruction_type) = parse_meteora_instruction(&inst.data) {
                        if is_significant_event(&instruction_type, &inst.data) {
                            significant_events += 1;
                            
                            // Track user activity for analytics
                            let signer = bs58::encode(&accounts[0]).into_string(); // Assuming first account is signer
                            *user_activity_map.entry(signer.clone()).or_insert(0) += 1;
                            
                            // Use foundational store to get account owner data
                            let account_key = accounts[inst.program_id_index as usize].clone();
                            let response = account_owner_store.get(&account_key);
                            
                            if let Some(account_data) = response.value {
                                substreams::log::info!(
                                    "Significant Meteora event in slot {}: {} by user {} on program {} with account data length {}", 
                                    slot, instruction_type, signer, program_id, account_data.value.len()
                                );
                            } else {
                                substreams::log::info!(
                                    "Significant Meteora event in slot {}: {} by user {} on program {} (no account owner data)", 
                                    slot, instruction_type, signer, program_id
                                );
                            }
                        } else {
                            substreams::log::info!(
                                "Meteora transaction in slot {}: {} on program {} (not significant)", 
                                slot, instruction_type, program_id
                            );
                        }
                    } else {
                        substreams::log::info!("Found Meteora transaction in slot {}: program_id={} (unknown instruction)", slot, program_id);
                    }
                }
            }
        }
    }

    if meteora_transaction_count > 0 {
        substreams::log::info!(
            "Processed {} Meteora transactions with foundational store in slot {}, {} significant events, {} active users", 
            meteora_transaction_count, slot, significant_events, user_activity_map.len()
        );
        
        // Log power users (users with multiple transactions)
        for (user, count) in &user_activity_map {
            if *count > 1 {
                substreams::log::info!("Power user {} has {} transactions in slot {}", user, count, slot);
            }
        }
    }

    Ok(block)
}

// New vault analytics module - focuses on vault-specific metrics
#[substreams::handlers::map]
pub fn map_vault_analytics(block: Block) -> Result<Block, substreams::errors::Error> {
    let mut vault_events = 0;
    let mut total_deposits = 0u64;
    let mut total_withdrawals = 0u64;
    let mut rebalance_events = 0;
    let slot = block.slot;

    for trx in block.clone().transactions_owned() {
        if let Some(ref transaction) = trx.transaction {
            let accounts = trx.resolved_accounts();

            for inst in &transaction.message.as_ref().unwrap_or(&Default::default()).instructions {
                let program_id = bs58::encode(&accounts[inst.program_id_index as usize]).into_string();

                // Focus only on Dynamic Vault Program
                if program_id == DYNAMIC_VAULT_PROGRAM {
                    if let Some(instruction_type) = parse_meteora_instruction(&inst.data) {
                        match instruction_type.as_str() {
                            "deposit" => {
                                if let Some(amount) = extract_amount_from_instruction(&inst.data) {
                                    total_deposits += amount;
                                    vault_events += 1;
                                    substreams::log::info!(
                                        "Vault deposit in slot {}: {} lamports", 
                                        slot, amount
                                    );
                                }
                            },
                            "withdraw" => {
                                if let Some(amount) = extract_amount_from_instruction(&inst.data) {
                                    total_withdrawals += amount;
                                    vault_events += 1;
                                    substreams::log::info!(
                                        "Vault withdrawal in slot {}: {} lamports", 
                                        slot, amount
                                    );
                                }
                            },
                            "rebalance" => {
                                rebalance_events += 1;
                                vault_events += 1;
                                substreams::log::info!(
                                    "Vault rebalancing event in slot {}", 
                                    slot
                                );
                            },
                            "initialize" => {
                                vault_events += 1;
                                substreams::log::info!(
                                    "New vault initialized in slot {}", 
                                    slot
                                );
                            },
                            _ => {
                                vault_events += 1;
                                substreams::log::info!(
                                    "Vault operation in slot {}: {}", 
                                    slot, instruction_type
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    if vault_events > 0 {
        let net_flow = total_deposits.saturating_sub(total_withdrawals);
        substreams::log::info!(
            "Vault analytics for slot {}: {} events, {} deposits, {} withdrawals, net flow: {}, {} rebalances", 
            slot, vault_events, total_deposits, total_withdrawals, net_flow, rebalance_events
        );
    }

    Ok(block)
}
