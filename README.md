# GitLab Releases Monitor (Tauri)

A lightweight cross-platform desktop tray application that keeps an eye on the latest releases of your selected GitLab projects.

## ✨ Features

* System-tray first – stays out of the way until you need it.
* Monitors multiple GitLab projects (configured via `.env`).
* Auto-refresh every 5 min + manual refresh (`R` or button).
* Blue-dot tray notification when new releases are detected.
* Global tray menu (**Show GitLab Releases**, **Quit**).
* Keyboard shortcuts:
  * `R` – refresh while window is focused.
  * `Ctrl + Alt + G` – *(to-be-added)* toggle window.

---

## ⚙️ Prerequisites

| Tool | Version | Notes |
|------|---------|-------|
| [Rust](https://rustup.rs/) | stable | `rustup default stable` |
| Tauri CLI | latest | `cargo install tauri-cli` *(or)* `npm i -g @tauri-apps/cli` |
| **Windows only:** Visual Studio Build Tools >= 2019 |  | required C++ build chain |

No Node build pipeline is used – the frontend is plain HTML/CSS/JS served by Tauri.

---

## 🔧 Setup

1. **Clone the repo**
   ```bash
   git clone https://your-repo-url/gitlab-monitor.git
   cd gitlab-monitor
   ```
2. **Configure environment variables**
   ```bash
   cp gitlab-config.example .env
   # then edit .env and fill in:
   #  GITLAB_API_TOKEN=<your-token>
   #  GITLAB_BASE_URL=https://gitlab.example.com       # optional, defaults to gitlab.com
   #  GITLAB_PROJECTS=namespace/project,another/project
   ```
3. **Install dependencies** *(only once)*
   ```bash
   cargo install tauri-cli      # if you haven’t already
   ```

---

## 🚀 Development

```bash
cd src-tauri
cargo tauri dev
```
The app window will appear; the binary reloads on source changes.

---

## 📦 Build a release EXE (Windows)

```powershell
cd src-tauri
cargo tauri build        # generates installer & portable exe in /target/release/bundle/windows
```
After the build finishes you'll find:
* `*.msi` – signed installer
* `*.exe` – portable version (shows correct GitLab icon thanks to `src-tauri/icons/icon.ico`)

---

## 🖥️ Using the App

| Action | How |
|--------|-----|
| Show window | Left-click tray icon<br/>**Show GitLab Releases** in tray menu |
| Hide window | Click window **−** button or right-click tray → Hide |
| Quit        | Tray menu → **Quit** |
| Open release page | Click any row in the list |
| Mark releases as seen | Simply open the window – blue-dot disappears |

---

## 👐 Contributing
Pull requests and issues are welcome! Please follow the existing code style and make sure `cargo fmt && cargo clippy` pass.

---


