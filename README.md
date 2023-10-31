Zbus is a command-line tool that is a part of the Zbus project. The project aims to solve one of the most persistent issues in the monitoring and observability world - telemetry distribution. In traditional architectures, the collection, distribution, and processing of telemetry are centralized around a single instance of the monitoring and observability platform. However, in some cases, the scale or complexity of the environment requires multiple collection and processing centers organized in a loosely connected federation of instances. In such cases, sharing telemetry collected by each instance becomes a challenge. The Zbus project aims to address this problem by providing a real-time, subscription-based telemetry distribution solution.

## How Zbus works?

Zbus operates on a network of Zenoh servers that can form a peer-to-peer network and offer distributed publish/subscribe services and queryable data storage. The highly effective Zenoh protocol supports near real-time telemetry storage and retrieval. Telemetry items are stored in Zenoh storage nodes as key-value pairs, using in-memory storage, where the value is a structured JSON object.

## What Zbus CLI tool can do?

Zbus CLI tool is a scriptable command-line tool for managing telemetry data on the Zenoh P2P network. Global parameters provide the necessary context for connecting to Zenoh nodes.

```
--bus - address of the neares Zenoh node
--listen - specify listen address for p2p operations
--discable-multicast-scout - do not use multicast for discovering Zenoh network
--set-connect-mode - switch from peer to client connect mode
```

### Store telemetry item on Zenoh network.

```
zbus put --help
```
To store telemetry data, you can use the 'put' subcommand in the Zbus tool. This command allows you to create and submit telemetry data using various parameters. With the CLI parameters of the 'put' subcommand, you can specify the type, timestamp, source, key, and value of the telemetry data. By default, the value is a scriptable expression that supports arithmetic and logical operators, statistical functions, and data generation capabilities.

Example:

```
zbus put --key answer --value "41+1"
```
### Query telemetry item stored on Zenoh network

```
zbus get --help
```

Using the get subcommand, you can retrieve data from the Zenoh network by properly forming the query key using the provided parameters.

Example:

```
zbus get --key answer
```

### Subscribe on updates stored on Zenoh network

```
zbus subscribe --help
```
By using command subscribe, you can run the subscription for the telemetry updates that has been stored on Zenoh network. 
