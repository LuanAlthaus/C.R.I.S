"use client";
// Code written with AI assistance (Claude / omp); see docs/SI_USAGE.md.
// Main Menu backdrop: Mars drawn in one fragment shader from the Viking Orbiter MDIM 2.1 colour mosaic
// (NASA/JPL/USGS, via JPL Mars Trek). Real albedo; the sunlight, haze and stars are rendered, not data.

import { useEffect, useRef } from "react";

const TEXTURE = "/landing/mars-viking-mdim21.jpg"; // equirectangular, 180°W..180°E, north up
const START_LON = (85 * Math.PI) / 180; // Jezero crater (77.45°E) drifts through the centre first
const TURN_S = 600; // one rotation; the real sol is 88 775 s

const VERT = `#version 300 es
in vec2 p;
void main() { gl_Position = vec4(p, 0.0, 1.0); }`;

const FRAG = `#version 300 es
precision highp float;
uniform sampler2D tex;
uniform vec3 globe; // centre x, centre y, radius, in px
uniform float lon0; // longitude facing the viewer, rad
out vec4 o;
const float PI = 3.14159265;
const vec3 SUN = normalize(vec3(-0.62, 0.42, 0.66));
const float TILT = 0.4396; // Mars obliquity, 25.19°
const vec3 DUST = vec3(0.92, 0.54, 0.36);
float hash(vec2 v) { return fract(sin(dot(v, vec2(127.1, 311.7))) * 43758.5453); }
void main() {
  vec2 q = (gl_FragCoord.xy - globe.xy) / globe.z;
  float r = length(q);

  vec2 cell = floor(gl_FragCoord.xy / 3.0);
  vec3 col = vec3(step(0.9982, hash(cell)) * (0.10 + 0.30 * hash(cell + 7.0)));
  float sunward = smoothstep(-0.35, 0.7, dot(q / max(r, 1e-4), SUN.xy));
  col += DUST * exp(-max(r - 1.0, 0.0) * 36.0) * 0.45 * sunward;

  if (r < 1.0) {
    vec3 n = vec3(q, sqrt(1.0 - r * r));
    float c = cos(TILT), s = sin(TILT);
    vec3 m = vec3(c * n.x + s * n.y, c * n.y - s * n.x, n.z);
    // atan's seam is at m.z < 0, the far side, so it is never drawn.
    vec2 uv = vec2((atan(m.x, m.z) + lon0) / (2.0 * PI) + 0.5, 0.5 - asin(clamp(m.y, -1.0, 1.0)) / PI);
    float day = smoothstep(-0.1, 0.6, dot(n, SUN));
    vec3 surf = texture(tex, uv).rgb * (0.015 + 1.12 * day);
    surf += DUST * pow(1.0 - n.z, 3.0) * day * 0.4;
    col = mix(col, surf, smoothstep(1.0, 1.0 - 1.5 / globe.z, r));
  }
  o = vec4(col, 1.0);
}`;

export default function MarsGlobe() {
  const ref = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = ref.current;
    const gl = canvas?.getContext("webgl2", { alpha: false, antialias: false });
    if (!canvas || !gl) return;

    const prog = gl.createProgram();
    for (const [type, src] of [
      [gl.VERTEX_SHADER, VERT],
      [gl.FRAGMENT_SHADER, FRAG],
    ] as const) {
      const sh = gl.createShader(type)!;
      gl.shaderSource(sh, src);
      gl.compileShader(sh);
      gl.attachShader(prog, sh);
    }
    gl.linkProgram(prog);
    if (!gl.getProgramParameter(prog, gl.LINK_STATUS)) {
      console.error("MarsGlobe shader:", gl.getProgramInfoLog(prog));
      return;
    }
    gl.useProgram(prog);
    gl.bindBuffer(gl.ARRAY_BUFFER, gl.createBuffer());
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([-1, -1, 3, -1, -1, 3]), gl.STATIC_DRAW);
    const aPos = gl.getAttribLocation(prog, "p");
    gl.enableVertexAttribArray(aPos);
    gl.vertexAttribPointer(aPos, 2, gl.FLOAT, false, 0, 0);
    const uGlobe = gl.getUniformLocation(prog, "globe");
    const uLon = gl.getUniformLocation(prog, "lon0");

    const still = matchMedia("(prefers-reduced-motion: reduce)").matches;
    let raf = 0;
    let ready = false;
    // Parallax: pointer offset from the centre (-1..1), eased.
    const aim = { x: 0, y: 0 };
    const now = { x: 0, y: 0 };

    const draw = (ms: number) => {
      const w = canvas.width;
      const h = canvas.height;
      const tall = w < h * 0.9;
      // Wide: cropped by the right and bottom edges, clear of the copy and the nav. Tall: rising from below.
      const radius = tall ? w * 0.62 : Math.min(h * 0.5, w * 0.36);
      now.x += (aim.x - now.x) * 0.04;
      now.y += (aim.y - now.y) * 0.04;
      const shift = radius * 0.012;
      gl.uniform3f(uGlobe, (tall ? 0.5 : 0.74) * w - now.x * shift, (tall ? 0.22 : 0.42) * h + now.y * shift, radius);
      gl.uniform1f(uLon, START_LON - (still ? 0 : (ms / 1000 / TURN_S) * 2 * Math.PI));
      gl.drawArrays(gl.TRIANGLES, 0, 3);
      if (!still) raf = requestAnimationFrame(draw);
    };

    const resize = () => {
      const dpr = Math.min(devicePixelRatio, 2);
      canvas.width = Math.round(canvas.clientWidth * dpr);
      canvas.height = Math.round(canvas.clientHeight * dpr);
      gl.viewport(0, 0, canvas.width, canvas.height);
      if (still && ready) draw(0);
    };
    const observer = new ResizeObserver(resize);
    observer.observe(canvas);

    const onPointer = (e: PointerEvent) => {
      aim.x = (e.clientX / innerWidth) * 2 - 1;
      aim.y = (e.clientY / innerHeight) * 2 - 1;
    };
    if (!still) addEventListener("pointermove", onPointer);

    const img = new Image();
    img.onload = () => {
      gl.bindTexture(gl.TEXTURE_2D, gl.createTexture());
      gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGB, gl.RGB, gl.UNSIGNED_BYTE, img);
      gl.generateMipmap(gl.TEXTURE_2D);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
      ready = true;
      resize();
      if (!still) raf = requestAnimationFrame(draw);
      canvas.style.opacity = "1";
    };
    img.src = TEXTURE;

    return () => {
      img.onload = null;
      cancelAnimationFrame(raf);
      observer.disconnect();
      removeEventListener("pointermove", onPointer);
    };
  }, []);

  return (
    <canvas
      ref={ref}
      aria-hidden="true"
      className="absolute inset-0 size-full opacity-0 transition-opacity duration-[2400ms] ease-out"
    />
  );
}
