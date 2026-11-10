TARGET := riscv64gc-unknown-none-elf
KERNEL := target/$(TARGET)/release/myos
QEMU   := qemu-system-riscv64
QFLAGS := -machine virt -nographic -bios none

.PHONY: build run debug clean

build:
	cargo build --release

# Boot the OS. Quit QEMU with: Ctrl+A then X  (or type 'shutdown')
run: build
	$(QEMU) $(QFLAGS) -kernel $(KERNEL)

# Start paused, waiting for a GDB connection on port 1234
debug: build
	$(QEMU) $(QFLAGS) -kernel $(KERNEL) -s -S

clean:
	cargo clean
