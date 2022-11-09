set_up_repo_for_development:
	pre-commit install
	cargo build

build_for_release:
	cargo build --release

install_cli_in_system: build_for_release
	cp target/release/t-rs $(HOME)/.local/bin/t
