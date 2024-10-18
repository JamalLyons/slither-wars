import { WsClientPacket, ClientMessage, WsServerPacket, ServerMessage } from './messages';
import { GameWorld } from '../game/GameWorld';
import { SnakeData } from '../game/types';

export class NetworkManager {
  ws: WebSocket | null = null;
  gameWorld: GameWorld;
  serverUrl: string;

  constructor(gameWorld: GameWorld, serverUrl: string) {
    this.gameWorld = gameWorld;
    this.serverUrl = serverUrl;
  }

  connect(playerName: string) {
    this.ws = new WebSocket(this.serverUrl);

    this.ws.onopen = () => {
      console.log('Connected to server');

      this.send({
        message: ClientMessage.JoinGame,
        data: playerName,
      });

      // Start the game after the connection is established
      setTimeout(() => {
        this.gameWorld.init()
      }, 100);
    };

    this.ws.onmessage = (event) => {
      const packet: WsServerPacket = JSON.parse(event.data);
      this.handleServerMessage(packet);
    };

    this.ws.onclose = () => {
      console.log('Disconnected from server');
      this.ws = null;
      this.gameWorld.reset_game();
      this.gameWorld.uiManager.reloadWindow();
    };
  }

  send(packet: WsClientPacket) {
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(packet));
    }
  }

  handleServerMessage(packet: WsServerPacket) {
    switch (packet.message) {
      case ServerMessage.PlayerInit:
        this.gameWorld.handlePlayerInit(packet.data as SnakeData);
        break;
      case ServerMessage.PlayerJoined:
        // We dont need to handle this event for ourself
        if(this.gameWorld.player?.id === packet.data.id) return
        this.gameWorld.handlePlayerJoined(packet.data as SnakeData);
        break;
      case ServerMessage.PlayerLeft:
        this.gameWorld.handlePlayerLeft(packet.data);
        break;
      case ServerMessage.UpdateSnake:
        this.gameWorld.handleUpdateSnake(packet.data as SnakeData);
        break;
        case ServerMessage.FoodEaten:
      this.gameWorld.handleFoodEaten(packet.data);
      break;
    case ServerMessage.SnakeDied:
      this.gameWorld.handleSnakeDied(packet.data);
      break;
    case ServerMessage.FoodSpawned:
      this.gameWorld.handleFoodSpawned(packet.data);
      break;
      default:
        console.warn(`Unhandled server message: ${packet.message}`);
        break;
    }
  }
}