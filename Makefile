.PHONY: all run clean

TARGET      := riscv64gc-unknown-none-elf
KERNEL_FILE := target/$(TARGET)/release/kernel
DEBUG_FILE  ?= $(KERNEL_FILE)

OBJDUMP     := rust-objdump --arch-name=riscv64
OBJCOPY     := rust-objcopy --binary-architecture=riscv64

all:
	@cargo build --release -p kernel
	@cp $(KERNEL_FILE) kernel-qemu

run: all
	@qemu-system-riscv64 \
    -machine virt \
    -bios default \
    -device loader,file=kernel-qemu,addr=0x80200000 \
    -kernel kernel-qemu \
    -nographic \
    -smp 4 -m 256M

debug: all
	@qemu-system-riscv64 \
    -machine virt \
    -bios default \
    -device loader,file=kernel-qemu,addr=0x80200000 \
    -kernel kernel-qemu \
    -nographic \
    -smp 4 -m 256M \
    -s -S
    
clean:
	@rm kernel-qemu
	@rm $(KERNEL_FILE)
