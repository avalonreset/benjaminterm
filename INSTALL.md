# Install BenjaminTerm

Official downloads are published on GitHub Releases:

https://github.com/avalonreset/BenjaminTerm/releases/latest

BenjaminTerm does not require Homebrew, WinGet, Flathub, Linuxbrew, or any other package manager. Package-manager channels are out of scope for the current release. The official release path is the GitHub release artifact for your platform.

## Windows

Download:

- `BenjaminTerm-v1.4.2-setup.exe`

Run the installer. Use the portable zip only if you specifically want a folder you can unpack and run without installation:

- `BenjaminTerm-windows-v1.4.2.zip`

## macOS

Download:

- `BenjaminTerm-macos-v1.4.2.zip`

Install:

```sh
unzip BenjaminTerm-macos-v1.4.2.zip
mv BenjaminTerm-macos-v1.4.2/BenjaminTerm.app /Applications/
open /Applications/BenjaminTerm.app
```

If macOS blocks the app because it is not notarized yet, remove the download quarantine flag and open it again:

```sh
xattr -dr com.apple.quarantine /Applications/BenjaminTerm.app
open /Applications/BenjaminTerm.app
```

## Linux

Download:

- `BenjaminTerm-linux-v1.4.2.tar.gz`

Run from the unpacked folder:

```sh
tar -xzf BenjaminTerm-linux-v1.4.2.tar.gz
cd BenjaminTerm-linux-v1.4.2
./BenjaminTerm-gui
```

Optional user-local install:

```sh
mkdir -p "$HOME/.local/opt" "$HOME/.local/bin"
tar -xzf BenjaminTerm-linux-v1.4.2.tar.gz -C "$HOME/.local/opt"
ln -sf "$HOME/.local/opt/BenjaminTerm-linux-v1.4.2/BenjaminTerm-gui" "$HOME/.local/bin/benjaminterm"
benjaminterm
```

Make sure `$HOME/.local/bin` is on your `PATH` if the `benjaminterm` command is not found.

## Checksums

Each release artifact has a matching `.sha256` file on the GitHub release page.

On macOS or Linux:

```sh
shasum -a 256 BenjaminTerm-macos-v1.4.2.zip
shasum -a 256 BenjaminTerm-linux-v1.4.2.tar.gz
```

On Windows PowerShell:

```powershell
Get-FileHash .\BenjaminTerm-v1.4.2-setup.exe -Algorithm SHA256
```
