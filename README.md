# MyOS

A tiny hobby operating system for **RISC-V 64-bit**, written in **Rust** (`no_std`).
It runs inside the **QEMU** emulator, so it never touches your real machine or macOS.

Current features: boot assembly, serial (UART) driver, `println!`, panic handler,
and an interactive shell (`help`, `clear`, `echo`, `info`, `uptime`, `panic`, `shutdown`).

## Run it in GitHub Codespaces (recommended)

1. Push this repo to GitHub.
2. Click **Code -> Codespaces -> Create codespace on main**.
   The devcontainer installs QEMU and the RISC-V Rust target automatically.
3. In the terminal:

   ```bash
   make run
   ```

4. You should see the `myos>` prompt. Type `help`.
5. Quit QEMU with **Ctrl+A, then X** (or type `shutdown`).

## Run it locally on a Mac (no OS changes)

QEMU and Rust are just normal apps/tools:

```bash
brew install qemu rustup
rustup-init -y            # if Rust is not installed yet
make run
```

## Project layout

```
src/boot.S     entry point: park extra harts, set stack, clear .bss, call kmain
src/main.rs    kernel entry (kmain) and panic handler
src/uart.rs    NS16550A serial driver + print!/println! macros
src/shell.rs   interactive shell
linker.ld      memory layout (RAM at 0x80000000)
Makefile       build / run / debug
.devcontainer  Codespaces setup
.github        CI: build + QEMU boot smoke test
```

## Roadmap

- [x] 1. Boot + serial output
- [x] 2. Interactive shell
- [ ] 3. Trap/interrupt handling (timer, UART interrupts)
- [ ] 4. Physical memory allocator, then a heap (`alloc`)
- [ ] 5. Paging / virtual memory
- [ ] 6. Multitasking (round-robin scheduler)
- [ ] 7. Simple filesystem
- [ ] 8. User mode + system calls

## Debugging

```bash
make debug                      # terminal 1: QEMU waits for GDB
gdb-multiarch target/riscv64gc-unknown-none-elf/release/myos \
  -ex "target remote :1234"     # terminal 2
```

## Learning resources

- The RISC-V Privileged Spec
- "Writing an OS in Rust" (x86, but the Rust ideas carry over)
- xv6-riscv (a small teaching OS for the same architecture)
