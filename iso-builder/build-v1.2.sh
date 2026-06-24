#!/bin/bash
set -e
F="$HOME/forge-os"
O="/tmp/forge-iso-output"
rm -rf "$O" /tmp/forge-rootfs /tmp/forge-efi
mkdir -p "$O/live" "$O/boot/grub" "$O/EFI/BOOT"
mkdir -p /tmp/forge-efi/EFI/BOOT

echo "=== Forge OS ISO Builder v1.2 ==="

mkdir -p "$O/live/usr/bin"
for b in forge-init forge-svc forge-hal forge-llm forge-voice forge-agent forge-gui forge-theme forge-guard; do
    cp "$F/target/release/$b" "$O/live/usr/bin/"
done
strip "$O/live/usr/bin/"* 2>/dev/null || true
echo "Binaries: $(du -sh "$O/live/usr/bin" | cut -f1)"

cp /boot/vmlinuz-*   "$O/live/vmlinuz"
cp /boot/initrd.img-* "$O/live/initrd"
echo "Kernel: $(du -sh "$O/live/vmlinuz" | cut -f1)"

mkdir -p /tmp/forge-rootfs/usr/bin
mkdir -p /tmp/forge-rootfs/etc/forge
mkdir -p /tmp/forge-rootfs/etc/forge/trusted_keys
mkdir -p /tmp/forge-rootfs/opt/forge/models
mkdir -p /tmp/forge-rootfs/opt/forge/themes/builtin
mkdir -p /tmp/forge-rootfs/opt/forge/themes/community
mkdir -p /tmp/forge-rootfs/opt/forge/widgets
mkdir -p /tmp/forge-rootfs/var/log/forge
cp "$O/live/usr/bin/"* /tmp/forge-rootfs/usr/bin/

cat > /tmp/forge-rootfs/etc/forge/init.json << 'J1'
{"svc_binary":"/usr/bin/forge-svc"}
J1
cat > /tmp/forge-rootfs/etc/forge/gui.json << 'J2'
{"theme":"Forge Default"}
J2
cat > /tmp/forge-rootfs/etc/forge/llm.json << 'J3'
{"model_path":"/opt/forge/models/forge-v1.gguf","context_size":8192,"threads":4,"gpu_layers":0,"bind_address":"127.0.0.1","bind_port":8080}
J3
cat > /tmp/forge-rootfs/etc/forge/guard.json << 'J4'
{"max_attempts":5,"lockout_secs":300,"critical_ops":["sys_shutdown","sys_reboot","fs_delete","fs_move","proc_kill"],"audit_log":"/var/log/forge/audit.log","usb_policy":"ask","sign_policy":"warn"}
J4
cat > /tmp/forge-rootfs/etc/forge/usb_whitelist.conf << 'W1'
# Forge OS USB Whitelist
# Format: vendor_id:product_id
# Common HID devices (keyboard, mouse)
046d:c000
046d:c001
046d:c002
046d:c003
046d:c004
046d:c005
W1

cp -r "$F/forge-gui/themes/default" /tmp/forge-rootfs/opt/forge/themes/builtin/ 2>/dev/null || true
cp "$HOME/model-downloader.sh" /tmp/forge-rootfs/usr/bin/forge-model-dl 2>/dev/null || true
chmod +x /tmp/forge-rootfs/usr/bin/forge-model-dl 2>/dev/null || true

mksquashfs /tmp/forge-rootfs "$O/live/filesystem.squashfs" -comp xz -noappend
echo "SquashFS: $(du -sh "$O/live/filesystem.squashfs" | cut -f1)"

cat > "$O/boot/grub/grub.cfg" << 'GRUBEOF'
set default=0
set timeout=5
menuentry "Forge OS v1.2" { linux /live/vmlinuz boot=live quiet; initrd /live/initrd }
GRUBEOF

echo "Building EFI boot image..."
dd if=/dev/zero of="$O/EFI/efiboot.img" bs=1K count=4096 2>/dev/null
mkfs.vfat "$O/EFI/efiboot.img" 2>/dev/null || true

ISO="$HOME/forge-os-v1.2.0-$(date +%Y%m%d-%H%M).iso"
xorriso -as mkisofs -iso-level 3 -full-iso9660-filenames -volid "FORGE_OS" -output "$ISO" -eltorito-boot boot/grub/grub.cfg -no-emul-boot -eltorito-alt-boot -e EFI/efiboot.img -no-emul-boot -isohybrid-gpt-basdat "$O" 2>&1

echo "===== ISO v1.2 Ready ====="
ls -lh "$ISO"