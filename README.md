<div align="center">

<img src="https://res.cloudinary.com/ddqedxovk/image/upload/v1777644756/zdmst5ng01o20lam01ou.png" width="120" height="120" style="border-radius:20px" />

# LidBridge

**Clean your projects. Push to GitHub. In seconds.**

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Built with Tauri](https://img.shields.io/badge/Built%20with-Tauri%202-24C8DB?logo=tauri)](https://tauri.app)
[![Next.js](https://img.shields.io/badge/Frontend-Next.js%2014-black?logo=next.js)](https://nextjs.org)
[![Rust](https://img.shields.io/badge/Backend-Rust-orange?logo=rust)](https://www.rust-lang.org)
[![Made by Lidprex](https://img.shields.io/badge/Made%20by-Lidprex%20Labs-8B5CF6)](https://lidprex.onrender.com)

</div>

---

## 📖 What is LidBridge?

LidBridge is an open-source **desktop application** that helps developers clean their local project directories and publish them to GitHub — with one click.

No more manually deleting `node_modules`, `__pycache__`, `.next`, or `target` folders before sharing your code. LidBridge handles all of that automatically, detects secret files before they leak, and pushes your clean project to a new GitHub repository.

---

## ✨ Features

| Feature | Description |
|---|---|
| 🧹 **Smart Cleaning** | Automatically removes 30+ types of junk directories and files |
| 🔍 **Secret Detection** | Scans for exposed API keys, tokens, and passwords before pushing |
| 🚀 **One-Click Push** | Creates a GitHub repository and pushes your code in one step |
| 🏢 **Org Support** | Push to personal accounts or any GitHub Organization you belong to |
| 📊 **Project Scan** | Displays file statistics before cleaning |
| 🌍 **Multilingual UI** | Supports English, Arabic, French, Hindi, and Chinese |
| 📜 **Repo History** | Tracks all repositories created through the app |

---

## 🖥️ Supported Platforms

| Platform | Status |
|---|---|
| Windows 10/11 | ✅ Fully supported |
| macOS | ✅ Supported (requires build on macOS) |
| Linux | ✅ Supported (requires build on Linux) |

---

## 🏗️ Tech Stack

| Layer | Technology |
|---|---|
| Desktop Framework | [Tauri v2](https://tauri.app) |
| Backend | [Rust](https://www.rust-lang.org) |
| Frontend | [Next.js 14](https://nextjs.org) + TypeScript |
| Styling | Tailwind CSS |
| Local Database | SQLite (via `rusqlite`) |
| Git Operations | `git2` (libgit2 — no system Git required) |
| Analytics DB | PostgreSQL (optional, via `sqlx`) |

---

## 🚀 Getting Started

### Prerequisites

- [Node.js](https://nodejs.org) ≥ 18
- [Rust](https://rustup.rs) (stable toolchain)
- [Tauri CLI prerequisites](https://tauri.app/start/prerequisites/) for your platform

### 1. Clone the Repository

```bash
git clone https://github.com/lidprex/lidbridge.git
cd lidbridge
```

### 2. Configure Environment Variables

```bash
cp .env.example .env
```

Open `.env` and fill in your values:

```env
GITHUB_CLIENT_ID=your_github_client_id
GITHUB_CLIENT_SECRET=your_github_client_secret
DATABASE_URL=postgresql://...   # Optional — leave empty to disable analytics
```

### 3. Set Up GitHub OAuth App

1. Go to [github.com/settings/developers](https://github.com/settings/developers)
2. Click **New OAuth App**
3. Fill in:
   - **Application name**: `LidBridge`
   - **Homepage URL**: `http://localhost:3000`
   - **Authorization callback URL**: `http://localhost:2026/callback`
4. Copy the **Client ID** and **Client Secret** into your `.env`

### 4. Set Up GitHub App Private Key (Optional — for Org push)

If you want to push to GitHub Organizations via a GitHub App:

1. Go to [github.com/settings/apps](https://github.com/settings/apps)
2. Create or select your GitHub App
3. Generate a private key
4. Place the file at: `src-tauri/src/keys/private-key.pem`

> ⚠️ The private key is listed in `.gitignore` and will never be committed.

### 5. Install Dependencies

```bash
npm install
```

### 6. Run in Development Mode

```bash
npm run tauri dev
```

---

## 🔨 Building for Production

```bash
npm run tauri build
```

The installer will be generated in `src-tauri/target/release/bundle/`.

---

## 📁 Project Structure

```
lidbridge/
├── src/                        # Next.js frontend
│   └── app/
│       ├── page.tsx            # Main application UI
│       ├── layout.tsx          # Root layout
│       └── globals.css         # Global styles & design tokens
│
├── src-tauri/                  # Rust/Tauri backend
│   ├── src/
│   │   ├── lib.rs              # Application entry point & Tauri commands
│   │   ├── main.rs             # Binary entry point
│   │   ├── auth/               # GitHub OAuth authentication
│   │   ├── cleaner/            # Project scanning & cleaning engine
│   │   ├── db/                 # Local SQLite + optional PostgreSQL
│   │   ├── git/                # Git operations via libgit2
│   │   ├── github_app.rs       # GitHub App JWT token generation
│   │   └── keys/               # Private key storage (gitignored)
│   ├── build.rs                # Build script — bakes env vars at compile time
│   ├── Cargo.toml              # Rust dependencies
│   └── tauri.conf.json         # Tauri window & bundle configuration
│
├── .env.example                # Environment variable template
├── .gitignore                  # Protects secrets from being committed
├── package.json                # Node.js scripts & dependencies
└── README.md                   # This file
```

---

## 🧹 What Gets Cleaned?

LidBridge removes the following categories of files and directories:

| Category | Examples |
|---|---|
| Version Control | `.git/`, `.svn/`, `.hg/` |
| Package Managers | `node_modules/`, `.npm/`, `bower_components/` |
| Build Outputs | `dist/`, `build/`, `target/`, `out/`, `.next/` |
| Python | `__pycache__/`, `.venv/`, `.tox/`, `*.pyc` |
| IDE Files | `.idea/`, `.vscode/`, `.vs/` |
| Lock Files | `package-lock.json`, `yarn.lock`, `Cargo.lock` |
| OS Artifacts | `.DS_Store`, `Thumbs.db`, `desktop.ini` |
| Archives | `.zip`, `.tar.gz`, `.rar`, `.7z` |
| Logs | `*.log`, `logs/` |
| Environment | `.env`, `.env.local`, `.env.production` |

---

## 🔒 Security

LidBridge scans your project for common secret patterns before pushing:

- GitHub tokens (`ghp_`, `gho_`, `ghu_`, `ghs_`)
- OpenAI API keys (`sk-...`)
- AWS Access Keys (`AKIA...`)
- Google API keys (`AIza...`)
- Stripe keys (`sk_live_...`)
- Discord bot tokens
- Generic `api_key=`, `password=`, `token=` patterns

---

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Commit your changes: `git commit -m 'feat: add my feature'`
4. Push to the branch: `git push origin feature/my-feature`
5. Open a Pull Request

Please make sure you do **not** commit `.env` or `private-key.pem`.

---

## 📄 License

This project is licensed under the **GNU General Public License v3.0**.
See [LICENSE.txt](src-tauri/LICENSE.txt) for the full license text.

```
LidBridge — Copyright (C) 2025 Lidprex Labs
This program comes with ABSOLUTELY NO WARRANTY.
This is free software, and you are welcome to redistribute it
under the conditions of the GNU GPL v3.
```

---

## 🔗 Links

| | |
|---|---|
| 🌐 Website | [lidprex.onrender.com](https://lidprex.onrender.com) |
| 🧪 Labs | [lidprex-labs.onrender.com](https://lidprex-labs.onrender.com) |
| 👤 Lead Developer | [github.com/bxat01](https://github.com/bxat01) |
| 🏢 Organization | [github.com/lidprex](https://github.com/lidprex) |

---

<div align="center">
  <sub>Built with ❤️ by <strong>Lidprex Labs</strong></sub>
</div>
