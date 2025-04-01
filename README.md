# Rust HTTP Server

Basic HTTP server implemented using stdlib.

## Building

Run `cargo run` in root directory.

## Configuration

In main.rs:
### Set IP to:

|Name                 |IP                        |Use-case                     |
|---------------------|--------------------------|-----------------------------|
|Unspecified (default)|`0.0.0.0`                 |Loopback + Local IP: allow connection using `127.0.0.1:port` and `<local-ip>:port`            |
|Loopback             |`127.0.0.1`               |only allow connections from server host machine with `127.0.0.1:port`          |
|Local IP             |run `ipconfig` in terminal|only allow connections from within the LAN using server local IP (cannot connect via `127.0.0.1:port`)

If unsure, leave default.

### Set PORT to:
Any from 1024-65535. If server not responding, try a different port.

### Configure port forwarding in your router:

 - internal port = your chosen port
 - external port = any (better to select chosen port)
 - internal host = server local IP

## Notes
`.html` and `.jpg` files can be added/removed easily within their directories without needing to modify source code.

Binding to unspecified or local IP (option 1 or 3) + configuring port forwarding allows external connections using 
`<public-IP>:<chosen external port>`.

Obtain `public-IP` by searching whatsmyip.
