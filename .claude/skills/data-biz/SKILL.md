---
name: data-biz
description: Análisis de datos orientado a decisiones e impacto ("data + business") — define el usuario y la decisión que el producto mejora, elige métricas, convierte resultados de datos de NASA en insights accionables, cuantifica impacto y arma el caso de valor y sostenibilidad para el jurado. Úsala al definir el problema, antes de hacer gráficos, al escribir el resumen del proyecto o cuando haya que responder "¿y esto a quién le sirve y cuánto?".
---

# Data + Biz: de datos a decisiones con impacto

> Un marco de preguntas para que **el equipo** encuadre su propia idea. La IA puede ayudar a ordenar y cuestionar, pero no a inventar la solución del challenge (regla oficial).

Un proyecto de Space Apps gana cuando la persona que lo ve entiende en 30 segundos **quién lo usa, qué decide mejor gracias a él y por qué hacen falta los datos de NASA**. Esta skill te obliga a responder eso antes de escribir código y a mostrarlo en el producto.

## 1. Encuadre (15 minutos, antes de programar)
Completa esto en `docs/PROJECT.md` (crealo):

```
Usuario principal:       (rol concreto: "enfermera de una escuela pública urbana", no "la gente")
Decisión que toma:       (qué elige, cuándo y con qué frecuencia)
Cómo la toma hoy:        (intuición, planilla, herramienta X) → dolor concreto
Pregunta que responde nuestra herramienta:  "¿...?"
Datos NASA que la responden:  (producto, variable, resolución, período)
Señal de éxito en la demo:   (qué ve el usuario que antes no veía)
Métrica de impacto (estimada): (horas ahorradas, hectáreas, personas alcanzadas, alertas anticipadas en días)
```

## 2. Del dato al insight (patrón "Qué → Y qué → Ahora qué")
Cada pantalla o gráfico necesita las tres capas:
(El ejemplo es de un dominio que no está entre los challenges de 2026: calidad del aire.)
- **Qué:** el dato ("el PM2.5 de julio en la ciudad subió un 35 % respecto de la media 2015–2025").
- **Y qué:** la interpretación ("es el julio con peor aire del registro").
- **Ahora qué:** la acción ("suspender la educación física al aire libre los días de alerta").

Ponle al gráfico un título que diga la conclusión, no la variable ("Julio fue el mes con peor aire en 10 años", no "PM2.5 mensual").

## 3. Métricas: elegir pocas y buenas
- Una **métrica norte** que represente el valor para el usuario y 2 o 3 de soporte. Más de eso confunde.
- Siempre con **línea base o comparación**: año anterior, promedio histórico u otra región. Un número solo no dice nada.
- Expón la **incertidumbre** sin esconderla: intervalos, "significativo / no significativo", cobertura de datos.

## 4. Análisis exploratorio rápido (notebook)
```python
import pandas as pd
df = pd.read_csv("data/processed/series.csv", parse_dates=["date"])
df.describe(); df.isna().mean()                    # rangos y huecos
m = df.set_index("date").resample("MS").mean()     # agregación mensual
clim = m.groupby(m.index.month).transform("mean")  # climatología
anom = m - clim                                    # anomalías
```
Comprueba: unidades, huecos, outliers, cambios de sensor o versión y zona horaria. Registra cada decisión en el notebook. Después sirve para la sección "Metodología".

## 5. Caso de valor (para el resumen y el pitch)
- **Problema → evidencia del problema** (un dato o una fuente).
- **Solución → por qué con datos de NASA** (cobertura global, historia larga, gratuidad).
- **Impacto estimado** con supuestos explícitos: "si 50 técnicos lo usan, cada uno ahorra 4 h por mes ⇒ 200 h/mes". Mejor conservador y defendible que inflado.
- **Escalabilidad:** qué cambia para llevarlo a otra región o a otro cultivo, cuerpo celeste o misión.
- **Sostenibilidad:** costo de operación (con free tiers, ≈ 0), quién lo mantendría (agencia, ONG, municipio, escuela), un siguiente paso concreto.
- **Usuarios y aliados posibles en la región**: ministerios, defensa civil, universidades, escuelas, ONG, agencias espaciales socias (CONAE, AEB, CSA…).

## Mapa a los criterios del jurado (Impact, Creativity, Validity, Relevance, Presentation)
- **Impact** ← encuadre (§1) + caso de valor (§5).
- **Validity** ← análisis exploratorio documentado (§4) + incertidumbre (§3).
- **Relevance** ← "Datos NASA que la responden" y requisitos del challenge cubiertos.
- **Presentation** ← títulos con conclusión (§2) + una historia clara.

## 6. Anti-patrones
- Dashboard con 12 gráficos y ninguna conclusión.
- "IA" o "predicción" sin validación ni línea base.
- Métricas de vanidad (cantidad de datos procesados) en lugar de valor para el usuario.
- Afirmaciones de impacto sin supuestos.

## Entregables de esta skill
- `docs/PROJECT.md` con el encuadre.
- Títulos con conclusión en cada visualización.
- Una sección "Impacto" en el README y en el pitch (ver `pitch-and-submission`).
