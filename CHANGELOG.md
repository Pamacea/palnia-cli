# Changelog

All notable changes to this project will be documented in this file.

## [0.2.5] - 2026-04-01

### Fixed

- Removed unused code (dead code warnings)
  - Removed unused `id` field from `UploadResponse` in events and tasks commands
  - Removed unused `filename` variable in images command

## [0.2.4] - 2026-04-01

### Fixed

- **npm wrapper**: Fixed security warning by using spawnSync instead of spawn
- **update command**: npm installations now show instructions to update via npm
  - Cannot self-update binary while running on Windows
  - Shows: `npm update -g @oalacea/palnia-cli` command

## [0.2.3] - 2026-04-01

### Fixed

- **npm wrapper**: Fixed security warning in run.js (removed shell option)
- **login flow**: No longer prompts for API URL - uses default automatically
  - Use `--url <URL>` flag for custom API URL
  - Default: `https://palnia.newalfox.fr/api`

## [0.2.2] - 2026-04-01

### Added

- **Auto-update**: `palnia update` command to update to the latest version
- **Update notifications**: Automatic check for updates on every command (non-blocking)
- **npm wrapper**: Fixed `run.js` for proper binary execution

### Changed

- README updated with npm installation instructions and update documentation

## [0.2.0] - 2026-03-27

### Added

- **Image management**: Complete image upload, download, list, rename, delete, quota commands
  - `palnia images list` - Gallery view with format badges [WebP][JPG][PNG]
  - `palnia images show <id>` - Detailed image info (size, date, linked entity)
  - `palnia images upload <file> --task <id>` - Attach image to task/event
  - `palnia images download <id> --format webp|original` - Download with format choice
  - `palnia images quota` - Storage quota with colored progress bar
  - `palnia images rename <id>` - Rename images
  - `palnia images delete <id>` - Delete images

- **Task updates**: Full task modification support
  - `palnia tasks update <id>` - Update title, category, priority, due date, notes, tags, status, archived
  - `palnia tasks archive <id>` / `unarchive <id>` - Archive management
  - `palnia tasks archived` - List archived tasks

- **Event updates**: Enhanced event modification
  - `palnia events update <id>` - Full update support
  - `palnia events add` - Now supports `--all-day` toggle for updates

- **Image attachment during creation**:
  - `palnia tasks add "Title" --image ~/path/to/photo.jpg`
  - `palnia events add "Title" --image ~/Downloads/image.png`
  - Supports `~` for home directory expansion

- **Home directory expansion**: All image paths now support `~` for home directory

### Changed

- **Renamed**: `palnia` branding unified across all files
- **Config directory**: `~/.palnia/`
- **Binary name**: `palnia-cli`
- **API token prefix**: Now uses `plt_*` tokens (Palnia tokens)

### Technical

- Updated `reqwest` dependency with `multipart` feature for file uploads
- Added `mime_guess` dependency for content-type detection
- Enhanced type definitions with `RecurrenceRule`, `ImageQuota`, `GalleryImage`
- Improved error messages and validation

## [0.2.1] - 2026-04-01

### Fixed

- **API URL**: Changed default API URL from `localhost` to `https://palnia.newalfox.fr/api`
- **npm install**: Added GitHub Actions workflow for automatic releases with cross-platform binaries

### Added

- **npm package**: `@oalacea/palnia-cli` for easy installation
  - Cross-platform binary download from GitHub Releases
  - Automatic OS detection (Windows, macOS Intel/ARM, Linux x86_64/ARM64)
  - Install with: `npm install -g @oalacea/palnia-cli`
- **GitHub Actions**: Automatic release workflow building binaries for all platforms

## [0.1.1] - 2026-03-18

### Added

- README with full CLI documentation

## [0.1.0] - 2026-03-18

### Added

- Initial release for Palnia productivity app
- Commands: login, logout, whoami, tasks, events, habits, agenda
- Tasks: add (with category, priority, due date, notes, tags, subtasks), done, doing, delete, subtask, all
- Events: list today/week, add (with category, description, notes, tags, all-day), delete
- Habits: list, add (with category, frequency), toggle (with custom date), delete
- Agenda: combined view (events + tasks) for today and week
- Config stored in `~/.palnia/config.toml`
- API token auth (plt_* tokens)
