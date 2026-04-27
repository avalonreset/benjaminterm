#!/usr/bin/env bash
set -euo pipefail
set -x

TARGET_DIR=${1:-target}
TAG_NAME=${TAG_NAME:-$(git -c "core.abbrev=8" show -s "--format=%cd-%h" "--date=format:%Y%m%d-%H%M%S")}

zipdir="benterm-macos-$TAG_NAME"
zipname="$zipdir.zip"
appdir="$zipdir/benterm.app"

rm -rf "$zipdir" "$zipname"
mkdir -p "$zipdir"
cp -r assets/macos/WezTerm.app "$appdir"

rm -f "$appdir"/*.dylib
mkdir -p "$appdir/Contents/MacOS"
mkdir -p "$appdir/Contents/Resources"
mkdir -p "$appdir/Contents/MacOS/fonts"
mkdir -p "$appdir/Contents/MacOS/sounds"

cp -r assets/shell-integration/* "$appdir/Contents/Resources"
cp -r assets/shell-completion "$appdir/Contents/Resources"
cp extras/benterm/benterm.lua "$appdir/Contents/MacOS/wezterm.lua"
cp -r assets/fonts/* "$appdir/Contents/MacOS/fonts/"
cp -r assets/sounds/benterm-soft-cues "$appdir/Contents/MacOS/sounds/"
tic -xe wezterm -o "$appdir/Contents/Resources/terminfo" termwiz/data/wezterm.terminfo

for bin in wezterm wezterm-mux-server wezterm-gui strip-ansi-escapes; do
  case "$bin" in
    wezterm) dest=benterm ;;
    wezterm-gui) dest=benterm-gui ;;
    wezterm-mux-server) dest=benterm-mux-server ;;
    *) dest=$bin ;;
  esac

  if [[ -f "$TARGET_DIR/release/$bin" ]]; then
    cp "$TARGET_DIR/release/$bin" "$appdir/Contents/MacOS/$dest"
  else
    lipo "$TARGET_DIR"/*/release/$bin -output "$appdir/Contents/MacOS/$dest" -create
  fi
 done

set +x
if [[ -n "${MACOS_TEAM_ID:-}" ]]; then
  MACOS_PW=$(echo "$MACOS_CERT_PW" | base64 --decode)
  def_keychain=$(eval echo $(security default-keychain -d user))
  security delete-keychain build.keychain || true
  security create-keychain -p "$MACOS_PW" build.keychain
  security default-keychain -d user -s build.keychain
  security unlock-keychain -p "$MACOS_PW" build.keychain
  echo "$MACOS_CERT" | base64 --decode > /tmp/certificate.p12
  security import /tmp/certificate.p12 -k build.keychain -P "$MACOS_PW" -T /usr/bin/codesign
  rm /tmp/certificate.p12
  security set-key-partition-list -S apple-tool:,apple:,codesign: -s -k "$MACOS_PW" build.keychain
  /usr/bin/codesign --keychain build.keychain --force --options runtime \
    --entitlements ci/macos-entitlement.plist --deep --sign "$MACOS_TEAM_ID" "$appdir/"
  security default-keychain -d user -s "$def_keychain"
  security delete-keychain build.keychain || true
fi

set -x
zip -r "$zipname" "$zipdir"
set +x

if [[ -n "${MACOS_TEAM_ID:-}" ]]; then
  xcrun notarytool submit "$zipname" --wait --team-id "$MACOS_TEAM_ID" --apple-id "$MACOS_APPLEID" --password "$MACOS_APP_PW"
fi
