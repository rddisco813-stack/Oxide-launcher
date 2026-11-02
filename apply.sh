#!/usr/bin/env bash
# Applies Oxide branding + test-mode login on top of upstream. Run from repo root.
set -e
cd upstream
# 1) Rename app
grep -rl --include=strings.xml 'app_name' . | xargs sed -i -E 's#(name="app_name">)[^<]+#\1Oxide Launcher#' || true
# 2) Swap icon
for f in $(find . -path '*res/mipmap-*' -name 'ic_launcher*.png'); do
  python3 -c "from PIL import Image;Image.open('../assets/logo.png').resize(Image.open('$f').size).save('$f')" 2>/dev/null || cp ../assets/ic_launcher_512.png "$f"
done
echo "Branding applied. See ../patches/README.md for login + Rust edition steps."
