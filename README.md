# gtm-10-auto-qbr

**GTM Engineering — Stage 10 of 12: The Automated QBR (Quarterly Business Review)**

Build the quarterly business review from data, not from a weekend of slides: quarter-over-quarter results vs goals, wins and risks picked by rule, chart SVGs, a slide outline, and an ROI model with a conservative case.

**Live demo:** https://gtm-10-auto-qbr.braylo777.workers.dev — the Rust core runs in your browser as WebAssembly. No server, no tracking.

## Why this matters

A QBR is the moment a customer decides whether to renew and expand. Most CSMs spend a day per account copying numbers into slides, and the ROI claim is often a guess the customer's CFO ignores. An automated QBR pulls the same metrics every quarter, applies the same rules for what counts as a win or a risk, and shows ROI with a conservative case the CFO can trust.

## What it does

- Quarter-over-quarter change per metric, with direction (higher or lower is better) and goal met / missed
- Rule-based wins (met goal or improved ≥ threshold) and risks (missed goal and declined); status on_track / improving / at_risk
- Bar-chart SVG per metric (last Q, this Q, dashed goal line), text escaped
- Slide outline in a fixed order: title, results vs goals, wins, value delivered, risks and asks, next quarter
- ROI model: time saved × loaded rate, revenue lift × gross margin, cost avoided; ROI %, net, payback months; base and conservative (haircut) cases

## What it does not do (yet)

- Outputs an outline and SVGs, not a .pptx or Google Slides file
- No CRM / product-analytics pull: metrics are synthetic JSON
- No narrative LLM writing: bullets are deterministic

## Operations

Every operation takes JSON in and gives JSON out. The CLI, the tests and the demo page all use the same entry point.

| Operation | What it does |
|---|---|
| `qbr_deck` | Compare Q3 and Q4 metrics against goals, then build wins, risks and the slide outline. |
| `roi_model` | Value the account's results in dollars, with a 30% haircut for the conservative case. |

## Run it

You need Rust (stable). No Node, no npm.

```bash
git clone https://github.com/braylo777/gtm-10-auto-qbr
cd gtm-10-auto-qbr
cargo test
cargo run -- ops
```

Try the first demo example from the command line:

```bash
python3 -c 'import json;print(json.dumps(json.load(open("web/examples.json"))["ops"][0]["input"]))' | cargo run -q -- qbr_deck
```

## Build and deploy the demo

```bash
./build.sh    # tests, then compiles the core to web/app.wasm
./deploy.sh   # serves web/ as a Cloudflare Workers static-assets site (needs a Cloudflare login)
```

`web/` is a plain static site (`index.html`, `app.js`, `app.wasm`). It has no build step, so any static host works.

## Layout

```
src/lib.rs        operation list and router
src/api.rs        JSON entry point + the 3 WASM exports (alloc, dealloc, run)
src/main.rs       CLI
src/*.rs          one module per capability, each with unit tests
tests/            integration tests (every demo example must pass)
web/              demo page; examples.json drives both the page and this README
```

## Design choices

- **Rust, few dependencies.** Only `serde_json`, pinned to exact versions with a committed `Cargo.lock`.
- **Deterministic.** Time, randomness and keys are inputs, so every result can be replayed in a test or an audit.
- **Fail closed.** Unknown input or missing fields give an error, never a silent default.

## Part of a series

This is one of 12 repos that work through a public Go-To-Market (GTM) Engineering roadmap: [12 stages by Suraj Sharma (@suraj_sharma14)](https://x.com/suraj_sharma14/status/2125552583452270692). The roadmap is the inspiration; the code here is original. All 12: [github.com/braylo777?q=gtm-](https://github.com/braylo777?tab=repositories&q=gtm-).

## Data and privacy

Every example uses synthetic data. Nothing calls a paid API, scrapes a site, identifies a real visitor or sends an email.

## License

MIT — see [LICENSE](LICENSE). Built by [Braylo SI](https://braylo.si).
