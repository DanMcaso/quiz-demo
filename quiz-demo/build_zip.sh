#!/bin/bash
# Create directory structure
mkdir -p quiz_demo/src

# Copy files
cp Cargo.toml quiz_demo/
cp src/main.rs quiz_demo/src/
cp src/quiz.rs quiz_demo/src/
cp src/livy.rs quiz_demo/src/
cp src/zkp.rs quiz_demo/src/
cp README.md quiz_demo/

# Create zip
zip -r quiz_demo.zip quiz_demo

# Clean up
rm -rf quiz_demo