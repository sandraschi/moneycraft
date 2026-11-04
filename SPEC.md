# MoneyCraft Technical Specification

**Version**: 0.3.0  
**Domain**: Quantitative Finance & Desktop Financial Terminal  
**Control Port**: 14117  

---

## 1. Crate Architecture

MoneyCraft follows the standard 4-tier sovereign Craft application layout:

```
moneycraft/
├── crates/
│   ├── moneycraft-feed/      # Market feeds, stochastic generator, technical indicators, sandbox EVM
│   ├── moneycraft-core/      # Quantitative command bus (Command::dispatch), port 14117, AI workflows
│   ├── moneycraft-mcp/       # FastMCP stdio server and CLI binary (moneycraft-cli)
│   └── moneycraft-app/       # Native egui 0.37 desktop studio (moneycraft)
```

## 2. Command Bus Contract (`Command::dispatch`)

All interactions route through typed command enumeration:

```rust
pub enum Command {
    Status,
    GetMarketFeed { symbol: String },
    GetNewsTicker,
    RunAiWorkflow {
        workflow_type: String, // "STRATEGY_GEN", "STRESS_TEST", "SENTIMENT_ALPHA", "EXECUTION_ROUTER"
        prompt: String,
        target_asset: String,
    },
    AuditContract { contract_address: String, chain: String },
    RunBacktest { strategy: String, symbol: String, initial_capital: f64 },
    PlaceOrder { symbol: String, side: String, quantity: f64, price: Option<f64> },
    PortfolioSummary,
}
```

## 3. UI Implementation Details

- **Framework**: `eframe` / `egui 0.37`.
- **Canvas Allocation**: `allocate_painter` with hover sense.
- **Color Palette**:
  - Background Obsidian: `#06080c`
  - Framing Border: `#212936`
  - Bloomberg Amber: `#ffb300`
  - Bull Jade: `#00e676`
  - Bear Crimson: `#ff1744`
  - SMA 20 Cyan: `#00e5ff`
  - EMA 50 Gold: `#ffd600`
  - Bollinger Bands: `#b388ff`
- **Zero-Panic Policy**: Enforced via `#![deny(clippy::unwrap_used)]` across all library crates.
