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

Aster provides a simple installation script that compiles the browser natively and installs it directly into your Windows roaming profile (`%APPDATA%\Aster`), alongside your Start Menu and Desktop shortcuts.

Ensure you have [Rust](https://rustup.rs/) installed on a Windows machine.

```powershell
# Clone the repository
git clone https://github.com/ahyanistheEmty/Aster/
cd aster

# Run the installation script
.\install.ps1
```

## 🤝 Contributing

Contributions, issues, and feature requests are welcome! Feel free to check the [issues page](https://github.com/ahyanistheEmty/Aster/issues/).

## 📝 License

This project is [MIT](LICENSE) licensed.
