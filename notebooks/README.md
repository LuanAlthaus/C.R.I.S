# notebooks/ — pre-procesamiento de datos (Python)

Aquí se baja, limpia y reduce los datos científicos (HDF5, NetCDF, GeoTIFF, FITS, CSV grandes). **La web app nunca procesa archivos crudos**: solo consume el resultado liviano.

```bash
python3 -m venv .venv && source .venv/bin/activate
pip install -r notebooks/requirements.txt
jupyter lab            # o abre los .ipynb en VS Code
```

Convenciones:
- `data/raw/` → descargas originales (**ignoradas por git**).
- `data/processed/` → intermedios (ignorados).
- `public/data/` → **solo** lo que la app sirve (JSON/GeoJSON/CSV/PNG/COG chicos, que sí van a git).
- Credenciales por variables de entorno (`EARTHDATA_TOKEN`, `NASA_API_KEY`…), nunca escritas en el notebook.
- Para cargar a Supabase desde un script usa `SUPABASE_SECRET_KEY` (solo local, nunca en el frontend).

```python
# Subir un DataFrame a Supabase
import os, pandas as pd
from supabase import create_client
sb = create_client(os.environ["NEXT_PUBLIC_SUPABASE_URL"], os.environ["SUPABASE_SECRET_KEY"])
df = pd.read_csv("data/processed/result.csv")
for i in range(0, len(df), 500):                       # en lotes
    sb.table("observations").insert(df.iloc[i:i+500].to_dict("records")).execute()
```

Skills relevantes: `nasa-data-apis`, `earth-geospatial-data`, `planetary-data`, `data-biz`.
