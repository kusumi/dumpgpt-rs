bin:
	cargo build
fmt:
	cargo fmt
	git status
clean:
	cargo clean
lint:
	cargo clippy --fix
	git status
