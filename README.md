# ⚙️ Rustcat ⚙️ ![Workflow](https://github.com/robiot/rustcat/actions/workflows/rust.yml/badge.svg) [![dependency status](https://deps.rs/crate/rustcat/0.0.2/status.svg)](https://deps.rs/crate/rustcat/0.0.2)

**-- Basic Netcat Alternative --**

<img src="https://raw.githubusercontent.com/robiot/rustcat/main/.github/assets/example.png"/>

About
------------------------
Rustcat is a port listener that can be used for different purposes.\
It is basically like netcat but with fewer options

Why use Rustcat?
------------------------
* Serves it purpose of listening to ports
* Has command history
* It is easy to use
* Supports udp
* Uses colors

Installation
------------------------
### Debian/Ubuntu
```
wget https://github.com/robiot/rustcat/releases/latest/download/rustcat_amd64.deb
sudo apt install ./rustcat_amd64.deb
```
### Arch
```
git clone https://aur.archlinux.org/rustcat.git
cd rustcat
makepkg -si
```
Or with yay:
```
yay -S rustcat
```
### Other Distributions
To install from crates.io:
```
cargo install rustcat
```
To install the latest github release without compiling yourself:
```
bash <(curl -s https://raw.githubusercontent.com/robiot/rustcat/main/install.sh)
```
*If you want it on windows you need to remove everything related to termion and rustyline from the source*

Compiling yourself
------------------------
To compile yourself
1. Download the source
2. Cd into the dir and run...
```
cargo build --release
```

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
Create a reverse shell on port **55660**:
```
rc -rp 55660
```
