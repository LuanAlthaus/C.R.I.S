"use client";

// Hosts the Bevy WASM canvas. public/sim/boot.js loads the module and exposes window.marsSim.
// Code written with AI assistance (Claude / omp); see docs/SI_USAGE.md.
import { useEffect, useState } from "react";
import { onSimEvent } from "@/lib/marsSim";

const BOOT_SRC = "/sim/boot.js";

export default function MarsSim({ className = "" }: { className?: string }) {
  const [status, setStatus] = useState<"loading" | "ready" | "error">("loading");

  useEffect(() => {
    const offReady = onSimEvent("mars-sim-ready", () => setStatus("ready"));
    const onError = () => setStatus("error");
    window.addEventListener("mars-sim-error", onError);
    if (!document.querySelector(`script[src="${BOOT_SRC}"]`)) {
      const s = document.createElement("script");
      s.type = "module";
      s.src = BOOT_SRC;
      s.onerror = onError;
      document.body.appendChild(s);
    }
    return () => {
      offReady();
      window.removeEventListener("mars-sim-error", onError);
    };
  }, []);

  return (
    <div className={`relative h-full w-full overflow-hidden ${className}`}>
      {/* Focus on press: the sim only receives keyboard camera controls while the canvas has focus. */}
      <canvas
        id="mars-canvas"
        tabIndex={0}
        aria-label="3D map of the Jezero crater EVA area. Click to place route points; the keyboard moves the camera."
        onPointerDown={(e) => e.currentTarget.focus({ preventScroll: true })}
        // Right-drag rotates the map; the browser menu would swallow the drag.
        onContextMenu={(e) => e.preventDefault()}
        className="absolute inset-0 h-full w-full outline-none"
      />
      {status !== "ready" && (
        <div
          role="status"
          className="pointer-events-none absolute inset-0 grid place-items-center text-sm text-[var(--muted)]"
        >
          {status === "error" ? "3D view failed to load (WebGL2 required)." : "Loading 3D terrain…"}
        </div>
      )}
    </div>
  );
}
