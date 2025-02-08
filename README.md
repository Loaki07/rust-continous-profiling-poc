# Rust Continuous Profiling POC

A proof-of-concept system for continuous profiling of Rust applications, enabling real-time
performance monitoring and analysis through flame graphs.

## Installation

### Prerequisites

- Rust 1.75 or later
- Node.js 20 or later
- Docker (for containerized deployment)
- Kubernetes cluster (for k8s deployment)

### Local Development Setup

1. Clone the repository:
```bash
git clone https://github.com/yourusername/rust-profiling-system
cd rust-profiling-system
```

2. Build and run the backend:
```bash
# Build the project
cargo build

# Start the server
RUST_LOG=debug cargo run --bin server

# In another terminal, start the daemon
RUST_LOG=debug cargo run --bin daemon
```

3. Set up the frontend:
```bash
# Install dependencies
cd web
npm install

# Start development server
npm run dev
```

The development setup will be available at:
- Frontend: http://localhost:5173
- Backend API: http://localhost:3000
- Task Daemon: http://localhost:3001

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
```
data/
  ├── {profile-id}/
  │   ├── profile.pb  (raw pprof data)
  │   └── profile.json (processed flame graph data)
```

## Deployment

### Docker

#### Quick Start with Docker Compose

The fastest way to get everything running:
```bash
# Build and start all services
docker-compose up --build
```

#### Manual Container Build

Build the images:
```bash
# Build server
docker build -f Dockerfile.server -t profiling-server:latest .

# Build daemon
docker build -f Dockerfile.daemon -t profiling-daemon:latest .

# Build frontend
docker build -f web/Dockerfile -t profiling-frontend:latest .
```

#### Docker Compose Operations

```bash
# Start all services
docker-compose up

# Start in detached mode
docker-compose up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down

# Rebuild and start
docker-compose up --build
```

The services will be available at:
- Frontend: http://localhost:5173
- Backend API: http://localhost:3000
- Task Daemon: http://localhost:3001

Profile data is persisted in a named volume `profile-data`.

### Kubernetes

#### Prerequisites
- Kubernetes cluster (local or remote)
- kubectl configured to access your cluster
- Container registry access (if using remote cluster)

#### Quick Start

Deploy all components with a single command:
```bash
kubectl apply -f k8s/deployment.yaml
```

#### Verify the Deployment
```bash
kubectl get pods -n profiling-system

# Check all resources
kubectl get all -n profiling-system

# Check storage
kubectl get pvc -n profiling-system
```

#### Access the Application
```bash
kubectl port-forward svc/profiling-frontend -n profiling-system 8080:80
```

The application will be available at `http://localhost:8080`

#### Production Considerations

For production deployments:
1. Use a proper ingress controller
2. Set up TLS certificates
3. Configure proper storage class
4. Set up monitoring and logging
5. Configure backups for profile data

Example ingress configuration (not included in basic deployment):
```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: profiling-ingress
  namespace: profiling-system
spec:
  rules:
  - host: profiling.example.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: profiling-frontend
            port:
              number: 80
```

To clean up:
```bash
kubectl delete -f k8s/deployment.yaml
```

### Configuration

The system uses the following default ports:
- Frontend: 80 (in k8s) or 5173 (local dev)
- Backend Server: 3000 (HTTP) and 50051 (gRPC)
- Task Daemon: 3001

These can be configured through environment variables or Kubernetes ConfigMaps.

### Troubleshooting

Common issues and solutions:

1. Profile Data Not Persisting
```bash
# Check volume mount
kubectl describe pvc profile-data-pvc -n profiling-system

# Verify data directory permissions
kubectl exec -it <server-pod> -n profiling-system -- ls -la /usr/local/bin/data
```

2. Services Not Connecting
```bash
# Check service discovery
kubectl get svc -n profiling-system

# Test service connectivity
kubectl exec -it <pod-name> -n profiling-system -- curl profiling-server:3000/health
```

3. Pod Startup Issues
```bash
# Check pod logs
kubectl logs -f <pod-name> -n profiling-system

# Check pod status and events
kubectl describe pod <pod-name> -n profiling-system
```

4. Resource Constraints
```bash
# Check resource usage
kubectl top pods -n profiling-system

# View pod resource limits
kubectl describe pod <pod-name> -n profiling-system | grep -A 5 Limits
```

5. Common Error Messages

- "Task execution failed": Check daemon logs for execution details
- "Profile not found": Verify profile ID and storage persistence
- "Connection refused": Ensure all services are running and healthy

### Development Tips

1. Debugging
```bash
# VS Code debugging
# Press F5 to start debugging the server or daemon
# Or use the "Run and Debug" sidebar:
# - "Debug Server": Launch server with debugger
# - "Debug Daemon": Launch daemon with debugger
# - "Server + Daemon": Launch both with debugger

# Set breakpoints in VS Code
# Use debug console to evaluate expressions
# Use watch window to monitor variables
```

2. Local Testing
```bash
# Run with debug logging
RUST_LOG=debug cargo run --bin server

# Test profile generation
curl -X POST http://localhost:3000/api/tasks/run -H "Content-Type: application/json" -d '{"type":"cpu"}'
```

3. Container Debugging
```bash
# Shell into containers
docker exec -it <container-id> /bin/bash

# View logs in real-time
docker-compose logs -f server
```

4. Performance Monitoring
```bash
# Monitor container resources
docker stats

# Check profile data size
du -sh data/*
```

5. Common Debug Points
- Task execution flow: Set breakpoints in `execute_task` function
- Profile processing: Debug `handle_request` in server
- Data storage: Monitor `create_profile_dir` and `get_profile_path`
- Error handling: Check task message handling in daemon 