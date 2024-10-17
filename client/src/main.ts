import "./style.css";
import { createDisplayName } from "./utils";

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
  PlayerInit = "PlayerInit",
  PlayerJoined = "PlayerJoined",
  PlayerLeft = "PlayerLeft",
}

interface Snake {
  id: string;
  name: string;
  position: {
    x: number;
    y: number;
  };
  direction: number;
  speed: number;
  body: {
    x: number;
    y: number;
  }[];
  length: number;
  is_dead: boolean;
  score: number;
  is_bot: boolean;
  color: number[];
}

let connected = false;
let ws: WebSocket | null = null;
let playerName: string | null = null;
let player: Snake | null = null;
let otherPlayers: Record<string, Snake> = {};

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
  if (connected) {
    return;
  }

  // Initialize WebSocket connection
  ws = new WebSocket("ws://127.0.0.1:9001");

  ws.onopen = () => {
    connected = true;
    console.log("Connected");

    // Send JoinGame message
    ws!.send(
      JSON.stringify({
        message: ClientMessage.JoinGame,
        data: playerName,
      })
    );
  };

  ws.onclose = () => {
    connected = false;
    console.log("Disconnected from server");
  };

  ws.onmessage = (event) => {
    const packet: WsServerPacket = JSON.parse(event.data);

    if (packet.message === ServerMessage.PlayerInit) {
      console.log("Player initialized:", createDisplayName(packet.data.name));
      player = packet.data;

      console.log(`Player ${player?.name} (${player?.id}) joined.`);
    }

    if (packet.message === ServerMessage.PlayerJoined) {
      // We already have our player data, we dont need it again
      if (player?.id === packet.data.id) return;

      otherPlayers[packet.data.id] = packet.data;

      console.log(
        "Another Player joined:",
        createDisplayName(packet.data.name)
      );
    }

    if (packet.message === ServerMessage.PlayerLeft) {
      let p = otherPlayers[packet.data.id];

      console.log("Player left:", createDisplayName(p.name));

      delete otherPlayers[packet.data.id];
    }
  };
}
