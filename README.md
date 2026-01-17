# ⚡ qtex — Cloud LaTeX Compiler

<p align="center">
  <img src="https://raw.githubusercontent.com/srsergiolazaro/qtex/main/docs/assets/banner.jpeg" alt="qtex - Cloud LaTeX Compiler" width="500">
</p>

<p align="center">
  <a href="https://www.npmjs.com/package/qtex"><img src="https://img.shields.io/npm/v/qtex?style=flat-square&logo=npm&color=cb3837" alt="npm version"></a>
  <a href="https://www.npmjs.com/package/qtex"><img src="https://img.shields.io/npm/dm/qtex?style=flat-square&color=blue" alt="npm downloads"></a>
  <a href="./LICENSE"><img src="https://img.shields.io/badge/license-Fair_Source-blue?style=flat-square" alt="License"></a>
  <a href="https://latex.taptapp.xyz"><img src="https://img.shields.io/badge/engine-Tectonic-8b5cf6?style=flat-square&logo=rust" alt="Tectonic Engine"></a>
</p>

<p align="center">
  <strong>Compile LaTeX documents in the cloud. Sub-second builds. Zero configuration.</strong><br>
  No TeXLive. No MikTeX. No 5GB downloads. Just fast PDF generation.
</p>

<p align="center">
  <a href="#install">Install</a> •
  <a href="#usage">Usage</a> •
  <a href="#features">Features</a> •
  <a href="https://srsergiolazaro.github.io/qtex/">Website</a>
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

**Or use directly with npx (zero install):**
```bash
npx qtex ./my-project
```

**Or install globally via npm:**
```bash
npm install -g qtex
```

---

## Usage

```bash
# Compile a LaTeX project
qtex ./my-thesis

# Live watch mode — auto-recompile on save
qtex ./my-thesis --watch

# Custom output filename
qtex ./my-thesis --output final.pdf

# Show all options
qtex --help
```

---

## Features

| | Feature | Description |
|-|---------|-------------|
| ⚡ | **Blazing Fast** | Rust-based Tectonic engine compiles in milliseconds |
| 📦 | **Zero Config** | No TeXLive, no MikTeX, no local dependencies |
| 👀 | **Watch Mode** | Auto-recompile on `.tex`, `.bib`, `.sty`, and image changes |
| 🔍 | **Validation** | Pre-flight syntax checks before compilation |
| 📂 | **Multi-file** | Recursive asset discovery with nested folders |
| 🔒 | **Private** | Stateless & ephemeral — your files are never stored |

---

## How it Works

<p align="center">
  <img src="https://raw.githubusercontent.com/srsergiolazaro/qtex/main/docs/assets/flow.png" alt="qtex workflow diagram" width="600">
</p>

1. **Scan** — Discover all `.tex` files and dependencies
2. **Validate** — Pre-flight syntax check via API
3. **Compile** — Cloud processing with Tectonic engine
4. **Download** — Get your PDF instantly

---

## Why qtex?

| Traditional LaTeX | qtex |
|-------------------|------|
| 5GB+ TeXLive install | **Zero install** |
| Minutes to compile | **Milliseconds** |
| Complex setup | **One command** |
| Local resources | **Cloud-powered** |

---

## API & Infrastructure

- **Endpoint**: `https://latex.taptapp.xyz`
- **Engine**: [Tectonic](https://tectonic-typesetting.github.io/) (Rust/XeTeX)
- **Privacy**: Stateless processing — data is never stored

---

## License

**Fair Source License**

✅ Free for individuals and teams ≤3 users  
📧 Contact for enterprise licensing

See [LICENSE](./LICENSE) for details.

---

<p align="center">
  Built with ❤️ by <strong>Tachyon-Tex</strong>
</p>
