//! MoneyCraft Historical Deep-Archive & Financial Panics Engine (1720 to 2026).
//!
//! Models 300+ years of capital market history:
//! London Exchange Alley, The Honourable East India Company (EIC),
//! Bank of England 3% Perpetual Consols, South Sea Bubble,
//! and all 19 major historical panics with interactive visual chart annotations.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricPanicMarker {
    pub year: u32,
    pub date_str: String,
    pub name: String,
    pub epicenter: String,
    pub peak_to_trough_drop_pct: f64,
    pub primary_catalyst: String,
    pub narrative: String,
    pub affected_instruments: Vec<String>,
    pub lessons_for_modern_quants: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalAsset {
    pub symbol: String,
    pub name: String,
    pub inception_year: u32,
    pub sunset_year: Option<u32>,
    pub primary_exchange: String,
    pub base_currency: String,
    pub historical_notes: String,
}

pub struct MarketHistory;

impl MarketHistory {
    /// Returns the comprehensive chronicle of historical market crises.
    #[must_use]
    pub fn all_panics() -> Vec<HistoricPanicMarker> {
        vec![
            HistoricPanicMarker {
                year: 1720,
                date_str: "1720-09".to_string(),
                name: "The South Sea Bubble & Mississippi Collapse".to_string(),
                epicenter: "London (Exchange Alley) & Paris".to_string(),
                peak_to_trough_drop_pct: -84.0,
                primary_catalyst: "Debt-for-equity swap frenzy and John Law's fiat credit expansion".to_string(),
                narrative: "South Sea Company stock surged from £128 to £1,050 in 8 months before crashing. Sir Isaac Newton lost £20,000 (~£4M today), lamenting: 'I can calculate the motion of heavenly bodies, but not the madness of people.'".to_string(),
                affected_instruments: vec!["SOUTH_SEA".to_string(), "BANK_OF_ENGLAND".to_string(), "EIC".to_string()],
                lessons_for_modern_quants: "Reflexive liquidity loops collapse immediately once new capital inflows slow down.".to_string(),
            },
            HistoricPanicMarker {
                year: 1772,
                date_str: "1772-06".to_string(),
                name: "The London Credit Crisis of 1772".to_string(),
                epicenter: "London (Lombard Street) & Edinburgh".to_string(),
                peak_to_trough_drop_pct: -38.0,
                primary_catalyst: "Speculator Alexander Fordyce absconding to France after shorting EIC stock".to_string(),
                narrative: "Fordyce's default caused the collapse of Neal, James, Fordyce and Down, triggering a massive domino run across Scottish private banks (Ayr Bank collapsed) and leading directly to the Tea Act of 1773 and Boston Tea Party.".to_string(),
                affected_instruments: vec!["EIC".to_string(), "UK_CONSOLS".to_string(), "AYR_BANK".to_string()],
                lessons_for_modern_quants: "Interbank shadow-lending chains can turn a single rogue trader's short squeeze into an empire-wide systemic crunch.".to_string(),
            },
            HistoricPanicMarker {
                year: 1825,
                date_str: "1825-12".to_string(),
                name: "The Panic of 1825 (Poyais Fraud & Country Banks)".to_string(),
                epicenter: "London Stock Exchange".to_string(),
                peak_to_trough_drop_pct: -45.0,
                primary_catalyst: "Speculative Latin American sovereign bond bubble and Gregor MacGregor's fictitious 'Principality of Poyais' bonds".to_string(),
                narrative: "Over 70 London and country banks closed within weeks. The Bank of England was reduced to just £1 million in gold sovereigns, avoiding total suspension only when an old box of unissued £1 notes was discovered in the vaults.".to_string(),
                affected_instruments: vec!["UK_CONSOLS".to_string(), "POYAIS_BONDS".to_string(), "EIC".to_string()],
                lessons_for_modern_quants: "Exotic foreign yield chasing without verifiable underlying collateral always ends in a liquidity run.".to_string(),
            },
            HistoricPanicMarker {
                year: 1837,
                date_str: "1837-05".to_string(),
                name: "The Panic of 1837 & The Great Deflation".to_string(),
                epicenter: "New York & New Orleans".to_string(),
                peak_to_trough_drop_pct: -42.0,
                primary_catalyst: "Andrew Jackson's Specie Circular demanding hard gold/silver for public lands".to_string(),
                narrative: "Over 600 American banks collapsed; cotton prices plunged 50%. Triggered a 7-year global deflationary depression where US state debts defaulted.".to_string(),
                affected_instruments: vec!["US_BANKS".to_string(), "COTTON".to_string(), "UK_CONSOLS".to_string()],
                lessons_for_modern_quants: "Sudden central shifts from fiat credit to hard monetary settlement induce instantaneous credit contraction.".to_string(),
            },
            HistoricPanicMarker {
                year: 1847,
                date_str: "1847-10".to_string(),
                name: "The Railway Mania Collapse".to_string(),
                epicenter: "London (Great George Street)".to_string(),
                peak_to_trough_drop_pct: -67.0,
                primary_catalyst: "British Parliament chartering 9,500 miles of duplicate railway track".to_string(),
                narrative: "Investors overcommitted on uncalled margin instalments. When the Bank of England raised rates due to the Irish potato famine grain imports, railway shares imploded and the Bank Charter Act of 1844 had to be suspended.".to_string(),
                affected_instruments: vec!["BRITISH_RAILS".to_string(), "UK_CONSOLS".to_string()],
                lessons_for_modern_quants: "Infrastructure overbuilding funded by retail leveraged calls creates catastrophic capital drawdowns.".to_string(),
            },
            HistoricPanicMarker {
                year: 1857,
                date_str: "1857-08".to_string(),
                name: "The Panic of 1857 (First World Telegraph Panic)".to_string(),
                epicenter: "Wall Street & London (City)".to_string(),
                peak_to_trough_drop_pct: -35.0,
                primary_catalyst: "Ohio Life failure and the sinking of SS Central America with 30,000 lbs of gold".to_string(),
                narrative: "The loss of 400 lives and $1.6M in California bullion on the SS Central America shook New York banks. Thanks to the newly laid electric telegraph, panic spread worldwide within hours.".to_string(),
                affected_instruments: vec!["EIC".to_string(), "UK_CONSOLS".to_string(), "NY_BANKS".to_string()],
                lessons_for_modern_quants: "Communication speed compresses the timeframe of bank runs from weeks to minutes.".to_string(),
            },
            HistoricPanicMarker {
                year: 1866,
                date_str: "1866-05-11".to_string(),
                name: "Black Friday & The Overend Gurney Collapse".to_string(),
                epicenter: "London (Lombard Street)".to_string(),
                peak_to_trough_drop_pct: -42.0,
                primary_catalyst: "Insolvency of Overend, Gurney & Co. ('The Bankers' Bank') with £11M liabilities".to_string(),
                narrative: "Crowds besieged Lombard Street. The Bank of England advanced £4M in a single afternoon, establishing Walter Bagehot's principle: 'Lend freely at a high rate on good collateral'.".to_string(),
                affected_instruments: vec!["OVEREND_GURNEY".to_string(), "UK_CONSOLS".to_string(), "EIC".to_string()],
                lessons_for_modern_quants: "Wholesale discount houses holding illiquid long-dated assets with short-term repo funding are structurally fragile.".to_string(),
            },
            HistoricPanicMarker {
                year: 1873,
                date_str: "1873-05".to_string(),
                name: "The Panic of 1873 & The Vienna Gründerkrach".to_string(),
                epicenter: "Vienna Stock Exchange (Börse) & Wall Street".to_string(),
                peak_to_trough_drop_pct: -48.0,
                primary_catalyst: "Collapse of Vienna World Exhibition stock speculation and Jay Cooke bankruptcy".to_string(),
                narrative: "Vienna's May 9 'Black Friday' halted Austro-Hungarian banking; by September, Jay Cooke failed in NYC, shuttering the NYSE for 10 days. The British government dissolved the East India Company in 1874.".to_string(),
                affected_instruments: vec!["EIC".to_string(), "VIENNA_BOERSE".to_string(), "NORTHERN_PACIFIC".to_string(), "UK_CONSOLS".to_string()],
                lessons_for_modern_quants: "Global monetary transitions (demonetization of silver) create multi-decade deflationary drags.".to_string(),
            },
            HistoricPanicMarker {
                year: 1890,
                date_str: "1890-11".to_string(),
                name: "The Baring Crisis of 1890".to_string(),
                epicenter: "London (Bishopsgate) & Buenos Aires".to_string(),
                peak_to_trough_drop_pct: -32.0,
                primary_catalyst: "Argentine sovereign debt default and Buenos Aires waterworks underwriting".to_string(),
                narrative: "Barings Bank, the world's most prestigious merchant bank, faced imminent insolvency. Bank of England Governor William Lidderdale organized an unprecedented £17M consortium guarantee without touching taxpayer funds.".to_string(),
                affected_instruments: vec!["BARINGS".to_string(), "ARGENTINE_BONDS".to_string(), "UK_CONSOLS".to_string()],
                lessons_for_modern_quants: "Illiquid sovereign underwriting syndicates require private consortium backstops to prevent contagion.".to_string(),
            },
            HistoricPanicMarker {
                year: 1907,
                date_str: "1907-10".to_string(),
                name: "The Knickerbocker Panic of 1907".to_string(),
                epicenter: "New York (Wall Street)".to_string(),
                peak_to_trough_drop_pct: -48.0,
                primary_catalyst: "Failed United Copper corner by F. Augustus Heinze; run on Knickerbocker Trust".to_string(),
                narrative: "J. Pierpont Morgan locked New York's top bank presidents in his library until 4:45 AM until they pledged $25 million in liquidity to save the trust companies. Led directly to the Federal Reserve Act of 1913.".to_string(),
                affected_instruments: vec!["KNICKERBOCKER".to_string(), "DOW_JONES".to_string(), "US_STEEL".to_string()],
                lessons_for_modern_quants: "Shadow banks outside clearinghouse regulation trigger system-wide liquidity spirals.".to_string(),
            },
            HistoricPanicMarker {
                year: 1929,
                date_str: "1929-10-29".to_string(),
                name: "The Great Wall Street Crash (Black Tuesday)".to_string(),
                epicenter: "New York Stock Exchange".to_string(),
                peak_to_trough_drop_pct: -89.2,
                primary_catalyst: "Excessive 10% margin debt and speculative investment trusts".to_string(),
                narrative: "The Dow collapsed from 381 down to 41.22 in 1932. Ushered in the Great Depression, 25% US unemployment, and the creation of the SEC and Glass-Steagall.".to_string(),
                affected_instruments: vec!["DJIA".to_string(), "UK_CONSOLS".to_string(), "EQUITIES".to_string()],
                lessons_for_modern_quants: "Leverage turns orderly re-pricings into cascading insolvency spirals.".to_string(),
            },
            HistoricPanicMarker {
                year: 1973,
                date_str: "1973-10".to_string(),
                name: "The 1973 Oil Shock & Stagflation Crisis".to_string(),
                epicenter: "Middle East / Global FX".to_string(),
                peak_to_trough_drop_pct: -48.0,
                primary_catalyst: "OAPEC oil embargo following Yom Kippur War; end of Bretton Woods gold standard".to_string(),
                narrative: "Crude oil quadrupled; Western economies plunged into simultaneous double-digit inflation and high unemployment (stagflation). Gold soared from $35 to $850.".to_string(),
                affected_instruments: vec!["SPX".to_string(), "DJIA".to_string(), "GOLD".to_string(), "CRUDE_OIL".to_string()],
                lessons_for_modern_quants: "Supply-side commodity shocks cannot be solved by monetary easing.".to_string(),
            },
            HistoricPanicMarker {
                year: 1987,
                date_str: "1987-10-19".to_string(),
                name: "Black Monday (-22.6% in 1 Day)".to_string(),
                epicenter: "NYSE / Chicago Merc (CME)".to_string(),
                peak_to_trough_drop_pct: -33.5,
                primary_catalyst: "Automated portfolio insurance stop-loss cascade and S&P futures decoupling".to_string(),
                narrative: "Largest single-day percentage decline in financial history. Computerized hedging algorithms automatically sold index futures, triggering NYSE order-queue congestion.".to_string(),
                affected_instruments: vec!["DJIA".to_string(), "SPX".to_string(), "FTSE".to_string(), "DAX".to_string()],
                lessons_for_modern_quants: "Vectorized mechanical hedging strategies create fatal end-user feedback loops without human circuit breakers.".to_string(),
            },
            HistoricPanicMarker {
                year: 1997,
                date_str: "1997-07".to_string(),
                name: "The Asian Financial Crisis (Tom Yam Kung Crisis)".to_string(),
                epicenter: "Bangkok, Seoul, Jakarta, Hong Kong".to_string(),
                peak_to_trough_drop_pct: -65.0,
                primary_catalyst: "Thai Baht peg break; dollar-denominated short-term corporate debt mismatch".to_string(),
                narrative: "South Korea's KOSPI crashed; major chaebols went bankrupt. Citizens donated personal gold jewelry to the government to repay emergency IMF loans.".to_string(),
                affected_instruments: vec!["KOSPI".to_string(), "HSI".to_string(), "USD/THB".to_string(), "USD/KRW".to_string()],
                lessons_for_modern_quants: "Currency pegs with inadequate foreign reserves will always be broken by macro momentum quants.".to_string(),
            },
            HistoricPanicMarker {
                year: 1998,
                date_str: "1998-08".to_string(),
                name: "Russian Ruble Default & The LTCM Liquidation".to_string(),
                epicenter: "Moscow & Greenwich, Connecticut".to_string(),
                peak_to_trough_drop_pct: -22.0,
                primary_catalyst: "Russian GKO debt default and Long-Term Capital Management 100:1 leverage blowout".to_string(),
                narrative: "LTCM, managed by Nobel laureates Robert Merton and Myron Scholes, blew through $4.6B in equity in 6 weeks. The Federal Reserve coordinated a $3.6B private Wall Street bailout.".to_string(),
                affected_instruments: vec!["LTCM".to_string(), "RBL".to_string(), "SPX".to_string(), "GLOBAL_BONDS".to_string()],
                lessons_for_modern_quants: "Gaussian normal distribution models underestimate fat tails by multiple orders of magnitude.".to_string(),
            },
            HistoricPanicMarker {
                year: 2000,
                date_str: "2000-03".to_string(),
                name: "The Dot-Com Tech Bubble Crash".to_string(),
                epicenter: "NASDAQ (Silicon Valley)".to_string(),
                peak_to_trough_drop_pct: -78.4,
                primary_catalyst: "Mass speculative euphoria in non-profitable internet IPOs with extreme price-to-eyeball multiples".to_string(),
                narrative: "The Nasdaq Composite peaked at 5,048 in March 2000 before plunging to 1,114 in October 2002. Trillions in paper wealth evaporated; Pets.com, Webvan, and WorldCom collapsed.".to_string(),
                affected_instruments: vec!["NDX".to_string(), "SPX".to_string(), "DJIA".to_string()],
                lessons_for_modern_quants: "Cashflow matters. When liquidity dries up, pre-revenue speculative growth drops 90%+.".to_string(),
            },
            HistoricPanicMarker {
                year: 2008,
                date_str: "2008-09-15".to_string(),
                name: "The Global Financial Crisis (GFC / Lehman Bankruptcy)".to_string(),
                epicenter: "Wall Street, London, Frankfurt, Reykjavik".to_string(),
                peak_to_trough_drop_pct: -56.8,
                primary_catalyst: "US Subprime mortgage securitization collapse (MBS/CDO) and shadow bank run".to_string(),
                narrative: "Lehman Brothers filed for Chapter 11 bankruptcy; AIG, Fannie Mae, Freddie Mac, and Royal Bank of Scotland were nationalized. Ushered in ZIRP (Zero Interest Rate Policy) and Quantitative Easing (QE).".to_string(),
                affected_instruments: vec!["SPX".to_string(), "DJIA".to_string(), "DAX".to_string(), "FTSE".to_string(), "GOLD".to_string()],
                lessons_for_modern_quants: "Credit derivatives can mask extreme correlation risk until an entire asset class goes bidless.".to_string(),
            },
            HistoricPanicMarker {
                year: 2020,
                date_str: "2020-03".to_string(),
                name: "The Covid-19 Liquidity Shock & Negative Oil Flash".to_string(),
                epicenter: "Global Markets".to_string(),
                peak_to_trough_drop_pct: -34.0,
                primary_catalyst: "Global pandemic economic shutdown and massive margin liquidation across all asset classes".to_string(),
                narrative: "Fastest 30% drop in market history. S&P 500 halted by Level 1 circuit breakers 4 times in 10 days. WTI crude oil briefly traded at -$37.63/barrel due to physical storage saturation.".to_string(),
                affected_instruments: vec!["SPX".to_string(), "DJIA".to_string(), "BTC/USD".to_string(), "OIL_WTI".to_string()],
                lessons_for_modern_quants: "In an acute liquidity crunch, all correlations converge to 1.0 except the US Dollar and physical cash.".to_string(),
            },
            HistoricPanicMarker {
                year: 2022,
                date_str: "2022-01".to_string(),
                name: "The Global Inflation & Rate Hiking Shock (SVB Run)".to_string(),
                epicenter: "US Treasuries, Tech Equities, Silicon Valley Bank".to_string(),
                peak_to_trough_drop_pct: -25.4,
                primary_catalyst: "Post-Covid fiscal expansion meeting supply shocks, forcing the fastest 500 bps Fed rate hikes in 40 years".to_string(),
                narrative: "Long-dated bonds suffered their worst bear market in 200 years (-40% on 30-year US Treasuries). In March 2023, Silicon Valley Bank suffered a $42 billion social-media bank run in under 10 hours.".to_string(),
                affected_instruments: vec!["US_TREASURIES".to_string(), "NDX".to_string(), "SVB".to_string(), "CRYPTO".to_string()],
                lessons_for_modern_quants: "Duration risk kills when deposits are flighty; nominal safety in bonds is destroyed by real interest rate moves.".to_string(),
            },
        ]
    }

    /// Returns historic instruments like East India Company and Consols.
    #[must_use]
    pub fn historical_instruments() -> Vec<HistoricalAsset> {
        vec![
            HistoricalAsset {
                symbol: "EIC".to_string(),
                name: "Honourable East India Company Stock".to_string(),
                inception_year: 1600,
                sunset_year: Some(1874),
                primary_exchange: "London (Exchange Alley & Royal Exchange)".to_string(),
                base_currency: "GBP (£)".to_string(),
                historical_notes: "The world's first mega-corporation, chartered by Queen Elizabeth I. Commanded its own private army of 260,000 men. Traded in London until dissolved by the East India Stock Dividend Redemption Act of 1873.".to_string(),
            },
            HistoricalAsset {
                symbol: "UK_CONSOLS".to_string(),
                name: "Bank of England 3% Consolidated Annuities".to_string(),
                inception_year: 1751,
                sunset_year: None,
                primary_exchange: "Bank of England / London Stock Exchange".to_string(),
                base_currency: "GBP (£)".to_string(),
                historical_notes: "The world's longest-running continuous sovereign bond. Funded the Napoleonic Wars and the British Empire for over 200 years without defaulting.".to_string(),
            },
            HistoricalAsset {
                symbol: "SOUTH_SEA".to_string(),
                name: "The Governor and Company of the merchants of Great Britain, trading to the South Seas".to_string(),
                inception_year: 1711,
                sunset_year: Some(1853),
                primary_exchange: "London (Exchange Alley & South Sea House)".to_string(),
                base_currency: "GBP (£)".to_string(),
                historical_notes: "Formed in 1711 to reduce British national debt. Led to the catastrophic 1720 South Sea Bubble and the Bubble Act of 1720.".to_string(),
            },
        ]
    }
}
