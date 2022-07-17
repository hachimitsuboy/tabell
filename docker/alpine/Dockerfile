FROM alpine:3.15.4 AS builder

WORKDIR /home/tabell

COPY . .

RUN  apk add --no-cache gcc musl-dev rust cargo \
  && cargo build --release

FROM alpine:3.15.4

RUN  apk add --no-cache libgcc musl-dev \
  && adduser -D tabell

COPY --from=builder /home/tabell/target/release/tabell /opt/tabell/tabell

WORKDIR /home/tabell
USER tabell

ENTRYPOINT [ "/opt/tabell/tabell" ]