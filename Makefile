.PHONY: all run clean

TARGET      := riscv64gc-unknown-none-elf
DEBUG_KERNEL_FILE := target/$(TARGET)/debug/kernel
RELEASE_KERNEL_FILE := target/$(TARGET)/release/kernel

OBJDUMP     := rust-objdump --arch-name=riscv64
OBJCOPY     := rust-objcopy --binary-architecture=riscv64


build_debug:
	@cargo build --target $(TARGET)
	cp $(DEBUG_KERNEL_FILE) kernel-qemu

build_release:
	@cargo build --target $(TARGET) -r
	cp $(RELEASE_KERNEL_FILE) kernel-qemu

run: build_release
	@qemu-system-riscv64 \
    -machine virt \
    -bios default \
    -device loader,file=kernel-qemu,addr=0x80200000 \
    -kernel kernel-qemu \
    -nographic \
    -smp 4 -m 2G

debug_run: build_debug
	@qemu-system-riscv64 \
    -machine virt \
    -bios default \
    -device loader,file=kernel-qemu,addr=0x80200000 \
    -kernel kernel-qemu \
    -nographic \
    -smp 8 -m 2G \

debug: build_debug
	@qemu-system-riscv64 \
    -machine virt \
    -bios default \
    -device loader,file=kernel-qemu,addr=0x80200000 \
    -kernel kernel-qemu \
    -nographic \
    -smp 8 -m 2G \
    -s -S
    
clean:
	@rm kernel-qemu
	@rm $(DEBUG_KERNEL_FILE) $(RELEASE_KERNEL_FILE)
