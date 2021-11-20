# rustcat - A modern port listener and reverse shell.

<div>
  <img alt="Contributors" src="https://img.shields.io/github/contributors/robiot/rustcat?label=Contributors">
  <img alt="AUR version" src="https://img.shields.io/aur/version/rustcat">
  <img alt="Crates.io" src="https://img.shields.io/crates/d/rustcat?label=Cargo%20Downloads">
  <img alt="GitHub All Releases" src="https://img.shields.io/github/downloads/robiot/rustcat/total?label=GitHub%20Downloads">
  <img alt="Build Stats" src="https://github.com/robiot/rustcat/actions/workflows/rust.yml/badge.svg">
</div>

[Installation](https://github.com/robiot/rustcat#installation-guide) • [Usage](https://github.com/robiot/rustcat/wiki/Basic-Usage)

# Features
* Listen on ports
* Command history & Tab completion
* Reverse shell
* Udp
* Colors

# Why Rustcat?

## Everything Easy
![Ex](https://raw.githubusercontent.com/robiot/rustcat/main/img/easy-revshell.gif)

Starting a listener is just as simple as `rcat -lp port`. While in netcat you would do something like `nc -nlvp port` to get the same results.

And creating a reverse shell has never been easier, `rcat host port -r shell` is all you need. No more long /dev/tcp stuff that you always forget.

## Command History
![Ex](https://raw.githubusercontent.com/robiot/rustcat/main/img/history.gif)

Command history is something we all need. How annoying isn't it when you accidently type a long command wrong, then you have to rewrite the entire thing.

But Rustcat fixes that with command history. Just add -H and you will get command history (`rcat -lHp port`) and command completion, unlike netcat where you have to use another application and do `rlwrap -cAr nc -nlvp port` to get the same results.

When adding -H, the shell will almost work like a ssh shell.

## Appearance
A little splash of color doesn't hurt :).

## Cross Platform
Rustcat supports unix and windows systems.

## Installation

| **[Cargo][cargo_lnk]**                | **[Arch Linux][arch_lnk]**            | **[BlackArch][barch_lnk]**            | **[Debian/Kali][kali_lnk]**
|:-------------------------------------:|:-------------------------------------:|:-------------------------------------:|:-------------------------------------:|
| [![cargo_img]][cargo_lnk]             | [![arch_img]][arch_lnk]               | [![barch_img]][barch_lnk]             | [![kali_img]][kali_lnk]               |
| `cargo install rustcat`               | `yay -S rustcat`                      | `pacman -S rustcat`                   | [Read the install guide][debian_guide]|
| **[Funtoo][funtoo_lnk]**              | **[Other][other_lnk]**                |                                       |                                       |
| [![funtoo_img]][funtoo_lnk]           | [![other_img]][other_lnk]             |                                       |                                       |
| `emerge rustcat`                      | [Read the install guide][other_guide] |                                       |                                       |

[cargo_lnk]: https://crates.io/crates/rustcat
[cargo_img]: https://raw.githubusercontent.com/robiot/rustcat/main/img/icons/cargo.png

[arch_lnk]: https://aur.archlinux.org/packages/rustcat/
[arch_img]: https://raw.githubusercontent.com/robiot/rustcat/main/img/icons/arch.png

[barch_lnk]: https://www.blackarch.org/tools.html
[barch_img]: https://raw.githubusercontent.com/robiot/rustcat/main/img/icons/blackarch.png

[kali_lnk]: https://github.com/robiot/rustcat
[kali_img]: https://raw.githubusercontent.com/robiot/rustcat/main/img/icons/kali.png
[debian_guide]: https://github.com/robiot/rustcat/wiki/Installation-Guide#kalidebian

[funtoo_lnk]: https://github.com/funtoo/net-kit/tree/1.4-release/net-analyzer/rustcat
[funtoo_img]: https://raw.githubusercontent.com/robiot/rustcat/main/img/icons/funtoo.png

[other_lnk]: https://github.com/robiot/rustcat
[other_img]: https://raw.githubusercontent.com/robiot/rustcat/main/img/icons/others.png
[other_guide]: https://github.com/robiot/rustcat/wiki/Installation-Guide
<hr>

You can find more information [here](https://github.com/robiot/rustcat/wiki/Installation-Guide)

## Usage
A basic usage guide can be found [here](https://github.com/robiot/rustcat/wiki/Basic-Usage)
