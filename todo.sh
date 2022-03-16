#!/usr/bin/env bash

cd functions && \
cargo build --target x86_64-unknown-linux-musl && \
mkdir -p lambda && \
cp target/x86_64-unknown-linux-musl/debug/bootstrap lambda && \
cd .. && \
cdk synth --no-staging && \
sam local invoke rust-lambda \
--event functions/json/event.json \
--env-vars functions/json/env.json \
--template ./cdk.out/Cdkv2SamLambdaRustStack.template.json
