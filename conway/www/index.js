import { Universe, Cell } from "conway";
import { memory } from "conway/conway_bg";

const CELL_SIZE = 8; // In pixels
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";
const HEIGHT = 64;
const WIDTH = 64;
let fps = 20;
let lastTickAt = 0;
const fpsScaleFactor = 800;

const universe = Universe.pattern_new(WIDTH, HEIGHT);

const playPauseButton = document.getElementById("play-pause");
const fpsElement = document.getElementById("fps-element");

const canvas = document.getElementById("conway-canvas");
canvas.height = (CELL_SIZE + 1) * HEIGHT + 1;
canvas.width = (CELL_SIZE + 1) * WIDTH + 1;

const ctx = canvas.getContext("2d");

let animationId = null;

const isPaused = () => {
	return animationId === null;
};

const renderLoop = () => {
	// debugger;
	const now = Date.now();
	const elapsed = now - lastTickAt;

	if (elapsed * fps > fpsScaleFactor) {
		universe.tick();

		drawGrid();
		drawCells();

		lastTickAt = now;
	}
	animationId = requestAnimationFrame(renderLoop);
};

const drawGrid = () => {
	ctx.beginPath();
	ctx.strokeStyle = GRID_COLOR;

	// Vertical lines
	for (let i = 0; i <= WIDTH; i++) {
		ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
		ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * HEIGHT + 1);
	}

	// Horizontal lines
	for (let j = 0; j <= HEIGHT; j++) {
		ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
		ctx.lineTo((CELL_SIZE + 1) * WIDTH + 1, j * (CELL_SIZE + 1) + 1);
	}

	ctx.stroke();
};

const getIndex = (row, column) => {
	return row * WIDTH + column;
};

const drawCells = () => {
	const cellsPtr = universe.cells();
	const cells = new Uint8Array(memory.buffer, cellsPtr, WIDTH * HEIGHT);

	ctx.beginPath();

	for (let row = 0; row < HEIGHT; row++) {
		for (let col = 0; col < WIDTH; col++) {
			const idx = getIndex(row, col);

			ctx.fillStyle = cells[idx] === Cell.Dead ? DEAD_COLOR : ALIVE_COLOR;

			ctx.fillRect(col * (CELL_SIZE + 1) + 1, row * (CELL_SIZE + 1) + 1, CELL_SIZE, CELL_SIZE);
		}
	}

	ctx.stroke();
};

const play = () => {
	playPauseButton.textContent = "⏸";
	renderLoop();
};

const pause = () => {
	playPauseButton.textContent = "▶";
	cancelAnimationFrame(animationId);
	animationId = null;
};

playPauseButton.addEventListener("click", () => {
	if (isPaused()) play();
	else pause();
});

canvas.addEventListener("click", (event) => {
	const boundingRect = canvas.getBoundingClientRect();

	const scaleX = canvas.width / boundingRect.width;
	const scaleY = canvas.height / boundingRect.height;

	const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
	const canvasTop = (event.clientY - boundingRect.top) * scaleY;

	const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), HEIGHT - 1);
	const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), WIDTH - 1);

	universe.toggle_cell(row, col);

	drawGrid();
	drawCells();
});

fpsElement.addEventListener("input", function () {
	fps = parseInt(this.value);
	console.log(`FPS set to ${fps}.`);
});

drawGrid();
drawCells();
pause();
