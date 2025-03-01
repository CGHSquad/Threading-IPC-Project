# Rust Threading and IPC Project

## Overview
This project demonstrates essential operating system concepts such as:

- Multi-threading using Rust's `std::thread` and `tokio`
- Synchronization using `Mutex` and `Arc`
- Deadlock handling and resolution
- Inter-Process Communication (IPC) using `nix` and pipes

## Prerequisites
To build and run this project, you need the following dependencies:

- **Rust** (Install via [Rust official site](https://www.rust-lang.org/tools/install))
- **Cargo** (Rust's package manager, included with Rust installation)
- **Linux** (Required for `nix` IPC support; tested on Ubuntu 24.04.2 LTS)
- **Git** (For version control)
- **VirtualBox** (For running a Linux VM if using Windows)
- **Ubuntu 24.04.2 LTS** (Recommended OS for development)

## Installation
### 1. Set Up the Virtual Machine (For Windows Users)
If you are on Windows, you can set up a virtual machine using VirtualBox:

1. Download and install [VirtualBox](https://www.virtualbox.org/).
2. Download the Ubuntu 24.04.2 LTS ISO from the [official site](https://ubuntu.com/download/desktop).
3. Create a new VM in VirtualBox and install Ubuntu using the ISO.
4. Once Ubuntu is set up, update and install dependencies:

   ```sh
   sudo apt update && sudo apt upgrade -y
   sudo apt install build-essential curl git -y
   ```

5. Install Rust:
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

### 2. Clone the Repository
```sh
git clone https://github.com/your-username/rust-threading-ipc-project.git
cd rust-threading-ipc-project
```

### 3. Build the Project
```sh
cargo build --release
```

### 4. Run the Program
```sh
cargo run
```

## Dependencies
This project uses the following Rust crates:

- `tokio` - Asynchronous runtime for Rust
- `nix` - Unix-like APIs for IPC
- `parking_lot` - Advanced synchronization primitives
- `clap` - Command-line argument parser

Install dependencies using Cargo:
```sh
cargo install --path .
```
