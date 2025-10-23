# KNR Bridge App

A bridge application to execute Python scripts on a robot via a web interface. Fully migrated from Electron to Tauri for better performance and security.

## Features

- **System Tray Integration**: The app runs in the background with a system tray icon
- **Settings Management**: Configure robot IP, API endpoints and other settings
- **Bridge Protocol**: Automatic polling for jobs, downloading Python scripts, and uploading to robot
- **File Upload**: Test functionality to upload files directly to the robot
- **Environment Configuration**: Supports `.env` files for configuration

## Development

### Prerequisites

- [Rust](https://rustup.rs/) (voor Tauri backend)
- [Node.js](https://nodejs.org/) (voor build tools)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

### Setup

1. Install dependencies:
```bash
npm install
```

2. Configure your robot settings in `.env`:
```bash
ROBOT_BASE=http://192.168.0.57:31950
API_BASE=http://localhost:8000
```

3. Start development server:
```bash
npm run tauri:dev
```

### Build

Build for production:
```bash
npm run tauri:build
```

## Architecture

### Tauri Backend (Rust)

- **`src-tauri/main.rs`**: Main application with system tray and Tauri commands
- **`src-tauri/bridge.rs`**: Bridge polling loop and protocol implementation
- **`src-tauri/config.rs`**: Configuration management and persistence
- **`src-tauri/model.rs`**: Data structures for API responses

### Frontend (HTML/JS)

- **`frontend/settings.html`**: Settings UI interface
- **`robots.html`**: Robot interface page

## Migration from Electron

This application has been fully migrated from Electron to Tauri. The original Electron files have been removed to clean up the codebase.

### Key Changes

- **System Tray**: From Electron's `tray` to Tauri's `SystemTray`
- **IPC**: From Electron's `ipcMain`/`ipcRenderer` to Tauri's `invoke` system
- **HTTP Requests**: From Node.js `fetch` to Rust `reqwest`
- **File Operations**: From Node.js `fs` to Rust `std::fs` and `rfd`
- **Background Processing**: From Node.js event loop to Rust `tokio`

## Configuration

The application uses the following configuration options:

- **`ROBOT_BASE`**: Base URL of the robot API (e.g. `http://192.168.0.57:31950`)
- **`API_BASE`**: Base URL of the backend API (e.g. `http://localhost:8000`)

These can be set via:
1. `.env` file in the root directory
2. Settings UI in the application
3. Environment variables

## Scripts

- **`npm run tauri:dev`**: Start development server
- **`npm run tauri:build`**: Build voor productie
- **`npm run dev`**: Legacy script (gebruik `tauri:dev`)

## Troubleshooting

### White Screen in Settings

Als je een wit scherm ziet bij settings, controleer of `frontend/settings.html` bestaat en correct is.

### Robot Connection Issues

1. Controleer of `ROBOT_BASE` correct is ingesteld in `.env`
2. Verificeer dat de robot bereikbaar is op het opgegeven IP adres
3. Check de console output voor verbindingsfouten

### Build Errors

Als je build errors krijgt:
1. Zorg dat Rust en Tauri CLI geïnstalleerd zijn
2. Run `cargo clean` in de `src-tauri` directory
3. Herstart de development server

## License

[Licentie informatie hier toevoegen]