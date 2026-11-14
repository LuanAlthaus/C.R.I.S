// Loads the Bevy WASM sim built by scripts/build-sim.sh and exposes its exports as window.marsSim.
import init, * as sim from "./mars_sim.js";
try {
  await init();
} catch (e) {
  // winit hands control to the browser event loop by throwing; that is not a failure.
  if (!String(e).includes("Using exceptions for control flow")) {
    window.dispatchEvent(new Event("mars-sim-error"));
    throw e;
  }
}
window.marsSim = sim;
window.dispatchEvent(new Event("mars-sim-loaded"));
