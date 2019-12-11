PREFIX = /usr/local
BRAPH = braph
OBJ = target/release/$(BRAPH)
CARGO := $(shell command -v cargo 2> /dev/null)

build:
ifndef CARGO
	$(error "cargo is not available. Please install rustup (https://www.rust-lang.org/tools/install)")
endif
	$(CARGO) build --release

install:
	mkdir -p $(DESTDIR)$(PREFIX)/bin
	cp -f $(OBJ) $(DESTDIR)$(PREFIX)/bin
	chmod 755 $(DESTDIR)$(PREFIX)/bin/$(BRAPH)

uninstall:
	rm -f $(DESTDIR)$(PREFIX)/bin/$(BRAPH)

.PHONY: install build uninstall