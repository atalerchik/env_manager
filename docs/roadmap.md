# Roadmap

## Phase 1: Foundation and Core Functionality

### Milestone 1: Basic CLI Structure and Project Initialization

Task 1.1: Set up the basic CLI framework in Rust, including argument parsing.
Task 1.2: Implement the init [project_name] command to create a new project configuration structure.
Task 1.3: Design the .config/env_manager/[project_name]/config.toml file to store global project settings, including project path and general settings.

### Milestone 2: Environment and Microservice Configuration

Task 2.1: Implement init-service [project_name] [service_name] to initialize configurations for microservices.
Task 2.2: Develop the structure for environment-specific configurations within each microservice.
Task 2.3: Create commands for listing, creating, editing, and deleting environments (list, create, edit, delete).

## Phase 2: Environment Synchronization and Validation

### Milestone 3: Configuration Validation

Task 4.1: Develop the validate [project_name] --env [env_name] command to check environment configurations against a schema or .env.example.
Task 4.2: Implement validate-service for microservice-specific environment validation.

## Phase 3: Advanced Features and Integration

### Milestone 4: Advanced Environment Management

Task 5.1: Introduce bulk activation commands (activate-all) for simultaneous environment activation across microservices.
Task 5.2: Implement environment templates for common configurations across microservices.

## Phase 4: Security, Usability, and Documentation

### Milestone 5: Security Enhancements

Task 7.1: Implement encryption and decryption functionality for sensitive environment variables (encrypt, decrypt).
Task 7.2: Develop secure storage solutions for encrypted environment variables.

## Milestone 6: Usability Improvements and Documentation

Task 8.1: Enhance CLI usability with interactive prompts and better error handling.
Task 8.2: Comprehensive documentation covering setup, usage, advanced features, and best practices for managing environments.
