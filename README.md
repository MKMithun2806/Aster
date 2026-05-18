<div align="center">
  <img src="assets/aster.ico" width="128" height="128" alt="Aster Logo">
  <h1>Aster Browser</h1>
  <p>A fast, Chromium-based desktop browser built in Rust with native vertical tabs, infinite nesting, and gorgeous aesthetics.</p>
</div>

## ✨ Features

- **Blazing Fast**: Built natively in Rust using WebView2. Extremely lightweight with minimal memory overhead.
- **Vertical Tabs**: Side-docked tabs optimized for modern widescreen monitors.
- **Infinite Nested Folders**: Organize your workflow with recursive folder trees.
- **Zen Mode Session Restore**: Tabs wake up gracefully in a sleeping/unloaded state to preserve memory.
- **Stunning Aesthetics**: Premium indigo accent colors, glassmorphism gradients, and beautifully indented typography.
- **Fully Portable**: Runs as a standalone executable. State data integrates cleanly with your standard Windows `%APPDATA%` profile.

## 🚀 Installation

Aster provides a native Windows `.msi` installer generated via the WiX Toolset.

1. Go to the [Releases](https://github.com/yourusername/aster/releases) page.
2. Download the latest `Aster.msi` installer.
3. Run the installer and launch Aster!

## 🛠️ Building from Source

Ensure you have [Rust](https://rustup.rs/) installed on a Windows machine.

```bash
# Clone the repository
git clone https://github.com/yourusername/aster.git
cd aster

# Build the release executable
cargo build --release

# The compiled binary will be located at target/release/Aster.exe
```

### Building the Installer (.msi)

We use `cargo-wix` to build the native Windows installer.

```bash
cargo install cargo-wix
cargo wix
# The installer will be located at target/wix/Aster.msi
```

## 🤝 Contributing

Contributions, issues, and feature requests are welcome! Feel free to check the [issues page](https://github.com/yourusername/aster/issues).

## 📝 License

This project is [MIT](LICENSE) licensed.
