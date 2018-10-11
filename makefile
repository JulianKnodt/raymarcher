check:
	cargo check

test:
	cargo test -- --nocapture

first:
	cargo run --bin first

release:
	cargo build --release
