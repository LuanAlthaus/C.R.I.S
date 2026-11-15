// Every data source the planner shows, in text (no agency logos, per Space Apps rules).
// Code written with AI assistance (Claude / omp); see docs/SI_USAGE.md.
import { BookIcon } from "./icons";
import { Collapsible, type SiteMeta } from "./ui";

const FIXED_SOURCES: [string, string][] = [
  [
    "Fine relief near the camera",
    "Estimated from the shading of the HiRISE orthomosaic (photoclinometry, 2 m grid). Visual only: it follows the HiRISE DTM loosely and is not used to plan routes",
  ],
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
    <Collapsible id="sec-sources" icon={<BookIcon />} title="Sources" summary={`${sources.length} cited`}>
      <dl className="space-y-2 text-xs leading-relaxed text-muted">
        {sources.map(([layer, text]) => (
          <div key={layer + text}>
            <dt className="font-mono text-[10px] uppercase tracking-wider text-ink">{layer}</dt>
            <dd className="break-words">{text}</dd>
          </div>
        ))}
      </dl>
      <p className="text-xs text-muted">Code written with AI assistance — see docs/SI_USAGE.md.</p>
    </Collapsible>
  );
}
