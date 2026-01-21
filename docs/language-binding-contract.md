# Language Binding Contract
*(Contrato de Bindings por Lenguaje)*

---

## Naturaleza del Contrato

Pollux Polyglot Native Bridge define el contrato que **todos** los bindings por lenguaje
deben cumplir.

No define implementación.  
Define **frontera**.

---

## Responsabilidades de Bindings

Todo binding por lenguaje (JS, .NET, etc.) **debe**:

1. Traducir intención del lenguaje → `BridgeRequest`.
2. Invocar `PolyglotBridge::process_request`.
3. Traducir `BridgeResponse` → representación del lenguaje.
4. Propagar errores sin silenciarlos.
5. No implementar lógica de autorización.
6. No cachear decisiones.
7. No adaptar comportamiento por contexto.

---

## Responsabilidades Prohibidas

Todo binding por lenguaje **no debe**:

- Acceder directamente a Pollux Runtime.
- Acceder directamente a Pollux Contracts.
- Acceder directamente a Pollux Capabilities.
- Implementar lógica de validación.
- Inferir valores por defecto.
- Negociar compatibilidad.
- Suavizar rechazo.
- Ocultar estados de error.

---

## Modelo de Interacción

### Unidireccionalidad

Binding → Bridge → Pollux

Nunca:

Pollux → Bridge → Binding

No existen callbacks.  
No existen notificaciones inversas.  
No existe push de eventos.

---

### Aislamiento de Estado

Cada invocación es independiente.

No existe:

- Estado de sesión.
- Caché de autorización.
- Contexto persistente.

---

### Fail-Fast

Ante cualquier fallo, el binding debe:

- Propagar el error explícitamente.
- No reintentar automáticamente.
- No silenciar.
- No interpretar.

---

## Compatibilidad de Bindings

### Versionado

Los bindings deben:

- Especificar versión de Bridge soportada.
- Rechazar invocaciones si Bridge es incompatible.
- No asumir retrocompatibilidad implícita.

---

### Evolución

Si Bridge cambia su modelo:

- Los bindings deben actualizarse.
- No existe adaptación automática.
- No existe negociación de versión.

---

## Principio de Subordinación

Los bindings **sirven** al Bridge.

El Bridge no sirve a los bindings.

Si un binding requiere funcionalidad no soportada,  
el binding está mal diseñado.

---

## Principio de Cierre

La frontera no se adapta.

Los bindings se adaptan.

