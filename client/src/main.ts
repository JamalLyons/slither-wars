import './style.css'


interface Position {
  x: number,
  y: number
}

interface Player {
  position: Position
}

enum Action {
  GameInit = "GameInit",
  GameLoad = "GameLoad",
  GameUpdate = "GameUpdate",
  GameOver = "GameOver"
}

interface Payload {
  action: Action,
}

const socket = new WebSocket('ws://127.0.0.1:9001');

socket.onopen = () => {
  console.log('Connected to server');

  const payload: Payload = {
    action: Action.GameInit,
  }

  console.log('Sending Payload:', payload);

  writeMessage(payload);
};

socket.onclose = () => {
  // todo - set page state to landing screen when game is over
  console.log('Disconnected from server');
}

socket.onmessage = (event) => {
  console.log('Server message received:', event.data);
};

function writeMessage(message: unknown) {
  let json = JSON.stringify(message);
  let bin = new TextEncoder().encode(json);
  socket.send(bin)
}

