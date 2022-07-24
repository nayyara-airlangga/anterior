# Stage 1 - Plan recipe for dependencies
FROM rust as planner
WORKDIR /osiris

RUN cargo install cargo-chef

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 2 - Build dependencies to cache
FROM rust as cacher
WORKDIR /osiris

RUN cargo install cargo-chef

COPY --from=planner /osiris/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Stage 3 - Build the application
FROM rust as builder
WORKDIR /osiris

# Copy source code
COPY . .

# Copy cached dependencies and cargo registry
COPY --from=cacher /osiris/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

# Build release image
RUN cargo build --release

# Stage 4 - Create runtime
FROM gcr.io/distroless/cc-debian11 AS runtime
WORKDIR /osiris

# Copy release binary from builder
COPY --from=builder /osiris/target/release/osiris /osiris

EXPOSE 7000

# Start application
CMD [ "./osiris" ]
