// If you only use `npm` you can simply
// import { Chart } from "complex-visualizer" and remove `setup` call from `bootstrap.js`.
import { Chart, Point, default as init } from "complex-visualizer";

const canvas = document.getElementById("canvas");
const coord = document.getElementById("coord");
const status = document.getElementById("status");
const vector1Real = document.getElementById("vector1-real");
const vector1Imaginary = document.getElementById("vector1-imaginary");
const vector2Real = document.getElementById("vector2-real");
const vector2Imaginary = document.getElementById("vector2-imaginary");

let chart = null;
let drag_button_pressed = false;

initialize();

async function initialize() {
	await init();
	main();
}

/** Main entry point */
export function main() {
	chart = Chart.new("canvas");
	setupUI();
	setupCanvas();
	setupVectors();
}

/** Add event listeners. */
function setupUI() {
	status.innerText = "WebAssembly loaded!";
	window.addEventListener("resize", setupCanvas);
	window.addEventListener("mousemove", onMouseMove);
	canvas.addEventListener("wheel", onScroll, false);
	window.addEventListener("mousedown", onMouseDown);
	window.addEventListener("mouseup", onMouseUp);
}

function onMouseDown(evt) {
	drag_button_pressed = true;
}

function onMouseUp(evt) {
	drag_button_pressed = false;
}

function setupVectors() {
	vector1Real.addEventListener("change", (evt) => {
		chart.vector1 = Point.init(Number(evt.target.value), chart.vector1.y);
		updatePlot();
	})
	vector1Imaginary.addEventListener("change", (evt) => {
		chart.vector1 = Point.init(chart.vector1.x, Number(evt.target.value));
		updatePlot();
	})
	vector2Real.addEventListener("change", (evt) => {
		chart.vector2 = Point.init(Number(evt.target.value), chart.vector1.y);
		updatePlot();
	})
	vector2Imaginary.addEventListener("change", (evt) => {
		chart.vector2 = Point.init(chart.vector1.x, Number(evt.target.value));
		updatePlot();
	})
}

/** Setup canvas to properly handle high DPI and redraw current plot. */
function setupCanvas() {
	const aspectRatio = canvas.width / canvas.height;
	const size = canvas.parentNode.offsetWidth * 0.8;
	canvas.style.width = size + "px";
	canvas.style.height = size / aspectRatio + "px";
	canvas.width = size;
	canvas.height = size / aspectRatio;
	updatePlot();
}

function onScroll(event) {
	const viewport = chart.get_viewport();
	const diff = -event.deltaY * 0.0001;
	const zoom_diff = (1 - diff);
	chart.scale(Point.init(zoom_diff, zoom_diff));
	chart.translate(Point.init(
		(diff / 2) * viewport.width,
		(diff / 2) * viewport.height
	));
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

			if (drag_button_pressed) {
				drag_button_pressed = event;
				const viewport = chart.get_viewport();
				const x = -(event.movementX / actualRect.width) * viewport.width;
				const y = (event.movementY / actualRect.height) * viewport.height;
				chart.translate(Point.init(x, y));
				updatePlot();
			}
		}
		coord.innerText = text;
	}
}

/** Redraw currently selected plot. */
function updatePlot() {
	const start = performance.now();
	chart.update();
	const end = performance.now();
	status.innerText = `Rendered in ${Math.ceil(end - start)}ms`;
}
