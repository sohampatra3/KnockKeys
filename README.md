# TypeTrainer (Terminal TUI)

A fully offline terminal typing trainer inspired by keybr.com, built with Rust, `ratatui`, and `crossterm`.

## Build

```bash
cargo build --release
```

## Run

```bash
./target/release/typetrainer
```

## Optional AI analytics (Typing Guru report)

Set your API key before launching to enable the Home -> `Typing Guru AI Report` button:

```bash
export GROQ_API_KEY=\"your_key_here\"
./target/release/typetrainer
```

Optional overrides:

```bash
export TYPETRAINER_GROQ_MODEL=\"gpt-oss-120b\"
export TYPETRAINER_GROQ_URL=\"https://api.groq.com/openai/v1/chat/completions\"
```

## CLI utilities

```bash
./target/release/typetrainer export-csv
./target/release/typetrainer backup
./target/release/typetrainer import ~/.typetrainer/backups/progress-YYYYMMDD-HHMMSS.json
```

## Data storage

Progress is saved locally at:

- `~/.typetrainer/progress.json`
- backups: `~/.typetrainer/backups/`
- CSV exports: `~/.typetrainer/exports/`

## Extra features included

- Focus Mode (hides metrics sidebar while typing)
- Weak Keys Drill mode
- Daily Goal tracking (minutes per day)
- Session recap screen with problematic keys and WPM sparkline
- Export stats to CSV + backup/import JSON
- Code Drill mode with per-language speed tracking:
  - Languages: C++, C, Rust, Python, Go
  - Topics: loops, if/else, sorting, searching, mixed
- Lessons mode now includes:
  - Mixed Keyboard Practice
  - Full Keyboard Practice
- Expanded themes: Ocean, Ember, Forest, Slate, Solarized, Neon

## ASCII sample (home)

```text
+-----------------------------------------------------------+
| TypeTrainer - Adaptive Typing Coach                       |
+-----------------------------------------------------------+
| Daily Goal [####------] 4/10 min                          |
+---------------------------+-------------------------------+
| > Start Training          | Unlocked keys: fjdksla;       |
|   Typing Test             | Best WPM: 42.7                |
|   Lessons (Key Practice)  | Current streak: 3 day(s)      |
|   Progress & Stats        | Recent sessions: ...          |
|   Settings                |                               |
|   Exit                    |                               |
+---------------------------+-------------------------------+
```
