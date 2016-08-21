FROM jimmycuadra/rust

WORKDIR /app

COPY ./Cargo.toml /app
COPY ./Config.toml /app
COPY ./src /app/src
RUN cargo build --release

CMD target/release/importer
