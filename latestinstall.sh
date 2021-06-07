#!/bin/bash

main(){
    echo "Downloading Rustcat"
    which wget >/dev/null && echo "Wget installed, moving on..." || installwget
    which unzip >/dev/null && echo "Unzip is installed, moving on..." || installunzip
    which cargo >/dev/null && echo "Cargo is installed, moving on..." || installrust
    cd /tmp
    sudo rm -rf ./rustcat-main
    wget https://github.com/robiot/rustcat/archive/refs/heads/main.zip

    echo "Unzipping!"
    
    unzip ./main.zip
    rm -rf ./main.zip
    cd ./rustcat-main
    
    echo "Building!"
    cargo build --release
    sudo mkdir -p /usr/local/bin/
    sudo cp -f ./target/release/rustcat /usr/local/bin/rc
    sudo chmod 755 /usr/local/bin/rc
    cd ..
    rm -rf rustcat-main

    echo "Rustcat sucessfully installed in /usr/local/bin/rc | Run with 'rc"
}

installwget() {
    which apt >/dev/null 2>&1 && sudo apt install wget
    which pacman >/dev/null 2>&1 && sudo pacman -S wget
    which zypper >/dev/null 2>&1 && sudo zypper install wget
    which dnf >/dev/null 2>&1 && sudo dnf install wget
    which xbps-install >/dev/null 2>&1 && sudo xbps-install -S wget
    which eopkg >/dev/null 2>&1 && sudo eopkg install wget
    which emerge >/dev/null 2>&1 && sudo emerge net-misc/wget
}

installunzip() {
    which apt >/dev/null 2>&1 && sudo apt install unzip
    which pacman >/dev/null 2>&1 && sudo pacman -S unzip
    which zypper >/dev/null 2>&1 && sudo zypper install unzip
    which dnf >/dev/null 2>&1 && sudo dnf install unzip
    which xbps-install >/dev/null 2>&1 && sudo xbps-install -S unzip
    which eopkg >/dev/null 2>&1 && sudo eopkg install unzip
    which emerge >/dev/null 2>&1 && sudo emerge app-arch/unzip
}

installrust() {
    which apt >/dev/null 2>&1 && sudo apt install rustup
    which pacman >/dev/null 2>&1 && sudo pacman -S rustup
    which zypper >/dev/null 2>&1 && sudo zypper install rustup
    which dnf >/dev/null 2>&1 && sudo dnf install rustup
    which xbps-install >/dev/null 2>&1 && sudo xbps-install -S rustup
    which eopkg >/dev/null 2>&1 && sudo eopkg install rustup
    which emerge >/dev/null 2>&1 && sudo emerge dev-lang/rust-bin
}
main