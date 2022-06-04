//
// This is only a SKELETON file for the 'React' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

class ListenerCell {
  constructor() {
    this.listeners = [];
  }

  addListener(listener) {
    this.listeners.push(listener);
  }

  updateListeners() {
    this.listeners.forEach((listener) => listener.update());
  }
}

export class InputCell extends ListenerCell {
  constructor(value) {
    super();
    this.setValue(value);
  }

  setValue(value) {
    this.value = value;
    this.updateListeners();
  }
}

export class ComputeCell extends ListenerCell {
  constructor(inputCells, fn) {
    super();
    this.inputCells = inputCells;
    this.fn = fn;
    inputCells.forEach((cell) => cell.addListener(this));

    this.value = this.fn(this.inputCells);
    this.initCallbacks();
  }

  initCallbacks() {
    this.callbacks = new Set();
    this.updatesBeforeTriggeringCallbacks = this.inputCells.length;
    this.valueAtLastCallback = this.value;
  }

  update() {
    this.value = this.fn(this.inputCells);
    this.updateListeners();
    this.queueCallbacks();
  }

  addCallback(cb) {
    this.callbacks.add(cb);
  }

  queueCallbacks() {
    this.updatesBeforeTriggeringCallbacks -= 1;
    if (this.updatesBeforeTriggeringCallbacks === 0) this.updateCallbacks();
  }

  updateCallbacks() {
    this.updatesBeforeTriggeringCallbacks = this.inputCells.length;
    if (this.valueAtLastCallback !== this.value) {
      this.callbacks.forEach((cb) => cb.update(this));
      this.valueAtLastCallback = this.value;
    }
  }

  removeCallback(cb) {
    this.callbacks.delete(cb);
  }
}

export class CallbackCell {
  constructor(fn) {
    this.fn = fn;
    this.values = [];
  }

  update(cell) {
    this.values.push(this.fn(cell));
  }
}
