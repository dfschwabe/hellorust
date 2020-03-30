FROM rust:1.40.0 AS build
WORKDIR /usr/src
ARG APP_NAME=hellorust
ARG APP_VSN

# Download the target for static linking.
RUN rustup target add x86_64-unknown-linux-musl

# Create a dummy project and build the app's dependencies.
# If the Cargo.toml or Cargo.lock files have not changed,
# we can use the docker build cache and skip these (typically slow) steps.
RUN USER=root cargo new ${APP_NAME}
WORKDIR /usr/src/${APP_NAME}
COPY Cargo.toml Cargo.lock ./
RUN if [ "$APP_VSN" ] ; then sed -ie "s/version = \".*\"/version = \"${APP_VSN}\"/" ./Cargo.toml; rm ./Cargo.tomle; fi
RUN cargo build --release


# Copy the source and build the application.
COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Copy the statically-linked binary into a scratch container.
FROM scratch
COPY --from=build /usr/local/cargo/bin/hellorust .
USER 1000
CMD ["./hellorust"]