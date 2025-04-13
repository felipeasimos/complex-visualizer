// If you only use `npm` you can simply
// import { Chart } from "complex-visualizer" and remove `setup` call from `bootstrap.js`.
import { Chart, default as init } from "complex-visualizer";

const canvas = document.getElementById("canvas");
const coord = document.getElementById("coord");
const status = document.getElementById("status");
let zoom = 1;

let chart = null;

initialize();

async function initialize() {
	await init();
	main();
}

/** Main entry point */
export function main() {
	setupUI();
	setupCanvas();
}

/** Add event listeners. */
function setupUI() {
	status.innerText = "WebAssembly loaded!";
	window.addEventListener("resize", setupCanvas);
	canvas.addEventListener("wheel", onScroll, false);
	window.addEventListener("mousemove", onMouseMove);
}

/** Setup canvas to properly handle high DPI and redraw current plot. */
function setupCanvas() {
	const dpr = window.devicePixelRatio || 1.0;
	const aspectRatio = canvas.width / canvas.height;
	const size = canvas.parentNode.offsetWidth * 0.8;
	canvas.style.width = size + "px";
	canvas.style.height = size / aspectRatio + "px";
	canvas.width = size;
	canvas.height = size / aspectRatio;
	updatePlot();
}

function onScroll(event) {
	zoom *= 1 - event.deltaY / 10000
	updatePlot();
}

/** Update displayed coordinates. */
function onMouseMove(event) {
	if (chart) {
		var text = "Mouse pointer is out of range";

		if (event.target == canvas) {
			let actualRect = canvas.getBoundingClientRect();
			let logicX = event.offsetX * canvas.width / actualRect.width;
			let logicY = event.offsetY * canvas.height / actualRect.height;
			const point = chart.coord(logicX, logicY);
			text = (point)
				? `${point.x.toFixed(3)} + ${point.y.toFixed(3)}i`
				: text;
		}
		coord.innerText = text;
	}
}

/** Redraw currently selected plot. */
function updatePlot() {
	const start = performance.now();
	chart = Chart.complex("canvas", zoom)
	const end = performance.now();
	status.innerText = `Rendered in ${Math.ceil(end - start)}ms`;
}
