# MoneyCraft UI Parity

**Stack:** `eframe` / `egui` 0.37 (L3 `moneycraft-app`), calls L1 dispatch
directly in-process. Obsidian canvas `#06080c`, amber ticker ribbons. No
WebView2, no HTTP sidecar for the desktop path.

## 1. Oracle loop

The terminal canvas is judged against Bloomberg-grade readability, not
against a web chart library.

1. Feed a fixed fixture series (known SMA/EMA/Bollinger/RSI values).
2. Render candlestick, mountain-area, and Heikin-Ashi modes headless.
3. Screenshot the real window at 60-bar density; compare overlays and
   crosshair HUD (time, OHLCV) against the fixture math.
4. Lock with `egui_kittest` goldens: one per chart mode plus the forex
   desk and the news ribbon.

## 2. L3 requirements

- `--smoke` headless CI mode (exits 0, no window).
- Golden PNGs for: candlestick + indicators, mountain area, Heikin-Ashi,
  forex desk (4-decimal and 2-decimal pip axes), news ribbon with
  BULLISH/BEARISH/NEUTRAL tags, Monte Carlo summary panel.
- `glyph_source_scan` tofu guard: candle glyphs, arrows, and CJK pair
  labels (CNY/RMB) must render from bundled faces.
- Snapping crosshair with OHLCV readout; date presets (1D..1Y) switch the
  same fixture deterministically in tests.
- Paper-only badges stay visible wherever a fill is simulated; nothing in
  the UI implies a live order.

## 3. Current status

Feed verbs and backtest arms are declared; fixture tests with known
numbers are the next step per README. Goldens lock once the fixture
series is frozen.

## 4. Screenshots

`docs/screenshots/` (to be added with the fixture freeze):

- `terminal-candles.png` - 60-bar canvas with SMA/EMA/Bollinger/RSI.
- `terminal-forex.png` - EUR/USD desk with pip axes.
- `terminal-news.png` - news ribbon with sentiment tags.
