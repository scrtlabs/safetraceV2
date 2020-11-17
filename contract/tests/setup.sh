#!/usr/bin/env bash

function wait_for_tx() {
    until (./secretcli q tx "$1"); do
        echo "$2"
        sleep 5
    done
}


secretcli config chain-id enigma-pub-testnet-3
secretcli config output json
secretcli config keyring-backend test
secretcli config indent true
secretcli config trust-node true
secretcli config node tcp://localhost:26657

data_file="data/datamsg2.json"
docker_name=secretdev
base_dir=.
label=$(date +"%T")

# copy contract to docker container
scrt_contract="$base_dir/../contract.wasm.gz"
docker cp "$scrt_contract" "$docker_name:/contract.wasm.gz"

# store contract on the chain
docker exec -it $docker_name secretcli tx compute store "/contract.wasm.gz" --from a --gas 2000000 -b block -y

code_id=$(secretcli query compute list-code | jq '.[-1]."id"')

docker exec -it $docker_name secretcli tx compute instantiate $code_id '{"start_time": 1600129528950}' --label $label --from a --gas 2000000 -b block -y

address=$(docker exec -it $docker_name secretcli query compute list-contract-by-code $code_id | jq '.[-1].address')

echo "Deployed at address: $address"


for i in {1..5}
do
  echo "Uploading data... $i"

  docker cp "$data_file" "$docker_name:/data.json"

  export STORE_TX_HASH=$(
        docker exec -it secretdev secretcli tx compute execute --label $label --file /data.json --from a --gas 900000000 -y |
        jq -r .txhash
  )
done

wait_for_tx "$STORE_TX_HASH" "Waiting for store to finish on-chain..."

docker exec -it secretdev secretcli q compute query $address '{"hot_spot": {}}'