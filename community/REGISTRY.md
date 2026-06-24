# Forge OS Community Theme Registry

## How to Submit

No backend server needed. The registry is a simple JSON file hosted on GitHub.

1. Create your theme (see THEME_SPEC.md)
2. Package it: `forge-theme pack my-theme`
3. Fork this repo, add your entry to `registry.json`, and submit a PR:

```json
{
  "themes": [
    {
      "id": "neon-nights",
      "name": "Neon Nights",
      "author": "YourName",
      "version": "1.0.0",
      "description": "A vibrant neon theme",
      "tags": ["dark", "neon", "purple"],
      "download_url": "https://github.com/yourname/neon-nights/releases/download/v1.0.0/neon-nights.forge-theme",
      "preview_url": "https://raw.githubusercontent.com/yourname/neon-nights/main/preview.png",
      "license": "MIT"
    }
  ]
}
```

## How Users Install

Users don't need to browse a website. They install directly from the CLI:

```bash
forge-theme install https://github.com/yourname/neon-nights/releases/download/v1.0.0/neon-nights.forge-theme
forge theme set "Neon Nights"
```

## Manual Sharing

Simplest way — just zip and share:
```bash
tar -czf my-theme.forge-theme my-theme/
# Send to friends via USB, email, or any file sharing service
# They install with:
forge-theme install my-theme.forge-theme
```

## Discovery Tags

Use these tags in your README/description so themes are searchable:
- `#forge-os-theme` on GitHub/GitLab
- Style: `dark` `light` `oled` `neon` `pastel` `monochrome` `nature`
- Mood: `minimal` `vibrant` `retro` `futuristic` `warm` `cool`
- Purpose: `high-contrast` `low-light` `colorblind-friendly` `compact`