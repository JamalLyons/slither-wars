
import { Position, FoodData } from './types';

export class Food {
  position: Position;
  value: number;
  color: [number, number, number];

  constructor(data: FoodData) {
    this.position = data.position;
    this.value = data.value;
    this.color = data.color;
  }

  render(ctx: CanvasRenderingContext2D) {
    ctx.fillStyle = `rgb(${this.color.join(',')})`;
    ctx.fillRect(this.position.x, this.position.y, 5, 5);
  }
}