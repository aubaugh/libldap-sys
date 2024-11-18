#!/bin/sh -xe

# requires podman and jq

base_image=docker.io/alpine:3.20

client_container=example-libldap-sys
server_container=openldap-server
socket_volume=openldap-socket

# replace $socket_volume if it exists
if podman volume inspect $socket_volume > /dev/null 2>&1; then
    podman volume rm -f $socket_volume
fi
podman volume create $socket_volume

# trap to clean up resources on exit
cleanup() {
    set +e
    podman rm -f $server_container 2>/dev/null
    podman volume rm -f $socket_volume 2>/dev/null
}
trap cleanup EXIT INT TERM ERR

_socket_path=/run/shared
_socket_file=ldapi

# start openldap server container
_server_commands=$(
    local _socket_url=$(echo "${_socket_path}/${_socket_file}" | jq "@uri" -jRr)

    set -- \
        apk add openldap openldap-back-mdb ';' \
        slapd -h ldapi://$_socket_url/ -d stats

    echo "$*"
)
podman run \
    -d \
    --rm \
    --replace \
    --name $server_container \
    -v ${socket_volume}:${_socket_path}:rw,z \
    $base_image sh -c -xe "$_server_commands"

# run libldap-sys client example container
_client_commands=$(
    set -- \
        apk add openldap-dev cargo ';' \
        cargo run --example sasl_external

    echo "$*"
)
_work_dir=/app
podman run \
    --rm \
    -v ${socket_volume}:${_socket_path}:rw,z \
    -v .:$_work_dir:rw,Z \
    -w $_work_dir \
    -e RUST_LOG=debug \
    -e LDAP_SOCKET_PATH="${_socket_path}/${_socket_file}" \
    --name $client_container \
    $base_image sh -c -xe "$_client_commands"

# print server logs
podman logs $server_container
