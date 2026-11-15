"use client";

// Perseverance first-person raw images for one sol, via /api/nasa/raw-images.
// Code written with AI assistance (Claude / omp); see docs/SI_USAGE.md.
import { useEffect, useRef, useState, type FormEvent } from "react";
import { CameraIcon } from "./icons";
import { buttonCls, citeCls, Collapsible, inputCls, labelCls } from "./ui";

interface RawImage {
  id: string;
  camera: string;
  src: string;
  large: string;
  credit: string;
  date_utc: string;
}

const CAMERA_NAMES: Record<string, string> = {
  NAVCAM_LEFT: "Navcam left",
  MCZ_LEFT: "Mastcam-Z left",
  FRONT_HAZCAM_LEFT_A: "Front Hazcam",
  REAR_HAZCAM_LEFT: "Rear Hazcam",
};

export interface NearestStop {
  which: "start" | "end";
  sol: number | null;
  dist_m: number | null;
}

export default function RawImagesSection({
  sol,
  onSol,
  near,
}: {
  sol: number;
  onSol: (sol: number) => void;
  /** Nearest Perseverance waypoint to the latest picked point (null before any pick). */
  near: NearestStop | null;
}) {
  const [res, setRes] = useState<{ sol: number; images?: RawImage[]; error?: string } | null>(null);
  const [active, setActive] = useState<RawImage | null>(null);
  const dialogRef = useRef<HTMLDialogElement>(null);

  useEffect(() => {
    const ac = new AbortController();
    fetch(`/api/nasa/raw-images?sol=${sol}`, { signal: ac.signal })
      .then(async (r) => {
        const j = await r.json();
        setRes(r.ok ? { sol, images: j.images } : { sol, error: r.status === 502 ? "The NASA raw image feed is not responding." : j.error });
      })
      .catch(() => {
        if (!ac.signal.aborted) setRes({ sol, error: "Could not reach the server." });
      });
    return () => ac.abort();
  }, [sol]);

  useEffect(() => {
    if (active && !dialogRef.current?.open) dialogRef.current?.showModal();
  }, [active]);

  const submit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    onSol(Number(new FormData(e.currentTarget).get("sol")));
  };
  const loading = res?.sol !== sol;

  return (
    <Collapsible id="sec-view" icon={<CameraIcon />} title="Rover photos" summary={`Sol ${sol}`}>
      <p className="text-sm leading-relaxed text-muted">
        Real ground-level photos Perseverance took near the point you picked, to sanity-check what the map layers
        predict about that terrain.
      </p>
      {near && (
        <p className="rounded-md border border-line bg-bg px-3 py-2 text-sm">
          {near.sol !== null ? (
            <>
              Nearest Perseverance stop to your {near.which} point:{" "}
              <strong className="font-mono font-medium tabular-nums">sol {near.sol}</strong>,{" "}
              <span className="font-mono tabular-nums">{near.dist_m ?? 0} m</span> away.
            </>
          ) : (
            <>No Perseverance stop within 400 m of your {near.which} point. Pick a sol manually.</>
          )}
        </p>
      )}
      {/* key: reset the uncontrolled input when a pick changes the sol. */}
      <form key={sol} onSubmit={submit} className="flex items-end gap-2">
        <div className="flex-1">
          <label htmlFor="raw-sol" className={labelCls}>
            Sol (0–3000)
          </label>
          <input
            id="raw-sol"
            name="sol"
            type="number"
            inputMode="numeric"
            required
            min={0}
            max={3000}
            step={1}
            defaultValue={sol}
            className={`${inputCls} mt-1`}
          />
        </div>
        <button type="submit" className={buttonCls}>
          Load images
        </button>
      </form>

      <div aria-live="polite" aria-busy={loading}>
        {loading ? (
          <p className="text-sm text-muted">Loading sol {sol} images…</p>
        ) : res?.error ? (
          <p className="rounded-md bg-danger-soft px-3 py-2 text-sm text-danger">Images unavailable: {res.error}</p>
        ) : !res?.images?.length ? (
          <p className="text-sm text-muted">No Navcam, Mastcam-Z or Hazcam images for sol {sol}.</p>
        ) : (
          <>
            <p className="sr-only">
              {res.images.length} images for sol {sol}.
            </p>
            <ul className="grid grid-cols-3 gap-2">
              {res.images.map((img) => (
                <li key={img.id}>
                  <button
                    type="button"
                    onClick={() => setActive(img)}
                    className="group block w-full overflow-hidden rounded-md border border-line bg-bg text-left"
                  >
                    {/* eslint-disable-next-line @next/next/no-img-element -- remote NASA images, no optimizer config */}
                    <img
                      src={img.src}
                      alt={`${CAMERA_NAMES[img.camera] ?? img.camera}, sol ${sol}`}
                      width={160}
                      height={160}
                      loading="lazy"
                      decoding="async"
                      className="aspect-square w-full object-cover transition-transform duration-200 group-hover:scale-[1.03]"
                    />
                    <span className="block truncate px-1.5 pt-1 font-mono text-[10px] uppercase tracking-wide">
                      {CAMERA_NAMES[img.camera] ?? img.camera}
                    </span>
                    <span className="block truncate px-1.5 pb-1 text-[10px] text-muted">{img.credit}</span>
                  </button>
                </li>
              ))}
            </ul>
          </>
        )}
      </div>

      <dialog
        ref={dialogRef}
        onClose={() => setActive(null)}
        aria-labelledby="raw-dialog-title"
        className="m-auto max-h-[92dvh] w-[min(56rem,94vw)] overflow-auto rounded-xl border border-line bg-surface p-0 text-ink backdrop:bg-black/75"
      >
        {active && (
          <div>
            <div className="flex items-center justify-between gap-3 border-b border-line py-2 pl-4 pr-2">
              <h3 id="raw-dialog-title" className="text-sm font-medium">
                {CAMERA_NAMES[active.camera] ?? active.camera} · sol {sol}
              </h3>
              <button type="button" autoFocus onClick={() => dialogRef.current?.close()} className={buttonCls}>
                Close
              </button>
            </div>
            {/* eslint-disable-next-line @next/next/no-img-element -- remote NASA images, no optimizer config */}
            <img
              src={active.large}
              alt={`${CAMERA_NAMES[active.camera] ?? active.camera} full image, sol ${sol}`}
              className="max-h-[75dvh] w-full bg-black object-contain"
            />
            <p className={`${citeCls} px-4 py-3`}>
              {active.credit} · {active.date_utc.replace("T", " ").replace("Z", "")} UTC ·{" "}
              <a href={active.large} target="_blank" rel="noreferrer" className="underline hover:text-ink">
                Open full size
              </a>
            </p>
          </div>
        )}
      </dialog>
    </Collapsible>
  );
}
