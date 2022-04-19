#!/bin/sh

# Install deconz (stable)
sudo gpasswd -a $USER dialout
wget -O - http://phoscon.de/apt/deconz.pub.key | \
           sudo apt-key add -

sudo sh -c "echo 'deb http://phoscon.de/apt/deconz \
            $(lsb_release -cs) main' > \
            /etc/apt/sources.list.d/deconz.list"

sudo apt update
sudo apt install deconz

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo apt install libssl-dev