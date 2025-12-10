# Stardust

An experimental project to refactor [spark-store](https://github.com/spark-store-project/spark-store) using a more modern design.

## About

Stardust is a modern application store interface built with:
- **Language**: Rust
- **GUI Framework**: [Iced](https://github.com/iced-rs/iced) - A cross-platform GUI library for Rust

This project aims to provide a clean, performant, and modern alternative to the traditional spark-store application.

## Features

- **Modern, responsive UI** built with Iced GUI framework
- **Real-time data fetching** from Spark Store servers (https://cdn-d.spark-app.store/)
- **Category-based browsing** with 8 categories (All, Development, Graphics, Office, Games, Multimedia, Network, Utilities)
- **Search functionality** using Spark Store's search API
- **Async/await architecture** for non-blocking network operations
- **Architecture support** for x86_64, aarch64, and loongarch64
- **Cross-platform** support (Linux, macOS, Windows)

## Building

### Prerequisites

- Rust 1.70 or later
- Cargo

### Build Instructions

```bash
# Clone the repository
git clone https://github.com/Avenger-285714/stardust.git
cd stardust

# Build the project
cargo build --release

# Run the application
cargo run --release
```

## Development

To run in development mode:

```bash
cargo run
```

To check the code:

```bash
cargo check
```

To run tests:

```bash
cargo test
```

## Architecture

Stardust follows the Elm architecture pattern used by Iced:

- **State**: The application state (search query, selected category, loaded apps, etc.)
- **Message**: User actions and events (search, category selection, API responses)
- **Update**: State update logic based on messages with async Task support
- **View**: UI rendering based on current state

### API Integration

The application connects to Spark Store's official infrastructure:

- **App List API**: `https://cdn-d.spark-app.store/{arch}/` - Fetches application catalogs by category
- **Search API**: `https://search.deepinos.org.cn/appinfo/search` - Searches for applications
- **Architecture Detection**: Automatically uses correct endpoints for x86_64, aarch64, or loongarch64

### Key Components

- `src/api.rs` - Spark Store API client with async HTTP requests
- `src/main.rs` - Main application logic and UI
- Uses `reqwest` with rustls-tls for HTTP client, `serde` for JSON parsing
- Iced provides the async runtime (no explicit tokio dependency needed)

## Troubleshooting

If you encounter issues with data loading or runtime errors, see the [Troubleshooting Guide](docs/TROUBLESHOOTING.md) for:
- Common error diagnoses
- Network connectivity tests
- Debug logging instructions
- Performance tips

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [spark-store](https://github.com/spark-store-project/spark-store) - The original application store project
- [Iced](https://github.com/iced-rs/iced) - The GUI framework used in this project
