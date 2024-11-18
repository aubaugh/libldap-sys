libldap-sys
===========

Rust FFI bindings to ldap.h and sasl/sasl.h

## Dependencies
The openldap shared libraries

## Regenerating bindings
To regenerate the bindings: ensure you have the development headers for openldap installed and run the following command:
```sh
cargo run \
  --features "generate-bindings" \
  --bin regenerate_bindings
```
or install podman and run the follwing script:
```sh
./gen-bindings.sh
```

## Example
Install podman and jq then run the following script:
```sh
./run-example.sh
```

## License
BSD-3-Clause
