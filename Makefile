RUST_DEBUG_BIN_DIR=./target/debug
MOZIMD_EXEC=mozimd
MOZIMD_EXEC_DEBUG=$(RUST_DEBUG_BIN_DIR)/$(MOZIMD_EXEC)
MOZIMC_EXEC=mozimc
MOZIMC_EXEC_DEBUG=$(RUST_DEBUG_BIN_DIR)/$(MOZIMC_EXEC)

$(MOZIMD_EXEC_DEBUG) $(MOZIMC_EXEC_DEBUG):
	cargo build --all

srv: $(MOZIMD_EXEC_DEBUG)
	sudo $(MOZIMD_EXEC_DEBUG)

cli: $(MOZIMC_EXEC_DEBUG)
	sudo $(MOZIMC_EXEC_DEBUG) $(ARGS)
