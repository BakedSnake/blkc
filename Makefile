PREFIX=/usr/local
INSTALL_DIR=$(PREFIX)/bin
CURRENT_USER=$(shell whoami)
CONFIG_DIR=/home/$(CURRENT_USER)/.config/blkc/

config:
	mkdir -p $(CONFIG_DIR) 
	install -m 0644 blkc.conf $(CONFIG_DIR)
	install -m 0644 src/template.json $(CONFIG_DIR) 

install:
	install -m 0755 target/release/blkc $(INSTALL_DIR) 

clean:
	rm -rf $(PREFIX)/$(INSTALL_DIR)/blkc
