################################################################################
# Create a stage for building the application.
FROM --platform=$BUILDPLATFORM rust:alpine AS build
ARG APP_NAME=axum-demo
WORKDIR /app

# Install host build dependencies.
RUN apk add --no-cache musl-dev

COPY . .

# This is the architecture you're building for, which is passed in by the builder.
# Placing it here allows the previous steps to be cached across architectures.
# https://docs.docker.com/reference/dockerfile/#automatic-platform-args-in-the-global-scope
ARG TARGETPLATFORM

# Build the application.
# Leverage a cache mount to /usr/local/cargo/registry/ for downloaded dependencies,
# a cache mount to /usr/local/cargo/git/db for git repository dependencies,
# and a cache mount to /app/target/ for compiled dependencies which will speed up subsequent builds.
# Leverage a bind mount to the src directory to avoid having to copy the source code into the container.
# Once built, copy the executable to an output directory before the cache mounted /app/target is unmounted.
RUN --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/,id=rust-cache-${APP_NAME}-${TARGETPLATFORM} \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    cargo build --release --target-dir ./target && \
    cp ./target/release/${APP_NAME} /app/

################################################################################
# Create a new stage for running the application that contains the minimal
# runtime dependencies for the application. This often uses a different base image
# from the build stage where the necessary files are copied from the build stage.

FROM busybox:musl AS final
WORKDIR /app

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/go/dockerfile-user-best-practices/
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

# Expose the port that the application listens on.
EXPOSE 3000

# Copy the executable from the "build" stage.
COPY --from=build /app/${APP_NAME} /app/

# What the container should run when it is started.
CMD ["/app/axum-demo"]
