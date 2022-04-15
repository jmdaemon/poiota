#!/bin/bash

# Make sure conda/mamba is initialized first
mamba=$(command -v mamba)
if [[ ! -z "$mamba" ]]; then
    mamba create --name poiota
    mamba activate poiota
    mamba install -c conda-forge pyrosm
    mamba install -c conda-forge python-language-server
else
    conda create --name poiota
    conda activate poiota
    conda install -c conda-forge pyrosm
    conda install -c conda-forge python-language-server
fi
