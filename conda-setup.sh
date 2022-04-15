#!/bin/bash

# Make sure conda/mamba is initialized first
mamba=$(command -v mamba)
if [[ ! -z "$mamba" ]]; then
    mamba create --name poiota
    mamba activate poiota
    mamba install -c conda-forge pyrosm
else
    mamba create --name poiota
    mamba activate poiota
    mamba install -c conda-forge pyrosm
fi
