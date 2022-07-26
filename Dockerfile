# 1. This tells docker to use the Rust official image
FROM rust:latest

ARG COMPILER_URL

ENV \
COMPILER_URL=${COMPILER_URL}

# 2. Copy the files in your machine to the Docker image
COPY ./ ./

# Build your program for release
RUN cargo build --release

EXPOSE 8080

# Run the binary
CMD ["./target/release/live-backend"]


#
#
#
### 1. This tells docker to use the Rust official image
##FROM rust:latest
##
### 2. Copy the files in your machine to the Docker image
##COPY ./ ./
##
### Build your program for release
##RUN cargo build --release
##
### Run the binary
##CMD ["./target/release/live-backend"]
##
#
#
##
### Rust as the base image
##FROM rust:latest
##
### 1. Create a new empty shell project
##RUN USER=root cargo new --bin live-backend
##WORKDIR /live-backend
##
### 2. Copy our manifests
##COPY ./Cargo.lock ./Cargo.lock
##COPY ./Cargo.toml ./Cargo.toml
##
### 3. Build only the dependencies to cache them
##RUN cargo build --release
##RUN rm src/*.rs
##
### 4. Now that the dependency is built, copy your source code
##COPY ./src ./src
##
### 5. Build for release.
##RUN cargo install --path .
##
##CMD ["live-backend"]
##
#
#
#
#####################################################################################################
### Builder
#####################################################################################################
#FROM rust:latest AS builder
#
#RUN update-ca-certificates
#
## Create appuser
#ENV USER=myip
#ENV UID=10001
#
#RUN adduser \
#    --disabled-password \
#    --gecos "" \
#    --home "/nonexistent" \
#    --shell "/sbin/nologin" \
#    --no-create-home \
#    --uid "${UID}" \
#    "${USER}"
#
#
#WORKDIR /myip
#
#COPY ./ .
#
## We no longer need to use the x86_64-unknown-linux-musl target
#RUN cargo build --release
#
#####################################################################################################
### Final image
#####################################################################################################
#FROM debian:buster-slim
#
## Import from builder.
#COPY --from=builder /etc/passwd /etc/passwd
#COPY --from=builder /etc/group /etc/group
#
#WORKDIR /myip
#
## Copy our build
#COPY --from=builder /myip/target/release/live-backend ./
#
## Use an unprivileged user.
#USER myip:myip
#
#CMD ["/myip/live-backend"]
