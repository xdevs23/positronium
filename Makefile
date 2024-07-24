OUT_DIR = out
OS_KERNEL_OUT := target/x86_64-unknown-none/debug/positronium
POST_BUILD_DIR := $(OUT_DIR)/post_build
POST_BUILD := $(POST_BUILD_DIR)/.make
EFI_ESP_OUT := $(POST_BUILD_DIR)/esp
EFI_LOADER_OUT := $(EFI_ESP_OUT)/EFI/BOOT/BOOTX64.EFI

$(POST_BUILD):
	mkdir -p $(POST_BUILD_DIR)
	@touch $@

OVMF_PACKAGE_URL := https://archlinux.org/packages/extra/any/edk2-ovmf/download
OVMF_PACKAGE_DEST := out/post_build/edk2-ovmf.pkg.tar.zst
OVMF_DEST := out/post_build/ovmf.fd

$(OVMF_PACKAGE_DEST): $(POST_BUILD)
	@echo "Downloading OVMF UEFI firmware..."
	curl -L $(OVMF_PACKAGE_URL) > $@

$(OVMF_DEST): $(OVMF_PACKAGE_DEST)
	@echo "Unpacking OVMF UEFI firmware..."
	@# https://stackoverflow.com/a/14295908/4479004
	zstd -d < $(OVMF_PACKAGE_DEST) | tar -xv --transform='s/.*\///' -C $(POST_BUILD_DIR) usr/share/edk2/x64/OVMF.fd
	mv $(POST_BUILD_DIR)/OVMF.fd $@

LIMINE_CFG := $(EFI_ESP_OUT)/boot/limine.cfg
LIMINE_HOST_CFG := $(EFI_ESP_OUT)/boot/limine.host.cfg

$(LIMINE_CFG): limine.cfg
	install -D limine.cfg $(LIMINE_CFG)

$(LIMINE_HOST_CFG): build/config/limine.host.cfg
	install -D build/config/limine.host.cfg $(LIMINE_HOST_CFG)

$(EFI_LOADER_OUT): $(LIMINE_CFG)
	@echo "Installing Limine"
	mkdir -p $$(dirname $(EFI_LOADER_OUT))
	curl -fsSL https://github.com/limine-bootloader/limine/raw/v7.12.0-binary/BOOTX64.EFI > $(EFI_LOADER_OUT)

INSTALLED_KERNEL := $(EFI_ESP_OUT)/boot/os.kernel.bin
$(INSTALLED_KERNEL): $(OS_KERNEL_OUT)
	install -D $(OS_KERNEL_OUT) $(INSTALLED_KERNEL)

.PHONY: esp_installed_kernel
esp_installed_kernel: $(INSTALLED_KERNEL)

.PHONY: start-x86_64
start-x86_64: $(OVMF_DEST) $(EFI_LOADER_OUT) $(INSTALLED_KERNEL)
	@echo "Launching x86_64 emulator..."
	@qemu-system-x86_64 \
		-no-reboot \
		-no-shutdown \
		-M q35 \
		-M accel=kvm:tcg \
		-cpu host \
		-smp sockets=1,dies=1,cores=4,threads=2 \
		-object memory-backend-file,id=pc.ram,size=512M,mem-path=/dev/shm/qemu-positronium-ram,share=on \
		-machine memory-backend=pc.ram \
		-drive if=pflash,format=raw,index=0,file=$(OVMF_DEST) \
		-drive file=fat:rw:$(EFI_ESP_OUT) \
		-device pci-testdev,id=testdev000,bus=pcie.0,addr=0x4 \
		-net none \
		-serial stdio \
		-parallel none \
		-vga std \
		-boot c \
		-nodefaults \
		-d int,cpu_reset,unimp

.PHONY: debug-x86_64
debug-x86_64: | start-x86_64

EFI_ROOT_DIR ?= /efi
EFI_INSTALL_DIR ?= $(EFI_ROOT_DIR)/EFI/os
EFI_LIMINE_DIR ?= $(EFI_ROOT_DIR)/boot/limine

.PHONY: install-efi
install-efi: $(INSTALLED_KERNEL) $(EFI_LOADER_OUT) $(LIMINE_HOST_CFG)
	@echo "This will install the $(TARGET_DEVICE) EFI loader and kernel to $(EFI_INSTALL_DIR)"
	sudo mkdir -p $(EFI_INSTALL_DIR)
	sudo cp -R $(EFI_ESP_OUT)/. $(EFI_INSTALL_DIR)/.
	sudo mkdir -p $(EFI_LIMINE_DIR)
	sudo install $(LIMINE_HOST_CFG) $(EFI_LIMINE_DIR)/limine.cfg

.PHONY: clean
clean:
	rm -rf $(OUT_DIR)


