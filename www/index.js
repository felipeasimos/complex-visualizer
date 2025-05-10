import { Chart, Point, ChartType, default as init } from "complex-visualizer";

const canvas = document.getElementById("canvas");
const coord = document.getElementById("coord");
const status = document.getElementById("status");
const result = document.getElementById("result");
const vector1Real = document.getElementById("vector1-real");
const vector1Imaginary = document.getElementById("vector1-imaginary");
const vector2Banner = document.getElementById("vector2");
const vector2Real = document.getElementById("vector2-real");
const vector2Imaginary = document.getElementById("vector2-imaginary");
const resetButton = document.getElementById("btn-reset");
const operationsDropdown = document.getElementById("operation");

let chart = null;
let drag_button_pressed = false;

initialize();

async function initialize() {
	await init();
	main();
}

export function main() {
	chart = Chart.new("canvas");
	setupUI();
	setupCanvas();
	setupVectors();
	resetVectors();
}

function resetVectors() {
	chart.vector1 = Point.init(0, 0);
	chart.vector2 = Point.init(0, 0);
	vector1Real.value = vector1Imaginary.value = 0;
	vector2Real.value = vector2Imaginary.value = 0;
	updatePlot();
}

function changeOperation(evt) {
	const op = evt.target.value;
	chart.chart_type = ChartType["Complex" + op.charAt(0).toUpperCase() + op.slice(1)];
	vector2Banner.innerText = op.charAt(0).toUpperCase() + op.slice(1);

	const isTranslate = op === "translate";
	vector2Imaginary.disabled = !isTranslate;
	vector2Imaginary.style.opacity = isTranslate ? "1" : "0.5";

	updatePlot();
}

function setupUI() {
	status.innerText = "WebAssembly loaded!";
	window.addEventListener("resize", setupCanvas);
	window.addEventListener("mousemove", onMouseMove);
	canvas.addEventListener("wheel", onScroll, false);
	window.addEventListener("mousedown", () => drag_button_pressed = true);
	window.addEventListener("mouseup", () => drag_button_pressed = false);
	resetButton.addEventListener("click", resetVectors);
	operationsDropdown.addEventListener("change", changeOperation);
}

function setupVectors() {
	const updateV1 = () => {
		chart.vector1 = Point.init(Number(vector1Real.value), Number(vector1Imaginary.value));
		updatePlot();
	};
	const updateV2 = () => {
		chart.vector2 = Point.init(Number(vector2Real.value), Number(vector2Imaginary.value));
		updatePlot();
	};

	vector1Real.addEventListener("input", updateV1);
	vector1Imaginary.addEventListener("input", updateV1);
	vector2Real.addEventListener("input", updateV2);
	vector2Imaginary.addEventListener("input", updateV2);
}

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
	const zoom_diff = 1 - diff;
	chart.scale(Point.init(zoom_diff, zoom_diff));
	chart.translate(Point.init(
		(diff / 2) * viewport.width,
		(diff / 2) * viewport.height
	));
	updatePlot();
}

function onMouseMove(event) {
	if (!chart || event.target !== canvas) return;

	const actualRect = canvas.getBoundingClientRect();
	const logicX = event.offsetX * canvas.width / actualRect.width;
	const logicY = event.offsetY * canvas.height / actualRect.height;
	const point = chart.coord(logicX, logicY);

	if (point) {
		coord.innerText = `${point.x.toFixed(3)} + ${point.y.toFixed(3)}i`;
	} else {
		coord.innerText = "Mouse pointer is out of range";
	}

	if (drag_button_pressed) {
		const viewport = chart.get_viewport();
		const x = -(event.movementX / actualRect.width) * viewport.width;
		const y = (event.movementY / actualRect.height) * viewport.height;
		chart.translate(Point.init(x, y));
		updatePlot();
	}
}

function operationResult() {
	switch (operationsDropdown.value) {
		case "translate": return chart.vector1.translate(chart.vector2);
		case "rotate": return chart.vector1.rotate(chart.vector2.x);
		case "scale": return chart.vector1.scale(chart.vector2.x);
	}
}

function updatePlot() {
	const start = performance.now();
	chart.update();
	const end = performance.now();
	status.innerText = `Rendered in ${Math.ceil(end - start)}ms`;
	const resultVector = operationResult();
	result.innerText = `Result: ${resultVector.x.toFixed(3)} + ${resultVector.y.toFixed(3)}i`;
}
