# Request Response Model
*(Modelo de Solicitud / Respuesta)*

---

## Naturaleza del Modelo

El modelo de solicitud/respuesta de Pollux Polyglot Native Bridge es:

- Cerrado.
- Explícito.
- Inmutable.
- Serializable.
- Sin estado compartido.

Si una interacción no puede expresarse en este modelo,  
no se soporta.

---

## Estructura de Solicitud

### BridgeRequest

Toda solicitud externa se estructura como:

```
BridgeRequest {
    request_type: BridgeRequestType,
    payload: BridgePayload,
}
```

**Invariantes:**

- `request_type` es cerrado (enumeración finita).
- `payload` corresponde directamente a `request_type`.
- No existen variantes "otras" o "personalizadas".

---

### BridgeRequestType

Tipos de solicitud soportados:

- `ValidateManifest`
- `ValidateCapabilityRequest`
- `ValidateLifecycleTransition`
- `ValidateTargetCompatibility`

Este conjunto es **exhaustivo**.

Cualquier operación no representable aquí no es soportada.

---

### BridgePayload

Carga útil específica por tipo de solicitud:

- `ManifestValidation`: capabilities, target, module_type.
- `CapabilityRequest`: capability.
- `LifecycleTransition`: from_phase, to_phase.
- `TargetCompatibility`: declared_target, runtime_target.

Cada payload es **cerrado**.  
No existen campos opcionales ambiguos.  
No existen valores por defecto implícitos.

---

## Estructura de Respuesta

### BridgeResponse

Toda respuesta es:

```
BridgeResponse {
    status: BridgeResponseStatus,
    reason: Option<String>,
}
```

**Invariantes:**

- `status` es cerrado (enumeración finita).
- `reason` es `Some` cuando `status` es `Rejected` o `InvalidRequest`.
- `reason` es `None` cuando `status` es `Approved`.

---

### BridgeResponseStatus

Estados de respuesta soportados:

- `Approved`: solicitud válida y conforme a contratos.
- `Rejected`: solicitud viola contratos.
- `InvalidRequest`: solicitud estructuralmente inválida.

Este conjunto es **exhaustivo**.

No existen estados parciales.  
No existen advertencias.  
No existen sugerencias.

---

## Semántica de Serialización

El modelo está diseñado para ser serializable en:

- JSON.
- MessagePack.
- Protocol Buffers.
- Cualquier formato estructurado.

No depende de:

- Callbacks.
- Closures.
- Referencias mutables.
- Estado ambiental.

---

## Invariantes del Modelo

### Inmutabilidad

Una vez construida, una solicitud no puede mutar.  
Una vez generada, una respuesta no puede mutar.

---

### Ausencia de Estado Compartido

Solicitud y respuesta son valores independientes.

No existe:

- Estado de sesión.
- Contexto compartido.
- Caché de decisiones.

Cada solicitud es **autónoma**.

---

### Determinismo

Dada la misma solicitud, la respuesta es siempre idéntica.

No depende de:

- Tiempo.
- Estado externo.
- Ejecuciones previas.

---

## Principio de Cierre

El modelo no crece por conveniencia.

Si una operación no puede expresarse aquí,  
no pertenece a la frontera.

