import { NetworkManager } from "../network/NetworkManager";

export class UIManager {
    loadingScreen: HTMLElement;
    startForm: HTMLFormElement;
    playerNameInput: HTMLInputElement;
    leaderboard: HTMLElement;
    leaderboardList: HTMLElement;
    gameCanvas: HTMLCanvasElement
    networkManager: NetworkManager

    constructor(networkManager: NetworkManager) {
      this.networkManager = networkManager;
      this.loadingScreen = document.getElementById('loadingScreen')!;
      this.startForm = document.getElementById('startForm') as HTMLFormElement;
      this.playerNameInput = document.getElementById('playerName') as HTMLInputElement;
      this.leaderboard = document.getElementById('leaderboard')!;
      this.leaderboardList = document.getElementById('leaderboardList')!;
      this.gameCanvas = document.getElementById('gameCanvas') as HTMLCanvasElement;
  
      this.init();
    }
  
    init() {
      // Handle form submission to start the game
      this.startForm.addEventListener('submit', (e) => {
        e.preventDefault();
        const playerName = this.playerNameInput.value || 'Anonymous';
        this.onStartGame(playerName);
      });
    }
  
    onStartGame(playerName: string) {
      this.hideLoadingScreen();
      this.networkManager.connect(playerName);
      this.gameCanvas.style.display = 'block';
    };
  
    hideLoadingScreen() {
      this.loadingScreen.style.display = 'none';
    }

    reloadWindow() {
      window.location.reload();
    }
  
    showNotification(message: string) {
      console.log(message);
      // Implement UI notification if needed
    }
  
    updateLeaderboard(players: any[]) {
      // Update leaderboard UI
    }
  }