# Nullping

## What's the base of this exploit

WS module has these errors documented in its doc/ws.md

- WS_ERR_EXPECTED_FIN
- WS_ERR_EXPECTED_MASK
- WS_ERR_INVALID_CLOSE_CODE
- WS_ERR_INVALID_CONTROL_PAYLOAD_LENGTH
- WS_ERR_INVALID_OPCODE
- WS_ERR_INVALID_UTF8
- WS_ERR_UNEXPECTED_MASK
- WS_ERR_UNEXPECTED_RSV_1
- WS_ERR_UNEXPECTED_RSV_2_3
- WS_ERR_UNSUPPORTED_DATA_PAYLOAD_LENGTH
- WS_ERR_UNSUPPORTED_MESSAGE_LENGTH

All of these errors are thrown by the WS module when it encounters an error while processing a WebSocket frame.

However, if the server doesn't handle the error event, it will crash the server.

Refer to lib/receiver.js

```js
      if (!this._fin) {
        const error = this.createError(
          RangeError,
          'FIN must be set',
          true,
          1002,
          'WS_ERR_EXPECTED_FIN'
        );

        cb(error);
        return;
      }
```

The point is to **generate an invalid frame and send it to the misconfigured server** to crash it.

## How is it practically implemented

Nullping accepts a variety of arguments to generate different types of invalid frames.

You're given an ability to generate frames with a custom opcode, payload, mask bit, mask key, and more.

Nullping will not check whether the frame is valid or not. It will just send the frame to the server.

For more information about the implementation and command line arguments, refer to [implementation.md](implementation.md)

## How to protect against this

1. Add an `error` event listener to the WebSocket server to handle errors gracefully.
2. Consider using try-catch blocks around the WebSocket server code to catch and handle errors.
3. Implement rate limiting to prevent abuse and denial-of-service attacks.
4. Switch to a more secure WebSocket library that handles errors gracefully. For example, use uWebSockets.js.

For more information on how to protect against this vulnerability, refer to [protection.md](protection.md)
