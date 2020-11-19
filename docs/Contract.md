## General

This document will document the implementation of the safetrace secret contract.

The secret contract is where the contract tracing application logic is implemented, as well as the 
api to interface with it.

## Design

It is important to know that when creating blockchain applications there are two types of functions (in most cases). State modifying,
or non-state modifying. 
 
State modifying refers to functions which change data that is stored between executions. These functions are execution as a part of blockchain transactions, and must be 
executed by all blockchain nodes.
Non-modifying functions are low overhead, and can be executed by a single node. 
This means they can be called asynchronously with a low performance overhead, and can be scaled to multiple nodes (query different nodes in parallel). 

In our case, these are referred to as "handles" (state modifying) and "queries" (non-state modifying).

We assume that all encryption & decryption of data, at rest and in transit is handled by the blockchain infrastructure. You can read more
about the encryption protocols and the design of the Secret Network [here](https://build.scrt.network/protocol/intro.html)

It is important to note that all data is encrypted when not in use. For this reason any usage of data requires expensive decryption, so we want to minimize the amount of 
data accessed.

## Data

Our data model works with geolocation data that is defined by <longitude, latitude, time>, where the longitude & latitude are sent as 8-byte integers, and the time is the amount of
milliseconds since 1.1.1970 (UNIX epoch). (see the exact schemas [here](../contract/schema) )

When importing, all data is converted to 9-digit precision [geohashes](https://en.wikipedia.org/wiki/Geohash), which allow for high-resolution matching.
We use geohashes, since they allow us to easily query for different resolutions of overlap, while still being performant and compact.

The geohashes are stored in two separate containers - 
* a Hashmap that counts the number of instances of each geohash (at a lower degree of accuracy). This structure maintains a list of the 
the most common geohashes to be used in hot-zone queries

* a Hashmap that maps geohash -> timestamps, spanning a 24 hour period. i.e, a map between a geohash, and the timestamps at which it has been seen. 
Storing the data in such a way allows us to optimize queries for overlap, by simply matching an input timestamp with the timestamps stored for a given geohash and its neighbors.
Splitting the data into 24-hour chunks allows us to easy perform invalidation of old data, since we just have to clear the invalidated container, as well as optimize the amount of data
accessed by lazy-loading containers during queries (i.e, if you only query overlap for a specific day, only data for that day will be loaded)

## Handles

These functions have a high performance overhead, which according to a model where large amounts of data is inputted to the system periodically 
should be acceptable for most use-cases.

### Import new data

Allows us to add new data to the contact-tracing contract. All the serialized geolocation data is sent using this function,
where it is sorted, processed and stored inside the contract.

### Invalidate old data

Data which is over two weeks old is no longer relevant for contact-tracing. This function allows deletion of such data

### Change owner

Management function, in case an administrator change is required

## Queries

### Overlap

This query checks for geolocation overlap between input data, and data stored in the contract. Matches are performed by 
checking whether there is an overlap of timestamps between the input data and stored data. The overlap is tested by checking
not only overlap for a specific geohash, but also the 8 neighboring geohashes (N, NE, E, SE, S, SW, W, NW). 

### Hotzones

This query returns the most active geohashes (on a 7-character resolution), that appear the most times in the input data.
It has a near-zero performance overhead, since the information is calculated during data input and cached. For this reason, the resolution
of the hot zones are not configurable by the caller.

Note: For small amounts of data it is possible to store data in a Trie and perform dynamic queries, but for large amount of data 
the decryption overhead becomes very large.