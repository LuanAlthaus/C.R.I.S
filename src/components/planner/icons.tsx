// Small line icons for the planner panel and the map overlay (16 px grid, currentColor).
// Code written with AI assistance (Claude / omp); see docs/SI_USAGE.md.
import type { ReactNode } from "react";

type P = { className?: string };

function Svg({ className = "size-4", children }: P & { children: ReactNode }) {
  return (
    <svg
      aria-hidden
      viewBox="0 0 16 16"
      fill="none"
      stroke="currentColor"
      strokeWidth="1.4"
      strokeLinecap="round"
      strokeLinejoin="round"
      className={className}
    >
      {children}
    </svg>
  );
}

export const ChevronIcon = (p: P) => (
  <Svg {...p}>
    <path d="m6 3.5 4.5 4.5L6 12.5" />
  </Svg>
);
export const CrewIcon = (p: P) => (
  <Svg {...p}>
    <circle cx="8" cy="4.6" r="2.4" />
    <path d="M3.2 13.8c.4-2.8 2.3-4.4 4.8-4.4s4.4 1.6 4.8 4.4" />
  </Svg>
);
export const ShieldIcon = (p: P) => (
  <Svg {...p}>
    <path d="M8 1.8 13 3.6v4c0 3.1-2.1 5.4-5 6.6-2.9-1.2-5-3.5-5-6.6v-4z" />
  </Svg>
);
export const ShieldCheckIcon = (p: P) => (
  <Svg {...p}>
    <path d="M8 1.8 13 3.6v4c0 3.1-2.1 5.4-5 6.6-2.9-1.2-5-3.5-5-6.6v-4z" />
    <path d="m5.7 8 1.6 1.6 3-3.2" />
  </Svg>
);
export const LayersIcon = (p: P) => (
  <Svg {...p}>
    <path d="M8 2 14 5.2 8 8.4 2 5.2z" />
    <path d="m2 8 6 3.2L14 8M2 10.8 8 14l6-3.2" />
  </Svg>
);
export const SunIcon = (p: P) => (
  <Svg {...p}>
    <circle cx="8" cy="8" r="2.8" />
    <path d="M8 1.5v1.4M8 13.1v1.4M1.5 8h1.4M13.1 8h1.4M3.4 3.4l1 1M11.6 11.6l1 1M3.4 12.6l1-1M11.6 4.4l1-1" />
  </Svg>
);
export const CameraIcon = (p: P) => (
  <Svg {...p}>
    <path d="M2 5.2c0-.7.5-1.2 1.2-1.2h1.6l1-1.5h4.4l1 1.5h1.6c.7 0 1.2.5 1.2 1.2v6.6c0 .7-.5 1.2-1.2 1.2H3.2c-.7 0-1.2-.5-1.2-1.2z" />
    <circle cx="8" cy="8.4" r="2.3" />
  </Svg>
);
export const BookIcon = (p: P) => (
  <Svg {...p}>
    <path d="M2.5 3c2-.6 3.9-.4 5.5.8 1.6-1.2 3.5-1.4 5.5-.8v10c-2-.6-3.9-.4-5.5.8-1.6-1.2-3.5-1.4-5.5-.8z" />
    <path d="M8 3.8v10" />
  </Svg>
);
export const PlayIcon = (p: P) => (
  <svg aria-hidden viewBox="0 0 16 16" className={p.className ?? "size-4"}>
    <path d="M4.5 2.8v10.4L13 8z" fill="currentColor" />
  </svg>
);
export const PauseIcon = (p: P) => (
  <svg aria-hidden viewBox="0 0 16 16" className={p.className ?? "size-4"}>
    <path d="M4 3h3v10H4zM9 3h3v10H9z" fill="currentColor" />
  </svg>
);
export const ResetIcon = (p: P) => (
  <Svg {...p}>
    <path d="M3 8a5 5 0 1 0 1.6-3.7M3 2.5v2.8h2.8" />
  </Svg>
);
export const PanelIcon = (p: P) => (
  <Svg {...p}>
    <rect x="2" y="2.5" width="12" height="11" rx="2" />
    <path d="M6.2 2.5v11" />
  </Svg>
);
export const PinIcon = (p: P) => (
  <Svg {...p}>
    <path d="M8 14.2s4.3-4 4.3-7.4a4.3 4.3 0 1 0-8.6 0c0 3.4 4.3 7.4 4.3 7.4z" />
    <circle cx="8" cy="6.7" r="1.5" />
  </Svg>
);
