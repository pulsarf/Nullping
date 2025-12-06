# Protection

As already's been mentioned, this exploit abuses the absence of proper error handling. 

## Simple fixes

Add proper error handling to the codebase.

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

Alternative solution: Use uWebSockets.js.
