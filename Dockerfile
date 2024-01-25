# ----------------------------------------
# build from rust:alpine, start from busybox
# ----------------------------------------

# From rust docker image base on alpine.
FROM rust:alpine AS builder
# To statically link to glibc.
RUN apk update && apk add --no-cache libc-dev
WORKDIR /project/
# Copy workspace scope cargo config.
COPY .cargo/config.toml ./.cargo/
COPY Cargo.toml .
# Download dependencies according to Cargo.toml.
RUN cargo fetch
COPY . .
RUN cargo build --release --offline

# Start from busybox with basic utilities.
FROM busybox:musl
# Set default envrionment variable.
ENV MONGODB_URI=mongodb://localhost:27017
ENV DB_NAME=axum-demo
# Copy the excutable.
COPY --from=builder \
    /project/target/release/axum_demo \
    /app/
# non-root user
RUN addgroup -S myapp && adduser -S myapp -G myapp
RUN chown -R myapp:myapp /app
USER myapp
# The network ports that this container will listen on.
EXPOSE 3000
# Provide defaults for an executing container.
CMD [ "/app/axum_demo" ]
