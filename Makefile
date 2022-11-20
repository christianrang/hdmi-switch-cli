install:
	cargo install --path ./hdmi-switch

build:
	cargo build --manifest-path=./hdmi-switch/Cargo.toml --release
