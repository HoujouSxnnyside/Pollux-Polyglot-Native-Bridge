# SECURITY
## Política de Divulgación Responsable

**Importante:**  
Pollux Polyglot Native Bridge es un proyecto de código abierto y frontera crítica
del ecosistema Pollux.

Toda vulnerabilidad, debilidad de frontera o vector de fuga de autoridad debe
ser reportada de forma responsable siguiendo este procedimiento.

---

## 1. Propósito

Este documento define el **procedimiento obligatorio** para reportar
vulnerabilidades, debilidades de frontera o implicaciones de seguridad relacionadas
con Pollux Polyglot Native Bridge.

Pollux Polyglot Native Bridge es **crítico para la seguridad del ecosistema**, ya que:

- Define la única frontera autorizada con sistemas externos.
- Traduce intención externa en solicitudes de Pollux.
- Previene fuga de autoridad.
- Valida contratos antes de cualquier invocación.
- Aísla el Kernel de acceso externo.

Un defecto en Pollux Polyglot Native Bridge puede:

- Permitir bypass de contratos.
- Exponer estructuras internas de Pollux.
- Habilitar invocaciones no autorizadas.
- Debilitar enforcement del Kernel.
- Filtrar autoridad a sistemas externos.
- Comprometer aislamiento de lenguajes.

Una frontera permeable es una vulnerabilidad estructural.

---

## 2. Alcance

Esta política aplica a cualquier hallazgo relacionado con:

- Modelos de solicitud/respuesta.
- Lógica de traducción de intención.
- Validación de contratos en frontera.
- Exposición de APIs internas.
- Serialización y deserialización.
- Manejo de estado en frontera.
- Interacción con bindings externos.
- Semántica de tipos y modelos.
- Documentación que pueda inducir uso inseguro.

Incluye vulnerabilidades confirmadas y **potenciales**.

---

## 3. Tipos de Amenazas

### Fuga de Autoridad

La frontera permite que un sistema externo:

- Invoque operaciones sin validación de contratos.
- Acceda a estructuras internas del Kernel.
- Bypasea enforcement de capacidades.
- Obtenga autoridad no concedida.

---

### Bypass de Contratos

Solicitudes que:

- Eluden validación de Pollux Contracts.
- Permiten invocaciones no autorizadas.
- Escapan enforcement de frontera.

---

### Ambigüedad de Modelos

Modelos de solicitud/respuesta que:

- Permiten interpretaciones múltiples.
- Introducen estados indefinidos.
- Habilitan comportamiento implícito.

---

### Exposición Indebida

La frontera expone:

- APIs internas del Kernel.
- Detalles de implementación.
- Estructuras de datos privadas.
- Lógica de enforcement.

---

### Abuso de Contribuciones

Contribuciones maliciosas que intentan:

- Relajar validación.
- Introducir backdoors.
- Debilitar frontera.
- Expandir autoridad externa.

---

## 4. Canales de Reporte

### Vulnerabilidades Confirmadas

Para vulnerabilidades confirmadas, use:

- **GitHub Security Advisory** (preferido para proyectos públicos)
- **Email cifrado:** security@pollux-runtime.org

No publique vulnerabilidades en:

- Issues públicos
- Pull requests
- Foros
- Redes sociales

---

### Vulnerabilidades Potenciales

Para debilidades potenciales o preguntas de seguridad:

- Abra un issue marcado como `security-question`
- Describa el escenario sin detalles de explotación
- Espere respuesta de mantenedores

---

## 5. Contenido Obligatorio del Reporte

Todo reporte debe incluir, como mínimo:

- Resumen técnico claro y conciso del hallazgo.
- Impacto estimado en términos de seguridad de frontera.
- Operaciones de frontera afectadas.
- Escenario de explotación (sin exploit funcional público).
- Análisis de traducción o validación cuando sea aplicable.
- Versión exacta de la crate o commit SHA.
- Información de contacto del reportante.

Reportes incompletos pueden retrasar la evaluación.

---

## 6. Proceso y Tiempos

- **Confirmación de recepción:**  
  Máximo 72 horas.

- **Evaluación inicial:**  
  Dentro de los siguientes 7 días se determinará severidad,
  impacto y plan de acción preliminar.

- **Mitigación y resolución:**  
  Los plazos dependen de la severidad.
  Fallos críticos que afecten impermeabilidad de frontera se priorizan para
  corrección inmediata y versionado controlado.

- **Divulgación coordinada:**  
  Se coordinará divulgación pública después de:
  - Corrección implementada
  - Release publicado
  - Usuarios notificados
  - Tiempo prudencial para actualización

---

## 7. Reglas de Divulgación Responsable

Queda estrictamente prohibido:

- Publicar vulnerabilidades antes de corrección.
- Implementar exploits públicos.
- Compartir detalles de explotación en canales no seguros.
- Usar vulnerabilidades para demostración pública.

Violaciones pueden resultar en:

- Exclusión del proyecto.
- Reporte a plataformas de hospedaje.
- Acción legal si corresponde.

---

## 8. Reconocimiento

Los reportantes de vulnerabilidades válidas serán:

- Reconocidos en el advisory de seguridad.
- Listados en archivo de créditos (si lo desean).
- Agradecidos públicamente después de corrección.

No ofrecemos recompensas monetarias en este momento.

---

## 9. Confidencialidad

Todos los reportes de seguridad son considerados **confidenciales**
hasta divulgación coordinada.

El acceso está limitado a:

- Mantenedores del proyecto.
- Equipo de seguridad de Pollux.
- Personas necesarias para corrección.

---

## 10. Contacto Legal

Para consultas legales relacionadas con vulnerabilidades o divulgación:

legal@pollux-runtime.org

---

## 11. Actualizaciones de Seguridad

Suscríbase a:

- GitHub Security Advisories de este repositorio
- Release notes (todas las correcciones de seguridad se documentan)

No existe lista de correo dedicada en este momento.

---

## 12. Alcance Fuera de Este Proyecto

Vulnerabilidades en:

- Pollux Runtime
- Pollux Capabilities
- Pollux Contracts

Deben reportarse en sus respectivos repositorios.

Este procedimiento aplica exclusivamente a Pollux Polyglot Native Bridge.

---

## 13. Revisión y Mantenimiento

Esta política será revisada periódicamente.

La versión vigente se mantiene en este repositorio.

