# ----------------------------------------
# build from rust:alpine, run from scratch
# ----------------------------------------

# 使用 rust:alpine 镜像用于构建, 其默认目标平台就是 alpine
FROM rust:alpine AS builder
# 添加 C 语言标准库, 某些 crate 编译时要用到
RUN apk add libc-dev
# 在 builder 容器中设置工作路径
WORKDIR /project/
# 使用国内的镜像仓库, 降低拉取第三方 crate 的耗时
COPY .cargo/config.toml ./.cargo/
# 首先仅复制 Cargo.toml, 只要此文件不发生修改, 之后构建时就会跳过这一步
COPY Cargo.toml .
# 根据 Cargo.toml 下载所有依赖的 crate
RUN cargo fetch
# 将所有的工程文件复制到工作目录 (被 .dockerignore 忽略的除外)
COPY . .
# 由于已经 cargo fetch, 因此构建时不需要联网
RUN cargo build --offline --release

# 不依赖于任何镜像, 仅利用 Linux 内核执行应用程序
FROM scratch
# 设置默认的环境变量
ENV MONGODB_URI=mongodb://localhost:27017
ENV DB_NAME=axum-demo
# 将构建好的应用程序 axum_demo 复制到根目录的 /app 路径中
COPY --from=builder /project/target/release/axum_demo /app/
# 开放容器端口
EXPOSE 3000
# 运行应用程序
CMD [ "/app/axum_demo" ]


# ----------------------------------------
# build from rust, run from ubuntu
# ----------------------------------------

# FROM rust AS builder
# WORKDIR /project/
# COPY .cargo/config.toml ./.cargo/
# COPY Cargo.toml .
# RUN cargo fetch
# COPY . .
# RUN cargo build --offline --release

# FROM ubuntu
# COPY --from=builder /project/target/release/axum_demo /app/
# EXPOSE 3000
# CMD [ "/app/axum_demo" ]
