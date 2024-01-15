import { AccessType, State, dumpGraph, formatState, } from "./main.js";

export const form = document.querySelector<HTMLFormElement>("#state-config");

export const inputs = {
    read: form.querySelector<HTMLInputElement>("#read"),
    write: form.querySelector<HTMLInputElement>("#write"),
    addr: form.querySelector<HTMLInputElement>("#addr"),
    value: form.querySelector<HTMLInputElement>("#value"),
    ending: form.querySelector<HTMLInputElement>("#ending")
}

let selectedState: State = null;

export const initForm = (state: State) => {
    selectedState = state;

    const { read, write, addr, value, ending } = inputs;

    switch (state.access) {
        case AccessType.Read:
            read.checked = true;
            break;
        case AccessType.Write:
            write.checked = true;
            break;
    }

    ending.checked = selectedState.ending;
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
    const { ending } = inputs;

    selectedState.ending = ending.checked;

    if (ending.checked)
        selectedState.groupElem.classList.add("ending");
    else
        selectedState.groupElem.classList.remove("ending");

    updateText();
    dumpGraph();
});

inputs.read.addEventListener("change", changeAccess);
inputs.write.addEventListener("change", changeAccess);
inputs.addr.addEventListener("input", inputAddr);
inputs.value.addEventListener("input", inputValue);
inputs.ending.addEventListener("change", changeType);
