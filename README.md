# Syndeos

Syndeos is a powerful desktop application for managing VPS servers across various cloud platforms like Digital Ocean, Linode, and more. Built with modern technologies, Syndeos provides a seamless experience for managing server infrastructure, databases, and deployments all from a single interface.

## Features

- **Server Management**: Connect to and manage multiple VPS instances across different providers
- **Database Management**: Create, configure, and maintain databases and database users
- **User Administration**: Manage server users and permissions
- **Server Maintenance**: Monitor and maintain server health and performance
- **PHP Version Management**: Easily switch between different PHP versions
- **Backup Management**: Schedule and manage backups of your server data
- **Web Server Configuration**: Configure and manage Nginx/Apache settings
- **SSH Key Management**: Generate SSH keys and securely connect to your servers

## Technology Stack

Syndeos is built using modern web and desktop technologies:

- **Tauri v2**: Cross-platform framework for building desktop applications
- **Rust**: Backend functionality and system integration
- **React v19**: Frontend interface
- **Tailwind CSS v4**: Utility-first CSS framework
- **Shadcn/UI**: Component library for beautiful, accessible UI elements
- **TypeScript**: Type-safe JavaScript
- **pnpm with Node LTS**: Package manager and development tooling

## Getting Started

### Prerequisites

Before installing Syndeos, ensure you have the following installed:

- [Node.js](https://nodejs.org/) (LTS version) with [pnpm](https://pnpm.io/)
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- Platform-specific dependencies for Tauri development:
  - **Windows**: Visual Studio C++ Build Tools
  - **macOS**: Xcode Command Line Tools
  - **Linux**: Various development packages (see [Tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites))

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/righteouswaters/syndeos.git
   cd syndeos
   ```

2. Install dependencies:
   ```bash
   pnpm install
   ```

3. Build the application:
   ```bash
   pnpm tauri build
   ```

4. The built application will be available in the `src-tauri/target/release` directory.

### Development Setup

1. Install dependencies:
   ```bash
   pnpm install
   ```

2. Start the development server:
   ```bash
   pnpm tauri dev
   ```

## Usage

### Connecting to a Server

1. Launch Syndeos
2. Click "Add Server" in the sidebar
3. Enter your server details (hostname, IP, credentials)
4. Choose your cloud provider
5. Click "Connect"

### Managing Databases
1. Select a connected server from the sidebar
2. Navigate to the "Databases" tab
3. From here you can:
   - Create new databases
   - Manage database users and permissions
   - Import/export database content
   - Monitor database performance

### Server Maintenance

1. Select a server from the sidebar
2. Navigate to the "Maintenance" tab
3. Here you can:
   - Update server packages
   - Configure automated maintenance
   - View server logs
   - Monitor resource usage

### Managing PHP Versions

1. Select a server from the sidebar
2. Navigate to the "PHP" tab
3. Select from available PHP versions to install
4. Configure PHP settings for your applications

### Backup Management

1. Select a server from the sidebar
2. Navigate to the "Backups" tab
3. Configure backup schedules
4. Restore from previous backups

## Architecture

Syndeos follows the MVC (Model-View-Controller) architecture:

- **Model**: Rust backend services that interact with server APIs and SSH connections
- **View**: React components built with Tailwind CSS and Shadcn/UI
- **Controller**: Tauri commands that bridge the frontend and backend

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

[GNU GPL3](https://www.gnu.org/licenses/gpl-3.0.en.html)

## Support

If you encounter any problems or have any questions, please open an issue on the GitHub repository.
