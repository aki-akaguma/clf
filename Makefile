
all: README.md

README.md: src/lib.rs
	cargo readme > $@

test:
	cargo test

clean:
	@cargo clean
	@rm -f z.*
