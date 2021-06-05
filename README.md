# ⚙️ Rustcat ⚙️
![Workflow](https://github.com/robiot/rustcat/actions/workflows/rust.yml/badge.svg)
<br />

**-- Basic Netcat Alternative --**

<img src="./imgassets/example.png" style="border-radius:0.43rem"/>

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
You can install rustcat from the [Releases](https://github.com/robiot/rustcat/releases/latest) tab and run
```
sudo chmod +x rc
```
Copy it to /usr/bin (optional)
```
cp ./rc /usr/bin
```

Compiling yourself
------------------------
To compile yourself
1. Download the source
2. Cd into the directory and run...
```
cargo build --release
```
3. Run 
```
sudo chmod ./install.sh
```
And
```
./install.sh
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
