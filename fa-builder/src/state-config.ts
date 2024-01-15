import { AccessType, State, dumpGraph } from "./main.js";

export const form = document.querySelector<HTMLFormElement>("#state-config");

export const inputs = {
    read: form.querySelector<HTMLInputElement>("#read"),
    write: form.querySelector<HTMLInputElement>("#write"),
    addr: form.querySelector<HTMLInputElement>("#addr"),
    value: form.querySelector<HTMLInputElement>("#value"),
}

let selectedState: State = null;

export const initForm = (state: State) => {
    selectedState = state;

    const { read, write, addr, value } = inputs;

    switch (state.access) {
        case AccessType.Read:
            read.checked = true;
            break;
        case AccessType.Write:
            write.checked = true;
            break;
    }

    addr.value = state.addr;
    value.value = state.value;
};

const checkNull = <T>(f: (_: T) => void) => (arg: T) => {
    if (selectedState !== null)
        f(arg);
};

const updateText = () => {
    const { textElem, access, addr, value } = selectedState;
    textElem.textContent = `${access}[${addr}]=${value}`;
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

inputs.read.addEventListener("change", changeAccess);
inputs.write.addEventListener("change", changeAccess);
inputs.addr.addEventListener("input", inputAddr);
inputs.value.addEventListener("input", inputValue);
