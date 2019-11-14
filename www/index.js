import {
    Universe
} from "gol";

const pre = document.getElementById("gol-canvas");
const universe = Universe.new();

const renderLoop = () => {
    pre.textContent = universe.render();
    universe.tick();
    requestAnimationFrame(renderLoop);
}

requestAnimationFrame(renderLoop);