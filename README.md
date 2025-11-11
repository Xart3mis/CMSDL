# CMSDL - CMS Downloader

A CLI application to download and sync content from GIU CMS (Course Management System).

## Features

- Download course materials from GIU CMS
- Support for batch downloads across multiple courses
- Interactive course selection
- Configurable download options
- Secure credential management

## Installation

### Option 1: Download Precompiled Binary (Recommended)

Download the latest precompiled binary for your platform from the [GitHub Releases](https://github.com/Xart3mis/CMSDL/releases) page.

1. Go to the [Releases page](https://github.com/Xart3mis/CMSDL/releases)
2. Download the binary for your operating system
3. Extract the archive (if applicable)
4. Make the binary executable (Linux/macOS): `chmod +x cms_dl`
5. Optionally, move it to a directory in your PATH for easy access

### Option 2: Build from Source

**Prerequisites:** Rust and Cargo (install from [rustup.rs](https://rustup.rs/))

```bash
git clone https://github.com/Xart3mis/CMSDL.git
cd CMSDL
cargo build --release
```

The compiled binary will be available at `target/release/cms_dl`.

## Usage

### First Run

On the first run, if no configuration file exists, the application will prompt you to create one:

```bash
cargo run
# or if installed:
./cms_dl
```

You'll be asked to provide:
- Username
- Password (minimum 8 characters)
- Download path

### Command Line Arguments

```bash
cms_dl [OPTIONS]
```

#### Options:

- `-u, --username <USERNAME>` - Your GIU account username
- `-p, --password <PASSWORD>` - Your GIU account password
- `--path <PATH>` - Where all downloaded content is saved
- `--courses <COURSES>` - Course IDs to download (downloads all if not specified)
  - Example: `--courses=34,2488`
- `-h, --help` - Print help information
- `-V, --version` - Print version

### Examples

#### Using Config File
```bash
# Run with existing config.toml
cargo run
```

#### Using Command Line Arguments
```bash
# Download all courses with credentials from command line
cargo run -- -u your_username -p your_password --path ./Downloads

# Download specific courses
cargo run -- -u your_username -p your_password --path ./Downloads --courses=34,2488,156
```

#### Interactive Mode
When you don't specify courses via `--courses`, the application will present an interactive list where you can:
- Use arrow keys to navigate
- Press Space to select/deselect courses
- Press ESC or 'q' to download all courses
- Press Enter to confirm selection

## Configuration File

The application uses a `config.toml` file for storing configuration. This file is created automatically on first run or when you provide credentials via command line arguments.

### Location

The configuration file is stored as `config.toml` in the current working directory.

### Configuration Entries

All possible configuration file entries:

```toml
[credentials]
# Required: Your GIU CMS username
username = "your_username"

# Required: Your GIU CMS password
password = "your_password"

[general_options]
# Required: Prompt user to filter specific courses
# Type: Boolean
# Default: false
interactive_filtering = false

[download_options]
# Optional: Maximum number of concurrent downloads
# Type: Integer (usize)
# Default: 3 (if not specified)
# Example: max_concurrency = 5
max_concurrency = 3

# Optional: Maximum file size limit in bytes
# Type: Integer (usize)
# Default: None (no limit)
# Example: max_file_size = 104857600  # 100MB in bytes
max_file_size = 104857600

# Required: Path where downloaded content will be saved
# Type: String (must be a valid path)
# Example: save_path = "./Downloads"
save_path = "/path/to/downloads"
```

### Configuration Entry Details

#### `[credentials]` Section

| Entry | Required | Type | Description |
|-------|----------|------|-------------|
| `username` | Yes | String | Your GIU CMS account username |
| `password` | Yes | String | Your GIU CMS account password (minimum 8 characters) |

#### `[general_options]` Section

| Entry | Required | Type | Description |
|-------|----------|------|-------------|
| `interactive_filtering` | Yes | Boolean | Whether to prompt user for course filtering |


#### `[download_options]` Section

| Entry | Required | Type | Default | Description |
|-------|----------|------|---------|-------------|
| `save_path` | Yes | String (Path) | N/A | Directory path where all downloaded content will be saved |
| `max_concurrency` | No | Integer | 3 | Maximum number of concurrent downloads. Controls how many files are downloaded simultaneously |
| `max_file_size` | No | Integer | None | Maximum file size in bytes. Files larger than this will be skipped (if set) |

### Example Configuration Files

#### Minimal Configuration
```toml
[credentials]
username = "john.doe"
password = "mySecurePassword123"

[general_options]
interactive_filtering = false

[download_options]
save_path = "./Downloads"
```

#### Full Configuration with All Options
```toml
[credentials]
username = "john.doe"
password = "mySecurePassword123"

[general_options]
interactive_filtering = false

[download_options]
max_concurrency = 5
max_file_size = 524288000  # 500MB
save_path = "/home/user/GIU_Downloads"
```

#### Configuration with Windows Path
```toml
[credentials]
username = "john.doe"
password = "mySecurePassword123"

[general_options]
interactive_filtering = false

[download_options]
max_concurrency = 3
save_path = "C:\\Users\\JohnDoe\\Downloads\\GIU"
```

## How It Works

1. **Authentication**: The application authenticates with GIU CMS using your credentials
2. **Course Scraping**: Retrieves all available courses from your account
3. **Course Selection**: Allows you to select specific courses or download all
4. **Content Scraping**: Fetches the list of files for each selected course
5. **Download**: Downloads all content to the specified path with concurrent downloads

## Security Notes

- The `config.toml` file contains your credentials in plain text
- Ensure the configuration file has appropriate permissions
- The file is included in `.gitignore` to prevent accidental commits
- Consider using command-line arguments for CI/CD environments

## Troubleshooting

### "Config file not found"
This is normal on first run. The application will guide you through creating a new configuration.

### "Invalid path"
Ensure the path specified in `save_path` is valid and accessible. The application validates paths during configuration.

### "Password must be longer than 8 characters long"
Passwords must be at least 8 characters. This is validated both during interactive setup and when using command-line arguments.

### Authentication Failures
- Verify your username and password are correct
- Check your internet connection
- Ensure you can log in to GIU CMS through a web browser

## Development

### Building for Release
```bash
cargo build --release
```

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.
