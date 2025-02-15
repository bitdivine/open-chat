#!/bin/bash

SCRIPT=$(readlink -f "$0")
SCRIPT_DIR=$(dirname "$SCRIPT")
cd $SCRIPT_DIR

WASM_SRC=$1 # WASM_SRC is either empty, "latest", "local", "prod" the commit Id or the release version

if [[ -z $WASM_SRC ]] || [[ $WASM_SRC = "latest" ]]
then
  WASM_SRC=$(curl -s https://openchat-canister-wasms.s3.amazonaws.com/latest)
fi

echo "Downloading wasms"

./download-canister-wasm.sh community $WASM_SRC || exit 1
./download-canister-wasm.sh cycles_dispenser $WASM_SRC || exit 1
./download-canister-wasm.sh escrow $WASM_SRC || exit 1
./download-canister-wasm.sh group $WASM_SRC || exit 1
./download-canister-wasm.sh group_index $WASM_SRC || exit 1
./download-canister-wasm.sh local_group_index $WASM_SRC || exit 1
./download-canister-wasm.sh local_user_index $WASM_SRC || exit 1
./download-canister-wasm.sh market_maker $WASM_SRC || exit 1
./download-canister-wasm.sh neuron_controller $WASM_SRC || exit 1
./download-canister-wasm.sh notifications $WASM_SRC || exit 1
./download-canister-wasm.sh notifications_index $WASM_SRC || exit 1
./download-canister-wasm.sh online_users $WASM_SRC || exit 1
./download-canister-wasm.sh proposal_validation $WASM_SRC || exit 1
./download-canister-wasm.sh proposals_bot $WASM_SRC || exit 1
./download-canister-wasm.sh registry $WASM_SRC || exit 1
./download-canister-wasm.sh storage_bucket $WASM_SRC || exit 1
./download-canister-wasm.sh storage_index $WASM_SRC || exit 1
./download-canister-wasm.sh user $WASM_SRC || exit 1
./download-canister-wasm.sh user_index $WASM_SRC || exit 1

echo "Wasms downloaded"