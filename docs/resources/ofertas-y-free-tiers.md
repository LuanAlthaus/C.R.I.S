# Ofertas de Space Apps 2026 y free tiers de infraestructura

> Revisado el **2026-11-13**. Los proveedores cambian los límites seguido: antes de depender de un número, confirmalo en la página oficial.
> Cuando un número no se pudo verificar en la fuente oficial, se marca como **"no verificado"**.

---

## 1. Global Offers 2026 (Global Collaborators)

Los **Global Collaborators de 2026** son **GoDaddy Registry, Google, Meteomatics, Microsoft y Miro**. Sus ofertas se publicaron el **13 de noviembre** y son opcionales.

Fuente: https://www.spaceappschallenge.org/2026/global-offers/ (**requiere iniciar sesión** en Space Apps), leída el 2026-11-13 con una cuenta de participante. Cada oferta se reclama con su botón **"Claim Offer"** en esa página. Las dudas van al contacto de soporte de cada oferta o a la categoría "Global Offers" de Space Apps Connect.

| Collaborator | Oferta | Qué da | Condiciones y vencimiento | Soporte |
|---|---|---|---|---|
| **GoDaddy Registry** | Dominio y hosting gratis | **Un dominio gratis y un año de hosting**, con certificado SSL por el mismo año. | Solo dominios **no premium**. Se pide con un email válido, llega un código por correo y el registro se termina en **Porkbun** (el socio de registro de GoDaddy). | support@porkbun.com |
| **Google** | Google Cloud | **US$ 300 de crédito por 90 días** para Google Cloud (incluye Firebase y la API de Google Maps). | Pide una tarjeta de crédito o débito para verificar que no sos un robot. No cobra solo al terminar la prueba: solo si pasás manualmente a una cuenta paga. | cloudstartupsupport@google.com |
| **Meteomatics** | Weather API | **1.000 consultas por día** en todas las funciones de la API: datos históricos, en tiempo real y pronósticos para cualquier lugar, más de 1.800 parámetros, en JSON, CSV, XML o PNG. Tiene conector para Python. | Se completa un formulario y mandan las credenciales. **Activo mientras dure el hackathon.** | support@meteomatics.com |
| **Microsoft** | Azure | **US$ 200 de crédito por 30 días** para clientes nuevos, más servicios gratis hasta cierto uso mensual (algunos siempre, otros por 12 meses). | Puede pedir teléfono, una tarjeta de crédito o débito **no prepaga** y una cuenta de Microsoft o GitHub. Al vencer el crédito hay que pasar a pago por uso para seguir con los servicios pagos. | samfal@microsoft.com |
| **Microsoft** | Planetary Computer | Catálogo de datos ambientales globales (varios petabytes) con API **STAC** y herramientas para ciencias de la Tierra. | **Público, sin cuenta.** La mayoría de los datasets se bajan sin iniciar sesión; algunos pedidos necesitan un token gratuito. | samfal@microsoft.com |
| **Microsoft** | GitHub Copilot | Plan **Copilot Free**: hasta **2.000 completions de código por mes**, más chat y agente con uso limitado. | Sin tarjeta. Los planes pagos (Pro, Pro+, Max) no están incluidos. | samfal@microsoft.com |
| **Microsoft** | Copilot | Asistente gratis en copilot.com y en sus apps. | Hay que iniciar sesión. | samfal@microsoft.com |
| **Microsoft** | Teams | Teams Free: reuniones de hasta 100 participantes y 60 minutos, chat ilimitado y 5 GB de archivos por usuario. | Con una cuenta de Microsoft. | samfal@microsoft.com |
| **Miro** | Education Plan | Miro gratis **dos años para estudiantes** y para siempre para educadores. | Hay que postularse como estudiante o educador. | help.miro.com |
| **Miro** | Business Plan | Plan Business **gratis por 14 días**. | **Pide datos de pago**: hay que cancelar antes de los 14 días para que no cobre. | help.miro.com |
| **Miro** | Plantillas | Dos tableros gratis: *Hackathon Template Board* (trabajo en equipo e ideas) y *Hackathon Project Coach Board* (guía del proyecto). | Con una cuenta de Miro. | community.miro.com |

**Qué conviene para este stack:**
- **Dominio gratis (GoDaddy Registry):** sirve para ponerle un dominio propio al deploy de Vercel. Se agrega en Vercel → Settings → Domains.
- **Planetary Computer:** otra fuente de datos de la Tierra por STAC, sin cuenta. No reemplaza a los datos de NASA que exige la entrega.
- **Meteomatics:** 1.000 consultas por día alcanzan si se cachea en una Route Handler. Vence al terminar el hackathon, así que el demo no puede depender de la API en vivo durante la evaluación (dic–ene): guardar las respuestas.
- **Azure, Google Cloud y Miro Business piden tarjeta.** No hacen falta para el stack gratuito de este repo (Vercel + Supabase).

---

## 2. Free tiers (revisados el 2026-11-13)

### Vercel: Hobby (gratis)
Fuentes: https://vercel.com/docs/plans/hobby (actualizada el 12 de noviembre de 2026) · https://vercel.com/docs/limits · https://vercel.com/legal/terms

| Recurso | Límite Hobby |
|---|---|
| Fast Data Transfer (ancho de banda) | **100 GB/mes** |
| Fast Origin Transfer | 10 GB/mes |
| CDN Requests | 1.000.000/mes |
| Function Invocations | **1.000.000/mes** |
| Active CPU | **4 CPU-horas/mes** |
| Provisioned Memory | 360 GB-horas/mes |
| Duración máxima de una función | **300 s (5 minutos)**: es el valor por defecto y el máximo |
| Image Transformations | 5.000/mes |
| Build | **45 minutos como máximo por deploy**, **1 deploy concurrente**, 2 vCPU y 8 GB de RAM |
| Deployments por día | 100 |
| Proyectos | 200 |
| Runtime logs | 1 hora |
| **Uso comercial** | **Prohibido.** Los Terms dicen: *"You shall only use the Services under a Hobby plan for your personal or non-commercial use."* |
| Si superás un límite | En la mayoría de los casos hay que **esperar 30 días** para volver a usar ese recurso. No hay cobro por excedente. |

Notas:
- Vercel **no publica una cuota mensual de "build minutes"** para Hobby. El límite está por deploy (45 minutos) y por concurrencia (1).
- Un proyecto de hackathon sin fines de lucro entra en el uso "non-commercial". El límite de 100 deploys por día puede pesar si se despliega en cada commit.

### Supabase: Free
Fuente: https://supabase.com/pricing

| Recurso | Límite Free |
|---|---|
| Base de datos | **500 MB por proyecto**, CPU compartida, 500 MB de RAM |
| File storage | **1 GB**, con un máximo de **50 MB por archivo** |
| Egress | **5 GB**, más 5 GB de cached egress |
| MAU (Auth) | **50.000** (más 50.000 de third-party MAU) |
| API requests | Ilimitados |
| Edge Functions | 500.000 invocaciones/mes |
| Realtime | 200 conexiones concurrentes y 2 millones de mensajes/mes |
| Backups automáticos | **No incluidos** |
| **Pausa** | **Los proyectos se pausan después de 1 semana de inactividad.** |
| **Proyectos gratis** | **2 proyectos activos** como máximo. Los proyectos pausados no cuentan para ese límite. |
| Log retention | 1 día |

Notas:
- Si nadie usa el proyecto por 7 días, se pausa. Esto importa entre el 15 de noviembre y las rondas de evaluación (3 y 17 de diciembre, 26 de enero). Si el demo depende de Supabase, alguien del equipo tiene que entrar o hacer algún request **al menos una vez por semana** hasta enero. **Este repo ya lo resuelve:** `vercel.json` define un cron diario (permitido en Hobby) que llama a `/api/cron/keepalive` y hace una consulta liviana a Supabase. Verificalo en Vercel → Settings → Cron Jobs.
- No hay backups. Exportá la base con `pg_dump` antes de cerrar.

### Cloudflare Pages, Workers y R2 (Free)
Fuentes: https://developers.cloudflare.com/pages/platform/limits/ · https://developers.cloudflare.com/workers/platform/pricing/ · https://developers.cloudflare.com/r2/pricing/

**Pages (Free):**
| Recurso | Límite |
|---|---|
| Builds | **500/mes**, 1 a la vez, **20 minutos** de timeout |
| Archivos por sitio | 20.000 |
| Tamaño máximo por archivo | **25 MiB** |
| Custom domains | 100 |
| Ancho de banda estático | No hay un límite publicado en la página de límites. Más allá de eso, **no verificado**. |

**Workers y Pages Functions (Free):** **100.000 requests por día** y **10 ms de CPU por invocación**. Las Pages Functions se facturan como Workers.

**R2 (Free, mensual):**
| Recurso | Límite |
|---|---|
| Storage | **10 GB-mes** |
| Class A operations (escritura) | 1 millón |
| Class B operations (lectura) | 10 millones |
| **Egress a Internet** | **Gratis** |

### Google AI Studio y Gemini API: free tier
Fuentes: https://ai.google.dev/gemini-api/docs/rate-limits · https://ai.google.dev/gemini-api/docs/pricing

- Hay un **Free tier** para un proyecto activo, sin billing. Algunos proyectos necesitan verificación.
- Los límites se aplican **por proyecto, no por API key**, y se miden en RPM, TPM y RPD. **El RPD se reinicia a la medianoche, hora del Pacífico.**
- **Los números exactos por modelo ya no aparecen en la documentación:** se ven en el panel de AI Studio (https://aistudio.google.com/rate-limit). Por eso **los RPM y RPD del free tier por modelo no están verificados.**
- Según la página de pricing, tienen free tier, entre otros: **Gemini 3.8 Flash, 3.6 Flash, 3.5 Flash-Lite, 3.1 Flash-Lite, 2.5 Pro, 2.5 Flash, 2.5 Flash-Lite, Gemini Embedding y Gemma 4**.
- **En el free tier, el contenido se usa para mejorar los productos de Google.** No mandes datos sensibles.
- Un número publicado en la documentación: el modelo **Gemini Decision 1** tiene, en el free tier, 200 RPM, 50.000 TPM y 10.000 RPD.
- La búsqueda con Grounding with Google Search tiene un límite por defecto de 1.500 RPD por proyecto en los modelos Gemini 3 y posteriores.

### Groq: free tier
Fuente: https://console.groq.com/docs/rate-limits

- Existe un **Free plan**. La página tiene pestañas "Free Plan Limits" y "Developer Plan Limits".
- La tabla pública dice que muestra **"the base limits for the Developer plan"**. **No se pudieron verificar los números del Free plan.** Cada organización ve sus límites exactos en https://console.groq.com/settings/limits.
- Como referencia (tabla del Developer plan): `openai/gpt-oss-120b` y `openai/gpt-oss-20b` tienen 30 RPM, 1.000 RPD, 8.000 TPM y 200.000 TPD. `whisper-large-v3` tiene 20 RPM y 2.000 RPD.
- Los límites se aplican por organización y, cuando se superan, la API devuelve `429` con el header `retry-after`.

### Hugging Face Spaces
Fuentes: https://huggingface.co/docs/hub/spaces-overview · https://huggingface.co/docs/hub/spaces-gpus · https://huggingface.co/pricing

- **Cambio importante:** **los Static Spaces son gratis para todos**, pero **crear Spaces Gradio o Docker (los que corren cómputo) ahora requiere un plan pago** (PRO para cuentas personales). Excepción: las cuentas personales gratuitas pueden tener **hasta 2 Spaces Gradio en ZeroGPU**.
- **CPU Basic:** 2 vCPU, 16 GB de RAM y 50 GB de disco **no persistente**. No tiene costo por hora, pero solo está disponible con las condiciones del punto anterior.
- **Sleep:** en hardware gratuito, el Space se duerme después de **48 horas** sin uso y se reactiva cuando entra una visita. En hardware gratuito no se puede cambiar el tiempo de sleep.
- **ZeroGPU:** aparece como "FREE" en la página de pricing, sin la cuota exacta para usuarios free. **La cuota no está verificada.**

### GitHub Pages
Fuente: https://docs.github.com/en/pages/getting-started-with-github-pages/github-pages-limits

| Recurso | Límite |
|---|---|
| Tamaño del sitio publicado | **1 GB** |
| Ancho de banda | **100 GB/mes** (límite blando) |
| Builds | **10 por hora** (límite blando) |
| Timeout de deploy | 10 minutos |
| Repositorio | Con GitHub Free, **solo repos públicos** |
| Uso comercial | **No está permitido** como hosting para negocios, e-commerce o SaaS. |

### Netlify: Free
Fuente: https://www.netlify.com/pricing/

- **"$0 forever"**, con un **límite de 300 créditos**. La página **no aclara** si los créditos se renuevan cada mes.
- El ancho de banda se cobra a **20 créditos por GB**: si solo se gasta en ancho de banda, alcanza para unos **15 GB**. Esta cuenta es propia, no un límite publicado. Las web requests cuestan 2 créditos cada 10.000.
- Los build minutes, las invocaciones de funciones, lo que pasa cuando se acaban los créditos y el uso comercial **no aparecen en la página de pricing: no verificado**.
- Incluye deploy previews ilimitados, custom domains con SSL, functions, Netlify Database, Blob storage y CDN global.

### Render: Free
Fuente: https://render.com/docs/free

- Un web service gratis **se apaga después de 15 minutos sin tráfico** y tarda en arrancar en el siguiente request (cold start).
- **750 horas de instancia por mes** por workspace. No se acumulan de un mes a otro.
- El ancho de banda y los build minutes incluidos **no aparecen en la documentación de free: no verificado**. Si se superan sin un método de pago cargado, los servicios gratuitos se suspenden hasta fin de mes.
- **Postgres gratis:** **1 GB**, **vence a los 30 días** de creado (con 14 días de gracia antes de borrarse) y solo hay uno por workspace. **Vencería antes de que terminen las rondas de evaluación.**
- **Key Value gratis:** no persiste a disco, así que los datos se pierden al reiniciar. Hay uno por workspace.

### Railway: Trial y Free
Fuente: https://railway.com/pricing

- **Trial:** **US$5 de crédito único durante 30 días**, hasta 2 vCPU y 1 GB por servicio, y volúmenes de 500 MB como máximo.
- **Free, después del trial:** **US$1 de crédito por mes**, hasta 1 vCPU y 0,5 GB por servicio, y volúmenes de 500 MB como máximo.
- Alcanza para el fin de semana, **pero no para mantener una API prendida hasta enero**.

---

## 3. Qué usar para qué

Hay que pensar en que **el demo siga vivo durante la evaluación, hasta el 26 de enero de 2027**.

| Necesidad | Primera opción | Alternativa | Ojo con |
|---|---|---|---|
| **Sitio estático o SPA** | **Cloudflare Pages** (sin límite de ancho de banda publicado) | GitHub Pages (repo público, 100 GB/mes), Vercel Hobby | GitHub Pages exige un repo público con cuenta Free. |
| **Frontend Next.js con API routes** | **Vercel Hobby** | Netlify Free | Las funciones duran 5 minutos como máximo, hay 4 CPU-horas por mes y el uso tiene que ser no comercial. |
| **API en Python (FastAPI o Flask)** | **Render Free** (750 h/mes) | Railway (trial de US$5), Vercel Python functions | Render se duerme a los 15 minutos (el cold start se nota en el demo). Railway se queda sin crédito. HF Spaces con Docker ahora necesita PRO. |
| **Base de datos y auth** | **Supabase Free** (500 MB, 50.000 MAU) | Render Postgres (vence a los 30 días, no sirve hasta enero) | Supabase se pausa después de 7 días sin uso. |
| **Archivos grandes (rasters, NetCDF, GeoTIFF, imágenes)** | **Cloudflare R2** (10 GB y egress gratis) | Supabase Storage (1 GB, 50 MB por archivo) | Pages y Vercel no sirven para archivos grandes: Pages admite hasta 25 MiB por archivo. Mejor leer los datos de NASA desde su origen (Earthdata, buckets S3 públicos) que duplicarlos. |
| **LLM** | **Gemini API free tier** (AI Studio) | Groq Free (inferencia rápida de modelos abiertos) | Free tier de Gemini: los datos se usan para entrenar y los límites se ven solo en el panel. Groq Free: números no verificados. **Declarar todo en el campo de SI.** |
| **Demo de ML con UI** | HF **Static Space**, o Gradio en ZeroGPU (2 gratis) | Streamlit o Gradio en Render | En HF, Gradio y Docker en CPU ahora necesitan PRO. |
| **Clima y meteorología** | **Meteomatics** (Global Offer) | Datos de NASA POWER | El acceso a Meteomatics dura "lo que dure el challenge". |
