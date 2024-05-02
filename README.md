## Overview

This is ADSB upload program written in Rust, suitable for various platforms that support Rust compilation.
If this project is helpful to you, please give me a star, thank you.
If you have any suggestions, feel free to raise an issue.

## Binary Usage

Since this project does not include Dump1090 and does not restrict whether the SBS service is running on the local machine, you may need to install Dump1090 first. You can search for specific details by yourself.

There are two configuration methods.

### General file mode (default)

You need to create a TOML file (in the same directory as the program name `conf.toml` is better), with the following content.

```toml
[receiver]
ip = "127.0.0.1"
port = RECEIVER-PORT

[service]
url = "YOUR-SERVICE-URL"
uuid = "YOUR-UUID"
```

And run like this

```bash
$ ./absb --config=./conf.toml
```

The default minimum logger level is 'info', but it can be changed to only output 'error' messages.

```bash
$ RUST_LOG=error ./absb --config=./conf.toml
```

The above shows the situation where Dump1090 is running on the local machine, you can also fill in according to the actual situation.

### Command line mode (advanced)

If you are familiar with terminal operations, you can use this method.

```bash
Usage: adsb [OPTIONS]

Options:
      --receiver-ip <RECEIVER_IP>      Receiver ip [default: 127.0.0.1]
      --receiver-port <RECEIVER_PORT>  Receiver port [default: 30003]
      --service-url <SERVICE_URL>      Service url
      --service-uuid <SERVICE_UUID>    Service uuid
      --config <TOML_FILE>             TOML config file path (look like: ./conf.toml)
  -h, --help                           Print help
  -V, --version                        Print version
```

### NOTE

Please be aware that you are required to comply with local laws and policies.
