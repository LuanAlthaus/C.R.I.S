// Catálogo de los 14 desafíos oficiales de NASA Space Apps Challenge 2026 ("The Next Frontier").
// Fuente: https://www.spaceappschallenge.org/2026/challenges/ (consultado 2026-11-13).
// El detalle de cada uno (datos, APIs, recursos) está en docs/challenges/.

export type Difficulty = "Beginner/Youth" | "Intermediate" | "Advanced";

export type Challenge = {
  n: number;
  slug: string;
  title: string;
  titleEs: string;
  difficulty: Difficulty[];
  subjects: string[];
  summaryEs: string;
  domain: "Tierra" | "Luna" | "Marte" | "Espacio profundo" | "Humanos en el espacio" | "Educación";
  doc: string;
};

const BASE = "https://www.spaceappschallenge.org/2026/challenges";

export const officialUrl = (c: Challenge) => `${BASE}/${c.slug}/`;

export const challenges: Challenge[] = [
  {
    n: 1,
    slug: "nasas-discarded-equipment-storytelling",
    title: "Abandoned but not Forgotten: Storytelling about NASA's Discarded Equipment on the Moon and Mars",
    titleEs: "Abandonado pero no olvidado",
    difficulty: ["Beginner/Youth", "Intermediate"],
    subjects: ["Astrophysics", "Planets & Moons", "Space Exploration"],
    summaryEs:
      "Contar la historia del hardware que NASA dejó en la Luna, Marte y el espacio profundo, y la ciencia que hizo posible, para estudiantes en edad escolar.",
    domain: "Educación",
    doc: "docs/challenges/01-abandoned-but-not-forgotten.md",
  },
  {
    n: 2,
    slug: "earth-system-trend-detective",
    title: "Be An Earth System Trend Detective!",
    titleEs: "Detective de tendencias del sistema Tierra",
    difficulty: ["Advanced"],
    subjects: ["Earth Science", "Software"],
    summaryEs:
      "Herramienta que selecciona variables de misiones o modelos de ciencias de la Tierra, visualiza su cambio en el tiempo y determina qué, dónde, cuánto cambia y si es estadísticamente significativo.",
    domain: "Tierra",
    doc: "docs/challenges/02-earth-system-trend-detective.md",
  },
  {
    n: 3,
    slug: "junior-astronaut-mission-trainer",
    title: "Build a Junior Astronaut Mission Trainer",
    titleEs: "Entrenador de misión para astronautas junior",
    difficulty: ["Beginner/Youth", "Intermediate"],
    subjects: ["Games", "Planets & Moons", "Software", "Space Exploration", "Sun"],
    summaryEs:
      "Juego o app donde estudiantes gestionan un puesto lunar o marciano equilibrando soporte vital, blindaje de radiación, energía y alimentos.",
    domain: "Educación",
    doc: "docs/challenges/03-junior-astronaut-mission-trainer.md",
  },
  {
    n: 4,
    slug: "health-monitoring-software",
    title: "Create Health Monitoring Software for Astronauts on Space Missions",
    titleEs: "Software de monitoreo de salud para astronautas",
    difficulty: ["Intermediate"],
    subjects: ["Human Exploration", "Software", "Space Exploration"],
    summaryEs:
      "Software que reúne indicadores de salud y permite a los astronautas evaluar y actuar sobre su estado en misiones de larga duración.",
    domain: "Humanos en el espacio",
    doc: "docs/challenges/04-astronaut-health-monitoring.md",
  },
  {
    n: 5,
    slug: "dancing-with-the-sars",
    title: "Dancing with the SARs",
    titleEs: "Bailando con los SAR",
    difficulty: ["Intermediate", "Advanced"],
    subjects: ["Earth Science"],
    summaryEs:
      "App interactiva con datos de radar NISAR (NASA-ISRO) para seguir y visualizar cambios de superficie: humedales, incendios, sismos, agricultura, glaciares.",
    domain: "Tierra",
    doc: "docs/challenges/05-dancing-with-the-sars.md",
  },
  {
    n: 6,
    slug: "adapting-farms-with-nasa-data",
    title: "Field Shift: Adapting Farms with NASA Data",
    titleEs: "Field Shift: adaptar fincas con datos de NASA",
    difficulty: ["Intermediate", "Advanced"],
    subjects: ["Arts & Multimedia", "Earth Science", "Software"],
    summaryEs:
      "Herramienta de apoyo a decisiones que combina observaciones de NASA con suelo, cultivos y prioridades del productor para explorar rotaciones que mejoren la salud del suelo.",
    domain: "Tierra",
    doc: "docs/challenges/06-field-shift-farms.md",
  },
  {
    n: 7,
    slug: "flame-in-freefall",
    title: "Flame in Freefall: SI-Powered Fire Safety Insights from Microgravity Combustion Data",
    titleEs: "Llama en caída libre",
    difficulty: ["Intermediate", "Advanced"],
    subjects: ["Space Exploration"],
    summaryEs:
      "Dashboard interactivo con IA que resume, ordena e interpreta décadas de experimentos de combustión en microgravedad para dar insights de seguridad contra incendios.",
    domain: "Humanos en el espacio",
    doc: "docs/challenges/07-flame-in-freefall.md",
  },
  {
    n: 8,
    slug: "modis-and-viirs-hot-spots",
    title: "Harmonization of MODIS and VIIRS Hot Spots",
    titleEs: "Armonización de focos de calor MODIS y VIIRS",
    difficulty: ["Intermediate", "Advanced"],
    subjects: ["Earth Science", "Software"],
    summaryEs:
      "Web app que armoniza los registros de focos activos de MODIS y VIIRS en un calendario de quemas para un área de interés: patrones históricos, anomalías y períodos críticos.",
    domain: "Tierra",
    doc: "docs/challenges/08-modis-viirs-hot-spots.md",
  },
  {
    n: 9,
    slug: "permanent-moon-base-locations-and-mars",
    title: "Identify Earth Locations that Analog the Permanent Moon Base Locations and Mars",
    titleEs: "Análogos terrestres de bases lunares y Marte",
    difficulty: ["Beginner/Youth", "Intermediate", "Advanced"],
    subjects: ["Earth Science", "Planets & Moons", "Software", "Space Exploration"],
    summaryEs:
      "Usar datos abiertos de la Tierra, Luna y Marte para identificar y caracterizar nuevos sitios análogos terrestres y presentarlos como mapa o visualización.",
    domain: "Luna",
    doc: "docs/challenges/09-earth-analogs-moon-mars.md",
  },
  {
    n: 10,
    slug: "interplanetary-survival-guide-martian-map",
    title: "Interplanetary Survival Guide: Martian Map",
    titleEs: "Guía de supervivencia interplanetaria: mapa marciano",
    difficulty: ["Intermediate"],
    subjects: ["Human Exploration", "Mars", "Planets & Moons", "Software", "Space Exploration"],
    summaryEs:
      "Vista integrada por capas de una ubicación o ruta en Marte con datos de varias misiones para planificar una caminata marciana segura y científica.",
    domain: "Marte",
    doc: "docs/challenges/10-martian-map.md",
  },
  {
    n: 11,
    slug: "lunar-landing-site-navigator",
    title: "Lunar Landing Site Navigator",
    titleEs: "Navegador de sitios de alunizaje",
    difficulty: ["Intermediate", "Advanced"],
    subjects: ["Human Exploration", "Moon", "Software", "Space Exploration"],
    summaryEs:
      "Herramienta intuitiva para comparar sitios y fechas en el polo sur lunar, visualizando Sol y Tierra sobre el horizonte para energía y ventanas de comunicación.",
    domain: "Luna",
    doc: "docs/challenges/11-lunar-landing-site-navigator.md",
  },
  {
    n: 12,
    slug: "planet-x-and-spherex",
    title: "Planet X and SPHEREx",
    titleEs: "Planeta X y SPHEREx",
    difficulty: ["Advanced"],
    subjects: ["Astrophysics", "Planets & Moons", "Software"],
    summaryEs:
      "Herramienta web pública para mostrar imágenes del cielo de SPHEREx y ver fácilmente cómo cambian con el tiempo (objetos en movimiento).",
    domain: "Espacio profundo",
    doc: "docs/challenges/12-planet-x-spherex.md",
  },
  {
    n: 13,
    slug: "space-mission-design-game",
    title: "Space Mission Design Game",
    titleEs: "Juego de diseño de misiones espaciales",
    difficulty: ["Beginner/Youth", "Intermediate"],
    subjects: ["Games"],
    summaryEs:
      "Juego interactivo para diseñar, gestionar y simular una misión completa: objetivos, nave, instrumentos, lanzador, presupuesto, potencia, masa y órbitas.",
    domain: "Educación",
    doc: "docs/challenges/13-space-mission-design-game.md",
  },
  {
    n: 14,
    slug: "the-earth-information-jukebox",
    title: "The Earth Information Jukebox",
    titleEs: "La rocola de información de la Tierra",
    difficulty: ["Beginner/Youth", "Intermediate"],
    subjects: ["Arts & Multimedia", "Earth Science", "Software"],
    summaryEs:
      "Interfaz que combina visualizaciones del Earth Information Center con sonificaciones generadas en tiempo real para hacer accesible la ciencia de la Tierra.",
    domain: "Tierra",
    doc: "docs/challenges/14-earth-information-jukebox.md",
  },
];
