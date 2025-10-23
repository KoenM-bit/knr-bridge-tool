# 🚀 KNR Bridge Tool

A high-performance bridge application to execute Python scripts on Opentrons robots via a web interface. Fully migrated from Electron to Tauri for better performance, security, and smaller bundle size.

## ✨ Features

- **🔧 System Tray Integration**: Runs in the background with a clean system tray interface
- **⚙️ Settings Management**: Easy configuration of robot IP, API endpoints, and other settings
- **🔄 Bridge Protocol**: Automatic polling for jobs, downloading Python scripts, and uploading to robot
- **📁 File Upload**: Test functionality to upload files directly to the robot
- **🌐 Environment Configuration**: Supports `.env` files for flexible configuration
- **🧹 Clean Codebase**: Optimized repository with comprehensive npm scripts for maintenance

## 🚀 Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (for Tauri backend)
- [Node.js](https://nodejs.org/) (for build tools)

### Setup

1. **Clone and install**:
```bash
git clone https://github.com/KoenM-bit/knr-bridge-tool.git
cd knr-bridge-tool
npm install
```

2. **Configure environment**:
```bash
cp .env.template .env
# Edit .env with your robot settings
```

3. **Start development**:
```bash
npm run dev
```

## 🛠️ Available Scripts

### **Development & Building**
```bash
npm run dev              # Start development mode
npm run build            # Production build
npm run build:debug     # Debug build with symbols
npm run mock             # Start mock robot server for testing
```

### **Cleaning & Maintenance**
```bash
npm run clean           # 🧹 Complete cleanup (cargo + temp + node_modules)
npm run clean:cargo     # Clean only Rust build artifacts
npm run clean:temp      # Clean temp files and .DS_Store
npm run clean:node      # Remove node_modules and package-lock.json

npm run start-fresh     # 🔄 Full reset + install + start dev
npm run reset           # Clean + reinstall dependencies
```

### **Health & Monitoring**
```bash
npm run size            # 📊 Check repository and build artifact sizes
npm run check           # �� Run all health checks
npm run check:cargo     # Check Rust code without building
npm run check:deps      # Run npm security audit
npm run lint            # ✨ Format and lint Rust code
npm run test            # 🧪 Run Rust tests
```

### **Helper**
```bash
npm run info            # 💡 Show all available commands
```

## 🏗️ Architecture

### Tauri Backend (Rust)

- **`src-tauri/main.rs`**: Main application with system tray and Tauri commands
- **`src-tauri/bridge.rs`**: Bridge polling loop and protocol implementation
- **`src-tauri/config.rs`**: Configuration management and persistence
- **`src-tauri/model.rs`**: Data structures for API responses

### Frontend (HTML/JS)

- **`frontend/settings.html`**: Settings UI interface
- **`robots.html`**: Robot interface page

## ⚙️ Configuration

The application supports multiple configuration methods:

### Environment Variables
```bash
ROBOT_BASE=http://192.168.0.57:31950    # Robot API base URL
API_BASE=http://localhost:8000          # Backend API base URL
```

### Configuration Priority
1. Settings UI in the application
2. `.env` file in the root directory  
3. Environment variables

## �� Migration from Electron

This application has been completely migrated from Electron to Tauri with significant improvements:

### Key Changes
- **System Tray**: Electron `tray` → Tauri `SystemTray`
- **IPC**: Electron `ipcMain`/`ipcRenderer` → Tauri `invoke` system
- **HTTP Requests**: Node.js `fetch` → Rust `reqwest`
- **File Operations**: Node.js `fs` → Rust `std::fs` and `rfd`
- **Background Processing**: Node.js event loop → Rust `tokio`

### Performance Improvements
- **Bundle Size**: ~100MB → ~15MB (85% reduction)
- **Memory Usage**: Significantly reduced
- **Startup Time**: Faster cold starts
- **Security**: Enhanced with Rust's memory safety

## 🧪 Development Workflow

### Regular Development
```bash
npm run dev          # Start developing
npm run mock         # Test with mock robot (separate terminal)
```

### When Things Get Messy
```bash
npm run start-fresh  # Nuclear option: clean everything and restart
npm run size         # Check if build artifacts are bloating the repo
```

### Before Committing
```bash
npm run lint         # Format and lint code
npm run test         # Run tests
npm run check        # Health check
npm run clean:cargo  # Clean build artifacts to keep repo lean
```

## 📊 Repository Health

This repository is optimized for cleanliness:
- **Size**: ~22MB (including dependencies)
- **Build artifacts**: Automatically excluded via `.gitignore`
- **Monitoring**: Use `npm run size` to track repository size

## 🔧 Troubleshooting

### White Screen in Settings
Check that `frontend/settings.html` exists and is properly formatted.

### Robot Connection Issues
1. Verify `ROBOT_BASE` is correctly set in `.env`
2. Ensure robot is reachable at the specified IP address
3. Check console output for connection errors
4. Test connectivity: `npm run mock` (starts local mock server)

### Build Errors
1. Clean build artifacts: `npm run clean:cargo`
2. Restart development server: `npm run start-fresh`
3. Check Rust installation: `rustc --version`

### Repository Size Issues
1. Check size: `npm run size`
2. Clean up: `npm run clean`
3. Monitor with: `du -sh .git/` (should be <50MB)

## 📄 License

MIT License - see LICENSE file for details.

---

**Tip**: Run `npm run info` anytime to see available commands! 🚀
