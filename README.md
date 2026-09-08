# Rust-Powered WebAssembly Portfolio

[![Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![WebAssembly](https://img.shields.io/badge/Target-WASM-blueviolet.svg)](https://webassembly.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A lightning-fast, highly modern developer portfolio built completely in **Rust** and compiled to **WebAssembly (WASM)**, featuring an interactive HTML5 Canvas particle animation and dynamic view rendering.

---

## 🚀 Overview & Key Features

- **100% Rust Core**: Navigation logic, event binding, and DOM manipulation powered by `wasm-bindgen` and `web-sys`.
- **Interactive Canvas Engine**: Real-time particle simulation running natively inside the browser via WebAssembly memory.
- **Zero JavaScript Bloat**: Minimal boilerplate; the entire UI state and application flow are driven by compiled systems code.
- **Responsive & Modern Design**: Dark-mode aesthetic tailored for systems engineers, featuring sleek typography and card layouts.

---

## 📂 Project Structure

```text
rust-wasm-portfolio/
├── Cargo.toml          # Rust package configuration & WASM dependencies
├── index.html          # Main HTML entrypoint
├── style.css           # Custom CSS styling and dark theme
├── src/
│   └── lib.rs          # Core Rust logic, DOM handlers & WASM bindings
└── README.md           # Project documentation
```

---

## 🛠️ Prerequisites & Installation Steps

Make sure you have Rust and `wasm-pack` installed on your system.

1. **Install Rust**: [rustup.rs](https://rustup.rs/)
2. **Install `wasm-pack`**:
   ```bash
   curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
   ```

3. **Clone the repository**:
   ```bash
   git clone https://github.com/alex-rivers-rust/rust-wasm-portfolio.git
   cd rust-wasm-portfolio
   ```

---

## 🚀 Quickstart & Usage Examples

### 1. Build the WASM package
Compile the Rust codebase into WebAssembly bindings:
```bash
wasm-pack build --target web
```

### 2. Serve Locally
Run a local static server to view your portfolio in action:
```bash
# Python 3
python3 -m http.server 8080

# Or using Node.js npx
npx serve .
```

Open your browser at `http://localhost:8080` to interact with the portfolio.

---

## 📄 License

Distributed under the MIT License. See `LICENSE` for more information.
