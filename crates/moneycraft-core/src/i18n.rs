//! MoneyCraft Internationalization (i18n) Engine.
//!
//! Follows the fleet standard: typed locale enums, zero-panic string lookups,
//! multi-language translations (EN, DE, FR, ES, JA, KO, ZH),
//! and locale-aware numeric/currency formatting.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AppLocale {
    #[default]
    En, // English (Global Default)
    De, // German (Austrian / German Fleet native)
    Fr, // French
    Es, // Spanish
    Ja, // Japanese
    Ko, // Korean
    Zh, // Chinese
}

impl AppLocale {
    pub fn all() -> &'static [AppLocale] {
        &[
            AppLocale::En,
            AppLocale::De,
            AppLocale::Fr,
            AppLocale::Es,
            AppLocale::Ja,
            AppLocale::Ko,
            AppLocale::Zh,
        ]
    }

    pub fn code(&self) -> &'static str {
        match self {
            AppLocale::En => "en",
            AppLocale::De => "de",
            AppLocale::Fr => "fr",
            AppLocale::Es => "es",
            AppLocale::Ja => "ja",
            AppLocale::Ko => "ko",
            AppLocale::Zh => "zh",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            AppLocale::En => "English (US/UK)",
            AppLocale::De => "Deutsch (DE/AT/CH)",
            AppLocale::Fr => "Français (FR)",
            AppLocale::Es => "Español (ES/LATAM)",
            AppLocale::Ja => "日本語 (JA)",
            AppLocale::Ko => "한국어 (KO)",
            AppLocale::Zh => "中文 (ZH)",
        }
    }

    pub fn flag(&self) -> &'static str {
        match self {
            AppLocale::En => "🇬🇧",
            AppLocale::De => "🇩🇪",
            AppLocale::Fr => "🇫🇷",
            AppLocale::Es => "🇪🇸",
            AppLocale::Ja => "🇯🇵",
            AppLocale::Ko => "🇰🇷",
            AppLocale::Zh => "🇨🇳",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum I18nKey {
    // Navigation & Workspaces
    NavTerminal,
    NavDataSources,
    NavPersonalWealth,
    NavMultiCompare,
    NavCryptoDerivatives,
    NavFantasyTrading,
    NavAiWorkflows,
    NavHoneypotAuditor,
    NavBacktester,
    NavPortfolioLedger,

    // Personal Wealth & Inflation
    WealthTitle,
    WealthSubtitle,
    MonthlyIncome,
    LivingCosts,
    CheckingAccount,
    FestgeldAccount,
    BausparerAccount,
    InflationRunRate,
    SimulationHorizon,
    CurrentSavings,
    MonthlySavingsFlow,
    InterestEarned,
    InflationErosion,
    NetRealReturn,
    RuleOf72Alert,
    NominalIllusion,
    RealPurchasingPower,
    SilentInflationLoss,
    ActionableRoadmap,

    // Charting & Historic Ranges
    ChartRange,
    ChartStyle,
    Overlays,
    HistoricPanics,
    FeedSource,
    RealTime,
    Delayed15Min,
    DailyFixing,
    ExchangePayola,
}

pub fn t(key: I18nKey, locale: AppLocale) -> &'static str {
    match (key, locale) {
        // Navigation
        (I18nKey::NavTerminal, AppLocale::En) => "📊 Time-Series Terminal",
        (I18nKey::NavTerminal, AppLocale::De) => "📊 Zeitreihen-Terminal",
        (I18nKey::NavTerminal, AppLocale::Fr) => "📊 Terminal de séries",
        (I18nKey::NavTerminal, AppLocale::Es) => "📊 Terminal de series",
        (I18nKey::NavTerminal, AppLocale::Ja) => "📊 時系列ターミナル",
        (I18nKey::NavTerminal, AppLocale::Ko) => "📊 시계열 터미널",
        (I18nKey::NavTerminal, AppLocale::Zh) => "📊 时间序列终端",

        (I18nKey::NavDataSources, AppLocale::En) => "📡 Data Feeds & Payola",
        (I18nKey::NavDataSources, AppLocale::De) => "📡 Datenquellen & Börsen-Payola",
        (I18nKey::NavDataSources, AppLocale::Fr) => "📡 Flux de données & Coûts",
        (I18nKey::NavDataSources, AppLocale::Es) => "📡 Fuentes de datos y Tarifas",
        (I18nKey::NavDataSources, AppLocale::Ja) => "📡 データソースと手数料",
        (I18nKey::NavDataSources, AppLocale::Ko) => "📡 데이터 피드 및 거래소 비용",
        (I18nKey::NavDataSources, AppLocale::Zh) => "📡 数据源与交易所费用",

        (I18nKey::NavPersonalWealth, AppLocale::En) => "💶 Personal Wealth & Inflation",
        (I18nKey::NavPersonalWealth, AppLocale::De) => "💶 Privatvermögen & Inflation",
        (I18nKey::NavPersonalWealth, AppLocale::Fr) => "💶 Patrimoine & Inflation",
        (I18nKey::NavPersonalWealth, AppLocale::Es) => "💶 Patrimonio e Inflación",
        (I18nKey::NavPersonalWealth, AppLocale::Ja) => "💶 個人資産とインフレ侵食",
        (I18nKey::NavPersonalWealth, AppLocale::Ko) => "💶 개인 자산 및 인플레이션",
        (I18nKey::NavPersonalWealth, AppLocale::Zh) => "💶 个人财富与通货膨胀",

        (I18nKey::NavMultiCompare, AppLocale::En) => "📈 Multi-Asset Compare",
        (I18nKey::NavMultiCompare, AppLocale::De) => "📈 Multi-Asset Vergleich",
        (I18nKey::NavMultiCompare, AppLocale::Fr) => "📈 Comparaison multi-actifs",
        (I18nKey::NavMultiCompare, AppLocale::Es) => "📈 Comparación de activos",
        (I18nKey::NavMultiCompare, AppLocale::Ja) => "📈 複数資産比較",
        (I18nKey::NavMultiCompare, AppLocale::Ko) => "📈 다중 자산 비교",
        (I18nKey::NavMultiCompare, AppLocale::Zh) => "📈 多资产对比图",

        (I18nKey::NavCryptoDerivatives, AppLocale::En) => "🔥 Crypto Perps & Heatmap",
        (I18nKey::NavCryptoDerivatives, AppLocale::De) => "🔥 Krypto-Perps & Heatmap",
        (I18nKey::NavCryptoDerivatives, AppLocale::Fr) => "🔥 Perps crypto & Heatmap",
        (I18nKey::NavCryptoDerivatives, AppLocale::Es) => "🔥 Perps cripto y mapa",
        (I18nKey::NavCryptoDerivatives, AppLocale::Ja) => "🔥 暗号無期限先物＆ヒートマップ",
        (I18nKey::NavCryptoDerivatives, AppLocale::Ko) => "🔥 무기한 선물 및 청산 맵",
        (I18nKey::NavCryptoDerivatives, AppLocale::Zh) => "🔥 加密永续合约与热力图",

        (I18nKey::NavFantasyTrading, AppLocale::En) => "🎮 Fantasy Futures Sandbox",
        (I18nKey::NavFantasyTrading, AppLocale::De) => "🎮 Fantasy-Futures Sandbox",
        (I18nKey::NavFantasyTrading, AppLocale::Fr) => "🎮 Simulateur de contrats à terme",
        (I18nKey::NavFantasyTrading, AppLocale::Es) => "🎮 Simulador de futuros",
        (I18nKey::NavFantasyTrading, AppLocale::Ja) => "🎮 先物ペーパートレード",
        (I18nKey::NavFantasyTrading, AppLocale::Ko) => "🎮 모의 선물 트레이딩",
        (I18nKey::NavFantasyTrading, AppLocale::Zh) => "🎮 模拟期货沙盒",

        (I18nKey::NavAiWorkflows, AppLocale::En) => "🧠 AI Quant Workflows",
        (I18nKey::NavAiWorkflows, AppLocale::De) => "🧠 KI-Quant-Workflows",
        (I18nKey::NavAiWorkflows, AppLocale::Fr) => "🧠 Flux quantitatifs IA",
        (I18nKey::NavAiWorkflows, AppLocale::Es) => "🧠 Flujos cuantitativos IA",
        (I18nKey::NavAiWorkflows, AppLocale::Ja) => "🧠 AIクオンツ・ワークフロー",
        (I18nKey::NavAiWorkflows, AppLocale::Ko) => "🧠 AI 퀀트 워크플로우",
        (I18nKey::NavAiWorkflows, AppLocale::Zh) => "🧠 AI量化工作流",

        (I18nKey::NavHoneypotAuditor, AppLocale::En) => "🛡️ EVM Honeypot Auditor",
        (I18nKey::NavHoneypotAuditor, AppLocale::De) => "🛡️ EVM Honeypot-Prüfer",
        (I18nKey::NavHoneypotAuditor, AppLocale::Fr) => "🛡️ Auditeur de honeypots EVM",
        (I18nKey::NavHoneypotAuditor, AppLocale::Es) => "🛡️ Auditor de honeypots EVM",
        (I18nKey::NavHoneypotAuditor, AppLocale::Ja) => "🛡️ EVMハニーポット監査",
        (I18nKey::NavHoneypotAuditor, AppLocale::Ko) => "🛡️ EVM 허니팟 감사기",
        (I18nKey::NavHoneypotAuditor, AppLocale::Zh) => "🛡️ EVM蜜罐审计器",

        (I18nKey::NavBacktester, AppLocale::En) => "⚙️ Quantitative Backtester",
        (I18nKey::NavBacktester, AppLocale::De) => "⚙️ Quantitativer Backtester",
        (I18nKey::NavBacktester, AppLocale::Fr) => "⚙️ Backtest quantitatif",
        (I18nKey::NavBacktester, AppLocale::Es) => "⚙️ Backtester cuantitativo",
        (I18nKey::NavBacktester, AppLocale::Ja) => "⚙️ クオンツ・バックテスト",
        (I18nKey::NavBacktester, AppLocale::Ko) => "⚙️ 퀀트 백테스터",
        (I18nKey::NavBacktester, AppLocale::Zh) => "⚙️ 量化回测引擎",

        (I18nKey::NavPortfolioLedger, AppLocale::En) => "💼 Sovereign Portfolio",
        (I18nKey::NavPortfolioLedger, AppLocale::De) => "💼 Souveränes Portfolio",
        (I18nKey::NavPortfolioLedger, AppLocale::Fr) => "💼 Portefeuille souverain",
        (I18nKey::NavPortfolioLedger, AppLocale::Es) => "💼 Cartera soberana",
        (I18nKey::NavPortfolioLedger, AppLocale::Ja) => "💼 ソブリン・ポートフォリオ",
        (I18nKey::NavPortfolioLedger, AppLocale::Ko) => "💼 프라이빗 포트폴리오",
        (I18nKey::NavPortfolioLedger, AppLocale::Zh) => "💼 个人私密投资组合",

        // Wealth
        (I18nKey::WealthTitle, AppLocale::En) => "💶 PERSONAL WEALTH & INFLATION EROSION SIMULATOR",
        (I18nKey::WealthTitle, AppLocale::De) => "💶 PRIVATVERMÖGEN & INFLATIONS-EROSIONS-SIMULATOR",
        (I18nKey::WealthTitle, AppLocale::Fr) => "💶 SIMULATEUR DE PATRIMOINE ET D'ÉROSION INFLATIONNISTE",
        (I18nKey::WealthTitle, AppLocale::Es) => "💶 SIMULADOR DE PATRIMONIO PERSONAL Y EROSIÓN POR INFLACIÓN",
        (I18nKey::WealthTitle, AppLocale::Ja) => "💶 個人資産とインフレ購買力侵食シミュレーター",
        (I18nKey::WealthTitle, AppLocale::Ko) => "💶 개인 자산 성장 및 인플레이션 구매력 잠식 시뮬레이터",
        (I18nKey::WealthTitle, AppLocale::Zh) => "💶 个人财富与通货膨胀实际购买力侵蚀模拟器",

        (I18nKey::WealthSubtitle, AppLocale::En) => "Surfacing the 'Bank App Illusion' — Why 3% Festgeld sounds safe while 3.8% inflation quietly destroys your purchasing power.",
        (I18nKey::WealthSubtitle, AppLocale::De) => "Entlarvt die 'Banking-App-Illusion' — Warum 3% Festgeld sicher klingen, während 3,8% Inflation lautlos Ihr Erspartes vernichten.",
        (I18nKey::WealthSubtitle, AppLocale::Fr) => "Révèle l'illusion bancaire — Pourquoi 3% semblent sûrs alors que 3,8% d'inflation détruisent votre pouvoir d'achat.",
        (I18nKey::WealthSubtitle, AppLocale::Es) => "Revela la ilusión bancaria — Por qué el 3% parece seguro mientras que el 3.8% de inflación destruye su poder adquisitivo.",
        (I18nKey::WealthSubtitle, AppLocale::Ja) => "「銀行アプリの錯覚」を解明 — 3%の定期預金が安全に見えても、3.8%のインフレが購買力を静かに奪う理由。",
        (I18nKey::WealthSubtitle, AppLocale::Ko) => "‘은행 앱의 착시’ 공개 — 3% 정기예금이 안전해 보여도 3.8% 인플레이션이 실질 구매력을 파괴하는 이유.",
        (I18nKey::WealthSubtitle, AppLocale::Zh) => "揭开“银行APP的数字幻觉” — 为什么3%的定期看似安全，却在3.8%的通胀下悄然缩水。",

        (I18nKey::MonthlyIncome, AppLocale::En) => "Monthly Net Salary",
        (I18nKey::MonthlyIncome, AppLocale::De) => "Monatliches Nettoeinkommen",
        (I18nKey::MonthlyIncome, AppLocale::Fr) => "Salaire net mensuel",
        (I18nKey::MonthlyIncome, AppLocale::Es) => "Salario neto mensual",
        (I18nKey::MonthlyIncome, AppLocale::Ja) => "月々の手取り収入",
        (I18nKey::MonthlyIncome, AppLocale::Ko) => "월 실수령액",
        (I18nKey::MonthlyIncome, AppLocale::Zh) => "每月税后净收入",

        (I18nKey::LivingCosts, AppLocale::En) => "Monthly Living Costs",
        (I18nKey::LivingCosts, AppLocale::De) => "Monatliche Lebenshaltungskosten",
        (I18nKey::LivingCosts, AppLocale::Fr) => "Coût de la vie mensuel",
        (I18nKey::LivingCosts, AppLocale::Es) => "Costos mensuales de vida",
        (I18nKey::LivingCosts, AppLocale::Ja) => "月々の生活費",
        (I18nKey::LivingCosts, AppLocale::Ko) => "월 고정 생활비",
        (I18nKey::LivingCosts, AppLocale::Zh) => "每月生活开支",

        (I18nKey::CheckingAccount, AppLocale::En) => "Checking Account (Girokonto)",
        (I18nKey::CheckingAccount, AppLocale::De) => "Girokonto (0,0% Zinsen)",
        (I18nKey::CheckingAccount, AppLocale::Fr) => "Compte courant (0,0% d'intérêt)",
        (I18nKey::CheckingAccount, AppLocale::Es) => "Cuenta corriente (0,0% interés)",
        (I18nKey::CheckingAccount, AppLocale::Ja) => "当座・普通預金（金利0%）",
        (I18nKey::CheckingAccount, AppLocale::Ko) => "보통예금 (이자율 0%)",
        (I18nKey::CheckingAccount, AppLocale::Zh) => "活期账户（0%利息）",

        (I18nKey::FestgeldAccount, AppLocale::En) => "Festgeld (Time Deposit)",
        (I18nKey::FestgeldAccount, AppLocale::De) => "Festgeld (Gebunden)",
        (I18nKey::FestgeldAccount, AppLocale::Fr) => "Dépôt à terme (Fixe)",
        (I18nKey::FestgeldAccount, AppLocale::Es) => "Depósito a plazo fijo",
        (I18nKey::FestgeldAccount, AppLocale::Ja) => "定期預金（満期拘束）",
        (I18nKey::FestgeldAccount, AppLocale::Ko) => "정기예금 (만기 묶임)",
        (I18nKey::FestgeldAccount, AppLocale::Zh) => "定期存款（期限锁定）",

        (I18nKey::BausparerAccount, AppLocale::En) => "Bausparer (Home-Loan Savings)",
        (I18nKey::BausparerAccount, AppLocale::De) => "Bausparvertrag",
        (I18nKey::BausparerAccount, AppLocale::Fr) => "Épargne logement (Bauspar)",
        (I18nKey::BausparerAccount, AppLocale::Es) => "Ahorro para vivienda",
        (I18nKey::BausparerAccount, AppLocale::Ja) => "住宅積立貯蓄（バウスパー）",
        (I18nKey::BausparerAccount, AppLocale::Ko) => "주택청약 / 주택저축",
        (I18nKey::BausparerAccount, AppLocale::Zh) => "住房储蓄公积金",

        (I18nKey::InflationRunRate, AppLocale::En) => "Annual Inflation Run-Rate",
        (I18nKey::InflationRunRate, AppLocale::De) => "Jährliche Inflationsrate",
        (I18nKey::InflationRunRate, AppLocale::Fr) => "Taux d'inflation annuel",
        (I18nKey::InflationRunRate, AppLocale::Es) => "Tasa de inflación anual",
        (I18nKey::InflationRunRate, AppLocale::Ja) => "年間実質インフレ率",
        (I18nKey::InflationRunRate, AppLocale::Ko) => "연간 인플레이션율",
        (I18nKey::InflationRunRate, AppLocale::Zh) => "年化实际通胀率",

        (I18nKey::NominalIllusion, AppLocale::En) => "Nominal Total (Bank App)",
        (I18nKey::NominalIllusion, AppLocale::De) => "Nominales Guthaben (Bank-App)",
        (I18nKey::NominalIllusion, AppLocale::Fr) => "Solde nominal (App bancaire)",
        (I18nKey::NominalIllusion, AppLocale::Es) => "Saldo nominal (App bancaria)",
        (I18nKey::NominalIllusion, AppLocale::Ja) => "名目残高（銀行アプリ表示）",
        (I18nKey::NominalIllusion, AppLocale::Ko) => "명목 잔액 (은행 앱 표시)",
        (I18nKey::NominalIllusion, AppLocale::Zh) => "名义总资产（银行APP数值）",

        (I18nKey::RealPurchasingPower, AppLocale::En) => "Real Value (True Goods)",
        (I18nKey::RealPurchasingPower, AppLocale::De) => "Reale Kaufkraft (Warenwert)",
        (I18nKey::RealPurchasingPower, AppLocale::Fr) => "Valeur réelle (Pouvoir d'achat)",
        (I18nKey::RealPurchasingPower, AppLocale::Es) => "Valor real (Poder adquisitivo)",
        (I18nKey::RealPurchasingPower, AppLocale::Ja) => "実質価値（購入可能な物品）",
        (I18nKey::RealPurchasingPower, AppLocale::Ko) => "실질 구매력 (실제 상품 가치)",
        (I18nKey::RealPurchasingPower, AppLocale::Zh) => "实际价值（真实购买商品力）",

        (I18nKey::SilentInflationLoss, AppLocale::En) => "Silent Inflation Loss",
        (I18nKey::SilentInflationLoss, AppLocale::De) => "Lautloser Inflationsverlust",
        (I18nKey::SilentInflationLoss, AppLocale::Fr) => "Perte silencieuse d'inflation",
        (I18nKey::SilentInflationLoss, AppLocale::Es) => "Pérdida silenciosa por inflación",
        (I18nKey::SilentInflationLoss, AppLocale::Ja) => "インフレによるステルス損失",
        (I18nKey::SilentInflationLoss, AppLocale::Ko) => "보이지 않는 인플레이션 손실",
        (I18nKey::SilentInflationLoss, AppLocale::Zh) => "隐形通胀蒸发亏损",

        (I18nKey::HistoricPanics, AppLocale::En) => "🚩 Historic Financial Panics",
        (I18nKey::HistoricPanics, AppLocale::De) => "🚩 Historische Finanz-Paniken",
        (I18nKey::HistoricPanics, AppLocale::Fr) => "🚩 Paniques financières historiques",
        (I18nKey::HistoricPanics, AppLocale::Es) => "🚩 Pánicos financieros históricos",
        (I18nKey::HistoricPanics, AppLocale::Ja) => "🚩 歴史的金融パニック",
        (I18nKey::HistoricPanics, AppLocale::Ko) => "🚩 역사적 금융 대공황",
        (I18nKey::HistoricPanics, AppLocale::Zh) => "🚩 历史重大金融恐慌",

        (I18nKey::FeedSource, AppLocale::En) => "Feed Source",
        (I18nKey::FeedSource, AppLocale::De) => "Datenquelle",
        (I18nKey::FeedSource, AppLocale::Fr) => "Source de données",
        (I18nKey::FeedSource, AppLocale::Es) => "Fuente de datos",
        (I18nKey::FeedSource, AppLocale::Ja) => "データフィード供給元",
        (I18nKey::FeedSource, AppLocale::Ko) => "데이터 공급원",
        (I18nKey::FeedSource, AppLocale::Zh) => "数据行情来源",

        // Default fallbacks
        _ => "MoneyCraft",
    }
}
