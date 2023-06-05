#!/usr/bin/env sh

cargo doc --no-deps
rm -rf ./docs
echo "<meta http-equiv=\"refresh\" content=\"0; url=poc_offtake_parser\">" > target/doc/index.html
cp -r target/doc ./docs
