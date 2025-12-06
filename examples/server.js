const WebSocket = require("ws");

/**
 * @name Server
 * @classdesc
 * This server is made vulnerable to Nullping intentionally
 *
 * Normally, you would add an error listener, but
 * the absence of it, is what nullping exploit abuses
 */
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
    });

    this.server.on("listening", () => {
      console.log(`Running at port ${Server.PORT}`);
    });
  }

  #onMessage(message) {
    console.log("Received:", message);
  }
}).start();
