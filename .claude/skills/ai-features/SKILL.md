---
name: ai-features
description: Añadir funciones de IA/LLM ("Super Intelligence", SI, en la terminología de Space Apps 2026) a la app con free tiers — resúmenes, clasificación, chat sobre documentos (RAG) con citas, extracción estructurada desde PDFs, embeddings en Supabase pgvector. Incluye guardrails, las reglas oficiales de uso de SI (marca de agua, declaración, no idear la solución) y cómo cumplirlas. Úsala siempre que el proyecto use un LLM o contenido generado.
---

# Funciones de IA / SI (con presupuesto cero)

## Reglas oficiales de Space Apps 2026 (ver `docs/resources/space-apps-reglas-entrega-jurado.md` §4)
- El sitio llama **"Super Intelligence (SI)"** a la IA. Se permite usarla para desarrollar, pero **no para idear la solución del challenge**.
- La entrega **debe describir cómo se usó la SI** (campo "Use of Super Intelligence").
- **Imágenes y videos generados: marca de agua visible** que diga que son generados con SI. Audio, código y datos generados: identificalos en texto y metadatos (README, pie de la UI).
- El contenido generado no puede usar ni modificar logos o marcas de NASA.

## Principios para el jurado
1. **La IA es un medio, no el producto.** Muestra qué decisión mejora (ver `data-biz`).
2. **Todo resultado con cita a la fuente** (paper, experimento, dataset de NASA). Sin cita, no se muestra.
3. **Transparencia:** etiqueta en la UI ("Generado con SI") cada texto, imagen o audio generado.

## Proveedores gratis (verifica los límites en `docs/resources/ofertas-y-free-tiers.md`)
- **Google AI Studio / Gemini API:** free tier con key de https://aistudio.google.com/ . Google es Global Collaborator de Space Apps 2026: revisa la oferta.
- **Groq:** modelos open-weight muy rápidos, free tier con límites por minuto.
- **Hugging Face Inference / Spaces:** modelos de embeddings y demos.
- **Embeddings locales en Python** (`sentence-transformers`, por ejemplo `all-MiniLM-L6-v2`, 384 dimensiones): gratis y sin límites. Genera los embeddings una vez en el notebook y súbelos a Supabase.

Usa el modelo **desde un Route Handler** (`src/app/api/...`), con la key en una variable de entorno del servidor. Pon un límite de tokens y un timeout. Las funciones Hobby de Vercel tienen duración máxima: haz streaming de respuestas largas.

## RAG sobre documentos (patrón genérico)
1. **Ingesta (notebook):** PDFs o páginas → texto (`pymupdf`) → chunks de unos 800 tokens con solapamiento → embeddings → tabla `chunks(id, doc_id, title, url, page, content, embedding vector(384))`.
2. **Supabase pgvector:**
   ```sql
   create extension if not exists vector;
   create table chunks (id bigserial primary key, doc_id text, title text, url text, page int,
                        content text, embedding vector(384));
   create index on chunks using hnsw (embedding vector_cosine_ops);
   create or replace function match_chunks(q vector(384), k int default 8)
   returns table (id bigint, title text, url text, page int, content text, similarity float)
   language sql stable as $$
     select id, title, url, page, content, 1 - (embedding <=> q) as similarity
     from chunks order by embedding <=> q limit k $$;
   ```
   Activa RLS en `chunks`, con una política de solo lectura para `anon`.
3. **Consulta:** embebe la pregunta (mismo modelo que en la ingesta), llama `supabase.rpc("match_chunks", { q, k: 8 })` y pasa los chunks al LLM con la instrucción "responde solo con este contexto; cita [n]; si no está, di que no hay evidencia".
4. **UI:** respuesta + lista de fuentes clicables + cuántas fuentes la respaldan.

## Extracción estructurada (de texto a tabla)
Pide al LLM JSON con un esquema fijo que defina el equipo, por ejemplo `{title, date, variables: [...], finding, source_url}`. Valídalo con `zod` y guárdalo en una tabla. El dashboard se arma sobre esa tabla con datos verificables, no sobre texto libre. Revisa a mano una muestra de 10 filas y reporta la precisión.

## Guardrails
- Temperatura baja (0–0.3) para extracción y resúmenes.
- Rechaza preguntas fuera de dominio con un mensaje claro.
- Rate-limit por IP en el endpoint para no gastar la cuota en la demo. Basta un contador en memoria o en una tabla de Supabase.
- Ten respuestas cacheadas para las 5 preguntas de la demo, por si el proveedor falla en vivo.
