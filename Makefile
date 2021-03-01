.PHONY: debug release clean


debug:
	cargo fmt --all --
	cargo test
	cargo clippy --all-targets --all-features --
	cargo build

release:
	cargo fmt --all --
	cargo test
	cargo clippy --all-targets --all-features -- -D warnings
	cargo build --release

clean:
	rm -rf target
