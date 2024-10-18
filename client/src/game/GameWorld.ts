import { Snake } from "./Snake";
import { Food } from "./Food";
import { Position, SnakeData } from "./types";
import {
  CANVAS_WIDTH,
  CANVAS_HEIGHT,
  WORLD_WIDTH,
  WORLD_HEIGHT,
} from "./constants";
import { NetworkManager } from "../network/NetworkManager";
import { UIManager } from "../ui/UIManager";

import { loadImage } from "./utils";
import { ClientMessage } from "../network/messages";

export class GameWorld {
  canvas: HTMLCanvasElement;
  ctx: CanvasRenderingContext2D;
  snakes: Map<string, Snake>;
  foods: Food[];
  player: Snake | null;
  networkManager: NetworkManager;
  uiManager: UIManager;
  lastUpdateTime: number;

  worldWidth: number;
  worldHeight: number;

  backgroundImage: HTMLImageElement | null;
  backgroundPattern: CanvasPattern | null;

  keysPressed: Set<string>;

  constructor(
    canvas: HTMLCanvasElement,
    networkManager: NetworkManager,
    uiManager: UIManager
  ) {
    this.canvas = canvas;
    this.ctx = canvas.getContext("2d")!;
    this.snakes = new Map();
    this.foods = [];
    this.player = null;
    this.networkManager = networkManager;
    this.uiManager = uiManager;
    this.lastUpdateTime = Date.now();

    this.worldWidth = WORLD_WIDTH;
    this.worldHeight = WORLD_HEIGHT;

    this.backgroundImage = null;
    this.backgroundPattern = null;

    this.canvas.width = CANVAS_WIDTH;
    this.canvas.height = CANVAS_HEIGHT;

    this.keysPressed = new Set();
  }

  init() {
    this.loadBackgroundImage();
    this.setupInputHandlers();
    // Initialize event listeners, game loop, etc.
    requestAnimationFrame(() => this.gameLoop());
  }

  private gameLoop() {
    console.log("Game loop running...");
    this.update();
    this.render();
    requestAnimationFrame(() => this.gameLoop());
  }

  private update() {
    const currentTime = Date.now();
    const deltaTime = currentTime - this.lastUpdateTime;
    this.lastUpdateTime = currentTime;

    // Update player movement
    this.updatePlayerMovement(deltaTime);

    // Update all snakes
    this.snakes.forEach((snake) => {
      snake.update(deltaTime);
    });
  }

  private render() {
    if (this.backgroundPattern) {
      // Save the context state
      this.ctx.save();

      // Calculate camera offset (centered on player)
      const cameraX = (this.player?.position.x || 0) - this.canvas.width / 2;
      const cameraY = (this.player?.position.y || 0) - this.canvas.height / 2;

      // Translate the context to simulate camera movement
      this.ctx.translate(-cameraX, -cameraY);

      // Fill the background with the pattern
      this.ctx.fillStyle = this.backgroundPattern;
      this.ctx.fillRect(
        cameraX, // Start drawing from the camera's top-left corner
        cameraY,
        this.canvas.width,
        this.canvas.height
      );

      // Render other game entities relative to the camera
      this.renderEntities();

      // Restore the context state
      this.ctx.restore();
    } else {
      // Fallback to a solid color if the pattern isn't ready
      this.ctx.fillStyle = "#000000";
      this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);
    }
  }

  private renderEntities() {
    // Render foods
    for (const food of this.foods) {
      food.render(this.ctx);
    }

    // Render snakes
    this.snakes.forEach((snake) => {
      snake.render(this.ctx);
    });
  }

  private updatePlayerMovement(deltaTime: number) {
    if (this.player) {
      const speed = this.player.speed * (deltaTime / 1000);
  
      let moved = false;
  
      if (this.keysPressed.has("ArrowUp") || this.keysPressed.has("w")) {
        this.player.position.y -= speed;
        moved = true;
      }
      if (this.keysPressed.has("ArrowDown") || this.keysPressed.has("s")) {
        this.player.position.y += speed;
        moved = true;
      }
      if (this.keysPressed.has("ArrowLeft") || this.keysPressed.has("a")) {
        this.player.position.x -= speed;
        moved = true;
      }
      if (this.keysPressed.has("ArrowRight") || this.keysPressed.has("d")) {
        this.player.position.x += speed;
        moved = true;
      }
  
      // Clamp player position to world bounds
      this.player.position.x = Math.max(0, Math.min(this.player.position.x, this.worldWidth));
      this.player.position.y = Math.max(0, Math.min(this.player.position.y, this.worldHeight));
  
      if (moved) {
        // Update the player's body
        this.player.body.unshift({ x: this.player.position.x, y: this.player.position.y });
  
        // Remove last segment if body is longer than length
        if (this.player.body.length > this.player.length) {
          this.player.body.pop();
        }
      }
    }
  }

  private setupInputHandlers() {
    window.addEventListener("keydown", (e) => {
      const direction = this.getDirectionFromKey(e.key);
      if (direction !== null) {
        this.networkManager.send({
          message: ClientMessage.MoveSnake,
          data: direction,
        });
      }
    });

    window.addEventListener("keyup", (e) => {
      this.keysPressed.delete(e.key);
    });
  }

  private getDirectionFromKey(key: string): number | null {
    switch (key) {
      case "ArrowUp":
      case "w":
        return 270; // Assuming 0 degrees is to the right
      case "ArrowRight":
      case "d":
        return 0;
      case "ArrowDown":
      case "s":
        return 90;
      case "ArrowLeft":
      case "a":
        return 180;
      default:
        return null;
    }
  }

  addSnake(data: SnakeData) {
    const snake = new Snake(data);
    console.log(`Added snake:`);
    console.log(snake);
    this.snakes.set(snake.id, snake);
  }

  removeSnake(id: string) {
    this.snakes.delete(id);
  }

  handlePlayerInit(data: SnakeData) {
    console.log(`Creating snake (${data.id})...`);
    this.player = new Snake(data);
    this.addSnake(data);
    console.log(`Player ${data.name} initialized.`);
  }

  handlePlayerJoined(data: SnakeData) {
    if (this.player && data.id === this.player.id) return;
    this.addSnake(data);
    this.uiManager.showNotification(`${data.name} joined the game.`);
    console.log(`Player ${data.name} joined the game.`);
  }

  handlePlayerLeft(data: { id: string }) {
    this.removeSnake(data.id);
    this.uiManager.showNotification(`A player left the game.`);
    console.log(`Player ${data.id} left the game.`);
  }

  handleUpdateSnake(data: SnakeData) {
    const snake = this.snakes.get(data.id);
    if (snake) {
      snake.updateFromData(data);
    } else {
      // If the snake is not in the map, add it
      this.addSnake(data);
    }
  }

  handleFoodEaten(data: { position: Position }) {
    // Remove the food from the game world
    this.foods = this.foods.filter(
      (food) => food.position.x !== data.position.x || food.position.y !== data.position.y
    );
  }

  handleSnakeDied(data: { id: string }) {
    this.removeSnake(data.id);
    this.uiManager.showNotification(`Snake ${data.id} has died.`);
  }

  handleFoodSpawned(data: { positions: Position[], color: [number, number, number] }) {
    data.positions.forEach((pos) => {
      this.foods.push(new Food({ position: pos, value: 1, color: data.color }));
    });
  }

  private async loadBackgroundImage() {
    try {
      this.backgroundImage = await loadImage("/bg.jpg");
      console.log("Background image loaded");

      // Create a repeating pattern from the image
      this.backgroundPattern = this.ctx.createPattern(
        this.backgroundImage,
        "repeat"
      );
    } catch (error) {
      console.error("Failed to load background image:", error);
    }
  }

  reset_game() {
    this.snakes = new Map();
    this.foods = [];
    this.player = null;
    this.keysPressed = new Set();
  }
}
