#!/usr/bin/env bash

address=$1

docker exec -it secretdev secretcli q compute query $address '{"hot_spot": {"accuracy": 5}}'