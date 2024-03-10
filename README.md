# Simple CLI Environment Manager

Simple CLI Environment Manager is a lightweight, fast, and flexible tool designed in Rust to help developers manage and switch between different project environments seamlessly. Leveraging Rust's performance and safety, it provides a user-friendly command-line interface to simplify development workflows.

## Features

- **Fast and Safe:** Built with Rust for maximum efficiency and safety.
- **Easy Configuration:** Simple configuration process using TOML/YAML files.
- **Versatile:** Supports various programming languages and tools.
- **Cross-Platform:** Works on Linux, macOS, and Windows.

## Getting Started

### Prerequisites

- Rust (latest stable version recommended)
- Cargo (comes with Rust)

### Installation

To install Simple CLI Environment Manager, run the following command in your terminal:

```bash
cargo install env_manager
```

### Command Structure for the Environment Manager CLI

1. Initializing a New Project Configuration

Command: env_manager init [project_name]

Description: Initializes a new environment configuration for the specified project. This creates a base directory structure under .config/env_manager/[project_name]/ and generates a global.toml for global configurations.

Configuration structure:

```
.config/env_manager/[project_name]/
├── global.toml
├── local.toml
├── development.toml
├── testing.toml
└── production.toml
```

2. Creating Environment Configurations

   Command: env_manager create [environment_name]
   Description: Creates a new environment configuration file named [environment_name].toml under the project's directory. If global.toml exists, it can optionally pre-populate the new environment file with global settings.

3. Editing Environment Configurations

   Command: env_manager edit [environment_name]
   Description: Opens the specified environment configuration file ([environment_name].toml) in the default text editor for modifications.

4. Listing Available Environments

   Command: env_manager list
   Description: Lists all the environment configurations available for the current project.

5. Activating an Environment

   Command: env_manager activate [environment_name]
   Description: Activates the specified environment for the current shell session. This command sets the environment variables defined in the [environment_name].toml file.

6. Deleting an Environment Configuration

   Command: env_manager delete [environment_name]
   Description: Deletes the specified environment configuration file ([environment_name].toml) from the project's directory.

7. Viewing Environment Configuration

   Command: env_manager view [environment_name]
   Description: Displays the content of the specified environment configuration file ([environment_name].toml) in the terminal.

8. Importing Environment Configurations

   Command: env_manager import [file_path] [environment_name]
   Description: Imports environment variables from an existing file (e.g., .env) into the specified environment configuration file ([environment_name].toml).

9. Exporting Environment Configurations

   Command: env_manager export [environment_name] [file_path]
   Description: Exports the specified environment's variables to a file, allowing for compatibility with other tools or systems that require .env files or similar.

10. Help and Documentation

    Command: env_manager help
    Description: Displays help information, including a list of commands and their descriptions.

11. Set Global Variables

    Command: env_manager set-global [project_name] --var "KEY=value"
    Description: Sets or updates a global variable in the global.toml file for the specified project.

### Extended Core Commands for Microservice Projects

Initialize Microservice Configuration
Command: env_manager init-service [project_name] [service_name]
Description: Initializes environment configurations for a new microservice within the project. This creates a dedicated directory or configuration file for the microservice's environments.

List Microservices
Command: env_manager list-services [project_name]
Description: Lists all microservices within the specified project, providing an overview of the project's structure.

Edit Microservice Environment Configuration
Command: env_manager edit [project_name] [service_name] --env [env_name]
Description: Opens the environment configuration file for a specific microservice, allowing users to modify it as needed.

### Advanced Commands for Microservice Management

Bulk Activate Environments
Command: env_manager activate-all [project_name] --env [env_name]
Description: Activates the specified environment across all microservices in the project, ensuring consistency in environment variables for a comprehensive development or deployment session.

Validate Microservice Configurations
Command: env_manager validate-service [project_name] [service_name] --env [env_name]
Description: Validates a microservice's environment configuration, checking for completeness and correctness according to project standards or schemas.

### Utility Commands for Microservice Projects

Show Microservice Configuration
Command: env_manager show [project_name] [service_name] --env [env_name]
Description: Displays the current environment configuration for a specified microservice, including any global or project-wide settings that apply.

#### Handling Microservice-Specific Configurations

Modular Configuration Files: Maintain separate environment configuration files for each microservice. This could involve a directory structure that segregates configurations by microservice and environment, e.g., .config/env_manager/[project_name]/[service_name]/[env_name].toml.

Global and Local Overrides: Allow global (project-wide) settings to be overridden by microservice-specific configurations. This enables shared settings across all microservices while providing the flexibility to specify exceptions.

Environment Templates: Support templates for common environment configurations that can be applied to multiple microservices. This could help in standardizing environment setups across the project.

### Microservice Configuration Example

```toml
[project]
name = "ExampleProject"
basePath = "/path/to/example/project"
description = "A project with multiple microservices."

# Global environment variables that apply to all microservices
[global]
LOG_LEVEL = "info"
DATABASE_HOST = "global-db.example.com"

# Microservice-specific configurations
[[microservice]]
name = "AuthService"
path = "/path/to/example/project/services/auth"
environments = ["development", "testing", "production"]

[[microservice]]
name = "PaymentService"
path = "/path/to/example/project/services/payment"
environments = ["development", "testing", "production"]

[[microservice]]
name = "EmailService"
path = "/path/to/example/project/services/email"
environments = ["development", "testing", "production"]

# Define additional metadata or configurations as needed
[metadata]
version = "1.0.0"
lastUpdated = "2024-03-10"
```
