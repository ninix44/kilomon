# 🦀 KiloMon (Killer Monitor)

**KiloMon** is a lightweight, high-performance terminal-based process monitor written in **Rust**.
It allows users to view system resources in real-time, sort processes, and terminate them directly from the TUI (Text User Interface).

> 🚀 **Performance:** Uses `crossterm` and `ratatui` for smooth rendering with minimal resource footprint.

## ⚡ Features
- [x] **Real-time Monitoring:** CPU & Memory usage updates every second.
- [x] **Smart Sorting:** Sort processes by CPU, Memory, PID, or Name (Press `S`).
- [x] **Killer Mode:** Terminate any process directly from the list (Press `K`).
- [x] **Interactive UI:** Navigate using Keyboard (Arrow Keys) or **Mouse Scroll**.
- [x] **Visual Alerts:** High CPU usage (>50%) is highlighted in **Red**.

## 🎮 Controls
| Key | Action |
| :--- | :--- |
| `↑` / `↓` | Navigate through the process list |
| `Mouse Scroll` | Navigate up/down |
| `K` | **Kill** the selected process |
| `S` | Change **Sort** order (CPU / Mem / PID) |
| `Q` | **Quit** the application |

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