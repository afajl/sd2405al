#!/bin/bash

rsync -avz --delete --exclude target . pi:sda2405al
ssh -t pi "cd sda2405al\
  && cargo build --examples \
  && sudo env RUST_BACKTRACE=1 target/debug/examples/simple"
