This an initial public release of ZBUS version 1.0.0, which offers a fundamental implementation of a telemetry bus. The bus allows for real-time or postponed mode publication and request of data. It is also tightly integrated with the Zabbix Observability platform, which means that the Zabbix server can access the telemetry data published on the P2P telemetry bus directly. Additionally, since the bus is not tied to a specific server, multiple Zabbix servers can share telemetry, resulting in a horizontally scalable federated observability platform. You can access the project repository with full access to the source code at https://github.com/vulogov/zbus

ZBUS project offers command-line tool called zbus. Current features of this tool are:

* Telemetry data publishing, request and subscription
* Synchronization metadata from Zabbix observability platform to the telemetry bus
* Export metrics from Zabbix metrics JSON files
* Export metrics from Zabbix real-time Web-callbacks
* Export Zabbix SLA data to the telemetry bus
* Export metrics from Prometheus exporter
* Query data on telemetry bus
* Interpreter for execute ZB-scripts
* Implementation of the pipeline instrumentation for programmatic (with ZB-scrpt) feed, generate, process, aggregate and sink of telemetry data


ZB-script standard library features:

* Metric() object abstracting a single telemetry metric
* Sampler() object abstracting a sample of metrics (up to 128 samples)
* Programmatic generation of the data distributions using various algorithms, such as triangular, sinusoidal, logarithmic, exponential and others
* Telemetry forecasting using Markov chains
* Smoothing and normalizing operations over sampled data for ML analysis
* Statistical oscillator telemetry forecasting is a method of predicting future trends based on the analysis of various oscillators. These oscillators measure the momentum and direction of telemetry value movements and are used to identify potential turning points. Through the use of statistical analysis, it is possible to forecast future trends and make informed decisions about future state of the controlled environment
* Interval() object abstracting an interval of metric samples. ZB-script supports interval arithmetic computation
* Managed thread pool, controlling number of threads that can be executed in parallel
* Internal pub/sub bus for passing data between running threads
* Conversion of JSON objects
* Basic Neural Net for ML-based pattern analysis
* Basic functions for handling operations with strings
* Basic timestamp functions
* Basic system::sleep, system::env functions
* Basic Zabbix primitives covering getting telemetry from Zabbix passive agent and sending telemetry to Zabbix Trapper
