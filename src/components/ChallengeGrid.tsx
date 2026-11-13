"use client";

import { useMemo, useState } from "react";
import { type Challenge, officialUrl } from "@/data/challenges";

const DOMAINS = ["All", "Tierra", "Luna", "Marte", "Espacio profundo", "Humanos en el espacio", "Educación"] as const;

const LEVEL_SHORT: Record<string, string> = { "Beginner/Youth": "BEG", Intermediate: "INT", Advanced: "ADV" };

export default function ChallengeGrid({ challenges, repoUrl }: { challenges: Challenge[]; repoUrl: string }) {
  const [domain, setDomain] = useState<(typeof DOMAINS)[number]>("All");
  const [q, setQ] = useState("");

  const shown = useMemo(() => {
    const needle = q.trim().toLowerCase();
    return challenges.filter(
      (c) =>
        (domain === "All" || c.domain === domain) &&
        (!needle || `${c.title} ${c.titleEs} ${c.subjects.join(" ")} ${c.summaryEs}`.toLowerCase().includes(needle)),
    );
  }, [challenges, domain, q]);

  return (
    <section aria-labelledby="challenges-heading" className="mt-16">
      <div className="flex flex-wrap items-end justify-between gap-4 border-b border-line pb-4">
        <h2 id="challenges-heading" className="font-display text-4xl">
          14 challenges
        </h2>
        <label className="flex items-center gap-2 font-mono text-xs uppercase tracking-wider text-muted">
          Buscar
          <input
            value={q}
            onChange={(e) => setQ(e.target.value)}
            placeholder="SAR, Mars, games…"
            className="w-48 rounded-md border border-line bg-surface px-3 py-1.5 font-sans text-sm normal-case tracking-normal text-ink placeholder:text-muted"
          />
        </label>
      </div>

      <div role="group" aria-label="Filtrar por dominio" className="mt-4 flex flex-wrap gap-2">
        {DOMAINS.map((d) => (
          <button
            key={d}
            onClick={() => setDomain(d)}
            aria-pressed={domain === d}
            className={`rounded-full border px-3 py-1 font-mono text-xs transition-colors ${
              domain === d ? "border-signal bg-signal-soft text-signal" : "border-line text-muted hover:text-ink"
            }`}
          >
            {d === "All" ? "Todos" : d}
          </button>
        ))}
      </div>

      <p className="mt-3 font-mono text-xs text-muted" aria-live="polite">
        {shown.length} / {challenges.length}
      </p>

      <ol className="mt-4 grid gap-px overflow-hidden rounded-lg border border-line bg-line sm:grid-cols-2 lg:grid-cols-3">
        {shown.map((c) => (
          <li key={c.slug} className="flex flex-col bg-surface p-5">
            <div className="flex items-center justify-between font-mono text-[11px] uppercase tracking-wider text-muted">
              <span>
                {String(c.n).padStart(2, "0")} · {c.domain}
              </span>
              <span className="flex gap-1">
                {c.difficulty.map((d) => (
                  <span key={d} title={d} className="rounded border border-line px-1">
                    {LEVEL_SHORT[d]}
                  </span>
                ))}
              </span>
            </div>
            <h3 className="mt-3 text-[15px] font-semibold leading-snug">{c.title}</h3>
            <p className="mt-1 font-display text-lg italic text-muted">{c.titleEs}</p>
            <p className="mt-3 flex-1 text-sm leading-relaxed text-muted">{c.summaryEs}</p>
            <div className="mt-4 flex gap-4 font-mono text-xs">
              <a href={officialUrl(c)} target="_blank" rel="noreferrer" className="text-signal underline-offset-4 hover:underline">
                Oficial ↗
              </a>
              <a
                href={`${repoUrl}/blob/main/${c.doc}`}
                target="_blank"
                rel="noreferrer"
                className="text-ink underline-offset-4 hover:underline"
              >
                Datos y recursos ↗
              </a>
            </div>
          </li>
        ))}
      </ol>
    </section>
  );
}
