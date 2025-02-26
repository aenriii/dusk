
build:
	@cd kernel; cargo build --release --target t60.json

clean:
	@cd kernel; cargo clean
	@if [ -e iso/boot/dusk.bin ]; then \
		rm iso/boot/dusk.bin; \
	fi


run_kernel: build
	@qemu-system-i386 -kernel kernel/target/t60/release/kernel -machine type=pc-i440fx-3.1

run_iso: iso
	@qemu-system-i386 -cdrom dusk.iso -machine type=pc-i440fx-3.1

test:
	@if grub-file --is-x86-multiboot kernel/target/t60/release/kernel; then \
		echo multiboot confirmed; \
	else \
		echo the file is not multiboot; \
	fi
objdump:
	objdump -t kernel/target/t60/release/kernel

read:
	readelf kernel/target/t60/release/kernel

iso: clean build
	cp kernel/target/t60/release/kernel iso/boot/dusk.bin
	grub-mkrescue -o dusk.iso iso
