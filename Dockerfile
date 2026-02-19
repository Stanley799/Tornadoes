# ─── Build stage ─────────────────────────────────────────────
FROM rust:1.77-slim AS builder
WORKDIR /app

# Copy manifests first for layer caching
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release 2>/dev/null || true

# Copy real source and build
COPY src ./src
RUN cargo build --release

# ─── Runtime stage ───────────────────────────────────────────
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/handball_team_app .
COPY static ./static
COPY migrations.sql .

ENV DATABASE_URL=sqlite:///data/tornadoes.db
EXPOSE 3000

CMD ["./handball_team_app"]
