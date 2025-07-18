FROM node:20-alpine AS frontend-builder

# Set working directory for frontend build
WORKDIR /app/frontend

# Copy frontend files
COPY frontend/package*.json ./
RUN npm install

# Copy the rest of the frontend code and build it
COPY frontend/ ./
RUN npm run build

# Rust build stage
FROM rust:latest AS rust-builder

# Install build dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Set working directory for Rust build
WORKDIR /app

# Copy Cargo configuration files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src/ ./src/

# Copy the built frontend files
COPY --from=frontend-builder /app/frontend/dist ./frontend/dist

# Build the Rust application
RUN cargo build --release

# Final stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy the compiled binary
COPY --from=rust-builder /app/target/release/responder ./responder

# Copy any necessary environment files
COPY .env* ./

# Create a non-root user to run the application
RUN groupadd -r responder && \
    useradd -r -g responder responder && \
    chown -R responder:responder /app

USER responder

# Expose the port the app will run on dynamically
ARG PORT=8000
ENV PORT=${PORT}
EXPOSE ${PORT}

# Command to run the application
CMD ["./responder"]
