import ChallengeGrid from "@/components/ChallengeGrid";
import { challenges } from "@/data/challenges";

// Página de inicio del template. Reemplázala por tu proyecto cuando empiece el hackathon.
const REPO_URL =
  process.env.NEXT_PUBLIC_REPO_URL || "https://github.com/marcosferr/nasa-space-apps-starter-template";

const STACK = [
  { k: "Frontend", v: "Next.js 16 · React 19 · Tailwind 4", href: "https://vercel.com/new" },
  { k: "Datos / Auth", v: "Supabase Postgres + RLS", href: "https://supabase.com/dashboard" },
  { k: "Deploy", v: "Vercel Hobby (gratis)", href: `${REPO_URL}/blob/main/docs/deploy/vercel-supabase.md` },
  { k: "Agentes", v: "Skills en .claude/skills", href: `${REPO_URL}/tree/main/.claude/skills` },
];

const ENDPOINTS = [
  { path: "/api/health", note: "qué está configurado" },
  { path: "/api/nasa/apod", note: "proxy a api.nasa.gov con caché" },
  { path: "/api/notes", note: "lectura/escritura en Supabase" },
];

export default function Home() {
  return (
    <div className="relative flex-1">
      <div aria-hidden className="grid-paper pointer-events-none absolute inset-x-0 top-0 h-[520px] opacity-60" />
      <main className="relative mx-auto w-full max-w-6xl px-4 pb-24 pt-14 sm:px-8">
        <p className="font-mono text-xs uppercase tracking-[0.2em] text-signal">
          Space Apps Challenge 2026 · 14–15 Nov · The Next Frontier
        </p>
        <h1 className="mt-5 max-w-4xl font-display text-5xl leading-[1.02] sm:text-7xl">
          Un punto de partida <em className="text-signal">sin costo</em> para construir con datos de NASA.
        </h1>
        <p className="mt-6 max-w-2xl text-lg leading-relaxed text-muted">
          Template no oficial: Next.js + Supabase listo para desplegar en Vercel, skills para agentes de código y los
          recursos de los 14 challenges. Sin lógica de ningún challenge: la idea la pone tu equipo.
        </p>

        <dl className="mt-10 grid gap-px overflow-hidden rounded-lg border border-line bg-line sm:grid-cols-4">
          {STACK.map((s) => (
            <a key={s.k} href={s.href} target="_blank" rel="noreferrer" className="group bg-surface p-4">
              <dt className="font-mono text-[11px] uppercase tracking-wider text-muted">{s.k}</dt>
              <dd className="mt-1 text-sm font-medium group-hover:text-signal">{s.v}</dd>
            </a>
          ))}
        </dl>

        <section aria-labelledby="api-heading" className="mt-10">
          <h2 id="api-heading" className="font-mono text-xs uppercase tracking-wider text-muted">
            Endpoints de ejemplo
          </h2>
          <ul className="mt-3 flex flex-wrap gap-3">
            {ENDPOINTS.map((e) => (
              <li key={e.path}>
                <a
                  href={e.path}
                  className="inline-flex items-baseline gap-2 rounded-md border border-line bg-surface px-3 py-2 font-mono text-sm hover:border-signal"
                >
                  <span className="text-ink">{e.path}</span>
                  <span className="text-xs text-muted">{e.note}</span>
                </a>
              </li>
            ))}
          </ul>
        </section>

        <ChallengeGrid challenges={challenges} repoUrl={REPO_URL} />

        <footer className="mt-20 border-t border-line pt-6 text-xs leading-relaxed text-muted">
          <p>
            Proyecto comunitario, no afiliado a NASA ni respaldado por NASA. Datos de los challenges:{" "}
            <a className="underline" href="https://www.spaceappschallenge.org/2026/challenges/">
              spaceappschallenge.org
            </a>
            . Sin logos de NASA, por las reglas del evento.
          </p>
        </footer>
      </main>
    </div>
  );
}
