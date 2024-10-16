# Drive-ABCI server

## Introduction

This document describes basic usage of drive-abci server.

## Configuration

Configuration is implemented using environment variables. Environment variables will be read from operating system and `.env` file. If a variable is both defined in the operating system and the `.env` file, the former one takes precedence.

### Logging

The logging configuration allows you to configure multiple logging destinations. Each destination is defined using a series of environment variables prefixed with ABCI_LOG_*key*, where *key* is an arbitrary name for the destination.

The log destination configuration consists of the following parameters:

#### DESTINATION (required)

Specifies the destination of the logs. It can have one of the following values:

* **stdout**: Logs will be sent to the standard output
* **stderr**: Logs will be sent to the standard error
* An absolute path to an existing directory where logs will be stored, for example `/var/log/dash/`

#### VERBOSITY (optional)

Specifies the verbosity level of the logs. It is an integer value ranging from 0 to 5. Defaults to 0. Higher values indicate more detailed and verbose logs. For more information on verbosity levels, refer to the `-v` option in the `drive-abci --help` command.

#### COLOR (optional)

Specifies whether or not to use colorful output for the logs. It is an optional boolean parameter. If not provided, the output colorization will be autodetected.

#### FORMAT (optional)

Specifies the output format to use for the logs. It can have one of the following values:

* **Full**: Logs will be formatted with full details.
* **Compact**: Logs will be formatted in a compact manner.
* **Pretty**: Logs will be formatted in a human-readable and visually appealing manner.
* **Json**: Logs will be formatted in JSON format.

For more detailed description for different formats, refer to the [tracing-subscriber](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/format/index.html#formatters) documentation.

#### MAX_FILES (optional)

Specifies the maximum number of daily log files to store. Defaults to 0.

This parameter is only used when logs are stored in a file. If this is set to 0, log rotation is disabled.

#### Configuring multiple destinations

To configure multiple logging destinations, each destination should have its own environment variables. The environment variable names should be prefixed with ABCI_LOG_*key*, where *key* is an arbitrary name that uniquely identifies the destination.

For example, to configure a logging destination with the key "example", you would set the following environment variables:

* ABCI_LOG_EXAMPLE_DESTINATION: Specifies the destination of logs for the "example" destination.
* ABCI_LOG_EXAMPLE_VERBOSITY: Specifies the verbosity level for the "example" destination.
* ABCI_LOG_EXAMPLE_COLOR: Specifies whether to use colorful output for the "example" destination (optional).
* ABCI_LOG_EXAMPLE_FORMAT: Specifies the output format for the "example" destination (optional).
* ABCI_LOG_EXAMPLE_MAX_FILES: Specifies the maximum number of daily log files to store for the "example" destination (optional).

Make sure to replace *key* with your desired destination name in the environment variable names.

##### Example

Here's an example configuration for a logging destination named "EXAMPLE":

```bash
export ABCI_LOG_EXAMPLE_DESTINATION="/var/log/example"
export ABCI_LOG_EXAMPLE_VERBOSITY=3
export ABCI_LOG_EXAMPLE_COLOR=false
export ABCI_LOG_EXAMPLE_FORMAT="Pretty"
export ABCI_LOG_EXAMPLE_MAX_FILES=10
```

This configuration specifies that logs for the "EXAMPLE" destination should be stored in the /var/log/example directory, with a verbosity level of 3. Colorful output should not be used, and the logs should be formatted in a human-readable and visually appealing manner. The maximum number of daily log files to store is set to 10.

Ensure that you adjust the values according to your specific logging requirements.
