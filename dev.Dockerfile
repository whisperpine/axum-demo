# This file is intended to be used in conjunction with `docker compose watch`.
# Caution: this file should only be used in development, **not** in production.

# For the multistage, renew the ARG by simply stating: ARG XXX
ARG APP_NAME=axum-demo

################################################################################
# Create a stage for building the application.
FROM rust:alpine AS build
ARG APP_NAME
WORKDIR /app

# Install host build dependencies.
RUN apk add --no-cache musl-dev

# Copy all project files while respecting .dockerignore
COPY --link . .

# Build the application.
# Leverage a cache mount to /usr/local/cargo/registry for downloaded dependencies,
# a cache mount to /usr/local/cargo/git/db for git repository dependencies,
# and a cache mount to /app/target/ for compiled dependencies which will speed up subsequent builds.
# Leverage a bind mount to the src directory to avoid having to copy the source code into the container.
# Once built, copy the executable to an output directory before the cache mounted /app/target is unmounted.
RUN --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=cache,target=/app/target/,id=rust-cache-${APP_NAME}-${TARGETPLATFORM} \
    --mount=type=cache,target=/usr/local/cargo/registry,id=crates-cache-${APP_NAME}-${TARGETPLATFORM} \
    --mount=type=cache,target=/usr/local/cargo/git/db,id=git-cache-${APP_NAME}-${TARGETPLATFORM} \
    cargo build --target-dir ./target && \
    cp ./target/debug/${APP_NAME} /app/

################################################################################
# Create a new stage for running the application that contains the minimal
# runtime dependencies for the application. This often uses a different base image
# from the build stage where the necessary files are copied from the build stage.

FROM busybox:musl AS final
ARG APP_NAME
WORKDIR /app

# Expose the port that the application listens on.
EXPOSE 3000

# Copy the executable from the "build" stage.
COPY --link --from=build /app/${APP_NAME} /app/

# What the container should run when it is started.
CMD ["/app/axum-demo"]
