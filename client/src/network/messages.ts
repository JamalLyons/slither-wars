export interface WsClientPacket {
    message: ClientMessage;
    data: any;
  }
  
  export enum ClientMessage {
    JoinGame = 'JoinGame',
    MoveSnake = 'MoveSnake',
    // Add other client messages
  }
  
  export interface WsServerPacket {
    message: ServerMessage;
    data: any;
  }
  
  export enum ServerMessage {
    PlayerInit = 'PlayerInit',
    PlayerJoined = 'PlayerJoined',
    PlayerLeft = 'PlayerLeft',
    UpdateSnake = 'UpdateSnake',
    FoodEaten = 'FoodEaten',
  SnakeDied = 'SnakeDied',
  FoodSpawned = 'FoodSpawned',
  }