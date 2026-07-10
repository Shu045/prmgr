````markdown
```text

██████╗ ██████╗ ███╗   ███╗ ██████╗ ██████╗
██╔══██╗██╔══██╗████╗ ████║██╔════╝ ██╔══██╗
██████╔╝██████╔╝██╔████╔██║██║  ███╗██████╔╝
██╔═══╝ ██╔══██╗██║╚██╔╝██║██║   ██║██╔══██╗
██║     ██║  ██║██║ ╚═╝ ██║╚██████╔╝██║  ██║
╚═╝     ╚═╝  ╚═╝╚═╝     ╚═╝ ╚═════╝ ╚═╝  ╚═╝
```

# prmgr

A fast, lightweight, keyboard-first terminal process manager written in Rust with Ratatui.

`prmgr` lets you monitor system processes, visualize resource usage, search running processes, and execute Vim-inspired commands—all without leaving your terminal.

---

## ✨ Features

- 📊 Live CPU usage graph
- 💾 Live memory usage graph
- 📋 Interactive process table
- 🔍 Real-time process search
- ⌨️ Keyboard-first workflow
- ⚡ Vim-inspired command mode
- 🎯 Tab completion for selected process PID
- 🚀 Fast and lightweight
- 🦀 Written entirely in Rust

---

## 📦 Installation

### Linux

Install the latest release:

```bash
curl -fsSL https://raw.githubusercontent.com/Shu045/prmgr/main/install.sh | bash
```

Run:

```bash
prmgr
```

---

## 🔨 Building from Source

### Prerequisites

- Rust (latest stable)
- Cargo

Clone the repository:

```bash
git clone https://github.com/Shu045/prmgr.git
cd prmgr
```

Build:

```bash
cargo build --release
```

Run the release binary:

```bash
./target/release/prmgr
```

Or run in development mode:

```bash
cargo run
```

---

## ⌨️ Keyboard Shortcuts

| Key | Action |
|------|--------|
| `↑` / `↓` | Navigate process list |
| `/` | Start process search |
| `:` | Enter command mode |
| `Tab` | Insert selected process PID into the search/command |
| `Esc` | Exit search or command mode |
| `q` | Quit |

---

## 📝 Command Mode

`prmgr` includes a Vim-inspired command mode.

Press `:` to enter command mode.

### Available Commands

| Command | Description |
|----------|-------------|
| `:kill <pid>` | Kill a process by PID |

### Example

```text
:kill 1234
```

### Tip

While typing a command, press **Tab** to automatically insert the PID of the currently selected process.

Example:

```text
:kill<Tab>
```

becomes

```text
:kill 1234
```

---

## 📸 Screenshot

![prmgr Screenshot](docs/screenshot.png)

---

## 🛣️ Roadmap

- [ ] Sort by CPU usage
- [ ] Sort by memory usage
- [ ] Process filtering
- [ ] Network usage graph
- [ ] Disk I/O statistics
- [ ] Multi-platform releases
- [ ] Automatic update checker

---

## 🤝 Contributing

Contributions, bug reports, and feature requests are always welcome.

If you have an idea or find a bug, feel free to open an issue or submit a pull request.

---

## 📄 License

This project is licensed under the MIT License.
````
