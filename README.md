# Internal API Service (Thesis)

This repository contains the internal API microservice for the thesis project: "Development of an analytics system for marketplace product grids using neural network technologies for automation and acceleration of assortment analysis".

## Purpose & Architecture

The internal-api service acts as a backend integration layer, handling communication with parser services and optimizing database queries. It exposes gRPC endpoints for parser integration and manages PostgreSQL connection pooling for high performance.

### Main Components
- `src/main.rs`: gRPC server, DB pool setup, endpoint logic
- `proto/api.proto`: Protobuf definitions for gRPC
- `.env-clear`: Example environment configuration

## Workflow
1. Receives parser requests via gRPC
2. Executes optimized queries on PostgreSQL
3. Returns parsed and enriched product data to API gateway
4. Handles keyword management, product parsing, and error handling

## Features
- gRPC server for parser integration
- PostgreSQL connection pooling (deadpool-postgres)
- Database query optimization and error handling
- Protobuf-based API definitions
- Dockerized deployment

## Endpoints
- gRPC: `ParserIntegrationService` (see `proto/api.proto`)
- Handles requests for product parsing, keyword management, etc.

## Usage
1. Copy `.env-clear` to `.env` and fill in DB credentials
2. Build and run with Docker:
   ```powershell
   docker build -t internal-api .
   docker run --env-file .env -p 50051:50051 internal-api
   ```
3. Or run locally:
   ```powershell
   cargo build --release
   .\target\release\internal-api
   ```

## Configuration
- All sensitive data and connection strings must be set in `.env`.
- See `.env-clear` for required variables (PostgreSQL, etc).

## Integration
- Communicates with parser and API gateway via gRPC
- Connects to PostgreSQL for all DB operations

## Development Notes
- Rust, tonic (gRPC), deadpool-postgres
- See thesis for full integration details, workflow, and error handling strategies
