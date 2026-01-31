# System Monitor CLI

A high-performance, terminal-based system monitoring tool written in Rust. It provides real-time visualization of system resources, process management, and network traffic analysis using a responsive TUI.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)

## Features

-   **Real-time Overview**: Live metrics for Global CPU usage, Memory consumption, and Disk space availability.
-   **Process Management**: detailed list of active processes sorted by CPU usage, including PID, Name, CPU%, and Memory usage.
-   **Network Traffic Analysis**: Interactive charts showing real-time Receiver (RX) and Transmitter (TX) data.
-   **Responsive TUI**: Built with `ratatui` for a smooth, flicker-free terminal experience.
-   **Cross-Platform**: Runs on Windows, Linux, and macOS.

## Prerequisites

-   **Rust & Cargo**: You need to have the Rust toolchain installed.
    -   To install Rust, run: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
    -   Or visit [rustup.rs](https://rustup.rs/) for more details.

## Installation

1.  **Clone the repository**:
    ```bash
    git clone https://github.com/Harshit250602/System-Monitor-CLI.git
    cd System-Monitor-CLI
    ```

2.  **Build the project**:
    ```bash
    cargo build --release
    ```

## Usage

To run the application directly:

```bash
cargo run --release
```

Or run the compiled binary from `target/release/system_monitor`.

### Key Bindings

| Key | Action |
| :--- | :--- |
| **Tab** | Switch to the next tab (Overview -> Processes -> Network) |
| **Shift + Tab** | Switch to the previous tab |
| **q** | Quit the application |

## Tech Stack

-   **Language**: [Rust](https://www.rust-lang.org/)
-   **TUI Library**: [Ratatui](https://github.com/ratatui-org/ratatui)
-   **System Info**: [sysinfo](https://github.com/GuillaumeGomez/sysinfo)
-   **Terminal Backend**: [Crossterm](https://github.com/crossterm-rs/crossterm)

## License

This project is licensed under the MIT License - see the LICENSE file for details.
