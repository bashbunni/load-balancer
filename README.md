# Load Balancer in Rust

## Purpose
This repo exists for me to deeply understand load balancers and practice rust development.

## What is a load balancer
Distributing computational workloads between two or more computers. RE: Internet this is commonly used as a traffic controller for multi-server setups.

Without load balancing, servers get overloaded. With load balancing, connections are distributed based on the chosen algorithms.

### Logistics
Can run on a server OR VM OR cloud. CDNs typically provide load balancing features. In this case, I'm using nginx running in a Docker container to test connections.

### Where Can We Find Load Balancers
- used WITH web apps; redistribute incoming traffic to servers hosting the app
- also LARGE localized networks; e.g. data center or office complex. These typically require a hardware appliance like an application delivery controller. Though you can also use a software-based load balancer.

### Common Algorithms for Load Balancing
#### Static (the unbothered king load balancer algorithms)
- quick to set up
- ! interest in current server states
- predetermined

#### Dynamic (FPS game healer)
Takes into account: current availability, workload, and health of ea. server

Dynamic servers are more difficult to configure as they require additional server monitoring. If a server is unavailable, the load balancers with failover to another server/group of server. This happens quickly to avoid any gaps in service.

##### What is availability
- health + capacity of the server
- size of tasks distributed

## Steps
### Most Basic Solution (Level 0)
- [] create a basic server (L4: TCP) that can start
- [] receive connection requests
- [] choose a load balancing algo to redirect traffic to our set of available servers: red robin (until you hit max_cap then switch algo to per service volume)
- [] health checks: AKA intermittent requests sent to servers to check that they're up and running
- [] yaml file with available servers; only on startup

### Level 1
- [] failure handling: do we retry if connection to server fails. Also mark as bad connection
- [] max number of in-progress requests per backend
- [] count no of connections for ea server
- [] graceful shutdown (no more connections, wrap up existing requests)

## Choosing an Algorithm
Step 1. We start with static. The goal here is to get an idea of the simplest possible configuration. Then when we move on to dynamic load balancers, we have a frame of reference meaning we can better assess the tradeoffs.

### Static
- round robin: in-order distribution
- weighted round robin: round robin + ea. server has a "power value" to dictate its capabilities. (e.g. server B is 3x faster than server A, it receives 3x more requests)
- source IP hash: get hash of client IP -> stores it so all requests from that IP go to the same server. Likely mostly for an entire country for example
(chat says this is not how hashes work...)
- random

### Dynamic
- least connections: (*most used*) go to server with fewest active connections
- least response time: prioritizes servers by fastest response times (historical data)
- weighted least response time: LRT + server capacity weights (power)
- resource-based: real-time CPU, memory, disk I/O status for servers to avoid overload.

## Development

To pretend we have servers to connect to use an nginx docker container `docker run -d -p 43003:80 nginx`
