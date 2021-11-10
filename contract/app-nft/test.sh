#!/bin/bash
[ ! -d "../target" ] && ./build.sh # build before testing
cargo test -- --nocapture