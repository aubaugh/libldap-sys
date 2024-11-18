#!/bin/sh -xe

client_image=libldap-sys

podman build \
    --target exporter \
    -t $client_image \
    -o . \
    .
