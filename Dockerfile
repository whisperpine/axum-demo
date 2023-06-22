# ----------------------------------------
# build from rust:alpine, start from scratch
# ----------------------------------------

# From rust docker image base on alpine.
FROM rust:alpine AS builder
# To statically link to glibc.
RUN apk update && apk add libc-dev
WORKDIR /project/
# Copy workspace scope cargo config.
COPY .cargo/config.toml ./.cargo/
COPY Cargo.toml .
# Download dependencies according to Cargo.toml.
RUN cargo fetch
COPY . .
RUN cargo build --release --offline

FROM scratch
# Set default envrionment variable.
ENV MONGODB_URI=mongodb://localhost:27017
ENV DB_NAME=axum-demo
# Copy the excutable.
COPY --from=builder \
    /project/target/release/axum_demo \
    /app/
EXPOSE 3000
CMD [ "/app/axum_demo" ]
