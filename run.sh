#!/bin/bash
set -ex
cargo run $1 < inputs/$1.txt
