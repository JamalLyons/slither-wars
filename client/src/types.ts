interface Position {
  x: number;
  y: number;
}

interface Player {
  position: Position;
}

export enum Action {
  GameInit = "GameInit",
  GameLoad = "GameLoad",
  GameSpawn = "GameSpawn",
  GameUpdate = "GameUpdate",
  GameOver = "GameOver",
}

export interface Payload {
  action: Action;
}
