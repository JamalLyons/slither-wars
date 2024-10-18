import "./style.css"

import { GameWorld } from './game/GameWorld';
import { NetworkManager } from './network/NetworkManager';
import { UIManager } from './ui/UIManager';
import { SERVER_URL } from './game/constants';

function main() {
  const canvas = document.getElementById('gameCanvas') as HTMLCanvasElement;
  resizeCanvas(canvas); // Set initial size
  window.addEventListener('resize', () => resizeCanvas(canvas)); // Handle window resize
  
  const networkManager = new NetworkManager(null as any, SERVER_URL)
  const uiManager = new UIManager(networkManager);
  const gameWorld = new GameWorld(canvas, networkManager, uiManager);
  
  networkManager.gameWorld = gameWorld;
}

function resizeCanvas(canvas: HTMLCanvasElement) {
  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;
}

main();