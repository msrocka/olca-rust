#!/bin/bash

cargo clean
cargo build --release

LIB=./target/release/libolcar

if [ -e "$LIB.so" ]
then
    LIB="$LIB.so"
fi

if [ -e "$LIB.dylib" ]
then
    LIB="$LIB.dylib"
fi

cp $LIB ./bin
