#!/bin/sh
set -e

RED=$(printf '\033[0;31m')
GRAY=$(printf '\033[1;30m')
GREEN=$(printf '\033[0;32m')
YELLOW=$(printf '\033[1;33m')
RESET=$(printf '\033[0m')

printf "\n${YELLOW}(>)${RESET} ${GRAY}Setting up project...${RESET}\n\n"

# -- Rust toolchain --
printf "${YELLOW}(→)${RESET} ${GRAY}Checking Rust toolchain...${RESET}\n"
if ! command -v rustup > /dev/null 2>&1; then
    printf "${RED}(!)${RESET} ${GRAY}Rust not found. Install it from ${RED}https://rustup.rs${RESET}\n"
    exit 1
fi
printf "${GREEN}(✔)${RESET} ${GRAY}Rust $(rustc --version | cut -d' ' -f2) found.${RESET}\n"

# -- WASM target --
printf "${YELLOW}(→)${RESET} ${GRAY}Adding wasm32-unknown-unknown target...${RESET}\n"
rustup target add wasm32-unknown-unknown > /dev/null 2>&1
printf "${GREEN}(✔)${RESET} ${GRAY}WASM target ready.${RESET}\n"

# -- Trunk --
printf "${YELLOW}(→)${RESET} ${GRAY}Checking Trunk...${RESET}\n"
if ! command -v trunk > /dev/null 2>&1; then
    printf "${YELLOW}(→)${RESET} ${GRAY}Installing Trunk...${RESET}\n"
    cargo install trunk
fi
printf "${GREEN}(✔)${RESET} ${GRAY}Trunk $(trunk --version | cut -d' ' -f2) ready.${RESET}\n"

# -- leptosfmt --
printf "${YELLOW}(→)${RESET} ${GRAY}Checking leptosfmt...${RESET}\n"
if ! command -v leptosfmt > /dev/null 2>&1; then
    printf "${YELLOW}(→)${RESET} ${GRAY}Installing leptosfmt...${RESET}\n"
    cargo install leptosfmt
fi
printf "${GREEN}(✔)${RESET} ${GRAY}leptosfmt ready.${RESET}\n"

# -- Compile dependencies --
printf "${YELLOW}(→)${RESET} ${GRAY}Compiling dependencies...${RESET}\n"
cargo check > /dev/null 2>&1
printf "${GREEN}(✔)${RESET} ${GRAY}Dependencies compiled.${RESET}\n"

# -- Git hooks --
printf "${YELLOW}(→)${RESET} ${GRAY}Configuring git hooks...${RESET}\n"
git config core.hooksPath .githooks
chmod +x .githooks/*
printf "${GREEN}(✔)${RESET} ${GRAY}Git hooks activated.${RESET}\n"

printf "\n${GREEN}(✔)${RESET} ${GRAY}Setup complete! Run ${GREEN}trunk serve --open${RESET}${GRAY} to start.${RESET}\n\n"
