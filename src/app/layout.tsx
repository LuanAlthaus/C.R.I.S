import type { Metadata } from "next";
import { IBM_Plex_Mono, IBM_Plex_Sans, Instrument_Serif } from "next/font/google";
import "./globals.css";

const sans = IBM_Plex_Sans({ variable: "--font-plex-sans", subsets: ["latin"], weight: ["400", "500", "600"] });
const mono = IBM_Plex_Mono({ variable: "--font-plex-mono", subsets: ["latin"], weight: ["400", "500"] });
const display = Instrument_Serif({ variable: "--font-display", subsets: ["latin"], weight: "400", style: ["normal", "italic"] });

export const metadata: Metadata = {
  title: "C.R.I.S",
  description:
    "Plan a safe spacesuit walk across 10 × 10 km of Jezero crater, Mars: 3D HiRISE terrain, slope, roughness, optical texture, hazard, thermal and sky-exposure layers, a load-aware route planner and EVA conditions from NASA data.",
  // Chrome's page translation breaks React (see RootLayout); the deliverable UI is English.
  other: { google: "notranslate" },
};

export default function RootLayout({ children }: LayoutProps<"/">) {
  return (
    // translate="no": Chrome's page translation wraps text in <font> nodes; React then crashes on the next text
    // swap (insertBefore error, seen on Android when "Play rover" turns into "Pause rover"). A tab already in
    // translate mode still rewrites <html lang/class>: suppressHydrationWarning covers only this element's attributes.
    <html
      lang="en"
      translate="no"
      suppressHydrationWarning
      className={`${sans.variable} ${mono.variable} ${display.variable} h-full antialiased`}
    >
      <body className="min-h-full flex flex-col">{children}</body>
    </html>
  );
}
