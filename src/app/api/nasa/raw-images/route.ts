import { cacheLife } from "next/cache";

// GET /api/nasa/raw-images?sol=N
// Perseverance first-person raw images (Navcam, Mastcam-Z, Hazcams) for one sol.
// Code written with AI assistance (Claude / omp); see docs/SI_USAGE.md.

const CAMERAS = new Set(["NAVCAM_LEFT", "MCZ_LEFT", "FRONT_HAZCAM_LEFT_A", "REAR_HAZCAM_LEFT"]);

interface RawImage {
  imageid: string;
  camera: { instrument: string };
  image_files: { medium: string; large: string };
  credit: string;
  date_taken_utc: string;
}

async function getRawImages(sol: number): Promise<{ images: RawImage[] }> {
  "use cache";
  cacheLife("days");

  const url = new URL("https://mars.nasa.gov/rss/api/");
  url.searchParams.set("feed", "raw_images");
  url.searchParams.set("category", "mars2020");
  url.searchParams.set("feedtype", "json");
  url.searchParams.set("sol", String(sol));

  const res = await fetch(url);
  if (!res.ok) throw new Error(`Mars raw images feed responded ${res.status}`);
  return res.json();
}

export async function GET(request: Request) {
  const raw = new URL(request.url).searchParams.get("sol") ?? "1";
  const sol = Number(raw);
  if (!/^\d+$/.test(raw) || sol > 3000) {
    return Response.json({ error: "sol must be an integer 0-3000" }, { status: 400 });
  }
  try {
    const data = await getRawImages(sol);
    const images = (data.images ?? [])
      .filter((im) => CAMERAS.has(im.camera?.instrument))
      .slice(0, 24)
      .map((im) => ({
        id: im.imageid,
        camera: im.camera.instrument,
        src: im.image_files.medium,
        large: im.image_files.large,
        credit: im.credit,
        date_utc: im.date_taken_utc,
      }));
    return Response.json({ sol, images });
  } catch (e) {
    return Response.json({ error: (e as Error).message }, { status: 502 });
  }
}
