# Planet X and SPHEREx
> El Planeta X y SPHEREx

- **URL oficial:** https://www.spaceappschallenge.org/2026/challenges/planet-x-and-spherex/
- **Dificultad:** Advanced (Avanzado)
- **Temas:** Astrophysics, Planets & Moons, Software

## Resumen
Desde 2025, la misión SPHEREx de NASA mapea todo el cielo cada seis meses en 102 bandas de luz del infrarrojo cercano, devolviendo imágenes de más de mil millones de objetos. Cometas, asteroides, estrellas, enanas marrones e incluso nuevos planetas se revelan al cambiar de posición entre imágenes, pero ninguna persona puede revisar sola todos esos datos. El desafío es crear una herramienta web pública que muestre imágenes del cielo de la misión SPHEREx, para que cualquiera pueda ver fácilmente cómo cambian con el tiempo.

## Objetivos / qué piden
Contexto (Background): asteroides y cometas orbitan el Sol, las estrellas cercanas se desplazan al orbitar la Galaxia y la órbita terrestre produce paralaje; observar esos movimientos permite descubrir objetos y medir distancias (Plutón se descubrió comparando dos imágenes tomadas en momentos distintos). Cientos de miles de voluntarios han descubierto asteroides, cometas, estrellas y enanas marrones con esta técnica, por ejemplo en backyardworlds.org. Desde mayo de 2025 SPHEREx es especialmente sensible a objetos ricos en hidrógeno, agua, CO2, CO y otras moléculas, y producirá más datos de los que una persona o un código puede abarcar.

Lo que piden:

- "Create a public-facing web tool to display images of the sky from the SPHEREx mission, making it easy for anyone to quickly see how they change over time."
- **Entrada:** el usuario ingresa las coordenadas de una zona del cielo (Right Ascension y declination).
- **Salida:** la herramienta muestra una serie de imágenes de esa zona "like a little video" (tipo blink/animación), para que sea fácil detectar objetos en movimiento.
- **Casos de prueba obligatorios:**
  - Barnard's Star
  - Epsilon Indi B (un par de enanas marrones que orbitan una estrella)
  - WISE 0855 (enana marrón de movimiento rápido; RA 08h55m10.831s, Dec -7d14m42.53s)
  - Estrellas variables RW Aurigae y R Draconis

## Consideraciones potenciales
*Opcionales, según la página oficial:*

- Llegan datos nuevos de SPHEREx rápido: diseñar la herramienta para que sea fácil actualizarla con los últimos data releases.
- Pensar creativamente cómo mostrar las 102 bandas infrarrojas; por ejemplo, sumar por separado todos los datos de cada uno de los seis detector arrays para mejorar la relación señal/ruido. "Extra kudos if your tool can generate spectra on the fly."
- Ofrecer opciones para sumar o restar imágenes tomadas en distintos momentos ayuda a descubrir objetos en movimiento.
- Dejar elegir el campo de visión (field of view): campos más grandes permiten seguir fuentes más rápidas a costa de mayor tiempo de transferencia.
- Permitir superponer datos de otros telescopios, como la misión Gaia o la misión WISE.
- Permitir consultar la herramienta con coordenadas en la URL, para poder embeberla en otros sitios y proyectos.
- Mostrar metadatos, como contamination flags.
- Buscar herramientas públicas para manejar datos astronómicos (como las herramientas Astropy, escritas "Astrophy" en la página) y para mostrarlos (como Aladin, del Strasbourg Astronomical Data Center, CDS).

## Datos y recursos de NASA
| Recurso | Qué es | Tipo de acceso | URL |
|---|---|---|---|
| NASA/IPAC Infrared Science Archive (IRSA) | Recursos de SPHEREx en el archivo infrarrojo de NASA/IPAC (imágenes y productos de la misión) | Dataset en catálogo (servicios IVOA/API + descarga directa) | https://irsa.ipac.caltech.edu/Missions/spherex.html |
| SPHEREx Tutorial Notebooks | Notebooks Jupyter de ejemplo para acceder a los datos de SPHEREx vía Python | Documento (notebooks en GitHub Pages) | https://caltech-ipac.github.io/irsa-tutorials/spherex/ |
| WiseView | Herramienta similar hecha para ver datos de la misión WISE; el público la usó para cientos de descubrimientos. Sirve como modelo o inspiración | Visor web | http://byw.tools/wiseview |
| Gaia Mission Data | Datos de la misión Gaia en MAST/STScI; usar el último release (Data Release 3) | Dataset en catálogo | https://archive.stsci.edu/missions-and-data/gaia |

## Recursos de agencias socias
| Recurso | Qué es | Tipo de acceso | URL |
|---|---|---|---|
| CSA – NEOSSat: Observing Asteroids, Space Debris and Exoplanets | Página sobre NEOSSat, primer telescopio espacial dedicado a detectar y rastrear asteroides, cometas, satélites y basura espacial | Documento (sitio informativo) | https://www.asc-csa.gc.ca/eng/satellites/neossat/ |
| Canadian Astronomy Data Centre (CADC) | Acceso a una gran colección de observaciones astronómicas de telescopios y misiones canadienses e internacionales | Dataset en catálogo (búsqueda web + servicios) | https://www.cadc-ccda.hia-iha.nrc-cnrc.gc.ca/en/ |
| CSA – NEOSSat Data Tutorial | Tutorial en Python para visualizar datos de NEOSSat paso a paso | Documento (repositorio GitHub) | https://github.com/asc-csa/NEOSSAT_Tutorial |
| CSA – NEOSSat Dataset | Dataset de observaciones de NEOSSat para detección y seguimiento de asteroides, cometas, satélites y basura espacial | Dataset en catálogo (portal de datos abiertos CSA) | https://data.asc-csa.gc.ca/dataset/neossat |

## Otros recursos
| Recurso | Qué es | URL |
|---|---|---|
| Backyard Worlds | Proyecto de ciencia ciudadana mencionado en el Background, donde voluntarios descubren enanas marrones y otros objetos comparando imágenes | http://backyardworlds.org/ |
| Astropy (mencionado como "Astrophy tools") | Herramientas públicas para manejar datos astronómicos, mencionadas en Potential Considerations (sin enlace) | No especificado en la página oficial |
| Aladin (CDS) | Herramienta de visualización de datos astronómicos del Strasbourg Astronomical Data Center, mencionada en Potential Considerations (sin enlace) | No especificado en la página oficial |

## Acceso a los datos (notas del repo)

> Por la regla oficial, no se puede trabajar en un challenge antes del hackathon, ni usar SI (IA) para idear la solución. Por eso esta sección solo trae notas de acceso a los datos, no enfoques de solución. La idea la pone el equipo.

*No es parte de la página oficial. Las URLs y endpoints que no están en la página son sugerencias adicionales: verificalos.*

- **Acceso a datos:** SPHEREx en IRSA se consulta con protocolos IVOA. Los tutoriales oficiales usan `pyvo` con el servicio SIA/TAP de IRSA para buscar "spectral images" (Level 2) que cubren una posición, y luego descargan cutouts. Sin autenticación para datos públicos. Formato: **FITS** multi-extensión (imagen, flags, varianza, WCS espectral). Usar `astropy.io.fits` + `astropy.wcs`.
- **Gotcha de tamaño:** cada imagen completa de SPHEREx pesa decenas de MB y cada posición del cielo tiene muchas exposiciones (6 detectores × distintas longitudes de onda × visitas). Nunca bajar imágenes completas en el navegador: usar el servicio de cutouts de IRSA (sugerencia adicional) o un backend Python que recorte y convierta a PNG/JPEG con estiramiento (asinh/zscale).
- **Cuidado con los objetivos de prueba:** WISE 0855 se mueve ~8"/año y Barnard's Star ~10"/año; los píxeles de SPHEREx son grandes (~6.2"), así que el movimiento será de 1–2 píxeles por año. Un campo de visión de 2–5 arcmin y buen escalado son claves para que se note.
- **Skills del repo aplicables:** `space-apps-challenge`, `planetary-data`, `nasa-data-apis`, `maps-and-dataviz`, `frontend-design`, `deploy-vercel-supabase`, `ai-features` (si se agrega detección automática de candidatos), `hackathon-sprint`, `pitch-and-submission`.
