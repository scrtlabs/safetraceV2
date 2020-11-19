## API

### Handle

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "HandleMsg",
  "anyOf": [
    {
      "description": "Adds new data to the contract, in the format specified by `GoogleTakeoutHistory`.",
      "type": "object",
      "required": [
        "import_google_locations"
      ],
      "properties": {
        "import_google_locations": {
          "type": "object",
          "required": [
            "data"
          ],
          "properties": {
            "data": {
              "$ref": "#/definitions/GoogleTakeoutHistory"
            }
          }
        }
      }
    },
    {
      "description": "ChangeDay is used to signal the contract that a day has passed, and all the oldest data, which pertains to 14 days ago is now invalid, and should be removed. This function may take a while, depending on how much data is stored in the contract",
      "type": "object",
      "required": [
        "change_day"
      ],
      "properties": {
        "change_day": {
          "type": "object"
        }
      }
    },
    {
      "description": "Admins have permissions to import data and invalidate old data This function adds a new admin which can manage the contract",
      "type": "object",
      "required": [
        "add_admin"
      ],
      "properties": {
        "add_admin": {
          "type": "object",
          "required": [
            "address"
          ],
          "properties": {
            "address": {
              "$ref": "#/definitions/HumanAddr"
            }
          }
        }
      }
    },
    {
      "description": "Admins have permissions to import data and invalidate old data This function removes an admin. Any admin can remove and other admin - consider customizing this functionality according to access control policies",
      "type": "object",
      "required": [
        "remove_admin"
      ],
      "properties": {
        "remove_admin": {
          "type": "object",
          "required": [
            "address"
          ],
          "properties": {
            "address": {
              "$ref": "#/definitions/HumanAddr"
            }
          }
        }
      }
    }
  ],
  "definitions": {
    "GoogleLocation": {
      "type": "object",
      "required": [
        "latitudeE7",
        "longitudeE7",
        "timestampMs"
      ],
      "properties": {
        "latitudeE7": {
          "type": "integer",
          "format": "uint64",
          "minimum": 0.0
        },
        "longitudeE7": {
          "type": "integer",
          "format": "uint64",
          "minimum": 0.0
        },
        "timestampMs": {
          "$ref": "#/definitions/Uint128"
        }
      }
    },
    "GoogleTakeoutHistory": {
      "type": "object",
      "required": [
        "locations"
      ],
      "properties": {
        "locations": {
          "type": "array",
          "items": {
            "$ref": "#/definitions/GoogleLocation"
          }
        }
      }
    },
    "HumanAddr": {
      "type": "string"
    },
    "Uint128": {
      "type": "string"
    }
  }
}

```

### Query

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "QueryMsg",
  "anyOf": [
    {
      "description": "This query returns all the data points from the input which overlap with data stored in the contract. Aka, all the points that overlap in both location and time, to the accuracy defined by the contract (10 meter/5 minutes by default)",
      "type": "object",
      "required": [
        "match_data_points"
      ],
      "properties": {
        "match_data_points": {
          "type": "object",
          "required": [
            "data_points"
          ],
          "properties": {
            "data_points": {
              "type": "array",
              "items": {
                "$ref": "#/definitions/GoogleLocation"
              }
            }
          }
        }
      }
    },
    {
      "description": "This query returns the 10 most active zone, accurate to about a ~70m radius",
      "type": "object",
      "required": [
        "hot_spot"
      ],
      "properties": {
        "hot_spot": {
          "type": "object",
          "properties": {
            "accuracy": {
              "description": "unused",
              "type": [
                "integer",
                "null"
              ],
              "format": "uint32",
              "minimum": 0.0
            },
            "zones": {
              "description": "unused",
              "type": [
                "integer",
                "null"
              ],
              "format": "uint32",
              "minimum": 0.0
            }
          }
        }
      }
    },
    {
      "description": "Returns the earliest and latest times allowed by the contract for data storage",
      "type": "object",
      "required": [
        "time_range"
      ],
      "properties": {
        "time_range": {
          "type": "object"
        }
      }
    }
  ],
  "definitions": {
    "GoogleLocation": {
      "type": "object",
      "required": [
        "latitudeE7",
        "longitudeE7",
        "timestampMs"
      ],
      "properties": {
        "latitudeE7": {
          "type": "integer",
          "format": "uint64",
          "minimum": 0.0
        },
        "longitudeE7": {
          "type": "integer",
          "format": "uint64",
          "minimum": 0.0
        },
        "timestampMs": {
          "$ref": "#/definitions/Uint128"
        }
      }
    },
    "Uint128": {
      "type": "string"
    }
  }
}

```

### Init

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "InitMsg",
  "type": "object",
  "required": [
    "start_time"
  ],
  "properties": {
    "start_time": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0.0
    }
  }
}

```