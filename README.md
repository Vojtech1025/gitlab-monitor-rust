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

### Development Setup

1. **Clone the repo**
   ```bash
   git clone https://github.com/Vojtech1025/gitlab-monitor-rust.git
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
   cargo install tauri-cli      # if you haven't already
   ```

### Post-Installation Setup (for built/installed versions)

When you run the installed application for the first time:

1. **The app will include configuration files** in the installation directory:
   - `gitlab-config.example` (configuration template)
   - `INSTALLATION-README.txt` (detailed setup instructions)
   - `gitlab-monitor.exe` (the application)

2. **Copy and rename the configuration file**:
   - Copy `gitlab-config.example` 
   - Rename the copy to `.env` (including the dot at the beginning)

3. **Edit the `.env` file** with your GitLab configuration:
   ```env
   GITLAB_API_TOKEN=glpat-xxxxxxxxxxxxxxxxxxxx
   GITLAB_BASE_URL=https://gitlab.com
   GITLAB_PROJECTS=mygroup/project1,mygroup/project2
   ```

4. **Restart the application** after creating and editing the `.env` file
5. The app will display helpful error messages if configuration is incomplete

**Required Configuration:**
- `GITLAB_API_TOKEN`: Your GitLab personal access token with `read_api` scope
- `GITLAB_PROJECTS`: Comma-separated list of project paths (e.g., `group/project1,group/project2`)
- `GITLAB_BASE_URL`: Your GitLab instance URL (optional, defaults to `https://gitlab.com`)

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
* `*.msi` – signed installer with configuration template and setup instructions included
* `*.exe` – portable version with configuration template and setup instructions included

**Important:** The installer does NOT automatically launch the application after installation. Users must:
1. Edit the `.env` file in the installation directory
2. Manually start the application after configuration

Both versions include:
- `gitlab-config.example` - Configuration template (users must copy and rename to `.env`)
- `INSTALLATION-README.txt` - Detailed setup instructions for users

---

## 🖥️ Using the App

### First-Time Setup (for installed versions)
1. **Install the application** - The installer will NOT launch the app automatically
2. **Find the installation directory** - Usually in `Program Files` or where you chose to install
3. **Edit the `.env` file** - Open with any text editor and configure your GitLab settings
4. **Start the application** - Double-click the executable or find it in your Start Menu
5. **Check the system tray** - The app will appear in the bottom-right corner

### Daily Usage
| Action | How |
|--------|-----|
| Show window | Left-click tray icon<br/>**Show GitLab Releases** in tray menu |
| Hide window | Click window **−** button or right-click tray → Hide |
| Quit        | Tray menu → **Quit** |
| Open release page | Click any row in the list |
| Mark releases as seen | Simply open the window – blue-dot disappears |
| Edit configuration | Edit the `.env` file in the application directory and restart |

---

## 📝 TODO

- **Add global shortcut toggle**: Implement `Ctrl+Alt+G` to toggle window visibility
- **Enhanced error handling**: Improve user feedback for network errors and API issues
- **Release filtering**: Add options to filter releases by tag patterns or dates

---

## 👐 Contributing
Pull requests and issues are welcome! Please follow the existing code style and make sure `cargo fmt && cargo clippy` pass.


