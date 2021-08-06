<div align="center" >
  <br>
  <img src="https://raw.githubusercontent.com/robiot/rustcat/main/.github/assets/banner.png">
  <br>
  
  <img alt="GitHub All Releases" src="https://img.shields.io/github/contributors/robiot/rustcat?label=Contributors">
  <img alt="GitHub Latest Release" src="https://img.shields.io/github/v/tag/robiot/rustcat?label=Latest%20Release">
  <img alt="AUR version" src="https://img.shields.io/aur/version/rustcat">
  <img alt="Crates.io" src="https://img.shields.io/crates/d/rustcat?label=Cargo%20Downloads">
  <img alt="GitHub All Releases" src="https://img.shields.io/github/downloads/robiot/rustcat/total?label=GitHub%20Downloads">
  <img alt="Build Stats" src="https://github.com/robiot/rustcat/actions/workflows/rust.yml/badge.svg">
  <hr>
</div>

| <p align="center"><a href="https://crates.io/crates/rustcat">:package: Cargo</a></p>            | <p align="center"><a href="https://github.com/robiot/rustcat/releases">:cyclone: Kali / Debian </p>                 | <p align="center"><a href="https://aur.archlinux.org/packages/rustcat/">🏗️ Arch </a></p>                       | <p align="center"><a href="https://github.com/robiot/rustcat/releases">:computer: Other </a></p>                     |
| ----------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| <p align="center"><img src="https://raw.githubusercontent.com/robiot/rustcat/main/.github/assets/cargo.png" width=200/></p> | <p align="center"><img src="https://raw.githubusercontent.com/robiot/rustcat/main/.github/assets/kali.png" /></p> | <p align="center"><img src="https://raw.githubusercontent.com/robiot/rustcat/main/.github/assets/arch.png" /></p> | <p align="center"><img src="https://raw.githubusercontent.com/robiot/rustcat/main/.github/assets/others.png" width=200/></p> |
| `cargo install rustcat` | [Read the install guide](https://github.com/robiot/rustcat/wiki/Installation-Guide#kalidebian)     | `yay -S rustcat`                                                                                               |  [Read the install guide](https://github.com/robiot/rustcat/wiki/Installation-Guide#other)     
<hr>

# :thinking: What Is Rustcat?
![Ex](.github/assets/usage-ex.gif)
:art: Rustcat is a modern port listener & Reverse Shell that is very easy to use.

# :sparkles: Features
* Listen on ports
* Command history
* Reverse shell
* Udp
* Colors

# :gear: Why Rustcat?
Rustcat is a modern port listener that is easier to use and understand than most of the older ones on the market.

## :baby: Everything Easy
![Ex](.github/assets/easy-revshell.gif)
Starting a listener is just as simple as `rc -lp port`. While in netcat you would do something like `nc -nlvp port`.

And creating a reverse shell has never been easier, `rc -rp port shell` is all you need. No more long piping stdin, stdout, stderr from /dev/tcp into /bin/shell that you always forget.

## :page_with_curl: Command History
![Appearance](.github/assets/history-ex.gif)
Command history is something we all need. How annoying isn't it when you accidently type a long command wrong, then you have to rewrite the entire thing.

But Rustcat fixes that with command history. Just add -H and you will get command history (`rc -lpH port`), unlike netcat where you have to use another application and do `rlwrap -cAr nc -nlvp port`.

## :art: Appearance
![Appearance](.github/assets/easy.png)

A little splash of color doesn't hurt :).

# :book: Full Installation Guide
You can find the guide [here](https://github.com/robiot/rustcat/wiki/Installation-Guide)

# :bulb: Usage
A basic usage guide can be found [here](https://github.com/robiot/rustcat/wiki/Basic-Usage)
