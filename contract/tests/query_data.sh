#!/usr/bin/env bash

address=$1

docker exec -it secretdev secretcli q compute query $address '{"hot_spot": {}}'

docker exec -it secretdev secretcli q compute query $address '{"match_data_points": {"data_points": [{"latitudeE7": 525331150, "longitudeE7": 134378710, "timestampMs": "1600693951455"}]}}'