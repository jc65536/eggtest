import { Edge, EdgeType } from "./main.js";
import {
    BezierControls, LineControls, ShortestLineControls
} from "./path-controls.js";

export const form = document.querySelector<HTMLFormElement>("#trans-config");

export const inputs = {
    edgeType: form.querySelector<HTMLInputElement>("#edge-type"),
    lineChoice: form.querySelector<HTMLInputElement>("#line-choice"),
    bezierChoice: form.querySelector<HTMLInputElement>("#bezier-choice"),
    shortestLine: form.querySelector<HTMLInputElement>("#shortest-line"),
    reverseEdge: form.querySelector<HTMLButtonElement>("#reverse-edge")
}

let selectedEdge: Edge = null;

export const initForm = (edge: Edge) => {
    selectedEdge = edge;

    const { edgeType, shortestLine, lineChoice, bezierChoice } = inputs;

    edgeType.value = edge.type;

    const controls = edge.controls;

    if (edge.startState === edge.endState) {
        bezierChoice.checked = true;
        lineChoice.disabled = true;
    } else {
        lineChoice.disabled = false;

        if (controls instanceof LineControls || controls instanceof ShortestLineControls) {
            lineChoice.checked = true;
            shortestLine.checked = edge.controls instanceof ShortestLineControls;
        } else if (controls instanceof BezierControls) {
            bezierChoice.checked = true;
        }
    }
};

const checkNull = <T>(f: (_: T) => void) => (arg: T) => {
    if (selectedEdge !== null)
        f(arg);
};

export const changeEdgeType = checkNull((evt: Event) => {
    const { edgeType: { value } } = inputs;
    selectedEdge.textPathElem.textContent = selectedEdge.type = EdgeType[value];
});

const changeShortestLine = checkNull((evt: Event) => {
    selectedEdge.controls.hide();
    const controlsType = inputs.shortestLine.checked ?
        ShortestLineControls : LineControls;
    selectedEdge.controls = new controlsType(selectedEdge);
    selectedEdge.controls.show();
});

const selectLineChoice = checkNull((evt: Event) => {
    selectedEdge.controls.hide();
    inputs.shortestLine.checked = false;
    selectedEdge.controls = new LineControls(selectedEdge);
    selectedEdge.controls.show();
});

const selectBezierChoice = checkNull((evt: Event) => {
    selectedEdge.controls.hide();
    selectedEdge.controls = new BezierControls(selectedEdge, false);
    selectedEdge.controls.show();
});

const clickReverseEdge = checkNull((evt: Event) => {
    selectedEdge.controls.toggleReversed();
    selectedEdge.controls.updatePath();
});

inputs.edgeType.addEventListener("change", changeEdgeType);
inputs.shortestLine.addEventListener("change", changeShortestLine);
inputs.bezierChoice.addEventListener("change", selectBezierChoice);
inputs.lineChoice.addEventListener("change", selectLineChoice);
inputs.reverseEdge.addEventListener("click", clickReverseEdge);

form.addEventListener("submit", e => e.preventDefault());
