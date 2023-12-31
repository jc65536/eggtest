import {
    AccessType,
    getStartingState, setStartingState, State, toggleAccept
} from "./main.js";

export const form = document.querySelector<HTMLFormElement>("#state-config");

export const inputs = {
    read: form.querySelector<HTMLInputElement>("#read"),
    write: form.querySelector<HTMLInputElement>("#write"),
    addr: form.querySelector<HTMLInputElement>("#addr"),
    value: form.querySelector<HTMLInputElement>("#value"),
    starting: form.querySelector<HTMLInputElement>("#starting"),
    accepting: form.querySelector<HTMLInputElement>("#accepting")
}

let selectedState: State = null;

export const initForm = (state: State) => {
    selectedState = state;

    const { read, write, addr, value, starting, accepting } = inputs;

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
    starting.checked = state === getStartingState();
    accepting.checked = state.accepting;
};

const checkNull = <T>(f: (_: T) => void) => (arg: T) => {
    if (selectedState !== null)
        f(arg);
};

const updateText = (state: State) => {
    const { textElem, access, addr, value } = state;
    textElem.textContent = `${access}[${addr}]=${value}`;
};

const inputAddr = checkNull((evt: Event) => {
    selectedState.addr = inputs.addr.value;
    updateText(selectedState);
});

const inputValue = checkNull((evt: Event) => selectedState.value = inputs.value.value);

export const changeStarting = checkNull((evt: Event) =>
    setStartingState(inputs.starting.checked ? selectedState : null));

const changeAccepting = checkNull((evt: Event) => toggleAccept(selectedState));

const changeAccess = checkNull((evt: Event) => {
    const { read, write } = inputs;

    if (read.checked)
        selectedState.access = AccessType.Read;
    else if (write.checked)
        selectedState.access = AccessType.Write;

    updateText(selectedState);
});

inputs.read.addEventListener("change", changeAccess);
inputs.write.addEventListener("change", changeAccess);
inputs.addr.addEventListener("input", inputAddr);
inputs.value.addEventListener("input", inputValue);
inputs.starting.addEventListener("change", changeStarting);
inputs.accepting.addEventListener("change", changeAccepting);
