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

### Prerequisites

- [Yarn](https://yarnpkg.com/) is used as a package manager for handling the Svelte project dependencies.
- [Rust](https://www.rust-lang.org/) for working with the Tauri backend.
- [MongoDB](https://www.mongodb.com/) should be set up and running.

### Environment Setup

Before running the application, you need to set up your environment variables. Create a `.env` file in the root of the project and add the following line:

```env
MONGODB_URI=mongodb://localhost:27017
```

This line specifies the URI for connecting to your MongoDB instance. Adjust the URI according to your MongoDB configuration if necessary.

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

## Contributing

We welcome contributions to Curvitia! Whether it's feature requests, bug reports, or code contributions, please feel free to make a pull request or open an issue.

## License

Curvitia is licensed under the MIT License - see the [LICENSE](https://github.com/SeaPants/curvitia/blob/main/LICENSE) file for details.
