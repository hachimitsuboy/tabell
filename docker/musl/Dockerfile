FROM ekidd/rust-musl-builder:latest AS builder

WORKDIR /opt/tabell

COPY . .

RUN  cargo build --release --target=x86_64-unknown-linux-musl

FROM scratch

COPY --from=builder /opt/tabell/target/x86_64-unknown-linux-musl/release/tabell /usr/bin/tabell

ENTRYPOINT [ "/usr/bin/tabell" ]