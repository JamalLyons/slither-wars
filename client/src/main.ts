import "./style.css";
import { Payload, Action } from "./types";

const game = document.getElementById("game");

if (!game) {
  throw new Error("Game element not found");
}

game.innerText = "Game is loading...";

const socket = new WebSocket("ws://127.0.0.1:9001");

socket.onopen = () => {
  console.log("Connected to server");

  const payload: Payload = {
    action: Action.GameInit,
  };

  console.log("Sending Payload:", payload);

  writeMessage(payload);
};

socket.onclose = () => {
  // todo - set page state to landing screen when game is over
  console.log("Disconnected from server");
};

socket.onmessage = (event) => {
  const handler = readMessage(event);

  if(handler) {
    switch (handler.action) {
      case Action.GameLoad:
        console.log("Game loaded");
        break;
      case Action.GameUpdate:
        console.log("Game update");
        break;
      case Action.GameOver:
        console.log("Game over");
        break;
      default:
        console.log("Unknown action");
        break;
    }
  }
};

/** Sends a binary encoded message to the server */
function writeMessage(message: unknown) {
  socket.send(new TextEncoder().encode(JSON.stringify(message)));
}

/** Reads a binary encoded message from the server */
function readMessage(message: MessageEvent): Payload | undefined {
  if (!message.data) {
    throw new Error("Server message did not contain data");
  }

  if (message.data instanceof Blob) {
    message.data.arrayBuffer().then((buffer) => {
      let parsed = JSON.parse(new TextDecoder().decode(buffer));
      console.log("Received Payload:", parsed);
      return parsed
    });
  }

  return undefined
}
