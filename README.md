# 🦀 KiloMon (Killer Monitor)

**KiloMon** is a lightweight, terminal-based process monitor written in **Rust**.
It allows users to view system resources in real-time within a TUI (Text User Interface).

## ⚡ Features
- [x] Real-time CPU & Memory usage monitoring.
- [x] Lightweight TUI using `ratatui` and `crossterm`.
- [ ] **Killer Mode:** Auto-kill processes based on memory/CPU limits (Coming soon).
- [ ] Process sorting and filtering.

## 🚀 How to Run

### Prerequisites
You need to have [Rust](https://www.rust-lang.org/tools/install) installed.

### Installation & Run
```bash
# Clone the repository
git clone https://github.com/ninix44/kilomon.git
cd kilomon

# Run in debug mode
cargo run

# Build release version (optimized)
cargo build --release
./target/release/kilomon.exe