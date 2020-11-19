## General 

The confidential compute platform is based on the Secret Network codebase, however it has been modified to fit a 
private enterprise use-case. To keep track of these changes, we use this document for reference.

The codebase can be found under the Secret Network repository, in the "safetrace" branch [here](https://github.com/enigmampc/SecretNetwork/tree/safetrace)

### Floating points

Floating points have been enabled. While in public blockchains floating point operations may be a source of non-determinism
in the enterprise scenario both hardware and application logic is known, and can be rigorously tested. For this reason we have 
enabled the use of floating point operations, which we use for geographic matching, but may also be used for machine learning 
algorithms in future developments

### Memory limits

Public blockchains must cater to the lowest common denominator in terms of hardware requirements, otherwise they risk low adoption
due to high barriers for entry. In a private setting, it is beneficial to increase hardware allocated to the infrastructure, to enable
better performance, and use-cases which require a large amount of resources (such as big-data processing)

### Consensus Layer optimizations

The consensus layer is responsible for making sure all nodes across the decentralized network stay synced, computing the 
same data, and achieving similar results. In a public blockchain, you must optimize for general application use, with many possible 
edge-cases. In addition, nodes may be scattered across large geographic areas, so there might be large amounts of latency in communications. 
Timeout timers, and limits on the amount of data that can be sent are enforced as to avoid timeouts, or large computation times in this case.

On private blockchains, we can hyper-optimize for a specific use-case. In our case, we want to be storing large amounts of data at once, and do not
mind computation times of minutes, rather than seconds. 

Tendermint data-size limits have been removed (from 10MB), and timeouts have been increased to 5 minutes per transaction.

### Blockchain parameters

Gas limits, and transaction size limits are meant to keep computations on blockchains in-check. We remove these limits, to allow
for larger data sizes, and longer compute times. 

While it may be beneficial to remove the gas functionality entirely, we have decided not to do make that change. 
Gas is still useful as a mechanism to avoid endless loops, and removing it requires a large amount of effort in modifying core blockchain code.  

