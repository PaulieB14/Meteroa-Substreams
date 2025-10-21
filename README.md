# Meteora Comprehensive Substreams Package

The **Meteora Comprehensive Substreams Package** contains a set of modules that allow you to easily retrieve and analyze data from Meteora's key programs on the Solana blockchain, including Dynamic Vault, Farm, and Zap programs. This package includes foundational store integration for enhanced account owner resolution, smart filtering for significant events, and vault analytics for comprehensive protocol insights.

The `substreams.toml` file defines all the different modules available, and also provides you with documentation about the usage of every module.

## 🚀 **New Features in v1.1.1**

### **Smart Event Filtering**
- **Instruction Discriminators**: Automatically identifies Meteora operation types (deposit, withdraw, rebalance, stake, etc.)
- **Significant Event Detection**: Only emits events that meet importance thresholds (e.g., >$10k transactions)
- **Low Egress Optimization**: Reduces data transfer costs by filtering out routine transactions

### **Vault Analytics**
- **TVL Tracking**: Monitors total value locked in vaults
- **Deposit/Withdrawal Analytics**: Tracks capital flows and net changes
- **Rebalancing Events**: Identifies when vaults optimize asset allocation
- **Vault Initialization**: Tracks new vault deployments

### **User Behavior Analytics**
- **Power User Detection**: Identifies users with multiple transactions
- **Activity Patterns**: Tracks user engagement across Meteora programs
- **Account Resolution**: Enhanced user identification through foundational stores

# Using this module to speed up a substreams

## Using the full "solana block" object (simplest if changing an existing substreams)

In your substreams.yaml,

1. Import this .spkg:

```
imports:
  meteora: https://spkg.io/your-username/meteora-comprehensive-v1.0.0.spkg
```

2. Replace any `source: sf.solana.type.v1.Block` input with `map: meteora:meteora_instructions` (you will be getting the same protobuf object, but with Meteora-specific data already processed)

3. Add block filtering to your "entry modules" (any module reading blocks or transactions before emitting your custom types):

If you know the instruction `program ID` of all transactions that you want, use the program-specific modules like this:

```
    blockFilter:
      module: meteora:meteora_vault_events
      query:
        string: "program:24Uqj9JCLxUeoC3hGfh5W3s9FM9uCHDS2SG3LYwBpyTi"
```

## Using the new 'transactions' object (simplest if writing a new substreams)

In your substreams.yaml,

1. Import this .spkg:

```
imports:
  meteora: https://spkg.io/your-username/meteora-comprehensive-v1.0.0.spkg
```

2. Set one of the Meteora-specific modules as your module input, ex:

```
  - name: my_cool_module
    kind: map
    inputs:
      - source: sf.substreams.v1.Clock
      - map: meteora:meteora_vault_events
```

3. Set the "block filter string" on the Meteora modules to match the data that you want to be fed to your module:

```
params:
  meteora:meteora_vault_events: "program:24Uqj9JCLxUeoC3hGfh5W3s9FM9uCHDS2SG3LYwBpyTi"
  meteora:meteora_farm_events: "program:FarmuwXPWXvefWUeqFAa5w6rifLkq5X6E8bimYvrhCB1"
  meteora:meteora_zap_events: "program:zapvX9M3uf5pvy4wRPAbQgdQsM1xmuiFnkfHKPvwMiz"
```

4. Run `substreams protogen` against your substreams.yaml to create the rust bindings of the protobuf definition inside the substreams.

## Modules

### `meteora_instructions` (map)

* This module provides comprehensive instruction data from all Meteora programs, including Dynamic Vault, Farm, and Zap programs. It processes and enriches instruction data with metadata, timestamps, and program-specific information.

### `map_spl_instructions` (map)

* This module provides foundational store integration for enhanced SPL token account owner resolution. It processes Meteora transactions while leveraging the SPL Initialized Account foundational store to resolve account ownership relationships. This is essential for comprehensive token transfer analysis where you need to know who actually sent/received tokens.

### `meteora_vault_events` (map)

* This module extracts and processes events from the Dynamic Vault Program (`24Uqj9JCLxUeoC3hGfh5W3s9FM9uCHDS2SG3LYwBpyTi`). It tracks:
  - Deposit/withdraw operations
  - Vault rebalancing events
  - Performance metrics (TVL, APY, fees)
  - User activity and vault analytics

Use it to get blocks that contain Dynamic Vault Program instructions:

```
  - name: my_module
    ...
    blockFilter:
      module: meteora_vault_events
      query:
        string: "program:24Uqj9JCLxUeoC3hGfh5W3s9FM9uCHDS2SG3LYwBpyTi"
```

### `meteora_farm_events` (map)

* This module extracts and processes events from the Farm Program (`FarmuwXPWXvefWUeqFAa5w6rifLkq5X6E8bimYvrhCB1`). It tracks:
  - Stake/unstake operations
  - Reward claims and distributions
  - Pool performance metrics
  - Staking analytics and user behavior

Use it to get blocks that contain Farm Program instructions:

```
  - name: my_module
    ...
    blockFilter:
      module: meteora_farm_events
      query:
        string: "program:FarmuwXPWXvefWUeqFAa5w6rifLkq5X6E8bimYvrhCB1"
```

### `meteora_zap_events` (map)

* This module extracts and processes events from the Zap Program (`zapvX9M3uf5pvy4wRPAbQgdQsM1xmuiFnkfHKPvwMiz`). It tracks:
  - Zap in/out transactions
  - Swap operations and routing
  - Price impact analysis
  - Volume and fee analytics

Use it to get blocks that contain Zap Program instructions:

```
  - name: my_module
    ...
    blockFilter:
      module: meteora_zap_events
      query:
        string: "program:zapvX9M3uf5pvy4wRPAbQgdQsM1xmuiFnkfHKPvwMiz"
```

### `meteora_analytics` (store)

* This module provides aggregated analytics and key performance indicators across all Meteora programs. It includes:
  - Total Value Locked (TVL) metrics
  - Volume and fee analytics
  - Active user counts
  - Program-specific performance metrics

## Program IDs

The following Meteora program IDs are supported:

- **Dynamic Vault Program**: `24Uqj9JCLxUeoC3hGfh5W3s9FM9uCHDS2SG3LYwBpyTi`
- **Farm Program**: `FarmuwXPWXvefWUeqFAa5w6rifLkq5X6E8bimYvrhCB1`
- **Zap Program**: `zapvX9M3uf5pvy4wRPAbQgdQsM1xmuiFnkfHKPvwMiz`

## Data Models

### Instruction Types

The package recognizes the following instruction types:

- `initialize` - Program initialization
- `deposit` - Deposit operations
- `withdraw` - Withdrawal operations
- `rebalance` - Vault rebalancing
- `claim_rewards` - Reward claims
- `stake` - Staking operations
- `unstake` - Unstaking operations
- `zap_in` - Zap in transactions
- `zap_out` - Zap out transactions

### Event Types

#### Vault Events
- **Deposit**: User deposits into vault
- **Withdraw**: User withdrawals from vault
- **Rebalance**: Vault rebalancing operations
- **Fee Collection**: Fee collection events

#### Farm Events
- **Stake**: User stakes tokens in farm
- **Unstake**: User unstakes tokens from farm
- **Claim Rewards**: User claims farming rewards
- **Reward Distribution**: System distributes rewards

#### Zap Events
- **Zap In**: One-click deposit with optimal routing
- **Zap Out**: One-click withdrawal with optimal routing
- **Swap**: Token swap operations
- **Route Optimization**: Path optimization events

## Installation

```bash
# Install Substreams CLI
curl -sSL https://substreams.dev/install.sh | bash

# Clone and build
git clone <repository-url>
cd meteora-substreams
substreams build
```

## Usage Examples

### Basic Usage

```bash
# Run all Meteora instructions
substreams run meteora_instructions

# Run specific program events
substreams run meteora_vault_events
substreams run meteora_farm_events
substreams run meteora_zap_events

# Run analytics
substreams run meteora_analytics
```

### Advanced Filtering

```bash
# Filter by specific program
substreams run meteora_vault_events --params meteora:meteora_vault_events="program:24Uqj9JCLxUeoC3hGfh5W3s9FM9uCHDS2SG3LYwBpyTi"

# Filter by multiple programs
substreams run meteora_instructions --params meteora:meteora_instructions="program:24Uqj9JCLxUeoC3hGfh5W3s9FM9uCHDS2SG3LYwBpyTi || program:FarmuwXPWXvefWUeqFAa5w6rifLkq5X6E8bimYvrhCB1"
```

### Integration with Other Tools

```bash
# Output to JSON
substreams run meteora_vault_events --output json

# Output to Parquet
substreams run meteora_analytics --output parquet

# Stream to database
substreams run meteora_farm_events --sink postgres://user:pass@localhost/db
```

## Performance Considerations

- **Block Filtering**: Use program-specific modules to reduce data processing overhead
- **Parallel Processing**: The package is optimized for parallel processing of multiple programs
- **Memory Usage**: Large blocks are processed efficiently with minimal memory footprint
- **Network Efficiency**: Only relevant transactions are processed, reducing bandwidth usage

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

- **Documentation**: [Substreams Docs](https://docs.substreams.dev/)
- **Community**: [Discord](https://discord.gg/substreams)
- **Issues**: [GitHub Issues](https://github.com/your-username/meteora-substreams/issues)

## Changelog

### v1.1.0 (Latest)
- Added foundational store integration
- New `map_spl_instructions` module with SPL Initialized Account foundational store
- Enhanced account owner resolution capabilities
- Improved token transfer analysis

### v1.0.0
- Initial release
- Support for Dynamic Vault Program
- Support for Farm Program  
- Support for Zap Program
- Comprehensive analytics module
- Professional documentation

---

**Total Downloads**: 0 (New Package)  
**Published**: Just Now  
**Network**: Solana  
**Publisher**: Your Name
