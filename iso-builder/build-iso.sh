# Forge OS ISO Builder
#
# Prerequisites:
#   - Rust toolchain (rustup)
#   - mkosi (pip install mkosi)
#   - xorriso (apt install xorriso)
#   - sudo access
#
# Usage:
#   chmod +x build-iso.sh
#   ./build-iso.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
FORGE_ROOT="$(dirname "$SCRIPT_DIR")"
BUILD_DIR="$FORGE_ROOT/target/release"
ISO_OUT_DIR="$SCRIPT_DIR/output"
OVERLAY_DIR="$SCRIPT_DIR/overlay"

echo "===== Forge OS ISO Builder ====="
echo "Project root: $FORGE_ROOT"
echo ""

# ── 1. Build all Forge Rust binaries ──
echo "[1/5] Building Forge binaries..."
cd "$FORGE_ROOT"
cargo build --release

# Copy binaries to overlay
mkdir -p "$OVERLAY_DIR/usr/bin"
cp "$BUILD_DIR/forge-init"   "$OVERLAY_DIR/usr/bin/"
cp "$BUILD_DIR/forge-svc"    "$OVERLAY_DIR/usr/bin/"
cp "$BUILD_DIR/forge-hal"    "$OVERLAY_DIR/usr/bin/"
cp "$BUILD_DIR/forge-llm"    "$OVERLAY_DIR/usr/bin/"
cp "$BUILD_DIR/forge-voice"  "$OVERLAY_DIR/usr/bin/"
cp "$BUILD_DIR/forge-agent"  "$OVERLAY_DIR/usr/bin/"
cp "$BUILD_DIR/forge-gui"    "$OVERLAY_DIR/usr/bin/"
strip "$OVERLAY_DIR/usr/bin/"* 2>/dev/null || true
echo "   Binaries installed to overlay/usr/bin/"

# ── 2. Copy config files ──
echo "[2/5] Installing configuration..."
mkdir -p "$OVERLAY_DIR/etc/forge"
mkdir -p "$OVERLAY_DIR/opt/forge/models"
mkdir -p "$OVERLAY_DIR/opt/forge/themes/builtin"
mkdir -p "$OVERLAY_DIR/opt/forge/themes/community"
mkdir -p "$OVERLAY_DIR/opt/forge/widgets"
mkdir -p "$OVERLAY_DIR/run/forge"

# Default configs
cat > "$OVERLAY_DIR/etc/forge/llm.json" << 'EOF'
{"model_path":"/opt/forge/models/forge-v1.gguf","context_size":8192,"threads":4,"gpu_layers":0,"bind_address":"127.0.0.1","bind_port":8080,"temperature":0.7,"top_p":0.95,"max_tokens":2048}
EOF

cat > "$OVERLAY_DIR/etc/forge/gui.json" << 'EOF'
{"theme": "Forge Default"}
EOF

# init config — tell forge-init that forge-svc is the init child
cat > "$OVERLAY_DIR/etc/forge/init.json" << 'EOF'
{"svc_binary": "/usr/bin/forge-svc"}
EOF

# Copy default theme
cp -r "$FORGE_ROOT/forge-gui/themes/default" "$OVERLAY_DIR/opt/forge/themes/builtin/"
echo "   Config OK"

# ── 3. Create initramfs hook ──
echo "[3/5] Creating initramfs configuration..."
mkdir -p "$OVERLAY_DIR/etc/initramfs-tools/scripts/init-bottom"
cat > "$OVERLAY_DIR/etc/initramfs-tools/scripts/init-bottom/forge" << 'SCRIPT'
#!/bin/sh
echo "Forge OS booting..."
mount -t proc proc /proc
mount -t sysfs sysfs /sys
mount -t devtmpfs devtmpfs /dev
exec /usr/bin/forge-init
SCRIPT
chmod +x "$OVERLAY_DIR/etc/initramfs-tools/scripts/init-bottom/forge"
echo "   Initramfs hook OK"

# ── 4. Build image with mkosi ──
echo "[4/5] Building OS image with mkosi..."
cd "$SCRIPT_DIR"
sudo mkosi --force --output forge-os --output-dir "$ISO_OUT_DIR"

# ── 5. Create hybrid ISO ──
echo "[5/5] Creating bootable hybrid ISO..."
# Extract kernel and initrd from the disk image
# Convert disk image to hybrid ISO (bootable on BIOS + UEFI)
#
# This step mounts the mkosi output and creates an ISO9660 filesystem
# with El Torito boot for BIOS and ESP for UEFI

ISO_PATH="$ISO_OUT_DIR/forge-os-$(date +%Y%m%d).iso"

if command -v xorriso &> /dev/null; then
    # We need a working directory with the kernel, initrd, and rootfs
    ISO_TMP="$ISO_OUT_DIR/iso-tmp"
    rm -rf "$ISO_TMP"
    mkdir -p "$ISO_TMP/live"

    # Copy the rootfs from mkosi output
    # (In a full implementation, this mounts the mkosi disk image
    #  and copies the kernel + initrd + squashfs to ISO_TMP)

    # Create ISO with UEFI + BIOS dual boot
    xorriso -as mkisofs \
        -iso-level 3 \
        -full-iso9660-filenames \
        -volid "FORGE_OS" \
        -eltorito-boot isolinux/isolinux.bin \
        -eltorito-catalog isolinux/boot.cat \
        -no-emul-boot -boot-load-size 4 -boot-info-table \
        -eltorito-alt-boot -e images/efiboot.img -no-emul-boot \
        -output "$ISO_PATH" \
        "$ISO_TMP"

    echo "   ISO created: $ISO_PATH"
else
    echo "   WARNING: xorriso not found. Install: sudo apt install xorriso"
    echo "   Raw disk image at: $ISO_OUT_DIR/forge-os.image"
fi

echo ""
echo "===== Build Complete ====="
echo "ISO: $ISO_PATH"
echo "Raw: $ISO_OUT_DIR/forge-os.image"
