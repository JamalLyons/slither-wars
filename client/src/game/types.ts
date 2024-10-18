export interface Position {
    x: number;
    y: number;
  }
  
  export interface SnakeData {
    id: string;
    name: string;
    position: Position;
    direction: number;
    speed: number;
    body: Position[];
    length: number;
    isDead: boolean;
    score: number;
    isBot: boolean;
    color: [number, number, number];
  }
  
  export interface FoodData {
    position: Position;
    value: number;
    color: [number, number, number];
  }