# Finance Manager

A desktop application for managing personal finances, built with Rust and GTK.

![alt text](https://github.com/timarques/finance-manager/blob/main/screenshot.png?raw=true)

## Installation

### Installation

#### Windows

Download the latest release from:
https://github.com/timarques/finance-manager/releases/latest

#### Linux

Download the flatpak manifest from:
https://github.com/timarques/finance-manager/releases/latest

```bash
flatpak remote-add --user flathub https://flathub.org/repo/flathub.flatpakrepo
flatpak install --user org.gnome.Sdk//47 org.gnome.Platform//47  org.freedesktop.Sdk.Extension.rust-stable//24.08 -y
flatpak-builder --user --install flatpak build/com.github.timarques.FinanceManager.yml
```

### Building from Source

#### Prerequisites

- Rust
- Cargo
- pkg-config
- cmake
- GTK4 development package
- libadwaita development package

1. Clone the repository:
```bash
git clone https://github.com/timarques/finance-manager.git
cd finance-manager
```

2. Build and install:

```bash
cmake -B build
cmake --build build
cmake --install build
```

2. Build flatpak and install:

```bash
cmake -B build -DBUILD_FLATPAK=ON
cmake --build build
```