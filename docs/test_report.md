### Test Report

This document will cover the tests ran on Safetrace to benchmark the performance of the
system.

#### Methodology

All tests were done with random data, generated uniformly randomly over a predefined area. The reason
for using random data is that it represents a "worst-case" scenario for our system, wherein data is scattered
equally across a large geographic area, over a two week period.

We assume that a single person, over a two week period creates about 4000 data points of geolocation data (longitude, latitude & time in json format). We will be
testing our system with simulated data that represents over 1000 people, or 4,000,000 data points. This represents about 400MB worth of raw data.

Each test will be performed multiple times to account for variance

#### Test

The core functions we wish to test:

* Inserting new data
* Querying overlap between stored data, and test data
* Querying stored data hotspots

We will test each case for 1, 10, 100 and 1000 person's worth of data, recording the time each test took, as well as
resource usage.

The test system we will be running on is an Azure Confidential Compute machine, Standard_DC4s_v2, with 4 cores, 16 GB of RAM and 112MB of EPC memory.
We will be running a single node for these tests.

The application assumes that data arrives in the form that is exported from Google Location History. As such, there is no significant pre-processing done
on the data.

### Results

##### Inserting new data

Number of data points stored - 4000 geolocation points for each individual.

Total time when inserting new data:

| data-points (#) | time (s) |
| ----------- | ----------- |
|  4000       | 0.23       |
|  40K        | 5   |
| 400K        | 210 |
| 4M          | 3000  |

The current SafeTrace application was written in such a way to optimize for efficient queries, rather than
insertion performance.

Inserting new data is an operation that would be done once every day, or couple of days, by a trusted operator.
In this case, a long compute time is acceptable, although it is important to note that without multithreading
this would cause the blockchain to become non-responsive for that amount of time.
In such a model, user data could also be encrypted if the operator is not trusted.

Since the insertion time is non-linear, we would recommend splitting up data inserted into smaller chunks, and performing
further optimizations to minimize time spent inserting data

In this test we observed an increased usage of RAM, up to 7 GB during the most
intensive tests.

It would be possible to optimize for insertion speed as well, although that was out of the scope of the current implementation
and would probably come at the cost of query performance

##### Overlap

This scenario tests a case where a user would like to query the application and discover
whether or not he has been exposed to a potential source of infection.

We will test scenarios when the application has different amounts of saved data,
as well as the performance impact when batching users.

It is important to note that these queries can be parallelized across multiple safetrace nodes,
since the computation happens on an individual node, and not on-chain across all nodes on the blockchain

| stored data-points (#) | queried data-points (#) | time (s)
| ----------- | ----------- |----------- |
| 4000 | 1 | 0.2 |
| 40K | 10 | 0.4 |
| 400K | 100 | 1.8 |
| 4M | 4000 | 27.9 |

We can see that as long as the amount of queried data-points remains small, the time is relatively fast, and can provide the user with immediate results.
This means that the answer to "was I in proximity to a known CoVid carrier today" can be performed by a single node,
but the results for 4000 data-points (representing a user's data in the past 14 days) would have to be performed asynchronously or concurrently to achieve similar results.

##### Hotspot Queries

This scenario tests the amount of time taken to perform hotspot analysis. This would be used when a researcher would like to
extract geographic areas of interest to perform epidemiological investigation.

The query returns the top 10 geographic areas of interest, which represent the most visited
areas for persons that were later discovered as infected.

| data-points (#) | time (s)|
| ----------- | ----------- |
| 4000 | 239ms +- 50ms|
| 40K  | 252ms +- 50ms|
| 400K | 249ms +- 50ms|
| 4M   | 256ms +- 50ms|


### Conclusions

The results of our benchmarks suggest that performance is adaquate for providing CoVid-19 related research services
for applications that wish to keep their user's data private, and perform data-analysis in a privacy-preserving manner.

The goal was to create an application for use-cases of querying overlap between large amounts of geolocation data, representing
that of up to 1000 users over a 14 day period, and providing hotspot analysis, and the results show that the application can handle
the target amount of data while providing a good user experience - even with the worst-case scenarios in these tests.

In regards to the viability of Secret Network smart contract infrastructure for processing large amounts of data,
we feel that the results shown here are an upper limit of the current Smart Contract infrastructure as it exists currently - processing up to 400-500MB worth of raw data by a smart contract.
Although, it is important to note that the requirements of public blockchains are much different than those
of private chains, and so it is probably realistic to assume that very large gains in these areas are possible
when removing many assumptions and restrictions that are currently in place, and tailoring solutions to more specific use-cases.