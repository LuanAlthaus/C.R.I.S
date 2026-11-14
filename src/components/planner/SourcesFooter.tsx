// Every data source the planner shows, in text (no agency logos, per Space Apps rules).
// Code written with AI assistance (Claude / omp); see docs/SI_USAGE.md.
import { labelCls, type SiteMeta } from "./ui";

const FIXED_SOURCES: [string, string][] = [
  ["Space weather", "DONKI WSA-Enlil simulations (NASA CCMC / M2M) via api.nasa.gov"],
  ["Mars weather", "NASA InSight APSS/TWINS via api.nasa.gov (Elysium Planitia, 2020)"],
  ["Earth comparison", "MODIS Terra/Aqua Land Surface Temperature via NASA GIBS / Worldview"],
  ["First-person view", "Perseverance raw images, NASA/JPL-Caltech (mars.nasa.gov)"],
  [
    "Rover 3D model",
    "Perseverance 3D model, NASA/JPL-Caltech (NASA 3D Resources); agency logos and mission insignia removed",
  ],
  [
    "Crew limits",
    "NASA-STD-3001 Space Flight Human-System Standard Vol. 1 & 2; OCHMO technical briefs TB-009 and TB-050",
  ],
  ["Suit mass", "NASA Mars suit mass target, NTRS 20260005410"],
];

export default function SourcesFooter({ meta }: { meta: SiteMeta | null }) {
  const sources: [string, string][] = [
    ...(meta?.credits ?? []).map((c): [string, string] => [c.layer, c.text]),
    ...FIXED_SOURCES,
  ];
  return (
    <footer aria-labelledby="sec-sources" className="bg-bg px-4 py-5 text-xs leading-relaxed text-muted">
      <h2 id="sec-sources" className={labelCls}>
        Sources
      </h2>
      <dl className="mt-3 space-y-2">
        {sources.map(([layer, text]) => (
          <div key={layer + text}>
            <dt className="font-mono text-[10px] uppercase tracking-wider text-ink">{layer}</dt>
            <dd className="break-words">{text}</dd>
          </div>
        ))}
      </dl>
      <p className="mt-4 font-medium text-ink">Data: NASA. Not endorsed by NASA.</p>
      <p className="mt-1">Code written with AI assistance — see docs/SI_USAGE.md.</p>
    </footer>
  );
}
