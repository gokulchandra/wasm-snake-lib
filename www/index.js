import {memory} from 'wasm-snake/wasm_snake_bg';
import {Map} from 'wasm-snake';

const CELL_SIZE = 10;
const map = Map.new();
const width = map.width();
const height = map.height();

const btn = document.getElementById('reset-btn');
btn.onclick = () => window.location.reload();

const score = document.getElementById('score');
const canvas = document.getElementById('snake-canvas');

canvas.height = (height + 1) * (CELL_SIZE + 1);
canvas.width = (width + 1) * (CELL_SIZE + 1);

const ctx = canvas.getContext('2d');

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = '#FABC44';

  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
};

const drawCells = () => {
  const cellsPtr = map.cells();
  const cells = new Uint32Array(memory.buffer, cellsPtr, width * height);
  let meatPosition = map.meat_position();
  const meat = {row: meatPosition['0'], col: meatPosition['1']};
  ctx.beginPath();
  const getIndex = (row, column) => {
    return (row * width) + column;
  };

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const index = getIndex(row, col);
      ctx.fillStyle = cells[index] === 1 ? '#000' : '#FFF';

      if ((row > 0 || col > 0) && (row === meat.row) && (col === meat.col)) {
        ctx.fillStyle = '#000';
      }

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }
  ctx.stroke();
};

drawGrid();
drawCells();

let animationId;
let skip = false;

const render = (tickFn) => {
  if (!!tickFn && !skip) {
    tickFn();
  }
  drawGrid();
  drawCells();

  score.innerText = `Score: ${map.score()}`;

  skip = !skip;
  animationId = requestAnimationFrame(() => render(tickFn));
};

render();

let previousDir = null;

const oppositeDirMapping = {
  38: 40,
  40: 38,
  37: 39,
  39: 37
};

const isOpposite = (newDir, currentDir) => {
  if (!currentDir) return false;
  return oppositeDirMapping[currentDir] === newDir;
};

function checkKey(e) {
  if (isOpposite(e.keyCode, previousDir)) return null;
  previousDir = e.keyCode;
  e = e || window.event;
  cancelAnimationFrame(animationId);
  animationId = null;


  if (e.keyCode === 38) {
    render(() => map.tick(2)); // up arrow
  } else if (e.keyCode === 40) {
    render(() => map.tick(3));// down arrow
  } else if (e.keyCode === 37) {
    render(() => map.tick(0));// left arrow
  } else if (e.keyCode === 39) {
    render(() => map.tick(1));// right arrow
  }

}

document.onkeydown = checkKey;
