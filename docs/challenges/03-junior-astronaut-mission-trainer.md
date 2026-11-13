# Build a Junior Astronaut Mission Trainer
> Construye un entrenador de misiones para astronautas junior

- **URL oficial:** https://www.spaceappschallenge.org/2026/challenges/junior-astronaut-mission-trainer/
- **Dificultad:** Intermediate, Beginner/Youth
- **Temas:** Games, Planets & Moons, Software, Space Exploration, Sun

## Resumen
El contenido STEM con temática espacial suele simplificar demasiado los compromisos (trade-offs) de ingeniería que definen una misión real, o presentarlos a un nivel demasiado complejo para mantener la atención de un aprendiz joven. Pocas herramientas hacen esos compromisos tangibles y divertidos a la vez. El desafío es diseñar y construir un juego o app interactiva que permita a estudiantes dirigir un puesto avanzado lunar o marciano, equilibrando demandas en competencia como soporte vital, blindaje contra radiación, energía y producción de alimentos, para que experimenten de primera mano las decisiones que determinan si una misión fracasa o tiene éxito.

## Objetivos / qué piden
Contexto (Background):
- Mientras la NASA busca una presencia sostenida en la Luna y llegar a Marte, un desafío persistente es que el público entienda qué requiere sobrevivir allí: energía limitada repartida entre soporte vital, comunicaciones y ciencia; exposición a radiación vs. objetivos de misión; cada decisión tiene un costo que se acumula.
- Los planificadores de la NASA usan datos reales: DONKI (actividad solar y riesgo de radiación), OSDR (dosimetría y biología de astronautas) y datos ambientales de Artemis y misiones de superficie en Marte.
- El problema es la traducción: el contenido para niños es demasiado abstracto o demasiado técnico; lo muy educativo aburre y lo muy entretenido pierde precisión.
- Priorizar **mecánicas, no solo visuales**: "a choice that saves power today might cost oxygen tomorrow". Se alienta usar datos abiertos reales de la NASA en lugar de estadísticas inventadas.

Objetivos:
- **Diseñar y construir un juego o app interactiva para niños de 7 a 12 años** que gestionen un puesto lunar o marciano, tomando decisiones reales sobre **soporte vital, blindaje contra radiación, presupuesto de energía, producción de alimentos y retraso de comunicaciones**, y viendo cómo se desarrollan esas decisiones a lo largo de una misión simulada.
- Representar recursos en competencia de modo que **"no choice is free"** y cada decisión enseñe algo cierto sobre lo que enfrentan astronautas y planificadores.
- Basar la simulación en datos abiertos reales de la NASA siempre que sea posible (DONKI, OSDR, datos de Artemis y Marte); datasets en vivo o archivados en lugar de números inventados.
- Formato abierto: juego de supervivencia y gestión de recursos, simulador narrativo de decisiones, dashboard tipo control de misión u otro; en navegador, móvil o escritorio.
- Meta: **"a working prototype that demonstrates the trade-off mechanic clearly enough that a young player walks away understanding a real constraint of deep space survival"**. Se aceptan interpretaciones creativas de "outpost", "site" o "mission" más allá de la Luna y Marte, mientras el objetivo educativo siga central.

## Consideraciones potenciales
**Audiencia y accesibilidad**
- Público objetivo: niños de 7-12 años; considerar si puede atraer también a hermanos mayores, padres o aulas.
- Lenguaje simple; si un término científico es inevitable (dosis de radiación, microgravedad, sol), explicarlo en contexto.
- Baja barrera de entrada: una experiencia jugable en navegador, sin descarga ni cuenta, llegará a más gente durante el juzgamiento y después.

**Basarse en datos reales**
- Usar APIs públicas y datasets abiertos de la NASA en vez de inventar números; incluso un poco de dato real (una lectura de radiación, una foto real de Marte, un hallazgo de salud) da mucha más credibilidad.
- Atribuir claramente las fuentes dentro de la herramienta.
- Si no es viable llamar APIs en vivo, un dataset real curado embebido es una alternativa razonable y honesta: **"just don't present static data as live."**

**Errores de diseño a evitar**
- No diseñar una opción "segura" sin costo.
- No dejar que el pulido visual reemplace la precisión de las mecánicas.
- No abrumar con demasiados sistemas: **dos o tres recursos centrales** (soporte vital, energía, blindaje) suelen funcionar mejor que cinco o seis.

**Qué haría destacar una solución**
- Que los jugadores vean las consecuencias reales de su decisión, por ejemplo vinculando el blindaje con datos actuales de clima espacial.
- Rejugabilidad: eventos que varían entre partidas para probar estrategias y aprender del fracaso sin que se sienta punitivo.
- Acompañar el prototipo con una breve explicación de qué datasets de la NASA se usaron y por qué, para que los jueces puedan verificar la ciencia.

## Datos y recursos de NASA
| Recurso | Qué es | Tipo de acceso | URL |
|---|---|---|---|
| NASA Open APIs | Portal central de APIs públicas de la NASA (APOD, Mars Rover Photos, DONKI, etc.); API key gratuita instantánea con más límite que la DEMO_KEY | API REST (API key gratuita) | https://api.nasa.gov/ |
| Space Weather Database Of Notifications, Knowledge, Information (DONKI) | Datos reales y actualizados de fulguraciones solares, eyecciones de masa coronal y otros eventos de clima espacial que generan riesgo de radiación | API REST (vía api.nasa.gov o CCMC) / visor web | https://ccmc.gsfc.nasa.gov/tools/DONKI/ |
| NASA Open Science Data Repository (OSDR) | Archivo público de biología espacial y salud de astronautas (densidad ósea, masa muscular, inmunidad, sueño, etc.), buscable por palabra clave. Incluye [search API](https://osdr.nasa.gov/osdr/data/search?term=KEYWORD) | Visor web + API REST (sin key) | https://osdr.nasa.gov/bio/repo/ |
| RadLab (OSDR Radiation Dosimetry Database) | Mediciones reales de dosis de radiación de dosímetros a bordo de la ISS, de varias agencias (p. ej. tasa de dosis absorbida) | API REST / visor web | https://visualization.osdr.nasa.gov/radlab/gui/data-api/ |
| Mars Trek | Portal interactivo de visualización y descarga de imágenes, elevación y terreno de Marte (incluye Curiosity y Perseverance) | Visor web / descarga directa (servicios de tiles) | https://trek.nasa.gov/mars/ |

## Recursos de agencias socias
| Recurso | Qué es | Tipo de acceso | URL |
|---|---|---|---|
| CSA: Space resources for STEM and youth organizations | Herramientas y actividades gratuitas y bilingües para educadores de museos, bibliotecas, campamentos, clubes y organizaciones comunitarias | Documento / página web | https://www.asc-csa.gc.ca/eng/youth-educators/resources-stem-and-youth-organizations.asp |
| CSA: Food production on the moon and in remote areas | Paquete para educadores y jóvenes sobre producción de alimentos en el espacio, salud y nutrición de astronautas y beneficios en la Tierra | Documento / toolkit web | https://www.asc-csa.gc.ca/eng/youth-educators/toolkits/food-production/ |

## Otros recursos
No especificado en la página oficial.

## Acceso a los datos (notas del repo)

> Por la regla oficial, no se puede trabajar en un challenge antes del hackathon, ni usar SI (IA) para idear la solución. Por eso esta sección solo trae notas de acceso a los datos, no enfoques de solución. La idea la pone el equipo.

*No es parte de la página oficial. Las URLs y endpoints que no están en la página son sugerencias adicionales: verificalos.*

- **DONKI:** `https://api.nasa.gov/DONKI/FLR?startDate=...&endDate=...&api_key=...` (también `CME`, `SEP`, `GST`) devuelve JSON. Pedir una API key propia: la `DEMO_KEY` tiene límite muy bajo (por IP/hora) y se agota rápido en demo frente a jueces. Llamar desde un proxy serverless para no exponer la key.
- **OSDR:** la search API devuelve JSON sin autenticación; útil para mostrar "hallazgos reales" en tarjetas (p. ej. pérdida ósea en ratones de la ISS). Curar 5-10 estudios a mano y guardar su título/ID en un JSON con link de atribución.
- **RadLab:** su data API permite bajar series de dosis de la ISS; precalcular una tasa de dosis diaria típica (en µGy/día) y usarla para escalar el daño por radiación en el juego. Las cifras de superficie lunar/marciana no vienen de ahí: si las usan, citarlas aparte.
- **Skills del repo aplicables:** space-apps-challenge, edu-games, nasa-data-apis, planetary-data, frontend-design, ai-features (narrador/tutor opcional), deploy-vercel-supabase, pitch-and-submission, hackathon-sprint.
