#!/bin/bash
[ ! -d "/res" ] && ./build.sh # build before testing
cargo test --workspace -- --nocapture