
FORCE:

dev_build:
	cargo build

tests: FORCE
	cargo test

prod: tests
	git commit -a
	git push origin main

