#!/bin/bash

cargo build

./target/debug/mfaws help &>./help/help.txt

for cmd in session-token \
   assume-role \
   list \
   clean; do
   ./target/debug/mfaws $cmd --help &>./help/$cmd.txt
done
