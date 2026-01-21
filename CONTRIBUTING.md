# CONTRIBUTING
## Política de Contribuciones a Pollux Polyglot Native Bridge

Este repositorio es **público y de código abierto**.

La apertura no implica:

- Flexibilidad arquitectónica.
- Toma de decisiones colaborativa.
- Negociación de principios de diseño.
- Adaptación a conveniencia externa.

Este documento define qué contribuciones son aceptables y bajo qué condiciones.

---

## Principio Rector

Pollux Polyglot Native Bridge es una **frontera de gobernanza**, no un proyecto comunitario.

Las contribuciones son bienvenidas únicamente si:

- Fortalecen la frontera.
- No debilitan el aislamiento.
- No expanden autoridad externa.
- No introducen ambigüedad.

---

## Contribuciones Aceptables

### Corrección de Defectos

Se aceptan correcciones que:

- Resuelven fallos en validación de contratos.
- Corrigen errores de traducción.
- Reparan fugas de validación.
- Restauran invariantes de frontera.

No se aceptan "correcciones" que:

- Relajan validación.
- Introducen valores por defecto.
- Agregan tolerancia a entradas inválidas.
- Suavizan rechazo.

---

### Mejoras de Robustez

Se aceptan mejoras que:

- Refuerzan fail-fast.
- Agregan validación faltante.
- Endurecen parsing.
- Eliminan ambigüedad estructural.

No se aceptan "mejoras" que:

- Agregan comodidad.
- Introducen lógica adaptativa.
- Permiten bypass de validación.
- Relajan restricciones.

---

### Corrección de Documentación

Se aceptan correcciones documentales que:

- Clarifican invariantes.
- Corrigen descripciones incorrectas.
- Refuerzan la naturaleza de frontera.
- Eliminan ambigüedad.

No se aceptan cambios documentales que:

- Agregan tutoriales.
- Introducen ejemplos de uso.
- Suavizan lenguaje.
- Sugieren flexibilidad.

---

### Actualización de Dependencias

Se aceptan actualizaciones de dependencias que:

- Corrigen vulnerabilidades.
- Mantienen compatibilidad de API.
- No expanden superficie.
- No introducen comportamiento nuevo.

Toda actualización requiere validación formal.

---

## Contribuciones Explícitamente Rechazadas

### Expansión de Superficie

No se aceptan contribuciones que:

- Agregan nuevos tipos de solicitud.
- Expanden modelos de payload.
- Introducen nuevos estados de respuesta.
- Agregan operaciones de frontera.

La superficie de la frontera es cerrada.

---

### Abstracciones de Conveniencia

No se aceptan contribuciones que:

- Agregan helpers.
- Introducen builders.
- Simplifican construcción de solicitudes.
- Proporcionan DSLs.

La frontera no busca ergonomía.

---

### Lógica Específica de Lenguaje

No se aceptan contribuciones que:

- Agregan soporte para lenguajes específicos.
- Introducen adaptadores por ecosistema.
- Implementan serialización personalizada.
- Agregan traits de conveniencia.

La frontera es agnóstica al lenguaje.

---

### Optimizaciones Prematuras

No se aceptan contribuciones que:

- Agregan caché.
- Introducen pooling.
- Optimizan para casos específicos.
- Comprometen claridad por rendimiento.

La frontera prioriza corrección sobre velocidad.

---

## Proceso de Contribución

### 1. Verificación de Admisibilidad

Antes de trabajar en una contribución:

- Abra un issue describiendo el problema.
- Espere confirmación de que el problema es válido.
- Espere confirmación de que la solución es aceptable.

No inicie trabajo sin aprobación previa.

---

### 2. Requisitos de Implementación

Toda contribución debe:

- Incluir tests que demuestren el problema.
- Incluir tests que validen la corrección.
- Mantener 100% de cobertura en código modificado.
- No introducir warnings.
- No degradar claridad.

---

### 3. Requisitos de Pull Request

Todo PR debe:

- Referenciar el issue aprobado.
- Incluir descripción técnica precisa.
- Explicar por qué el cambio fortalece la frontera.
- Demostrar que no relaja restricciones.
- Incluir evidencia de tests pasando.

PRs sin issue aprobado serán cerrados sin revisión.

---

### 4. Revisión

Todo PR será revisado por:

- Arquitectos del proyecto.
- Mantenedores de Pollux Runtime.
- Comité de gobernanza (si corresponde).

La revisión evaluará:

- Corrección técnica.
- Alineación arquitectónica.
- Impacto en frontera.
- Compatibilidad con bindings.

---

### 5. Decisión Final

Los mantenedores tienen autoridad final sobre:

- Aceptación o rechazo.
- Solicitud de cambios.
- Cierre sin merge.

Las decisiones arquitectónicas no son negociables.

---

## Expectativas de Contribuyentes

### Cumplimiento de Principios

Todo contribuyente debe:

- Leer y comprender `design-principles.md` del proyecto Pollux.
- Respetar invariantes de frontera.
- No proponer expansiones de autoridad.
- No intentar relajar restricciones.

---

### Comunicación

Toda comunicación debe ser:

- Técnica.
- Precisa.
- Respetuosa.
- Libre de presión política o social.

Argumentos basados en:

- "Esto es más fácil"
- "Otros proyectos lo hacen"
- "La comunidad lo espera"

No son válidos.

---

### Respeto a Decisiones

Las decisiones arquitectónicas son:

- Finales.
- No sujetas a votación.
- No sujetas a popularidad.
- No sujetas a presión.

Los contribuyentes que no respeten decisiones finales serán excluidos.

---

## Licencia de Contribuciones

Al contribuir a este proyecto, usted acepta:

- Licenciar su contribución bajo MIT License.
- Ceder derechos de modificación y redistribución.
- No imponer restricciones adicionales.
- No reclamar propiedad sobre código integrado.

---

## Código de Conducta

Este proyecto no adopta códigos de conducta externos.

Las expectativas son:

- Comunicación técnica profesional.
- Respeto a decisiones arquitectónicas.
- Ausencia de comportamiento disruptivo.

Violaciones resultarán en exclusión permanente.

---

## Contacto

Para consultas sobre contribuciones potenciales:

- Abra un issue describiendo la propuesta.
- Espere respuesta de mantenedores.
- No envíe PRs sin discusión previa.

No existe canal de discusión informal.  
No existe Slack, Discord ni foros comunitarios.

---

## Principio de Cierre

Este proyecto valora:

- Corrección sobre conveniencia.
- Gobernanza sobre popularidad.
- Arquitectura sobre ergonomía.

Las contribuciones que no compartan estos valores serán rechazadas.

