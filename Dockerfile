FROM rust:1.31


COPY . .

RUN rustup install nightly

RUN rustup default nightly

RUN cargo build --release

RUN cargo install --path .

EXPOSE 8000

CMD cargo run