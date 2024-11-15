#!/bin/sh

# Check if requirements file exists
if [ ! -f bitnet/requirements.txt ]; then
  echo "requirements.txt not found in bitnet directory."
  exit 1
fi

# Create a conda environment called bitnet-cpp
conda create -n bitnet-cpp python=3.9 -y

# Activate the conda environment
. "$(conda info --base)/etc/profile.d/conda.sh"
conda activate bitnet-cpp

# Install dependencies using pip
git submodule update --init --recursive
pip install --verbose -r ./bitnet/requirements.txt

# Check if the installation was successful
if [ $? -eq 0 ]; then
  echo "Dependencies installed successfully."
else
  echo "Failed to install dependencies."
  exit 1
fi