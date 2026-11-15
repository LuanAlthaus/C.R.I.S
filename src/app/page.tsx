// Code written with AI assistance (Claude / omp); see docs/SI_USAGE.md.
// Main Menu: Mars from orbit, the project name, and the way into the Map Viewer.

import { Antonio, Inter } from "next/font/google";
import Link from "next/link";
import MarsGlobe from "@/components/MarsGlobe";

const inter = Inter({ variable: "--font-inter", subsets: ["latin"] });
const antonio = Antonio({ variable: "--font-antonio", subsets: ["latin"] });

const MAP_VIEWER = "/map";
// The About Us window is not built yet: until it is, both About links scroll to the summary below.
const ABOUT = "#about";

const control =
  "inline-flex min-h-11 items-center text-[11px] font-bold uppercase tracking-[0.22em] transition-colors duration-300";

const FACTS: [string, string][] = [
  ["Site", "Jezero crater, around the Octavia E. Butler landing site, 18.4447° N 77.4508° E"],
  ["Area", "10 × 10 km"],
  ["Elevation", "HiRISE digital terrain model mosaic, 1 m per pixel"],
  ["Imagery", "HiRISE orthomosaic, down to 0.32 m per pixel near the camera"],
  ["Route grid", "1,000 × 1,000 cells of 10 m, searched with A*"],
  ["Slope limit", "Under 20° on foot, NASA's Artemis EVA limit (EVA-EXP-0042 Rev B), lowered for heavy loads"],
  ["Rover mode", "Under 45°, an experimental simulation setting, not a tested rover capability"],
  ["Step limit", "0.30 m of local relief, the suited step height NASA tested (ICES-2025-294)"],
  ["Default load", "80 kg crew member, 104 kg suit, 20 kg payload, in Mars gravity of 3.71 m/s²"],
];

function StartLink({ className = "" }: { className?: string }) {
  return (
    <Link
      href={MAP_VIEWER}
      className={`group relative inline-flex h-14 items-center gap-4 overflow-hidden bg-space-ink px-8 text-xs font-bold uppercase tracking-[0.24em] text-space ${className}`}
    >
      <span
        aria-hidden="true"
        className="absolute inset-0 origin-left scale-x-0 bg-mars transition-transform duration-500 ease-out group-hover:scale-x-100 group-focus-visible:scale-x-100"
      />
      <span className="relative">Start simulation</span>
      <span
        aria-hidden="true"
        className="relative h-px w-6 bg-current transition-[width] duration-500 ease-out group-hover:w-10"
      />
    </Link>
  );
}

export default function MainMenu() {
  return (
    <div className={`${inter.variable} ${antonio.variable} bg-space font-menu text-space-ink`}>
      <div className="relative isolate flex min-h-dvh flex-col overflow-hidden">
        <MarsGlobe />
        {/* Keeps the copy legible where it runs over the lit disc on narrow screens. */}
        <div
          aria-hidden="true"
          className="absolute inset-x-0 bottom-0 h-2/5 bg-linear-to-t from-space to-transparent lg:hidden"
        />

        <header className="relative flex items-center justify-between px-6 py-3 sm:px-10 lg:px-16 lg:py-6">
          <Link href="/" className="inline-flex min-h-11 items-center font-menu-display text-xl tracking-[0.18em]">
            C.R.I.S.
          </Link>
          <nav aria-label="Main" className="flex items-center gap-6 sm:gap-10">
            <a href={ABOUT} className={`${control} text-space-muted hover:text-space-ink`}>
              About us
            </a>
            <p className={`${control} gap-2.5 text-space-muted`}>
              <span aria-hidden="true" className="size-1.5 animate-status rounded-full bg-mars" />
              System online
            </p>
          </nav>
        </header>

        <main className="relative flex flex-1 flex-col justify-between px-6 pb-8 pt-4 sm:px-10 lg:justify-center lg:px-16 lg:pb-16 lg:pt-0">
          <div className="max-w-md">
            <h1 className="animate-menu-in font-menu-display text-[clamp(5.5rem,13vw,12rem)] leading-[0.92] font-light tracking-[0.02em]">
              C.R.I.S.
            </h1>
            <p className="mt-6 animate-menu-in text-lg font-thin tracking-[0.04em] [animation-delay:150ms] sm:text-xl">
              Center Reconnaissance Interplanetary Survival
            </p>
            <p className="mt-6 max-w-sm animate-menu-in leading-relaxed text-space-muted [animation-delay:300ms]">
              A Mars exploration simulator. Plan a spacesuit walk or a rover drive across Jezero crater, on terrain
              built from NASA orbital data.
            </p>
            <StartLink className="mt-9 animate-menu-in [animation-delay:450ms]" />
          </div>

          <section
            aria-labelledby="menu-about"
            className="mt-12 max-w-xs animate-menu-in border-t border-white/15 pt-4 [animation-delay:700ms] lg:absolute lg:bottom-10 lg:left-16 lg:mt-0"
          >
            <h2 id="menu-about" className="text-sm font-bold">
              About the project
            </h2>
            <p className="mt-1.5 text-sm leading-relaxed text-space-muted">
              One million 10 m cells of real Martian ground, and the safest way across them.
            </p>
            <a href={ABOUT} className={`${control} mt-1 text-space-ink hover:text-mars`}>
              Learn more
            </a>
          </section>
        </main>
      </div>

      <section
        id="about"
        aria-labelledby="about-title"
        className="mx-auto grid max-w-6xl scroll-mt-8 gap-x-20 gap-y-10 border-t border-space-line px-6 py-20 sm:px-10 lg:grid-cols-[5fr_6fr] lg:px-16 lg:py-28"
      >
        <div>
          <h2 id="about-title" className="text-4xl font-medium tracking-[-0.02em] sm:text-5xl">
            A walk on Mars, planned on the real ground
          </h2>
          <div className="mt-8 max-w-[52ch] space-y-5 leading-relaxed text-space-muted">
            <p>
              C.R.I.S. is a 3D simulator for planning extravehicular walks and rover drives across 10 × 10 km of
              Jezero crater, where NASA&rsquo;s Perseverance rover landed in 2021. The terrain is the one orbiters
              measured: elevation and imagery from the HiRISE camera on Mars Reconnaissance Orbiter.
            </p>
            <p>
              You pick a start and an end point. The planner returns the lowest-risk path between them. It never
              crosses a slope or a step beyond the limits below, and along the way it weighs mapped rocks, a terrain
              hazard model, the day-to-night temperature swing and exposure to the open sky.
            </p>
            <p>
              Built for the 2026 NASA Space Apps Challenge, &ldquo;Interplanetary Survival Guide: Martian
              Map&rdquo;.
            </p>
          </div>
          <StartLink className="mt-10" />
        </div>

        <div>
          <dl className="border-t border-space-line">
            {FACTS.map(([term, text]) => (
              <div key={term} className="grid grid-cols-[7.5rem_1fr] gap-4 border-b border-space-line py-3.5 text-sm">
                <dt className="text-space-muted">{term}</dt>
                <dd className="leading-relaxed">{text}</dd>
              </div>
            ))}
          </dl>
          <p className="mt-8 text-xs leading-relaxed text-space-muted">
            Data: NASA/JPL-Caltech, University of Arizona, USGS and Arizona State University. Globe: Viking Orbiter
            MDIM 2.1 colour mosaic, 232 m per pixel (NASA/JPL/USGS), via JPL Mars Trek; lighting and haze are
            rendered. Terrain: HiRISE DTM and orthomosaic (NASA/JPL/University of Arizona/USGS). Rocks and geology:
            Stack et al. 2020, doi:10.1007/s11214-020-00739-x; Sun and Stack 2020, USGS SIM 3464. Every layer is
            credited inside the Map Viewer.
          </p>
          <p className="mt-3 text-xs leading-relaxed text-space-muted">
            Not affiliated with or endorsed by NASA. Code written with AI assistance.
          </p>
        </div>
      </section>
    </div>
  );
}
