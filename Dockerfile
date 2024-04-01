FROM rust:1.75-bookworm as builder
WORKDIR /code

# Download crates-io index and fetch dependency code.
# This step avoids needing to spend time on every build downloading the index
# which can take a long time within the docker context. Docker will cache it.
RUN USER=root cargo init
COPY Cargo.toml Cargo.toml
RUN cargo fetch

# copy app files
COPY src src

# compile app
RUN cargo build --release


###############
## run stage ##
###############
FROM rust:1.75-bookworm as backend
WORKDIR /app

# copy server binary from build stage
COPY --from=builder /code/target/release/fan_feed_nashseverewx fan_feed_nashseverewx
COPY static_files static_files

# set user to non-root unless root is required for your app
USER 1001

# indicate what port the server is running on
EXPOSE 8080

# run server
CMD [ "/app/fan_feed_nashseverewx" ]