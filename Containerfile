ARG BASE_IMAGE=docker.io/alpine:3.20

FROM $BASE_IMAGE AS bindings-generator

WORKDIR /app

COPY Cargo.toml wrapper.h build.rs .
COPY src/ src/

ARG CLANG_VERSION_MAJOR=15

RUN --mount=type=cache,target=/var/cache/apk <<EOF
    apk update
    apk add clang${CLANG_VERSION_MAJOR} cargo openldap-dev
    cargo run --features "generate-bindings" --bin regenerate_bindings
EOF

FROM scratch AS exporter

COPY --from=bindings-generator /app/src/bindings.rs ./src/
COPY --from=bindings-generator /app/Cargo.lock .
