FROM registry.access.redhat.com/ubi10/ubi:latest as builder

WORKDIR /app

COPY . .

RUN dnf install -y rust-toolset openssl-devel

RUN cargo build --release

FROM registry.access.redhat.com/ubi10/ubi-minimal:latest

ARG REPO_FULL_NAME
LABEL org.opencontainers.image.title="weather-uploader" \
    org.opencontainers.image.description="Sensecap S1000 Weather Data Uploader" \
    org.opencontainers.image.vendor="sachaw" \
    org.opencontainers.image.licenses="GPL-3.0-or-later" \
    org.opencontainers.image.source="https://github.com/${REPO_FULL_NAME}" \
    org.opencontainers.image.documentation="https://github.com/${REPO_FULL_NAME}/README.md"

WORKDIR /app

COPY --from=builder /app/target/release/weather-uploader /app/

EXPOSE 8080

CMD ["./weather-uploader"]
