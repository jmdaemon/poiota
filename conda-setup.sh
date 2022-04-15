#!/bin/bash

# Make sure conda/mamba is initialized first
mamba=$(command -v mamba)
conda="conda"
if [[ ! -z "$mamba" ]]; then
    conda="mamba"
fi

$conda create --name poiota
$conda activate poiota
$conda install -c conda-forge --name poiota pyrosm
$conda install -c conda-forge --name poiota python-language-server
