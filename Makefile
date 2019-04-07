SOURCES = src/lib.rs app/main.rs
CARGO_HOME ?= $(HOME)/.cargo
CARGO_BIN_PATH = $(CARGO_HOME)/bin
BINARY_NAME = teleport-rs
PROJECT = teleport

define POST_INST_NOTES

Teleport installed!

Add this to your .bashrc, .zshrc or similar to enable:

if [ -f "$(CARGO_BIN_PATH)/teleport.sh" -a -x "$(CARGO_BIN_PATH)/teleport.sh" ]
then
  . $(CARGO_BIN_PATH)/teleport.sh
fi

To start using it in the current shell, run this now:

. $(CARGO_BIN_PATH)/teleport.sh

endef

all: teleport

.PHONY = clean
clean:
	cargo clean -p $(PROJECT) --release

teleport: $(SOURCES)
	cargo build --release $(CARGO_FLAGS)

export POST_INST_NOTES
install: teleport
	cargo install --path $(PWD) --force --bin $(BINARY_NAME)
	cp src/teleport.sh $(CARGO_BIN_PATH)/teleport.sh
	chmod 0755 $(CARGO_BIN_PATH)/teleport.sh
	@echo "$$POST_INST_NOTES"

uninstall:
	cargo uninstall --bin $(BINARY_NAME)
	rm $(CARGO_BIN_PATH)/teleport.sh

travis: CARGO_FLAGS += --target x86_64-unknown-linux-musl
travis: teleport
