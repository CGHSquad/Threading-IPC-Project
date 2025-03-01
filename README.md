# Rust Threading & IPC Project

## Overview
This project demonstrates essential operating system concepts such as:

- **Multi-threading** using Rust's `std::thread` and `tokio`
- **Synchronization** using `Mutex` and `Arc`
- **Deadlock handling** and resolution
- **Inter-Process Communication (IPC)** using `nix` and pipes

## Prerequisites
- **Rust** (Install via [Rust official website](https://www.rust-lang.org/tools/install))
- **Cargo** (Rust's package manager, included with Rust)
- **Git** (for version control)
- **Linux/macOS** (for `nix` IPC support)
- **Virtualization Software** (if using a virtual machine)

## Virtualization Options
If your hardware supports virtualization, you can choose one of the following methods to run a Linux environment:

### 1. Windows Users
#### Option 1: Hyper-V (Windows Pro/Education)
A built-in virtualization tool available on Windows Pro or Education editions. Enable it through **Turn Windows features on or off**.

#### Option 2: VirtualBox (Cross-platform)
A free and open-source virtualization tool that works on Windows, macOS, and Linux. Download it from [VirtualBox](https://www.virtualbox.org/).

#### Option 3: VMware Workstation Player (Cross-platform)
A robust option for virtualization, free for non-commercial use. Download it from [VMware](https://www.vmware.com/products/workstation-player.html).

### 2. Windows Subsystem for Linux (WSL)
If you are using Windows 10 or later, you can use WSL to run a Linux environment without a virtual machine. Follow these steps:

```sh
wsl --install
```

Then install a Linux distribution, such as Ubuntu, from the Microsoft Store.

### 3. macOS Users
For those using **ARM-based Macs** (e.g., Apple Silicon), consider:

- **UTM**: A virtualization tool optimized for ARM-based Macs. Download from [UTM](https://mac.getutm.app/).
- **Parallels Desktop**: A commercial virtualization tool that supports Linux on macOS. Available at [Parallels](https://www.parallels.com/products/desktop/).

## Choosing a Linux Distribution
If this is your first time working with Linux, I recommend using **Ubuntu**, as it is beginner-friendly and well-documented. However, if you are already familiar with Linux, you may explore other distributions:

- Fedora: Known for its cutting-edge features and strong community support. Download it from [Fedora](https://getfedora.org/).
- Debian: A stable and reliable distribution popular for development. Download it from [Debian](https://www.debian.org/).
- Arch Linux: For advanced users looking for complete customization and control. Download it from [Arch Linux](https://archlinux.org/).

## Installation
### Clone the Repository
```sh
git clone https://github.com/your-username/rust-threading-ipc-project.git
cd rust-threading-ipc-project
```

### Build the Project
```sh
cargo build --release
```

### Run the Project
```sh
cargo run
```

## Dependencies
- `tokio`
- `nix`
- `std::thread`
