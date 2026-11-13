# Flame in Freefall: SI-Powered Fire Safety Insights from Microgravity Combustion Data
> Llama en caída libre: información de seguridad contra incendios con SI a partir de datos de combustión en microgravedad

- **URL oficial:** https://www.spaceappschallenge.org/2026/challenges/flame-in-freefall/
- **Dificultad:** Advanced, Intermediate
- **Temas:** Space Exploration

## Resumen
Durante décadas, NASA ha estudiado cómo se comportan las llamas y el fuego en microgravedad, acumulando un gran volumen de datos experimentales. Ese conocimiento importa más que nunca ahora que los humanos se preparan para volver a la Luna y seguir hacia Marte, pero el volumen de resultados los hace difíciles de encontrar, comparar y entender. Tu desafío es crear un dashboard interactivo impulsado por *super intelligence* (SI) que resuma, clasifique e interprete esos hallazgos para generar información de seguridad contra incendios para la exploración humana del espacio.

### Qué significa "SI" (super intelligence)
- **En la página del desafío:** solo aparece la expansión "super intelligence (SI) powered dashboard" y el título "SI-Powered". La página del desafío no define el término ni establece reglas sobre su uso.
- **En el FAQ oficial de participantes** (https://www.spaceappschallenge.org/resources/participant-faqs/), pregunta "What is Super Intelligence? Can teams utilize it during the hackathon?":
  - "In accordance with the September 29, 2026 U.S. Executive Order, 'the executive branch shall use the terms 'Super Intelligence' and 'SI' in place of 'Artificial Intelligence' and 'AI.''"
  - "Super Intelligence (SI) refers to computer systems that perform tasks typically associated with human reasoning, decision-making, content creation, and problem-solving. Examples include large language models, conversational assistants, image and video generation tools, foundation models, and code generation tools."
  - "Participants may use SI tools to accelerate innovation, streamline development, and enhance creativity in support of their challenge solution. Each team's project submission must include a description of how SI tools were used during the development of their project."
- En la práctica, "SI" es el término oficial que reemplaza a "AI"/"IA": un dashboard "SI-powered" es un dashboard con IA (LLMs, embeddings, etc.). **Regla obligatoria:** la entrega debe describir cómo se usaron las herramientas de SI.

## Objetivos / qué piden
Contexto (Background):
- Los incendios son extremadamente peligrosos en el espacio. La página cita al comandante de Artemis II sobre una alarma de humo durante la misión, y recuerda la explosión del tanque de oxígeno de Apollo 13, el incendio en la estación Mir por una falla del generador de oxígeno y la muerte de la tripulación de Apollo 1 en 1967 durante una prueba previa al lanzamiento en una nave con atmósfera de oxígeno.
- En microgravedad las llamas son muy distintas por la ausencia de flotabilidad: la llama de una vela se vuelve "a ghostly blue sphere" que depende de la difusión para obtener oxígeno.
- La seguridad contra incendios de NASA se centra en **prevención, detección y supresión**. La prevención depende de controlar fuentes de ignición y de seleccionar materiales según sus límites de inflamabilidad. Se espera que la Moon Base y los rovers presurizados tengan más oxígeno y menos presión que la Tierra; la inflamabilidad depende mucho de la concentración de oxígeno y del flujo de aire.
- Investigaciones clave: inflamabilidad de sólidos en BASS y BASS-II y en Saffire-I, -II y -III; detección en SAME y SAME-R; supresión en FLEX (dilución con dióxido de carbono y helio).

Lo que piden:
- Los resultados de experimentos de combustión en baja gravedad (p. ej. en la ISS) se publican en la base de datos **Physical Science Informatics (PSI)** de NASA, pero el volumen "can be overwhelming. It can be hard to find, harder to compare, and harder yet to interpret."
- "Build an SI-powered, web-based dashboard that summarizes, ranks, and interprets combustion science investigations to deliver critical foundational insights for fire safety." Puede permitir a los usuarios:
  - Identificar las investigaciones más relevantes para la seguridad contra incendios en entornos habitados por tripulación: naves, estaciones espaciales, bases en la superficie lunar y marciana, y rovers presurizados.
  - Extraer datos que sirvan para modelado predictivo de inflamabilidad y escenarios de incendio en la exploración espacial.
  - Destacar investigaciones que informen el desarrollo de controles y equipos de seguridad contra incendios de nueva generación.
- Opcional: abordar aplicaciones de los datos PSI a la seguridad contra incendios en la Tierra (extintores de niebla de agua fina desarrollados para la ISS, detectores de humo, ropa de bombero y tanques de aire derivados de tecnología NASA).
- Fuera de alcance: "Fire safety involving rockets and propellants is generally beyond the scope of the research available in the PSI database."

## Consideraciones potenciales
*Opcionales ("You may (but are not required to) consider the following").*

- **Público objetivo:** profesores, estudiantes de posgrado y grado, investigadores e ingenieros aeroespaciales.
- **Atributos a resumir y mostrar:** objetivos experimentales; combustibles (incluida forma y tamaño de las muestras o quemadores); otras condiciones de ensayo como concentración de oxígeno y velocidad del flujo de aire; variables (en qué se diferenciaban y en qué coincidían los ensayos de una misma investigación); sensores o tipos de medición.
- Resumir y mostrar similitudes y diferencias **entre** investigaciones.
- **Otras investigaciones en PSI:**
  - Un instrumento de detección de humo de SAME y SAME-R se usó para detectar polvo en DAFT y DAFT-2.
  - El Burning Rate Emulator (BRE) del proyecto ACME quemaba combustibles gaseosos, pero estudiaba inflamabilidad de materiales en atmósferas quietas en función de sus propiedades.
  - Otras investigaciones de combustión de PSI no mencionadas siguen siendo relevantes, aunque de forma menos directa.
- **Hardware:**
  - Los seis experimentos de ACME usaron llamas de combustibles gaseosos con el mismo hardware modular en el Combustion Integrated Rack (CIR).
  - FLEX, FLEX-2 y Cool Flames Investigation (CFI) quemaban gotas de combustible líquido con el Multi-user Droplet Combustion Apparatus (MDCA) en el CIR.
  - BASS, BASS-II, SPICE (Smoke Point In Co-flow Experiment) y SLICE (Structure & Liftoff In Combustion Experiment) usaron combustibles sólidos y gaseosos dentro del Microgravity Science Glovebox (MSG) con el ducto de flujo con ventilador construido para SPICE.
- **Trajes espaciales y concentración de oxígeno:**
  - Los trajes usan 100 % oxígeno a solo 4,3 psi (la presión al nivel del mar es 14,7 psi).
  - Los astronautas pre-respiran oxígeno puro para expulsar el nitrógeno y evitar la enfermedad descompresiva.
  - Para acortar la pre-respiración, la Moon Base y los rovers presurizados tendrían presión y oxígeno intermedios entre el traje y la atmósfera terrestre.
  - El oxígeno en la Luna probablemente sea de 30 % o más, con presión de 10,2 psi o menos: más de un 40 % de aumento respecto de la Tierra, lo que hace a los materiales mucho más inflamables.
  - Los ensayos en microgravedad con mayor oxígeno fueron los de CFI-G (Cool Flames Investigation with Gases) de ACME: hasta 85 % de oxígeno saliendo de un quemador esférico, con una llama "de adentro hacia afuera".
- **Datos de gravedad parcial:**
  - Hoy no hay datos de combustión en gravedad parcial en PSI; se esperan a futuro, por ejemplo de LUCI (Lunar-g Combustion Investigation) y FM2 (Flammability of Materials on the Moon).
  - E-FIELD Flames de ACME imitó gravedad parcial mediante el efecto de un campo eléctrico sobre los iones de la llama.
  - Los resultados de microgravedad y gravedad normal podrían acotar lo que ocurre en niveles intermedios.

## Datos y recursos de NASA
| Recurso | Qué es | Tipo de acceso | URL |
|---|---|---|---|
| PSI Data Repository: Combustion Science | Repositorio Physical Sciences Informatics: la fuente de datos sobre la que debe basarse la solución. | Dataset en catálogo (búsqueda web; la descarga de archivos puede requerir registro gratuito en PSI) | https://psi.nasa.gov/physci/repo/search?q=combustion |
| NASA Combustion Science | Información básica sobre la mayoría de las investigaciones de combustión cuyos datos están en PSI. | Documento (página web) | https://www.nasa.gov/glenn/glenn-expertise-space-exploration/physical-sciences-program/combustion-science/ |
| NASA ScienceCasts: The Power of Studying Combustion on the ISS | Video de 3,6 minutos (2018) que introduce la investigación de combustión de NASA en la ISS. | Documento (video) | https://www.youtube.com/watch?v=W1oXykoz9zg |
| A Researcher's Guide to International Space Station Combustion Science | Revisión de 2015 (más de 50 páginas) de la investigación de combustión hecha o planificada en la ISS. | Documento | https://www.nasa.gov/missions/station/a-researchers-guide-to-combustion-science/ |
| Apollo 1 | Tripulación, misión, incendio en la prueba previa al lanzamiento, investigación y lecciones aprendidas. | Documento | https://sma.nasa.gov/sma-disciplines/accllp/apollo-1 |
| Apollo 13 | Tripulación, misión, explosión del tanque de oxígeno, investigación y lecciones aprendidas. | Documento | https://sma.nasa.gov/sma-disciplines/accllp/apollo-13 |
| 25 Years Ago: Fire Aboard Space Station Mir | Incendio de 1997 en la estación rusa Mir causado por una falla del generador de oxígeno. | Documento | https://www.nasa.gov/history/25-years-ago-fire-aboard-space-station-mir/ |
| Fireman's Air Tanks | Adaptación en los años 70 de tecnología de trajes espaciales a tanques de aire y equipos de respiración de bomberos. | Documento (Spinoff) | https://spinoff.nasa.gov/node/9582 |
| Fire Detector | Detector de humo de los años 70 desarrollado para el shuttle que distingue humo de polvo (menos falsas alarmas; usos industriales como minería). | Documento (Spinoff) | https://spinoff.nasa.gov/node/9407 |
| New Technology for the Fire Attire | Aplicación en los años 80 de tecnología de trajes espaciales a la ropa de bomberos. | Documento (Spinoff) | https://spinoff.nasa.gov/node/9918 |

## Recursos de agencias socias
No especificado en la página oficial.

## Otros recursos
No especificado en la página oficial.

## Acceso a los datos (notas del repo)

> Por la regla oficial, no se puede trabajar en un challenge antes del hackathon, ni usar SI (IA) para idear la solución. Por eso esta sección solo trae notas de acceso a los datos, no enfoques de solución. La idea la pone el equipo.

*No es parte de la página oficial. Las URLs y endpoints que no están en la página son sugerencias adicionales: verificalos.*

- **Los datos de PSI no son una API:** son páginas por investigación con metadatos (descripción, objetivos, hardware, condiciones) y archivos adjuntos (CSV, hojas de cálculo, imágenes, video, PDFs). Lo más eficiente en 36 h es armar a mano o con scraping liviano un **catálogo estructurado** (JSON o tabla en Supabase) con las ~15–25 investigaciones de combustión clave: BASS, BASS-II, Saffire-I/II/III, SAME, SAME-R, FLEX, FLEX-2, CFI, ACME (BRE, CFI-G, E-FIELD Flames, etc.), SPICE, SLICE, DAFT, DAFT-2. Revisen los términos de uso de PSI antes de raspar.
- **Obligatorio según el FAQ:** documentar en la entrega cómo se usaron las herramientas de SI, tanto en el producto como durante el desarrollo.
- **Gotchas:** algunos archivos de PSI son grandes (video, imágenes de alta velocidad); no los incluyan en el bundle, solo enlacen. Las unidades varían entre experimentos: normalicen O2 a fracción molar o porcentaje y presión a psi o kPa.
- **Skills del repo que aplican:** space-apps-challenge, nasa-data-apis, ai-features, maps-and-dataviz (dataviz, sin mapa), frontend-design, deploy-vercel-supabase, pitch-and-submission, hackathon-sprint.
