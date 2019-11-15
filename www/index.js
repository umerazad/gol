import {
    Universe,
    Cell
} from "gol";

import {
    memory
} from "gol/gol_bg";

const CELL_SIZE = 5;
const GRID_COLOR = "#F5B7B1";
const DEAD_COLOR = "#2C3E50";
const ALIVE_COLOR = "#28B463";

const universe = Universe.new();
const width = universe.width();
const height = universe.height();

const canvas = document.getElementById("gol-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

let animationId = null;

const fps = new class {
    constructor() {
        this.fps = document.getElementById("fps");
        this.frames = [];
        this.lastFrameTimestamp = performance.now();
    }

    render() {
        const now = performance.now();
        const delta = now - this.lastFrameTimestamp;
        this.lastFrameTimestamp = now;
        const fps = 1 / delta * 1000;

        this.frames.push(fps);
        if (this.frames.length > 100) {
            this.frames.shift();
        }

        let min = Infinity;
        let max = -Infinity;
        let sum = 0;

        for (let i = 0; i < this.frames.length; i++) {
            let f = this.frames[i];
            sum += f;
            min = Math.min(min, f);
            max = Math.max(max, f);
        }

        let mean = sum / this.frames.length;

        this.fps.textContent = `
     Frames per Second:
            latest = ${Math.round(fps)}
   avg of last 100 = ${Math.round(mean)}
   min of last 100 = ${Math.round(min)}
   max of last 100 = ${Math.round(max)}
        `.trim();
    }
}

const renderLoop = () => {
    fps.render();
    drawGrid();
    drawCells();
    universe.tick();
    animationId = requestAnimationFrame(renderLoop);
};

// Stuff related to interactivity i.e pause/resume.

const isPaused = () => {
    return animationId === null;
};

const playPauseButton = document.getElementById("play-resume");

const play = () => {
    console.log("Playing ... ");
    playPauseButton.textContent = "⏸";
    renderLoop();
};

const pause = () => {
    console.log("Pausing ...");
    playPauseButton.textContent = "▶";
    cancelAnimationFrame(animationId);
    animationId = null;
};

playPauseButton.addEventListener("click", event => {
    if (isPaused()) {
        play();
    } else {
        pause();
    }
});


// Stuff related to drawing canvas.

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
}

const getIndex = (row, col) => {
    return row * width + col;
}

const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);

            ctx.fillStyle = cells[idx] === Cell.Dead ?
                DEAD_COLOR :
                ALIVE_COLOR;

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

canvas.addEventListener("click", event => {
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const left = (event.clientX - boundingRect.left) * scaleX;
    const top = (event.clientY - boundingRect.top) * scaleY;

    const row = Math.min(Math.floor(top / (CELL_SIZE + 1)), height - 1);
    const col = Math.min(Math.floor(left / (CELL_SIZE + 1)), width - 1);

    universe.toggle_cell(row, col);

    drawGrid();
    drawCells();
});
// Start animation.

drawGrid();
drawCells();
play();