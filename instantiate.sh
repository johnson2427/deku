#!/bin/bash

pushd deku_contract
cargo contract instantiate --constructor new --suri //Alice --url ws://localhost:9944 --execute -y --output-json > deployed.json
popd
