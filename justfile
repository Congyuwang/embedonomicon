build:
    cargo build --bin app

build-release:
    cargo build --bin app --release

objdump:
    cargo objdump --bin app -- -d --no-show-raw-insn
