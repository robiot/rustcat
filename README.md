## ⚙️ Rustcat ⚙️

## -- Basic Netcat Alternative -- 

<img src="./imgassets/example.png" style="border-radius:0.43rem"/>
<br />
<br />
About
------------------------
Rustcat is a port listener that can be used for different purposes.\
It is basically like netcat but with fewer options

<br />

Why use Rustcat?
------------------------
* Serves it purpose of listening to ports
* It is easy to use
* Uses colors

<br />

Installation
------------------------
You can install rustcat from the [Releases](./releases/latest) tab and copy it to /usr/bin (optional)
```
cp ./rc /usr/bin
```

<br />

Compiling yourself
------------------------
To compile yourself
1. Download the source
2. Cd into the directory and run...
```
cargo build --release
```
3. Go into ./target/release and here you have "rc"
4. (Optional) copy it to /usr/bin
```
cp ./rc /usr/bin
```

<br />

Usage
------------------------
```
rc [options] [destination] [port]
```

<br />

Usage Examples
------------------------

Help :
```
rc --help
```
Listen to port 55660 on localhost :
```
rc -lp 55660
```
Listen to port 55660 on specified ip (192.168.1.10) :
```
rc -l 192.168.1.10 55660
```