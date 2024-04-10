This a release of ZBUS version 1.2.0, which continue to offer an improved features of a telemetry bus. The bus allows for real-time or postponed mode publication and request of data. It is also tightly integrated with the Zabbix Observability platform, which means that the Zabbix server can access the telemetry data published on the P2P telemetry bus directly. Additionally, since the bus is not tied to a specific server, multiple Zabbix servers can share telemetry, resulting in a horizontally scalable federated observability platform. You can access the project repository with full access to the source code at https://github.com/vulogov/zbus

ZBUS project offers command-line tool called zbus. Added features of this tool are:


ZB-script new standard library features:
* Sampler().n() and Sampler().n(value) - set the threshold of number of values for data consistency analysis
* Sampler().q(value) - set the value for data consistency threshold
* Sampler().set_consistent(value) - set the data in the sampler and return true or false. True, if the data in the Sampler() is consistent.
* Sampler().consistency() - return the computed consistency of the data (standard deviation).
