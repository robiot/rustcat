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
Rustcat is a modern port listener / Reverse Shell that is very easy to use.

Why use Rustcat?
------------------------
* Serves it purpose of listening to ports
* Has command history
* It is easy to use
* Supports udp
* Uses colors


Usage
------------------------
```
rc [options] [destination] [port]
```

Usage Examples
------------------------

Help :
```
rc --help
```
Listen to port **55660** on localhost :
```
rc -lp 55660
```
Listen to port **55660** on localhost with command history :
```
rc -lpH 55660
```
Listen to port **55660** on localhost udp :
```
rc -lpu 55660
```
Listen to port **55660** on specified ip (192.168.1.10) :
```
rc -l 192.168.1.10 55660
```
Create a bash reverse shell on port **55660**:
```
rc -rp 55660 bash
```
