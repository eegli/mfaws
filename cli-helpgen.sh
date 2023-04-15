#!/bin/bash

cargo build

for cmd in session-token \
            assume-role \
            list \
            clean
do
   ./target/debug/mfaws $cmd --help &> ./help/$cmd.txt
done