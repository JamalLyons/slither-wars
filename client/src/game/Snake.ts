import { Position, SnakeData } from "./types";

export class Snake {
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

  constructor(data: SnakeData) {
    this.id = data.id;
    this.name = data.name;
    this.position = data.position;
    this.direction = data.direction;
    this.speed = data.speed;
    this.body = data.body && data.body.length > 0 ? data.body : [{ ...data.position }];
    this.length = data.length || 10;
    this.isDead = data.isDead;
    this.score = data.score;
    this.isBot = data.isBot;
    this.color = data.color;
  }

  updateFromData(data: SnakeData) {
    this.position = data.position;
    this.direction = data.direction;
    this.speed = data.speed;
    this.body = data.body;
    this.length = data.length;
    this.isDead = data.isDead;
    this.score = data.score;
  }

  update(deltaTime: number) {
    // Update snake position and body
  }

  render(ctx: CanvasRenderingContext2D) {
    ctx.fillStyle = `rgb(${this.color.join(",")})`;
    for (const segment of this.body) {
      ctx.fillRect(segment.x - 5, segment.y - 5, 10, 10);
    }
  }
}
