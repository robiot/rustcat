#!/bin/bash
main(){
    echo "Installing Rustcat"
    which wget >/dev/null && echo "Wget installed, moving on..." || installwget
    cd /tmp
    sudo rm -rf /tmp/rc
    wget https://github.com/robiot/rustcat/releases/latest/download/rc
    sudo chmod +x rc
    sudo mkdir -p /usr/local/bin/
    sudo cp -f rc /usr/local/bin/rc
    sudo chmod 755 /usr/local/bin/rc
    cd ~/
    echo "Rustcat sucessfully installed in /usr/bin/rc | Run with 'rc"
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
main
