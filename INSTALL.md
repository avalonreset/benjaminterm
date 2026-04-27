# Install benterm

Official downloads are published on GitHub Releases:

https://github.com/avalonreset/benterm/releases/latest

benterm does not require Homebrew, WinGet, Flathub, Linuxbrew, or any other package manager. Package-manager channels are out of scope for the current release. The official release path is the GitHub release artifact for your platform.

## Windows

Download:

- `benterm-v2.0.0-setup.exe`

Run the installer. Use the portable zip only if you specifically want a folder you can unpack and run without installation:

- `benterm-windows-v2.0.0.zip`

## macOS

Download:

- `benterm-macos-v2.0.0.zip`

Install:

```sh
unzip benterm-macos-v2.0.0.zip
mv benterm-macos-v2.0.0/benterm.app /Applications/
open /Applications/benterm.app
```

If macOS blocks the app because it is not notarized yet, remove the download quarantine flag and open it again:

```sh
xattr -dr com.apple.quarantine /Applications/benterm.app
open /Applications/benterm.app
```

## Linux

Download:

- `benterm-linux-v2.0.0.tar.gz`

Run from the unpacked folder:

```sh
tar -xzf benterm-linux-v2.0.0.tar.gz
cd benterm-linux-v2.0.0
./benterm-gui
```

Optional user-local install:

```sh
mkdir -p "$HOME/.local/opt" "$HOME/.local/bin"
tar -xzf benterm-linux-v2.0.0.tar.gz -C "$HOME/.local/opt"
ln -sf "$HOME/.local/opt/benterm-linux-v2.0.0/benterm-gui" "$HOME/.local/bin/benterm"
benterm
```

Make sure `$HOME/.local/bin` is on your `PATH` if the `benterm` command is not found.

## Checksums

Each release artifact has a matching `.sha256` file on the GitHub release page.

On macOS or Linux:

```sh
shasum -a 256 benterm-macos-v2.0.0.zip
shasum -a 256 benterm-linux-v2.0.0.tar.gz
```

On Windows PowerShell:

```powershell
Get-FileHash .\benterm-v2.0.0-setup.exe -Algorithm SHA256
```
