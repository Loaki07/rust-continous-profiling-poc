# Rust Profiling System POC

A proof-of-concept system for collecting, processing, and visualizing performance profiles from Rust applications.

## System Architecture

The system consists of three main components:

### 1. Backend Server (`src/bin/server.rs`)
- Dual-protocol server that handles both gRPC and HTTP
- gRPC endpoint (`[::1]:50051`) receives raw pprof profile data
- HTTP endpoints (`[::1]:3000`):
  - `/api/tasks/run` - Triggers profiling tasks
  - `/api/profiles/{id}` - Retrieves processed profile data
  - `/health` - Health check endpoint
- Processes and stores profiles in memory and on disk
- Manages communication between components

### 2. Task Daemon (`src/bin/daemon.rs`)
- Long-running process that executes profiling tasks
- HTTP server (`[::1]:3001`) receives task requests
- Supports different types of workloads:
  - CPU-intensive (recursive calculations, heavy computation)
  - Memory-intensive (string manipulation, large allocations)
  - Mixed workload (combination of CPU, memory, and I/O operations)
- Collects profile data using pprof
- Sends profile data back to main server via gRPC

### 3. Frontend UI (`web/`)
- Vue.js application for interacting with the system
- Shows system architecture and component status
- Allows triggering different types of profiling tasks
- Visualizes profile data as flame graphs
- Displays task execution status and history

## Data Flow

1. User requests task execution through UI
2. Request goes to backend server
3. Server forwards request to daemon
4. Daemon executes task and collects profile data
5. Profile data sent to server via gRPC
6. Server processes and stores profile data
7. UI retrieves and displays profile visualization

## Data Storage

Profiles are stored in a structured directory format: 