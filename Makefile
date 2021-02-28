.PHONY: debug release clean


debug:
	cargo fmt --all --
	cargo build
	cargo test
	cargo clippy --all-targets --all-features -- -D warnings

release:
	cargo build --release

clean:
	rm -rf target
