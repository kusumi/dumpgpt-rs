bin:
	cargo build --release
fmt:
	cargo fmt
	git status
clean:
	cargo clean
test:
	cargo test --release
lint:
	cargo clippy --release --fix
	git status

xxx:	fmt lint test
