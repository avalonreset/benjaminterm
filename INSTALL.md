# Install benjaminterm

Official downloads are published on GitHub Releases:

https://github.com/avalonreset/benjaminterm/releases/latest

benjaminterm does not require Homebrew, WinGet, Flathub, Linuxbrew, or any other package manager. Package-manager channels are out of scope for the current release. The official release path is the GitHub release artifact for your platform.

## Windows

Download:

- `benjaminterm-v1.4.3-setup.exe`

Run the installer. Use the portable zip only if you specifically want a folder you can unpack and run without installation:

- `benjaminterm-windows-v1.4.3.zip`

## macOS

Download:

- `benjaminterm-macos-v1.4.3.zip`

Install:

```sh
unzip benjaminterm-macos-v1.4.3.zip
mv benjaminterm-macos-v1.4.3/benjaminterm.app /Applications/
open /Applications/benjaminterm.app
```

If macOS blocks the app because it is not notarized yet, remove the download quarantine flag and open it again:

```sh
xattr -dr com.apple.quarantine /Applications/benjaminterm.app
open /Applications/benjaminterm.app
```

## Linux

Download:

- `benjaminterm-linux-v1.4.3.tar.gz`

Run from the unpacked folder:

```sh
tar -xzf benjaminterm-linux-v1.4.3.tar.gz
cd benjaminterm-linux-v1.4.3
./benjaminterm-gui
```

Optional user-local install:

```sh
mkdir -p "$HOME/.local/opt" "$HOME/.local/bin"
tar -xzf benjaminterm-linux-v1.4.3.tar.gz -C "$HOME/.local/opt"
ln -sf "$HOME/.local/opt/benjaminterm-linux-v1.4.3/benjaminterm-gui" "$HOME/.local/bin/benjaminterm"
benjaminterm
```

Make sure `$HOME/.local/bin` is on your `PATH` if the `benjaminterm` command is not found.

## Checksums

Each release artifact has a matching `.sha256` file on the GitHub release page.

On macOS or Linux:

```sh
shasum -a 256 benjaminterm-macos-v1.4.3.zip
shasum -a 256 benjaminterm-linux-v1.4.3.tar.gz
```

On Windows PowerShell:

```powershell
Get-FileHash .\benjaminterm-v1.4.3-setup.exe -Algorithm SHA256
```
