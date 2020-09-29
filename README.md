# onet
ONet: Autonomous Network in rust

Work in progress

Inspired from [MaidSafe](https://maidsafe.net/) and their SafeNetwork.

Based on my own implementation of:

- [rsRPC](https://github.com/Champii/rsrpc): Rust Simple RPC
- [Rust-DHT](https://github.com/Champii/rust-dht): A DHT based on Kademlia used for routing and peer discovery
- [Hashgraph](https://github.com/Champii/hashgraph): A Hashgraph implementation for Asynchronous BFT consensus

## Concept

The goal of that project is to create an decentralized autonomous network that regulate itself 
in order to efficiently store some addressable and permissioned content.
This would allow to create a layer of applications that do not need centralized authorities
as well as a cryptocurrency used to regulate exchanges inside that network.

You can find more informations on the original idea here : [https://maidsafe.net/](https://maidsafe.net/)

## Usage

Create a bootstrap node that will wait for other connections

```
./onet -l 127.0.0.1:3000
```

Connect some other nodes

```
# Connect to the bootstrap node
./onet -l 127.0.0.1:3001 -c 127.0.0.1:3000

# Thanks to the inner DHT, you can connect to any node to reach the network.
# Here we connect to the previously created node
./onet -l 127.0.0.1:3002 -c 127.0.0.1:3001
```


# TODO

- Hashgraph Acceptlist/Denylist
- Store section events in datachain
  - Signature by majority
- ClientManager RPC
- Extract Routing from DHT
- Persona
  - Verification of persona
    - client manager
    - data manager 
    - vault
- 
- Split section
- Node aging
- Random join address
- Multiple identities
- Configuration file

