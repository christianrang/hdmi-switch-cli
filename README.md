## HDMI Switch 

Support for 4KMX44-H2 AV Access HDMI switch. This tool will access the telnet port and use API commands. Support is
limited to switching inputs and outputs.

### Installation

```
make install
```

### Configuration 

The default path for configuration is `$HOME/.config/hdmi-switch/configuration.yaml`. Alternatively, the configuration
file path can be set using the `-c` flag.

Example configuration:
```yaml
# Host supports FDQN or IPv4. IPv6 is untested.
server:
  host: 127.0.0.1
  port: 23 # Optional this field will default to 23
```

### Reference

- [AV Access 4KMX44-H2 API docs ](https://www.avaccess.com/wp-content/uploads/2022/03/API-Command-Set_4KMX44-H2-V1.0.0.pdf)
