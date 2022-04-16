#!/bin/bash

openmaptiles_tools_installed=$(command -v download-osm)
if [[ -z "$openmaptiles_tools_installed" ]]; then
    # Install download-osm script
    python3 -m pip install git+https://github.com/openmaptiles/openmaptiles-tools
fi

# Install dependencies with pacman
pacman=$(command -v pacman)
if [[ ! -z "$pacman" ]]; then
    sudo pacman -S graphviz sqlite3 aria2
    yay=$(command -v yay)
    if [[ ! -z "$yay" ]]; then
        yay -S osmconvert
    fi
fi

# Install dependencies with apt
apt=$(command -v apt)
if [[ ! -z "$apt" ]]; then
    sudo apt install -y graphviz sqlite3 aria2 osmctools
fi

mkdir -p maps & cd maps
download-osm geofabrik 'north-america/canada/british-columbia'
