
FORCE:

dev_build:
	cargo build

prod: FORCE
	git commit -a
	git push origin main

