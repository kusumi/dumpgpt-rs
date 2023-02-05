bin:
	cargo build
fmt:
	cargo fmt
	git status
clean:
	cargo clean
test:
	cargo test
lint:
	cargo clippy --fix
	git status
