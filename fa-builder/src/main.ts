import * as vec from "./vector.js";
import * as dragMan from "./drag-manager.js";
import { Vec } from "./vector.js";
import { edgeConfig, stateConfig } from "./config.js";
import {
    createSvgElement, screenToSvgCoords, setAttributes, uniqueStr
} from "./util.js";
import {
    DragAddStateCtx, DragEdgeCtx, DragSelectionCtx, DragStatesCtx
} from "./drag.js";
import {
    BezierControls, ControlHandle, PathControls,
    ShortestLineControls, StartingEdgeControls
} from "./path-controls.js";
import { cancelSelection, selectedStates } from "./selection.js";

// Important DOM elements

export const canvas = document.querySelector<SVGSVGElement>("#canvas");
export const stateLayer = document.querySelector<SVGGElement>("#state-layer");
export const edgeLayer = document.querySelector<SVGGElement>("#edge-layer");
export const topLayer = document.querySelector<SVGGElement>("#top-layer");

const addStateElem = document.querySelector<HTMLButtonElement>("#add-state");

export const configMenuContainer =
    document.querySelector<HTMLDivElement>("#config-menu-container");

// State machine data types and global structures for storing the state machine

export enum AccessType { Read = "R", Write = "W" }

export enum StateType { Starting, Ending, Neither }

export type State = {
    addr: string,
    access: AccessType,
    value: string,
    type: StateType,
    groupElem: SVGGElement,
    textElem: SVGTextElement,
    pos: Vec,
    inEdges: Set<Edge>,
    outEdges: Set<Edge>,
    handles: Set<ControlHandle>
};

export enum EdgeType {
    po = "po", rf = "rf", dmb = "dmb",
    lwsync = "lwsync", addr = "addr", ctrl = "ctrl",
    ctrlisb = "ctrlisb", co = "co", data = "data",
    fr = "fr"
}

export type Edge = {
    startState: State,
    type: EdgeType,
    endState: State,
    pathElem: SVGPathElement,
    textElem: SVGTextElement,
    textPathElem: SVGTextPathElement,
    controls: PathControls
};

export const states = new Set<State>();
export const edges = new Set<Edge>();

// Basic interactions with states/edges

export const formatState = (state: State) => `${state.access}${state.addr}=${state.value}`;

export const addState = (pos: Vec) => {
    const rad = stateConfig.radius.toString();
    const circle = createSvgElement("circle");
    circle.setAttribute("r", rad);

    const group = createSvgElement("g");
    group.classList.add("state");
    group.appendChild(circle);

    const trans = canvas.createSVGTransform();
    trans.setTranslate(pos[0], pos[1]);
    group.transform.baseVal.appendItem(trans);

    const text = createSvgElement("text");
    text.textContent = "R[x]=0";
    text.classList.add("state-name");
    group.appendChild(text);

    const state: State = {
        addr: "x",
        access: AccessType.Read,
        value: "0",
        type: StateType.Neither,
        groupElem: group,
        textElem: text,
        pos: pos,
        inEdges: new Set(),
        outEdges: new Set(),
        handles: new Set()
    };

    group.addEventListener("mousedown", startDragOnState(state));
    states.add(state);
    stateLayer.appendChild(group);
    dumpGraph();
};

export const deleteState = (state: State) => {
    state.outEdges.forEach(deleteEdge);
    state.inEdges.forEach(deleteEdge);

    state.groupElem.remove();
    states.delete(state);
    dumpGraph();
};

export const addEdge = (edge: Edge) => {
    if (edge.startState === edge.endState) {
        edge.controls = new BezierControls(edge, false);
    } else {
        edge.startState.outEdges.forEach(e => {
            if (e.endState === edge.endState &&
                e.controls instanceof ShortestLineControls)
                e.controls = new BezierControls(e, true);
        });

        edge.endState.outEdges.forEach(e => {
            if (e.endState === edge.startState &&
                e.controls instanceof ShortestLineControls)
                e.controls = new BezierControls(e, true);
        });

        edge.controls = new ShortestLineControls(edge);
    }

    edges.add(edge);
    edge.startState.outEdges.add(edge);
    edge.endState.inEdges.add(edge);

    const id = `edge-${uniqueStr("edge")}`;

    edge.pathElem.id = id;
    edgeLayer.appendChild(edge.pathElem);

    const transCharContainer = createSvgElement("text");
    transCharContainer.setAttribute("dy", edgeConfig.textVertOffset.toString());
    transCharContainer.classList.add("trans-char-container");
    edge.textElem = transCharContainer;

    const textPath = createSvgElement("textPath");
    textPath.setAttribute("startOffset", "50%");
    textPath.setAttribute("href", `#${id}`);
    textPath.textContent = edge.type;
    edge.textPathElem = textPath;
    transCharContainer.appendChild(textPath);

    edgeLayer.appendChild(transCharContainer);
    dumpGraph();
}

export const deleteEdge = (edge: Edge) => {
    edge.startState.outEdges.delete(edge);
    edge.endState.inEdges.delete(edge);
    edge.pathElem.remove();
    edge.textElem.remove();
    edges.delete(edge);
    dumpGraph();
};

// Drag initialization functions

const startDragOnState = (state: State) => (evt: MouseEvent) => {
    if (dragMan.hasContext())
        return;

    switch (evt.button) {
        case 0:
            const statesToDrag = selectedStates.has(state) ?
                selectedStates : new Set([state]);

            dragMan.setContext(new DragStatesCtx(statesToDrag,
                screenToSvgCoords([evt.x, evt.y])));
            break;

        case 2:
            dragMan.setContext(new DragEdgeCtx(state));
            break;
    }
}

const startDragAddState = (evt: MouseEvent) => {
    const circle = document.createElement("div");
    circle.classList.add("statelike", "draggable");

    const rect = addStateElem.getBoundingClientRect();
    circle.style.left = `${rect.x}px`;
    circle.style.top = `${rect.y}px`;

    const offset = vec.sub([evt.x, evt.y])([rect.x, rect.y]);

    dragMan.setContext(new DragAddStateCtx(offset, circle));

    addStateElem.appendChild(circle);
}

const startDragSelection = (evt: MouseEvent) => {
    if (dragMan.hasContext() || evt.button !== 0)
        return;

    const init = screenToSvgCoords([evt.x, evt.y]);
    const rect = createSvgElement("rect");
    rect.classList.add("selection");

    setAttributes(rect, ["x", "y", "width", "height"],
        init.concat([0, 0]).map(x => x.toString()));

    cancelSelection();

    dragMan.setContext(new DragSelectionCtx(init, rect));

    topLayer.appendChild(rect);
};

addStateElem.addEventListener("mousedown", startDragAddState);

canvas.addEventListener("contextmenu", evt => evt.preventDefault());
canvas.addEventListener("mousedown", startDragSelection);

document.addEventListener("mousemove", dragMan.handleDrag);
document.addEventListener("mouseup", dragMan.handleDrop);

const output = document.querySelector<HTMLPreElement>("#output");

export const dumpGraph = () => {
    const uniqueName = (() => {
        let i = 1;
        return () => i++;
    })();

    const stateNames = new Map<State, number>();

    states.forEach(state => stateNames.set(state, uniqueName()));

    const initial: string[] = ["0: ~"];

    const lines: string[] = [];

    stateNames.forEach((name, state) => {
        const ending = state.type === StateType.Ending ? "$" : "";
        const line = [`${ending}${name}: ${formatState(state)}`];
        state.outEdges.forEach(edge => line.push(`${edge.type} -> ${stateNames.get(edge.endState)}`));
        lines.push(line.join(" | ") + "\n");
        if (state.type === StateType.Starting)
            initial.push(`~ -> ${stateNames.get(state)}`);
    });

    const allLines = [initial.join(" | ") + "\n", ...lines];

    output.textContent = allLines.join("");
};
