# EnderCliTools

A collection of CLI utilities that enhance and streamline everyday terminal workflows.

## 🚀 About EnderCliTools

EnderCliTools is a collection of command-line tools that optimize and simplify everyday terminal workflows. The project provides various utilities specifically designed to make recurring tasks more efficient.

## 📦 Installation

### Using pre-built installers

The easiest way to install is using the pre-built installers:

- **Windows (MSI)**: [Download from GitHub Releases](https://github.com/Endkind/EnderCliTools/releases/latest)
- **Debian/Ubuntu (.deb)**: [Download from GitHub Releases](https://github.com/Endkind/EnderCliTools/releases/latest)

### Installation via GitHub Releases

1. Go to the [latest releases](https://github.com/Endkind/EnderCliTools/releases/latest)
2. Download the appropriate file for your system:
   - **Windows**: `EnderCliTools-x.x.x-x86_64.msi`
   - **Debian/Ubuntu**: `enderclitools_x.x.x-x_amd64.deb`
3. Run the installation

### Building from source

If you want to compile the project from source:

```bash
# Clone repository
git clone https://github.com/Endkind/EnderCliTools.git
cd EnderCliTools

# Compile with Cargo
cargo build --release
```

## 🛠️ Available Tools

EnderCliTools currently offers the following utilities:

### DPS (Docker Process Status)

A tool for clear display of Docker container information.

### DCPS (Docker Compose Process Status)

A tool for managing and displaying Docker Compose services.

### Configuration (Config)

Management of EnderCliTools configuration with subcommands for setting, retrieving, and resetting settings.

## 📚 Documentation

Comprehensive documentation and usage examples can be found in the official documentation:

- **Main Documentation**: [docs.endkind.net/enderclitools](https://docs.endkind.net/enderclitools)
- **DPS Documentation**: [docs.endkind.net/enderclitools/dps](https://docs.endkind.net/enderclitools/dps)
- **DCPS Documentation**: [docs.endkind.net/enderclitools/dcps](https://docs.endkind.net/enderclitools/dcps)
- **Config Documentation**: [docs.endkind.net/enderclitools/config](https://docs.endkind.net/enderclitools/config)

## 🎯 Quick Start

After installation, you can use the tools directly:

```bash
# Show help
ect --help

# List Docker containers
dps

# Show Docker Compose services
dcps

# Show configuration
ect config get
```

## 🔧 Configuration

EnderCliTools stores its configuration in the user directory. You can manage the configuration with the following commands:

```bash
# Show current configuration
ect config get [COMMAND] [OPTIONS]

# Set configuration value
ect config set <COMMAND> [OPTIONS]

# Reset configuration
ect config reset [COMMAND] [OPTIONS]
```

## 🤝 Contributing

Contributions are welcome! If you find bugs or want to suggest features:

1. Open an [Issue](https://github.com/Endkind/EnderCliTools/issues) on GitHub
2. Fork the repository and create a Pull Request
3. Make sure your code meets the project standards

## 📝 License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for more details.

---

For more information and detailed guides, visit the [official documentation](https://docs.endkind.net/enderclitools).
