#!/bin/bash

COLOR_REST="$(tput sgr0)"
COLOR_GREEN="$(tput setaf 2)"

echo "${COLOR_GREEN}wasm-pack build --out-dir ../../web/src/pkg${COLOR_REST}"
wasm-pack build --out-dir ../../web/src/pkg
# eval "wasm-pack build --out-dir ../../web/src/pkg"