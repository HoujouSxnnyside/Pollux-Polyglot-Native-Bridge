# Boundary Enforcement
*(Enforcement de Frontera en Pollux Polyglot)*

---

## Naturaleza del Enforcement

Pollux Polyglot Native Bridge no autoriza.  
No ejecuta.  
No decide.

**Valida**.

Toda solicitud externa es sometida a validación de contrato **antes** de cualquier
interacción con Pollux Runtime.

---

## Modelo de Validación

### Validación de Manifest

Solicitud externa provee:

- Lista de capacidades solicitadas.
- Target declarado.
- Tipo de módulo.

La frontera:

1. Parsea capacidades contra vocabulario canónico (Pollux Capabilities).
2. Parsea target y tipo de módulo.
3. Invoca `validate_authorization` de Pollux Contracts.
4. Retorna `Approved` o `Rejected` con razón.

No existe negociación.  
No existe retry automático.  
No existe escalada progresiva.

---

### Validación de Solicitud de Capacidad

Solicitud externa provee:

- Capability string.

La frontera:

1. Parsea contra vocabulario canónico.
2. Retorna `Approved` si existe, `InvalidRequest` si no.

No valida autorización contextual aquí.  
Solo valida existencia en vocabulario.

---

### Validación de Transición de Ciclo de Vida

Solicitud externa provee:

- Fase origen.
- Fase destino.

La frontera:

1. Parsea fases contra enumeración canónica.
2. Invoca `validate_lifecycle_transition` de Pollux Contracts.
3. Retorna `Approved` o `Rejected` con razón.

No modifica estado.  
No ejecuta transición.  
Solo valida admisibilidad.

---

### Validación de Compatibilidad de Target

Solicitud externa provee:

- Target declarado.
- Target de runtime.

La frontera:

1. Parsea targets.
2. Invoca `validate_target` de Pollux Contracts.
3. Retorna `Approved` o `Rejected` con razón.

No adapta targets.  
No negocia compatibilidad.

---

## Estados de Respuesta

### Approved

La solicitud es válida y cumple con todos los contratos.

No implica ejecución.  
Implica admisibilidad formal.

---

### Rejected

La solicitud viola uno o más contratos.

La razón es explícita.  
No existe retry automático.  
No existe sugerencia de corrección.

---

### InvalidRequest

La solicitud es estructuralmente inválida:

- Capacidad inexistente.
- Target inválido.
- Fase inválida.
- Tipo de módulo inválido.

No es una violación de contrato.  
Es una falla de traducción.

---

## Principio de No Interpretación

La frontera no interpreta intención.

Si una solicitud no se expresa exactamente en el modelo cerrado,  
no existe.

No hay:

- Inferencia de valores por defecto.
- Traducción de sinónimos.
- Adaptación a convenciones externas.
- Corrección automática.

---

## Principio de Cierre

La frontera es impermeable.

Si algo puede filtrarse,  
la frontera ha fallado.

