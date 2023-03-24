# ----------------------------------------
# build on rust, run on ubuntu
# ----------------------------------------

# FROM rust AS builder
# # 使用国内的镜像仓库, 降低拉取第三方 crate 的耗时
# COPY .docker/config.toml /usr/local/cargo/
# COPY Cargo.toml .
# RUN cargo fetch
# COPY . .
# RUN cargo build --offline --release

# FROM ubuntu
# COPY --from=builder /target/release/axum_demo /app/
# EXPOSE 3000
# ENTRYPOINT [ "/app/axum_demo" ]


# ----------------------------------------
# build on rust, run on alpine
# ----------------------------------------

# FROM rust AS builder
# # 使用国内的镜像仓库, 降低拉取第三方 crate 的耗时
# COPY .docker/config.toml /usr/local/cargo/
# RUN rustup target add x86_64-unknown-linux-musl
# COPY Cargo.toml .
# RUN cargo fetch
# COPY . .
# RUN cargo build --offline --release \
#     --target x86_64-unknown-linux-musl

# FROM alpine
# COPY --from=builder /target/x86_64-unknown-linux-musl/release/axum_demo /app/
# EXPOSE 3000
# ENTRYPOINT [ "/app/axum_demo" ]


# ----------------------------------------
# build on rust:alpine, run on alpine
# ----------------------------------------

FROM rust:alpine AS builder
# 添加 C 语言标准库, 某些 crate 编译时要用到
RUN apk add libc-dev
# 使用国内的镜像仓库, 降低拉取第三方 crate 的耗时
COPY .docker/config.toml /usr/local/cargo/
COPY Cargo.toml .
RUN cargo fetch
COPY . .
RUN cargo build --offline --release

FROM alpine
COPY --from=builder /target/release/axum_demo /app/
EXPOSE 3000
ENTRYPOINT [ "/app/axum_demo" ]