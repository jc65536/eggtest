import { AccessType, State, StateType, dumpGraph, formatState } from "./main.js";

export const form = document.querySelector<HTMLFormElement>("#state-config");

export const inputs = {
    read: form.querySelector<HTMLInputElement>("#read"),
    write: form.querySelector<HTMLInputElement>("#write"),
    addr: form.querySelector<HTMLInputElement>("#addr"),
    value: form.querySelector<HTMLInputElement>("#value"),
    starting: form.querySelector<HTMLInputElement>("#starting"),
    ending: form.querySelector<HTMLInputElement>("#ending"),
    neither: form.querySelector<HTMLInputElement>("#neither")
}

let selectedState: State = null;

export const initForm = (state: State) => {
    selectedState = state;

    const { read, write, addr, value, starting, ending, neither } = inputs;

    switch (state.access) {
        case AccessType.Read:
            read.checked = true;
            break;
        case AccessType.Write:
            write.checked = true;
            break;
    }

    ({
        [StateType.Starting]: starting,
        [StateType.Ending]: ending,
        [StateType.Neither]: neither
    })[state.type].checked = true;

    addr.value = state.addr;
    value.value = state.value;
};

const checkNull = <T>(f: (_: T) => void) => (arg: T) => {
    if (selectedState !== null)
        f(arg);
};

const updateText = () => {
    selectedState.textElem.textContent = formatState(selectedState);
};

const inputAddr = checkNull((evt: Event) => {
    selectedState.addr = inputs.addr.value;
    updateText();
    dumpGraph();
});

const inputValue = checkNull((evt: Event) => {
    selectedState.value = inputs.value.value;
    updateText();
    dumpGraph();
});

const changeAccess = checkNull((evt: Event) => {
    const { read, write } = inputs;

    if (read.checked)
        selectedState.access = AccessType.Read;
    else if (write.checked)
        selectedState.access = AccessType.Write;

    updateText();
    dumpGraph();
});

const changeType = checkNull((evt: Event) => {
    const {starting, ending, neither} = inputs;

    if (starting.checked) {
        selectedState.type = StateType.Starting;
        selectedState.groupElem.classList.remove("ending");
        selectedState.groupElem.classList.add("starting");
    } else if (ending.checked) {
        selectedState.type = StateType.Ending;
        selectedState.groupElem.classList.remove("starting");
        selectedState.groupElem.classList.add("ending");
    } else {
        selectedState.type = StateType.Neither;
        selectedState.groupElem.classList.remove("starting");
        selectedState.groupElem.classList.remove("ending");
    }

    updateText();
    dumpGraph();
});

inputs.read.addEventListener("change", changeAccess);
inputs.write.addEventListener("change", changeAccess);
inputs.addr.addEventListener("input", inputAddr);
inputs.value.addEventListener("input", inputValue);

inputs.starting.addEventListener("change", changeType);
inputs.ending.addEventListener("change", changeType);
inputs.neither.addEventListener("change", changeType);
