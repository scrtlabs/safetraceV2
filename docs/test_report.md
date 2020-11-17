### Test Report

This document will cover the tests ran on Safetrace to benchmark the performance of the
system. 

#### Methodology

All tests were done with random data, generated uniformly randomly over a predefined area. The reason
for using random data is that it represents a "worst-case" scenario for our system, wherein data is scattered
equally across a large geographic area, over a two week period. 

We assume that a single person, over a two week period creates about 3000 data points of geolocation data. We will be 
testing our system with simulated data that represents over 1000 people, or 4,000,000 data points.

#### Test 

The core functions we wish to test:

* Inserting new data
* Querying overlap between stored data, and test data
* Querying stored data hotspots

We will test each case for 1, 10, 100 and 1000 person's worth of data, recording the time each test took, as well as 
resource usage.

The test system we will be running on is an Azure Confidential Compute machine, Standard_DC4s_v2, with 4 cores, 16 GB of RAM and 112MB of EPC memory.
We will be running a single node for these tests.


#### Results

TBD
