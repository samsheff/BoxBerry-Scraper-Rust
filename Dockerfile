FROM rust

RUN mkdir /usr/local/boxberry
COPY . /usr/local/boxberry

WORKDIR /usr/local/boxberry

RUN cargo build --release

ENTRYPOINT ["/usr/local/boxberry/target/release/boxberry"]

