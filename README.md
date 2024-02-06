# Stealth Freedom
[![Continuous Integration](https://github.com/StealthFreedom/StealthFreedom/actions/workflows/ci.yml/badge.svg)](https://github.com/StealthFreedom/StealthFreedom/actions/workflows/ci.yml)

<img src="https://raw.githubusercontent.com/StealthFreedom/StealthFreedom/main/data/icons/io.github.StealthFreedom.StealthFreedom.svg" width="128" height="128" />
<p><strong>Get untraceable access to the Internet without censorship</strong></p>

Stealth Freedom helps you bypass government censorship by using the VLESS proxy protocol. VLESS masks your network activity as a regular web browser connection, so the government thinks you are just browsing the web. Under the hood, Stealth Freedom uses two podman containers: [crosssection](https://github.com/StealthFreedom/crosssection) and [sing-box](https://github.com/StealthFreedom/sing-box).

## Features
- Support for VLESS (with XTLS-Reality) connection links

## Screenshots
| ![Main window](https://raw.githubusercontent.com/StealthFreedom/StealthFreedom/main/data/resources/screenshots/en/main_window.png) |
| :--------------------------------------------------------------------------------------------------------------------------------: |
|                                                           *Main window*                                                            |

## Support
To run Stealth Freedom, your system must have:
- systemd
- CGroupsV2 (unified_cgroup_hierarchy)
- podman 4.8.0 or newer

All of these things are available in at least the following distributions:
- Fedora 38+
- CentOS Stream 9
- Arch Linux
- Gentoo Linux
- openSUSE Tumbleweed
- Nix unstable

Your issue can be closed or moved to discussions if:
- It's without debug info (you can get in in About > Troubleshooting > Debugging Information)
- It's not written in English
- Stealth Freedom not installed as a flatpak
- Stealth Freedom version is outdated
- It's a question. Please use [discussions](https://github.com/StealthFreedom/StealthFreedom/discussions) for questions

## Hack on Stealth Freedom
### GNOME Builder
To build the development version of Stealth Freedom and hack on the code see
the [general guide](https://builder.readthedocs.io/projects/index.html) for building
GNOME apps with Flatpak and GNOME Builder.

### VS Codium / VS Code
1. Install [flatpak-vscode](https://github.com/bilelmoussaoui/flatpak-vscode#download) extension
2. Install rust-analyzer from [Open VSX](https://open-vsx.org/extension/rust-lang/rust-analyzer) or [Visual Studio Marketplace](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
3. Build and run application (`Ctrl-Alt-B`)

### CLI / Other
#### Build
```
flatpak install org.gnome.Platform//45 org.gnome.Sdk//45 org.freedesktop.Sdk.Extension.rust-stable//23.08 org.freedesktop.Sdk.Extension.llvm16//23.08
flatpak-builder --state-dir=.flatpak/flatpak-builder --user .flatpak/repo build-aux/io.github.StealthFreedom.StealthFreedom.Devel.json
```

#### Run
```
flatpak-builder --run .flatpak/repo build-aux/io.github.StealthFreedom.StealthFreedom.Devel.json stealth_freedom
```

## Translating Stealth Freedom
Edit `po/<lang>.po` with your favourite translating tool

### How to add new language
1. Add language to [LINGUAS](./po/LINGUAS)
2. [Setup build dir](#setup-build-dir) and [Update po files](#update-po-files)

### For devs
Don't forget to make new strings translatable:
1. [Setup build dir](#setup-build-dir)
2. [Regenerate pot file](#regenerate-pot-file)
3. [Update po files](#update-po-files)

### Commands
#### Setup build dir
```
meson setup _po
```

#### Regenerate pot file
```
meson compile -C _po stealth_freedom-pot
```

#### Update po files
```
meson compile -C _po stealth_freedom-update-po
```

## Donate
TODO

## License
Stealth Freedom is distributed under the terms of the MIT License. See [LICENSE](./LICENSE) file for details.

## Credits
- [gtk-rust-template](https://gitlab.gnome.org/World/Rust/gtk-rust-template)
