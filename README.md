# ADS-B Service (Rust)

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/rust-1.85.0+-brightgreen.svg)](https://www.rust-lang.org)

A high-performance ADS-B (Automatic Dependent Surveillance-Broadcast) data processing service written in Rust. This service receives aircraft data via the SBS1 protocol, processes it, and uploads to remote services while providing a real-time web dashboard for monitoring.

## Table of Contents

- [Features](#features)
- [Quick Start](#quick-start)
- [Installation](#installation)
- [Configuration](#configuration)
- [Web Dashboard](#web-dashboard)
- [API Reference](#api-reference)
- [Building from Source](#building-from-source)
- [Cross-platform Builds](#cross-platform-builds)
- [Troubleshooting](#troubleshooting)
- [Contributing](#contributing)
- [License](#license)

## Features

- **ADS-B Data Processing**: Receive and process ADS-B data from SBS1 protocol sources (like dump1090)
- **Real-time Web Dashboard**: Beautiful web interface with live statistics and monitoring
- **Flexible Configuration**: Support both TOML configuration files and command-line arguments
- **Cross-platform**: Runs on Linux, macOS, Windows, and various ARM architectures
- **High Performance**: Built with Rust for optimal speed and memory efficiency
- **Statistics Tracking**: Message counts, rates, uptime, and memory usage monitoring
- **Easy Deployment**: Single binary with minimal dependencies

## Quick Start

1. **Install a data source** (if you don't have one):

   ```bash
   # Example: Install dump1090 on Ubuntu/Debian
   sudo apt install dump1090-mutability
   sudo systemctl start dump1090-mutability
   ```

2. **Download the latest release** from [Releases](../../releases) or build from source.

3. **Create configuration file** (`conf.toml`):

   ```toml
   [receiver]
   ip = "127.0.0.1"        # dump1090 server IP
   port = 30003            # SBS1 output port

   [service]
   url = "YOUR-SERVICE-URL"    # Your data upload endpoint
   uuid = "YOUR-16CHAR-UUID"   # Your service identifier

   dashboard_port = 8080   # Web dashboard port
   ```

4. **Run the service**:

   ```bash
   ./adsb --config=./conf.toml
   ```

5. **Access the dashboard**: Open http://localhost:8080/dashboard

## Installation

### Option 1: Download Binary (Recommended)

Download the appropriate compressed archive for your platform from the [releases page](../../releases):

**Tier 1 Platforms (Primary Support)**

- **Linux (x64)**: `adsb-linux-amd64.tar.gz`
- **Linux (ARM64)**: `adsb-linux-arm64.tar.gz`
- **Windows (x64)**: `adsb-windows-amd64.zip`
- **macOS (Universal)**: `adsb-darwin-universal2.tar.gz`

**Tier 2 Platforms (Tested)**

- **Linux (x86 32-bit)**: `adsb-linux-386.tar.gz`

**Raspberry Pi Support**

- **Raspberry Pi Zero/1**: `adsb-linux-armv6.tar.gz` (ARMv6)
- **Raspberry Pi 2/3**: `adsb-linux-armv7.tar.gz` (ARMv7)
- **Raspberry Pi 4/5**: `adsb-linux-arm64.tar.gz` (ARM64)

**Rockchip CPU Support**

- **RK3588/RK3588S**: `adsb-linux-arm64.tar.gz` (ARM64 - newer 8-core chips)
- **RK3566/RK3568**: `adsb-linux-arm64.tar.gz` (ARM64 - quad A55)
- **RK3399**: `adsb-linux-arm64.tar.gz` (ARM64 - dual A72 + quad A53)

**Experimental Platforms**

- **Linux (RISC-V 64-bit)**: `adsb-linux-riscv64.tar.gz`
- **Linux (LoongArch 64-bit)**: `adsb-linux-loongarch.tar.gz`

### Option 2: Install with Cargo

```bash
cargo install --git https://github.com/ywjno/service-adsb-rs
```

### Option 3: Build from Source

See [Building from Source](#building-from-source) section below.

## Configuration

### Configuration File (Recommended)

Create a `conf.toml` file with your settings:

```toml
[receiver]
ip = "127.0.0.1"        # IP address of your ADS-B receiver (or hostname like "dump1090")
port = 30003            # Port number (30003 is standard for SBS1)

[service]
url = "https://your-service.com/api/upload"  # Your data upload endpoint
uuid = "YOUR16CHARUUID1"                     # Your unique identifier (exactly 16 characters)

# Optional: Dashboard configuration
dashboard_port = 8080   # Web dashboard port (default: 8080)
```

**Run with config file:**

```bash
./adsb --config=./conf.toml
```

### Command Line Arguments

For advanced users, you can configure everything via command-line:

```bash
./adsb --receiver-ip 127.0.0.1 \
       --receiver-port 30003 \
       --service-url "https://your-service.com/api/upload" \
       --service-uuid "YOUR16CHARUUID1" \
       --dashboard-port 8080
```

**Available options:**

```
Usage: adsb [OPTIONS]

Options:
      --receiver-ip <RECEIVER_IP>       Receiver IP address or hostname [default: 127.0.0.1]
      --receiver-port <RECEIVER_PORT>   Receiver port [default: 30003]
      --service-url <SERVICE_URL>       Service upload URL (required)
      --service-uuid <SERVICE_UUID>     Service UUID - exactly 16 characters (required)
      --dashboard-port <DASHBOARD_PORT> Dashboard web port [default: 8080]
      --config <TOML_FILE>              TOML config file path
  -h, --help                            Print help information
  -V, --version                         Print version information
```

### Environment Variables

Control logging level:

```bash
# Show only errors
RUST_LOG=error ./adsb --config=./conf.toml

# Show all debug info
RUST_LOG=debug ./adsb --config=./conf.toml

# Default: info level
./adsb --config=./conf.toml
```

## Web Dashboard

The service includes a beautiful real-time web dashboard accessible at:

```
http://localhost:8080/dashboard
```

### Dashboard Features

- **Service Status**: Live connection status indicator
- **Message Statistics**:
  - Total messages received
  - Messages per minute rate
  - Last message timestamp
- **Uptime Tracking**: Service running time and start time
- **Memory Monitoring**:
  - Current memory usage (MB)
  - Peak memory usage (MB)
- **Auto-refresh**: Updates every 5 seconds

### Dashboard Screenshots

The dashboard features a modern, responsive design with:

- Real-time statistics cards
- Color-coded status indicators
- Gradient backgrounds and smooth animations
- Mobile-friendly responsive layout

## API Reference

### GET `/api/stats`

Returns current service statistics in JSON format.

**Response Example:**

```json
{
  "success": true,
  "data": {
    "total_messages": 15420,
    "messages_per_minute": 45,
    "last_message_time": "2024-01-15T10:30:45Z",
    "uptime_seconds": 3600,
    "start_time": "2024-01-15T09:30:45Z",
    "memory_usage_mb": 12.5,
    "memory_peak_mb": 18.2
  },
  "message": "Stats retrieved successfully"
}
```

### GET `/dashboard`

Serves the web dashboard HTML interface.

## Building from Source

### Prerequisites

- **Rust**: Minimum version 1.85.0
- **Git**: For cloning the repository

### Build Steps

1. **Clone the repository:**

   ```bash
   git clone https://github.com/ywjno/service-adsb-rs.git
   cd service-adsb-rs
   ```

2. **Build for your platform:**

   ```bash
   cargo build --release
   ```

3. **Run the binary:**
   ```bash
   ./target/release/adsb --help
   ```

## Cross-platform Builds

This project supports building for multiple architectures using the included `justfile` build system.

### Setup Cross-compilation Environment

1. **Install required tools:**

   ```bash
   # Install cargo-zigbuild for cross-compilation
   cargo install cargo-zigbuild

   # Install just for build automation
   cargo install just

   # Install zig (required by cargo-zigbuild)
   pip3 install ziglang
   ```

2. **Verify installation:**
   ```bash
   just --version
   cargo zigbuild --version
   ```

### Build Commands

```bash
# Build most common platforms (Linux, macOS, Windows x64)
just

# Build all supported platforms
just all-arch

# Build specific platform categories
just build-all-linux     # All Linux architectures
just build-all-darwin    # All macOS architectures
just build-all-windows   # All Windows architectures
just build-all-arm # All ARM architectures

# Build individual platforms
just build-linux-amd64              # Linux x64
just build-windows-amd64            # Windows x64
just build-universal2-apple-darwin  # macOS Universal (Intel + Apple Silicon)
just build-linux-armv6              # Raspberry Pi Zero
just build-linux-armv7              # Raspberry Pi 2/3
just build-linux-arm64              # Raspberry Pi 4+, RK3588/RK3588S, RK3566/RK3568, RK3399

# Create release packages with checksums
just release

# Clean build artifacts
just clean

# Show all available commands
just help
```

### Supported Platforms

**Tier 1** (Primary support):

- `linux-amd64` - Linux x86_64
- `linux-arm64` - Linux ARM64
- `universal2-apple-darwin` - macOS Universal (Intel + Apple Silicon)
- `windows-amd64` - Windows x86_64

**Tier 2** (Secondary support):

- `linux-386` - Linux i686 (32-bit)
- `linux-armv6` - Raspberry Pi Zero/1 (ARMv6)
- `linux-armv7` - Raspberry Pi 2/3 (ARMv7)
- `linux-riscv64` - RISC-V 64-bit
- `linux-loongarch` - LoongArch 64-bit

All release binaries are compressed (tar.gz for Unix-like systems, zip for Windows) and include SHA256 checksums for integrity verification.

## Contributing

We welcome contributions! Please feel free to submit issues, feature requests, or pull requests.

### Development Setup

1. Fork and clone the repository
2. Install development dependencies: `cargo install cargo-watch`
3. Run tests: `cargo test`
4. Format code: `cargo fmt`
5. Run linter: `cargo clippy`

### Running in Development

```bash
# Watch for changes and rebuild
cargo watch -x "run -- --config=./conf.toml"

# Run tests continuously
cargo watch -x test
```

## License

This project is dual-licensed under either:

- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE))
- **MIT License** ([LICENSE-MIT](LICENSE-MIT))

You may choose either license for your use.

## Legal Notice

**Important**: Please be aware that you are required to comply with local laws and policies regarding ADS-B data collection, processing, and transmission. This software is provided for educational and legitimate use only. Users are responsible for ensuring compliance with:

- Local aviation regulations
- Data privacy laws (GDPR, CCPA, etc.)
- Radio frequency monitoring regulations
- Any applicable terms of service for data upload destinations

The authors and contributors of this software are not responsible for any misuse or legal violations by users.

---

If this project is helpful to you, please give it a star!

For questions or suggestions, feel free to [open an issue](../../issues).
