taskie-test:
	cargo test -p taskie

taskie-build:
	cargo build --release -p taskie

taskie-install:
	cp target/release/taskie ~/bin/taskie

taskie-all:
	taskie-test taskie-build taskie-install
