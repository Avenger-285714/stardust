# Stardust

An experimental project to refactor [spark-store](https://github.com/spark-store-project/spark-store) using a more modern design.

## About

Stardust is a modern application store interface built with:
- **Language**: Rust
- **GUI Framework**: [Iced](https://github.com/iced-rs/iced) - A cross-platform GUI library for Rust

This project aims to provide a clean, performant, and modern alternative to the traditional spark-store application.

## Features

- Modern, responsive UI built with Iced
- Category-based application browsing
- Search functionality
- Cross-platform support (Linux, macOS, Windows)

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

- **State**: The application state (search query, selected category, etc.)
- **Message**: User actions and events
- **Update**: State update logic based on messages
- **View**: UI rendering based on current state

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [spark-store](https://github.com/spark-store-project/spark-store) - The original application store project
- [Iced](https://github.com/iced-rs/iced) - The GUI framework used in this project
