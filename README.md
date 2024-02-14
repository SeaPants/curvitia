# Curvitia

<div align="center">
<img src="/misc/logo.png" alt="Curvitia Logo" title="Curvitia" width="300" height="300">
</div>

Welcome to [Curvitia](https://github.com/SeaPants/curvitia), the next-generation platform designed to redefine the way you manage CVs. Our mission is to offer a seamless, intuitive, and comprehensive experience for both individuals and organizations looking to streamline their CV management processes. Built with cutting-edge technologies, Curvitia ensures efficiency, security, and scalability.

## Features

- **Effortless CV Management:** Organize, track, and manage CVs with ease.
- **Dynamic CV Creation:** Generate and customize CVs in real-time.
- **Secure Storage:** Your data is safe with our encrypted storage solutions.
- **Cross-Platform Accessibility:** Access your CVs from any device, anywhere.

## Technical Stack

- **Frontend:** Svelte + TypeScript
- **Backend:** Tauri (Rust)
- **Database:** MongoDB

## Getting Started

To get started with Curvitia, follow these steps to set up the project on your local machine.

### Prerequisites

- [Yarn](https://yarnpkg.com/) is used as a package manager for handling the Svelte project dependencies.
- [Rust](https://www.rust-lang.org/) for working with the Tauri backend.
- [MongoDB](https://www.mongodb.com/) should be set up and running.

### Installation

1. **Clone the repository**

```bash
git clone https://github.com/SeaPants/curvitia.git
cd curvitia
```

2. **Install dependencies**

- For the Svelte frontend and Tauri commands:

```bash
cd src
yarn install
```

### Running the Application

- **Start the Svelte frontend in development mode**

```bash
cd src
yarn dev
```

- **Run the Tauri application in development mode**

Ensure you're in the project root directory or navigate to it if you're elsewhere, then run:

```bash
yarn tauri dev
```

This command will start the Tauri development server, which compiles the Rust code and launches the application window. The `yarn tauri dev` command is correct if you have configured your `package.json` in the `src` directory to include Tauri commands, which is a common setup for projects integrating Tauri with a frontend framework like Svelte.

It's important to have the Tauri CLI installed globally or managed through your project's dependencies for these commands to work. The setup implies a Node.js environment for managing these aspects, even if Node.js is not required for executing the final, built application.

## Project Structure

- **Svelte Frontend:** Located in the `src` directory, it contains all the UI components, services, and state management for Curvitia.
- **Tauri Backend:** The Rust-based backend can be found in the `src-tauri` directory, handling secure operations, database interactions, and serving as the bridge between the frontend and MongoDB.

## Contributing

We welcome contributions to Curvitia! Whether it's feature requests, bug reports, or code contributions, please feel free to make a pull request or open an issue.

## License

Curvitia is licensed under the MIT License - see the [LICENSE](https://github.com/SeaPants/curvitia/blob/main/LICENSE) file for details.
