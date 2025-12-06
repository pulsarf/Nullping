# Implementation Details

Nullping uses a command-line argument interface to configure the websocket frame.

## Command-Line Arguments

### Server setup specifics

Add the following command-line arguments to configure the server:

- `--address`: The hostname or IP address of the victim server.
- `--port`: The port number to attack. Find it out with `nmap` if unsure.
- `--protocol`: Choose between `ws` or `wss`.
- `--use-tungstenite`: Use the tungstenite library for WebSocket communication. Automatically enables TLS, so you **MUST ENABLE THIS OPTION** if you want to use TLS (or, in simpler terms, `wss` as the protocol). Requires native TLS support via openssl.
- `--sec-websocket-key`: A base64 encoded string with a length of 16 bytes. May be random, but that's not a strict requirement for testing.
- `--path`: Websocket server path. Defaults to `/`.

Example:

```
nullping-rs --address 192.168.1.1 --port 8080 --sec-websocket-key "dGhlIHNhbXBsZSBub25jZQ==" --path "/ws"
```

### WebSocket Frame generation options

- `--fin`: Set the FIN bit in the WebSocket frame. Has a meaning of indicating the end of a message.
- `--rsv1`: Set the reserved bit 1 in the WebSocket frame.
- `--rsv2`: Set the reserved bit 2 in the WebSocket frame.
- `--rsv3`: Set the reserved bit 3 in the WebSocket frame.
- `--mask`: Set the mask bit in the WebSocket frame. In this case, you would put a mask key, if you were .
- `--opcode`: Set the opcode in the WebSocket frame. Refer to the [WebSocket RFC](https://tools.ietf.org/html/rfc6455#section-5.2) for more information.
- `--len`: Set the length of the payload in the WebSocket frame.
- `--extended-length`: Set the extended length of the payload in the WebSocket frame. 
- `--key`: Set the mask key in the WebSocket frame. A list of bytes separated by commas.
- `--payload`: Set the payload in the WebSocket frame. A list of bytes separated by commas.

### Miscellaneous options

- `--help`: Display help information.
- `--version`: Display version information.
