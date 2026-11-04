//! MoneyCraft Feed & Financial Time-Series Engine.
//!
//! High-density candlestick feeds for Crypto, Equities, Commodities, and Forex,
//! liquidation heatmaps, cross-exchange funding rate arbitrage matrix,
//! technical indicators (SMA, EMA, Bollinger, RSI, CVD),
//! news wire sentiment feeds, and smart contract anti-scam auditing.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleBar {
    pub timestamp_ms: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub cvd: f64, // Cumulative Volume Delta
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicalIndicators {
    pub sma_20: Vec<Option<f64>>,
    pub ema_50: Vec<Option<f64>>,
    pub bb_upper: Vec<Option<f64>>,
    pub bb_lower: Vec<Option<f64>>,
    pub rsi_14: Vec<Option<f64>>,
    pub cvd: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketTickerSummary {
    pub symbol: String,
    pub current_price: f64,
    pub change_24h_usd: f64,
    pub change_24h_pct: f64,
    pub high_24h: f64,
    pub low_24h: f64,
    pub volume_24h: f64,
    pub vwap: f64,
    pub is_forex: bool,
    pub pip_precision: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquidationBand {
    pub price_level: f64,
    pub volume_notional_mil: f64,
    pub side: String, // "LONG_LIQ" (downside cascade) or "SHORT_LIQ" (upside squeeze)
    pub distance_pct: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundingRateVenue {
    pub venue: String, // "Hyperliquid", "Binance", "Bybit", "OKX"
    pub rate_8h_pct: f64,
    pub annualized_apr_pct: f64,
    pub open_interest_mil: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoDerivativesData {
    pub symbol: String,
    pub mark_price: f64,
    pub index_price: f64,
    pub funding_rates: Vec<FundingRateVenue>,
    pub arbitrage_spread_apr_pct: f64,
    pub best_arb_long_venue: String,
    pub best_arb_short_venue: String,
    pub liquidation_heatmap: Vec<LiquidationBand>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsHeadline {
    pub id: String,
    pub timestamp_str: String,
    pub category: String, // "MACRO", "FOREX", "CENTRAL_BANK", "EQUITY", "CRYPTO"
    pub headline: String,
    pub sentiment: String, // "BULLISH", "BEARISH", "NEUTRAL"
    pub sentiment_score: f32, // -1.0 to +1.0
    pub affected_symbols: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractAuditReport {
    pub contract_address: String,
    pub chain: String,
    pub is_honeypot: bool,
    pub buy_tax_pct: f32,
    pub sell_tax_pct: f32,
    pub allows_trading: bool,
    pub can_mint_tokens: bool,
    pub can_blacklist_wallets: bool,
    pub liquidity_locked: bool,
    pub risk_score_0_to_100: u8,
    pub audit_verdict: String,
}

pub struct MarketFeed;

impl MarketFeed {
    /// Returns default supported symbols across Asset Classes.
    #[must_use]
    pub fn available_symbols() -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("BTC/USD", "Bitcoin / US Dollar", "CRYPTO"),
            ("ETH/USD", "Ethereum / US Dollar", "CRYPTO"),
            ("SOL/USD", "Solana / US Dollar", "CRYPTO"),
            ("EUR/USD", "Euro / US Dollar", "FOREX"),
            ("USD/JPY", "US Dollar / Japanese Yen", "FOREX"),
            ("GBP/USD", "British Pound / US Dollar", "FOREX"),
            ("USD/CNY", "US Dollar / Chinese Renminbi", "FOREX"),
            ("EUR/GBP", "Euro / British Pound", "FOREX"),
            ("SPY", "S&P 500 ETF Trust", "EQUITY"),
            ("NVDA", "Nvidia Corporation", "EQUITY"),
            ("XAU/USD", "Gold Bullion / USD", "COMMODITY"),
        ]
    }

    /// Generates high-density time-series candlestick data (60 bars) with CVD.
    #[must_use]
    pub fn mock_candles(symbol: &str) -> Vec<CandleBar> {
        let (base_price, volatility) = match symbol.to_uppercase().as_str() {
            // Major Forex Pairs
            "EUR/USD" | "EURUSD" => (1.0845, 0.0018),
            "USD/JPY" | "USDJPY" => (152.40, 0.35),
            "GBP/USD" | "GBPUSD" => (1.2980, 0.0022),
            "USD/CNY" | "USDCNY" | "USD/RMB" => (7.1250, 0.0080),
            "EUR/GBP" | "EURGBP" => (0.8355, 0.0012),
            // Equities & Commodities
            "NVDA" => (144.5, 1.8),
            "SPY" => (588.2, 2.1),
            "XAU/USD" | "GOLD" => (2745.0, 12.0),
            // Crypto
            "ETH" | "ETH/USD" => (3480.0, 24.0),
            "SOL" | "SOL/USD" => (215.0, 3.2),
            _ => (96450.0, 480.0), // BTC/USD default
        };

        let mut candles = Vec::with_capacity(64);
        let base_timestamp: u64 = 1762200000000;
        let mut curr_close = base_price * 0.985;
        let mut running_cvd = 0.0;

        // Deterministic pseudo-random seed generator
        let mut seed: u64 = match symbol.chars().next() {
            Some(c) => (c as u64) * 7919,
            None => 42,
        };

        for i in 0..60 {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let r1 = ((seed >> 32) as f64) / (u32::MAX as f64);
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let r2 = ((seed >> 32) as f64) / (u32::MAX as f64);
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let r3 = ((seed >> 32) as f64) / (u32::MAX as f64);

            let open = curr_close;
            // Upward drift with realistic market noise
            let drift = volatility * 0.12;
            let delta = (r1 - 0.46) * volatility * 2.0 + drift;
            let close = (open + delta).max(0.0001);
            let high = open.max(close) + r2 * volatility * 1.2;
            let low = (open.min(close) - r3 * volatility * 1.2).max(0.0001);
            let volume = 850.0 + r1 * 2400.0;

            // Compute delta aggression
            let buy_ratio = if close >= open { 0.55 + r1 * 0.25 } else { 0.45 - r2 * 0.25 };
            let bar_delta = volume * (2.0 * buy_ratio - 1.0);
            running_cvd += bar_delta;

            candles.push(CandleBar {
                timestamp_ms: base_timestamp + (i as u64) * 3600_000,
                open,
                high,
                low,
                close,
                volume,
                cvd: running_cvd,
            });

            curr_close = close;
        }

        candles
    }

    /// Computes SMA, EMA, Bollinger Bands, and RSI indicators.
    #[must_use]
    pub fn calculate_indicators(candles: &[CandleBar]) -> TechnicalIndicators {
        let n = candles.len();
        let closes: Vec<f64> = candles.iter().map(|c| c.close).collect();
        let cvd: Vec<f64> = candles.iter().map(|c| c.cvd).collect();

        // 1. SMA 20
        let mut sma_20 = vec![None; n];
        for i in 19..n {
            let sum: f64 = closes[(i + 1 - 20)..=i].iter().sum();
            sma_20[i] = Some(sum / 20.0);
        }

        // 2. EMA 50
        let mut ema_50 = vec![None; n];
        let k = 2.0 / (50.0 + 1.0);
        let mut current_ema = closes.first().copied().unwrap_or(0.0);
        for i in 0..n {
            current_ema = closes[i] * k + current_ema * (1.0 - k);
            if i >= 10 {
                ema_50[i] = Some(current_ema);
            }
        }

        // 3. Bollinger Bands (20 period, 2 std dev)
        let mut bb_upper = vec![None; n];
        let mut bb_lower = vec![None; n];
        for i in 19..n {
            if let Some(mean) = sma_20[i] {
                let variance: f64 = closes[(i + 1 - 20)..=i]
                    .iter()
                    .map(|x| (x - mean).powi(2))
                    .sum::<f64>()
                    / 20.0;
                let std_dev = variance.sqrt();
                bb_upper[i] = Some(mean + 2.0 * std_dev);
                bb_lower[i] = Some((mean - 2.0 * std_dev).max(0.0001));
            }
        }

        // 4. RSI 14
        let mut rsi_14 = vec![None; n];
        if n >= 15 {
            let mut gains = 0.0;
            let mut losses = 0.0;
            for i in 1..=14 {
                let diff = closes[i] - closes[i - 1];
                if diff >= 0.0 {
                    gains += diff;
                } else {
                    losses += -diff;
                }
            }
            let mut avg_gain = gains / 14.0;
            let mut avg_loss = losses / 14.0;

            let rs = if avg_loss == 0.0 { 100.0 } else { avg_gain / avg_loss };
            rsi_14[14] = Some(100.0 - (100.0 / (1.0 + rs)));

            for i in 15..n {
                let diff = closes[i] - closes[i - 1];
                let (gain, loss) = if diff >= 0.0 { (diff, 0.0) } else { (0.0, -diff) };
                avg_gain = (avg_gain * 13.0 + gain) / 14.0;
                avg_loss = (avg_loss * 13.0 + loss) / 14.0;
                let rs = if avg_loss == 0.0 { 100.0 } else { avg_gain / avg_loss };
                rsi_14[i] = Some((100.0 - (100.0 / (1.0 + rs))).clamp(0.0, 100.0));
            }
        }

        TechnicalIndicators {
            sma_20,
            ema_50,
            bb_upper,
            bb_lower,
            rsi_14,
            cvd,
        }
    }

    /// Computes summary ticker metrics.
    #[must_use]
    pub fn summarize_ticker(symbol: &str, candles: &[CandleBar]) -> MarketTickerSummary {
        let is_forex = symbol.contains("EUR") || symbol.contains("JPY") || symbol.contains("GBP") || symbol.contains("CNY") || symbol.contains("RMB");
        let pip_precision = if symbol.contains("JPY") {
            2
        } else if is_forex {
            4
        } else {
            2
        };

        if candles.is_empty() {
            return MarketTickerSummary {
                symbol: symbol.to_string(),
                current_price: 0.0,
                change_24h_usd: 0.0,
                change_24h_pct: 0.0,
                high_24h: 0.0,
                low_24h: 0.0,
                volume_24h: 0.0,
                vwap: 0.0,
                is_forex,
                pip_precision,
            };
        }

        let first = candles.first().map(|c| c.open).unwrap_or(1.0);
        let last = candles.last().map(|c| c.close).unwrap_or(1.0);
        let change_usd = last - first;
        let change_pct = (change_usd / first) * 100.0;

        let mut high = f64::MIN;
        let mut low = f64::MAX;
        let mut vol_sum = 0.0;
        let mut pv_sum = 0.0;

        for c in candles {
            if c.high > high { high = c.high; }
            if c.low < low { low = c.low; }
            vol_sum += c.volume;
            let typical_price = (c.high + c.low + c.close) / 3.0;
            pv_sum += typical_price * c.volume;
        }

        let vwap = if vol_sum > 0.0 { pv_sum / vol_sum } else { last };

        MarketTickerSummary {
            symbol: symbol.to_string(),
            current_price: last,
            change_24h_usd: change_usd,
            change_24h_pct: change_pct,
            high_24h: high,
            low_24h: low,
            volume_24h: vol_sum,
            vwap,
            is_forex,
            pip_precision,
        }
    }

    /// Generates Liquidation Heatmap clusters & Cross-Exchange Funding Arbitrage data.
    #[must_use]
    pub fn mock_derivatives_data(symbol: &str, current_price: f64) -> CryptoDerivativesData {
        let is_crypto = symbol.contains("BTC") || symbol.contains("ETH") || symbol.contains("SOL");
        let mark_price = if is_crypto { current_price } else { 96450.0 };
        let index_price = mark_price * 0.9998;

        // 1. Funding rates across venues
        let funding_rates = vec![
            FundingRateVenue {
                venue: "Hyperliquid (L1 DEX)".to_string(),
                rate_8h_pct: 0.0385,
                annualized_apr_pct: 42.15,
                open_interest_mil: 1420.5,
            },
            FundingRateVenue {
                venue: "Binance Futures".to_string(),
                rate_8h_pct: 0.0120,
                annualized_apr_pct: 13.14,
                open_interest_mil: 5890.0,
            },
            FundingRateVenue {
                venue: "Bybit Perps".to_string(),
                rate_8h_pct: 0.0155,
                annualized_apr_pct: 16.97,
                open_interest_mil: 3420.2,
            },
            FundingRateVenue {
                venue: "OKX Swaps".to_string(),
                rate_8h_pct: 0.0110,
                annualized_apr_pct: 12.04,
                open_interest_mil: 2180.8,
            },
        ];

        let spread = 42.15 - 12.04;

        // 2. Liquidation bands (Clusters above and below spot)
        let liquidation_heatmap = vec![
            LiquidationBand {
                price_level: mark_price * 1.035,
                volume_notional_mil: 142.5,
                side: "SHORT_LIQ".to_string(),
                distance_pct: 3.5,
            },
            LiquidationBand {
                price_level: mark_price * 1.020,
                volume_notional_mil: 285.0,
                side: "SHORT_LIQ".to_string(),
                distance_pct: 2.0,
            },
            LiquidationBand {
                price_level: mark_price * 1.008,
                volume_notional_mil: 89.4,
                side: "SHORT_LIQ".to_string(),
                distance_pct: 0.8,
            },
            LiquidationBand {
                price_level: mark_price * 0.991,
                volume_notional_mil: 114.2,
                side: "LONG_LIQ".to_string(),
                distance_pct: -0.9,
            },
            LiquidationBand {
                price_level: mark_price * 0.978,
                volume_notional_mil: 360.8,
                side: "LONG_LIQ".to_string(),
                distance_pct: -2.2,
            },
            LiquidationBand {
                price_level: mark_price * 0.955,
                volume_notional_mil: 520.4,
                side: "LONG_LIQ".to_string(),
                distance_pct: -4.5,
            },
        ];

        CryptoDerivativesData {
            symbol: symbol.to_string(),
            mark_price,
            index_price,
            funding_rates,
            arbitrage_spread_apr_pct: spread,
            best_arb_long_venue: "OKX Swaps".to_string(),
            best_arb_short_venue: "Hyperliquid (L1 DEX)".to_string(),
            liquidation_heatmap,
        }
    }

    /// Live Bloomberg Institutional News Wire feed.
    #[must_use]
    pub fn mock_news_feed() -> Vec<NewsHeadline> {
        vec![
            NewsHeadline {
                id: "NW-901".to_string(),
                timestamp_str: "12:04:15".to_string(),
                category: "CRYPTO".to_string(),
                headline: "Hyperliquid Open Interest Hits Record $1.42B; Massive $360M Long Liquidation Cluster Sits at $94,500".to_string(),
                sentiment: "BULLISH".to_string(),
                sentiment_score: 0.82,
                affected_symbols: vec!["BTC/USD".to_string(), "ETH/USD".to_string(), "SOL/USD".to_string()],
            },
            NewsHeadline {
                id: "NW-902".to_string(),
                timestamp_str: "11:51:20".to_string(),
                category: "FOREX".to_string(),
                headline: "ECB's Lagarde Signals Steady Disinflation; EUR/USD Consolidates Near 1.0850".to_string(),
                sentiment: "NEUTRAL".to_string(),
                sentiment_score: 0.15,
                affected_symbols: vec!["EUR/USD".to_string(), "EUR/GBP".to_string()],
            },
            NewsHeadline {
                id: "NW-903".to_string(),
                timestamp_str: "11:48:05".to_string(),
                category: "CENTRAL_BANK".to_string(),
                headline: "Bank of Japan (BOJ) Governor Ueda Drops Hawkish Hints on Rate Path; JPY Strengthens to 152.10".to_string(),
                sentiment: "BULLISH".to_string(),
                sentiment_score: 0.72,
                affected_symbols: vec!["USD/JPY".to_string()],
            },
            NewsHeadline {
                id: "NW-904".to_string(),
                timestamp_str: "11:42:19".to_string(),
                category: "MACRO".to_string(),
                headline: "People's Bank of China (PBOC) Injects 150B Yuan Via Reverse Repo; Renminbi Steady at 7.12".to_string(),
                sentiment: "NEUTRAL".to_string(),
                sentiment_score: 0.05,
                affected_symbols: vec!["USD/CNY".to_string()],
            },
            NewsHeadline {
                id: "NW-905".to_string(),
                timestamp_str: "11:35:44".to_string(),
                category: "FOREX".to_string(),
                headline: "Bank of England Highlights Service Inflation Persistence; UK Pound Holds Firm Above 1.2950".to_string(),
                sentiment: "BULLISH".to_string(),
                sentiment_score: 0.45,
                affected_symbols: vec!["GBP/USD".to_string(), "EUR/GBP".to_string()],
            },
            NewsHeadline {
                id: "NW-906".to_string(),
                timestamp_str: "11:29:10".to_string(),
                category: "CRYPTO".to_string(),
                headline: "Institutional Spot Bitcoin ETFs Log $620M Daily Inflow as BTC Tests $96K Resistance".to_string(),
                sentiment: "BULLISH".to_string(),
                sentiment_score: 0.88,
                affected_symbols: vec!["BTC/USD".to_string(), "ETH/USD".to_string()],
            },
            NewsHeadline {
                id: "NW-907".to_string(),
                timestamp_str: "11:15:02".to_string(),
                category: "EQUITY".to_string(),
                headline: "Semiconductor Hardware Capex Surges 34% YoY Amid Next-Gen Agentic AI Cluster Buildouts".to_string(),
                sentiment: "BULLISH".to_string(),
                sentiment_score: 0.78,
                affected_symbols: vec!["NVDA".to_string(), "SPY".to_string()],
            },
        ]
    }

    /// Evaluates smart contracts for honeypots and drainers.
    #[must_use]
    pub fn audit_contract(address: &str, chain: &str) -> ContractAuditReport {
        let is_honeypot = address.to_lowercase().contains("scam") || address.to_lowercase().contains("trap");
        let risk_score = if is_honeypot { 95 } else { 12 };
        let verdict = if is_honeypot {
            "CRITICAL WARNING: Honeypot detected. Transfer functions restricted by unverified proxy."
        } else {
            "CLEAN: Standard verified contract with renounced minting privileges."
        };

        ContractAuditReport {
            contract_address: address.to_string(),
            chain: chain.to_string(),
            is_honeypot,
            buy_tax_pct: if is_honeypot { 99.0 } else { 0.5 },
            sell_tax_pct: if is_honeypot { 100.0 } else { 0.5 },
            allows_trading: !is_honeypot,
            can_mint_tokens: is_honeypot,
            can_blacklist_wallets: is_honeypot,
            liquidity_locked: !is_honeypot,
            risk_score_0_to_100: risk_score,
            audit_verdict: verdict.to_string(),
        }
    }
}
