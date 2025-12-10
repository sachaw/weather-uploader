FROM registry.access.redhat.com/ubi10/ubi:latest as builder

WORKDIR /app

COPY . .

RUN dnf install -y rust-tools

RUN cargo build --release

FROM registry.access.redhat.com/ubi10/ubi-minimal:latest

WORKDIR /app

COPY --from=builder /app/target/release/weather-uploader /app/

EXPOSE 8080

CMD ["./weather-uploader"]
