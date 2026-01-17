# ⚡ qtex

<p align="center">
  <img src="https://raw.githubusercontent.com/srsergiolazaro/qtex/main/docs/assets/banner.jpeg" alt="qtex Banner" width="500">
</p>

<p align="center">
  <a href="https://www.npmjs.com/package/qtex"><img src="https://img.shields.io/npm/v/qtex?style=flat-square&logo=npm&color=cb3837" alt="NPM Version"></a>
  <a href="./LICENSE"><img src="https://img.shields.io/badge/License-Fair_Source-blue.svg?style=flat-square" alt="License"></a>
  <a href="https://latex.taptapp.xyz"><img src="https://img.shields.io/badge/Engine-Tectonic-8b5cf6?style=flat-square&logo=rust" alt="Engine"></a>
  <a href="#"><img src="https://img.shields.io/badge/Latency-<1s-22c55e?style=flat-square" alt="Latency"></a>
</p>

<p align="center">
  <strong>Cloud LaTeX compiler. Sub-second builds. Zero setup.</strong>
</p>

---

## Install

**macOS / Linux**
```bash
curl -fsSL https://raw.githubusercontent.com/srsergiolazaro/qtex/main/install.sh | bash
```

**Windows (PowerShell)**
```powershell
irm https://raw.githubusercontent.com/srsergiolazaro/qtex/main/install.ps1 | iex
```

**Or use directly without installing:**
```bash
npx qtex ./my-project
```

---

## Features

| Feature | Description |
|---------|-------------|
| ⚡ **Fast** | Rust-based Tectonic engine. Compile in milliseconds. |
| 📦 **Zero Config** | No TeXLive, no MikTeX. No 5GB downloads. |
| 👀 **Watch Mode** | Auto-recompile on save. Supports `.tex`, `.bib`, `.sty`, images. |
| 🔍 **Validation** | Pre-flight syntax checks before compilation. |
| 📂 **Multi-file** | Recursive asset discovery with nested folders. |
| 🔒 **Private** | Stateless & ephemeral. Files never stored. |

---

## Usage

```bash
# Compile a project
qtex ./my-project

# Watch mode (live recompilation)
qtex ./my-project --watch

# Custom output filename
qtex ./my-project --output thesis.pdf

# Help
qtex --help
```

---

## How it Works

<p align="center">
  <img src="https://raw.githubusercontent.com/srsergiolazaro/qtex/main/docs/assets/flow.png" alt="qtex Workflow" width="600">
</p>

1. **Scan** — Discover all TeX assets and dependencies
2. **Validate** — Pre-flight syntax check
3. **Compile** — Cloud processing via Tectonic
4. **Sync** — Download PDF locally

---

## Infrastructure

- **Endpoint**: `https://latex.taptapp.xyz`
- **Engine**: Tectonic (Rust / XeTeX)
- **Privacy**: Stateless processing. Data never stored.

---

## License

**Fair Source License**

- ✅ Free for individuals and teams up to 3 users
- 📧 Contact for enterprise/commercial use

See [LICENSE](./LICENSE) for details.

---

<p align="center">
  Built with ❤️ by <strong>Tachyon-Tex</strong>
</p>
