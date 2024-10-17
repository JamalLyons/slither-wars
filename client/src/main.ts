import "./style.css"

// Define types for server messages
interface Player {
  id: string;
  name: string | null;
  position: [number, number];
  direction: number;
  length: number;
  width: number;
  color: [number, number, number];
}

interface Food {
  position: [number, number];
  color: [number, number, number];
  size: number;
}

interface LeaderboardEntry {
  id: string;
  name: string | null;
  score: number;
}

interface GameStateUpdate {
  players: Player[];
  food: Food[];
  leaderboard: LeaderboardEntry[];
}

interface ServerMessage {
  GameStateUpdate?: GameStateUpdate;
}

interface ClientMessage {
  JoinGame?: {
      name: string | null;
  };
  PlayerInput?: {
      direction: number;
      boosting: boolean;
  };
}

// Game variables
let ws: WebSocket | null = null;
let playerId: string | null = null;
let players: { [key: string]: Player } = {};
let foodItems: Food[] = [];
let leaderboard: LeaderboardEntry[] = [];

let playerName: string | null = null;
let boosting: boolean = false;
let mousePosition: { x: number; y: number } = { x: 0, y: 0 };
let canvasRect: DOMRect;

// Get HTML elements
const loadingScreen = document.getElementById('loadingScreen') as HTMLDivElement;
const startForm = document.getElementById('startForm') as HTMLFormElement;
const playerNameInput = document.getElementById('playerName') as HTMLInputElement;
const playButton = document.getElementById('playButton') as HTMLButtonElement;
const canvas = document.getElementById('gameCanvas') as HTMLCanvasElement;
const ctx = canvas.getContext('2d')!;
const leaderboardElement = document.getElementById('leaderboard') as HTMLDivElement; // Renamed for clarity
const leaderboardList = document.getElementById('leaderboardList') as HTMLUListElement;

// Handle form submission to start the game
startForm.addEventListener('submit', (e) => {
  e.preventDefault();
  playerName = playerNameInput.value || null;
  startGame();
});

// Function to start the game
function startGame() {
  // Hide loading screen
  loadingScreen.style.display = 'none';

  // Show leaderboard
  leaderboardElement.classList.remove('hidden');

  // Initialize WebSocket connection
  ws = new WebSocket('ws://127.0.0.1:9001');

  ws.onopen = () => {
      console.log('Connected to server');
      // Send JoinGame message
      const joinMessage: ClientMessage = {
          JoinGame: {
              name: playerName,
          },
      };
      ws!.send(JSON.stringify(joinMessage));
  };

  ws.onmessage = (event) => {
      const message: ServerMessage = JSON.parse(event.data);
      if (message.GameStateUpdate) {
          const update = message.GameStateUpdate;
          players = {};
          update.players.forEach((player) => {
              players[player.id] = player;
          });
          // Set playerId if not already set
          if (!playerId) {
              playerId = findPlayerIdByName(playerName, players);
          }
          foodItems = update.food;
          leaderboard = update.leaderboard;
          draw();
          updateLeaderboard();
      }
  };

  ws.onclose = () => {
      console.log('Disconnected from server');
  };

  // Get canvas bounding rect for mouse position calculation
  canvasRect = canvas.getBoundingClientRect();

  // Mouse movement handler
  window.addEventListener('mousemove', (e) => {
      mousePosition.x = e.clientX - canvasRect.left;
      mousePosition.y = e.clientY - canvasRect.top;
      sendInput();
  });

  // Mouse click handlers for boosting
  window.addEventListener('mousedown', (e) => {
      if (e.button === 0) {
          boosting = true;
          sendInput();
      }
  });

  window.addEventListener('mouseup', (e) => {
      if (e.button === 0) {
          boosting = false;
          sendInput();
      }
  });
}

// Function to find player ID by name
function findPlayerIdByName(name: string | null, players: { [key: string]: Player }): string | null {
  for (let id in players) {
      if (players[id].name === name) {
          return id;
      }
  }
  return null;
}

// Function to send player input to the server
function sendInput() {
  if (!ws || ws.readyState !== WebSocket.OPEN || !playerId) return;
  const player = players[playerId];
  if (!player) return;

  // Calculate direction based on mouse position
  const deltaX = mousePosition.x - canvas.width / 2;
  const deltaY = mousePosition.y - canvas.height / 2;
  const angle = Math.atan2(deltaY, deltaX) * (180 / Math.PI);

  const message: ClientMessage = {
      PlayerInput: {
          direction: angle,
          boosting: boosting,
      },
  };
  ws.send(JSON.stringify(message));
}

// Drawing function
function draw() {
  // Clear the canvas
  ctx.clearRect(0, 0, canvas.width, canvas.height);

  // Save the context state
  ctx.save();

  // Get the player's position
  const player = players[playerId!];
  if (!player) {
      return; // Player not found, possibly not yet initialized
  }

  // Calculate zoom factor based on player's length or width
  const zoomFactor = calculateZoomFactor(player.length);

  // Center the view on the player
  const translateX = -player.position[0] + canvas.width / 2;
  const translateY = -player.position[1] + canvas.height / 2;

  // Apply scaling and translation
  ctx.scale(zoomFactor, zoomFactor);
  ctx.translate(translateX / zoomFactor, translateY / zoomFactor);

  // Draw background (optional)
  ctx.fillStyle = '#111827'; // Tailwind gray-900
  ctx.fillRect(0, 0, canvas.width / zoomFactor, canvas.height / zoomFactor);

  // Draw food
  foodItems.forEach((food) => {
      ctx.fillStyle = `rgb(${food.color[0]}, ${food.color[1]}, ${food.color[2]})`;
      ctx.beginPath();
      ctx.arc(food.position[0], food.position[1], food.size / 2, 0, 2 * Math.PI);
      ctx.fill();
  });

  // Draw players
  for (let id in players) {
      const player = players[id];
      ctx.fillStyle = `rgb(${player.color[0]}, ${player.color[1]}, ${player.color[2]})`;
      ctx.beginPath();
      ctx.arc(player.position[0], player.position[1], player.width / 2, 0, 2 * Math.PI);
      ctx.fill();
  }

  // Restore the context state
  ctx.restore();

  // Request the next frame
  requestAnimationFrame(draw);
}

// Function to calculate zoom factor based on player's length
function calculateZoomFactor(length: number): number {
  // Define min and max zoom levels
  const minZoom = 0.5; // Zoomed out (larger view)
  const maxZoom = 1.5; // Zoomed in (smaller view)

  // Define min and max lengths for scaling
  const minLength = 10; // Starting length
  const maxLength = 100; // Maximum length

  // Normalize the player's length between 0 and 1
  let normalizedLength = (length - minLength) / (maxLength - minLength);
  if (normalizedLength < 0) normalizedLength = 0;
  if (normalizedLength > 1) normalizedLength = 1;

  // Invert the normalized length to get zoom factor
  const zoomFactor = maxZoom - normalizedLength * (maxZoom - minZoom);

  return zoomFactor;
}

// Function to update the leaderboard
function updateLeaderboard() {
  leaderboardList.innerHTML = '';
  leaderboard.forEach((entry) => {
      const listItem = document.createElement('li');
      const displayName = entry.name ? entry.name : `Player ${entry.id}`;
      listItem.textContent = `${displayName} - Score: ${entry.score}`;
      leaderboardList.appendChild(listItem);
  });
}