import "./style.css";

interface WsClientPacket {
  message: ClientMessage;
  data: any;
}

enum ClientMessage {
  JoinGame = "JoinGame",
}

interface WsServerPacket {
  message: ServerMessage;
  data: any;
}

enum ServerMessage {
  PlayerJoined = "PlayerJoined",
  PlayerLeft = "PlayerLeft",
}

let connected = false;
let ws: WebSocket | null = null;
let playerName: string | null = null;

const startForm = document.getElementById("startForm") as HTMLFormElement;
const playerNameInput = document.getElementById(
  "playerName"
) as HTMLInputElement;

// Handle form submission to start the game
startForm.addEventListener("submit", (e) => {
  e.preventDefault();
  playerName = playerNameInput.value || null;
  startGame();
});

function startGame() {
    if(connected) {
        return;
    }

  // Initialize WebSocket connection
  ws = new WebSocket("ws://127.0.0.1:9001");

  ws.onopen = () => {
    connected = true;
    console.log("Connected to server");
    // Send JoinGame message
    const joinMessage: WsClientPacket = {
      message: ClientMessage.JoinGame,
      data: playerName
    }

    console.log("Sending JoinGame message:", joinMessage);

    ws!.send(JSON.stringify(joinMessage));
  };

  ws.onclose = () => {
    connected = false;
    console.log("Disconnected from server");
  };

  ws.onmessage = (event) => {
    const packet: WsServerPacket = JSON.parse(event.data);
    console.log("Received packet:", packet);

    if (packet.message === ServerMessage.PlayerJoined) {
      console.log("Player joined:", packet.data);
    }
  }
}
