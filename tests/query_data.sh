#!/usr/bin/env bash

address=$1

docker exec -it secretdev secretcli q compute query $address '{"hot_spot": {"accuracy": 5}}'

docker exec -it secretdev secretcli q compute query $address '{"match_data_points": {"data_points": [{"latitudeE7": 525490910, "longitudeE7": 133621018, "timestampMs": "1600140327870"}]}}'