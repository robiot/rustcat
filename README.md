# ⚙️ Rustcat ⚙️ ![Workflow](https://github.com/robiot/rustcat/actions/workflows/rust.yml/badge.svg)

**-- Basic Netcat Alternative --**

<img src="https://raw.githubusercontent.com/robiot/rustcat/main/.github/assets/example.png" style="border-radius:0.43rem"/>

About
------------------------
Rustcat is a port listener that can be used for different purposes.\
It is basically like netcat but with fewer options

Why use Rustcat?
------------------------
* Serves it purpose of listening to ports
* It is easy to use
* Uses colors

Installation
------------------------
To install the latest release you run
```
bash <(curl -s https://raw.githubusercontent.com/robiot/rustcat/main/install.sh)
```

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
Listen to port **55660** on specified ip (192.168.1.10) :
```
rc -l 192.168.1.10 55660
```
