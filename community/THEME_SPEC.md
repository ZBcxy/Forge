# Forge OS Theme Specification v1.0

## What is a Forge Theme?

A Forge theme is a directory containing a `theme.json` manifest and optional `assets/`.
Themes control colors, fonts, layout, animations, and sounds.

## Directory Structure

```
my-theme/
├── theme.json       # Required — theme manifest
├── assets/          # Optional — images, fonts, sounds
│   ├── wallpaper.png
│   ├── logo.svg
│   └── sounds/
│       ├── startup.ogg
│       └── notification.ogg
├── preview.png      # Recommended — 1280x720 screenshot
└── README.md        # Recommended — description, credits, license
```

## theme.json Format

```json
{
  "meta": {
    "name": "My Theme",           // Required — display name
    "author": "Your Name",        // Required
    "version": "1.0.0",           // Required — semver
    "description": "...",         // Recommended
    "homepage": "https://...",    // Optional
    "license": "MIT"              // Recommended
  },
  "colors": {
    "background": "#0a0a0f",      // Main background
    "surface": "#141420",         // Cards, panels
    "surface_alt": "#1a1a2e",     // Elevated panels
    "primary": "#6c63ff",         // Brand color
    "secondary": "#3b82f6",       // Secondary accent
    "accent": "#f59e0b",          // Highlights
    "danger": "#ef4444",          // Errors, destructive actions
    "on_primary": "#ffffff",      // Text on primary
    "on_background": "#e0e0e0",   // Body text
    "on_surface": "#c0c0d0",      // Muted text
    "border": "#2a2a3a"           // Dividers, outlines
  },
  "fonts": {
    "family": "Inter",            // UI font
    "size": 14,                   // Base size in px
    "weight": "regular",          // regular | medium | bold
    "mono_family": "JetBrains Mono",
    "mono_size": 13
  },
  "layout": {
    "padding": 12,                // Component padding in px
    "radius_px": 8,               // Border radius
    "gap_px": 8,                  // Spacing between elements
    "sidebar_width": 260,         // Sidebar width in px
    "topbar_height": 40           // Top bar height in px
  },
  "animations": {
    "enabled": true,
    "duration_ms": 200,           // Transition duration
    "easing": "ease-out"          // ease-in | ease-out | linear
  },
  "sounds": {
    "enabled": true,
    "volume": 0.7                 // 0.0 - 1.0
  }
}
```

## Installing a Theme

### Method 1: Drop folder
Copy your theme folder to `/opt/forge/themes/community/`:
```bash
sudo cp -r my-theme /opt/forge/themes/community/
```

### Method 2: User themes (no sudo)
Copy to `~/.forge/themes/`:
```bash
cp -r my-theme ~/.forge/themes/
```

### Method 3: .forge-theme package
Package and install:
```bash
forge theme pack my-theme          # creates my-theme.forge-theme
forge theme install my-theme.forge-theme
```

## Selecting a Theme

Edit `/etc/forge/gui.json` or run:
```bash
forge theme set "My Theme"
```

## Publishing

To share your theme with the community:
1. Zip your theme directory
2. Post on GitHub/GitLab with tag `forge-os-theme`
3. Submit to the community registry (see REGISTRY.md)