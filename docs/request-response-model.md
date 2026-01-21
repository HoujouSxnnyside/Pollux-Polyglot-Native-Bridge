# Request Response Model
*(Request / Response Model)*

---

## Nature of the Model

The request/response model of Pollux Polyglot Native Bridge is:

- Closed.
- Explicit.
- Immutable.
- Serializable.
- Without shared state.

If an interaction cannot be expressed in this model,  
it is not supported.

---

## Request Structure

### BridgeRequest

All external requests are structured as:

```
BridgeRequest {
    request_type: BridgeRequestType,
    payload: BridgePayload,
}
```

**Invariants:**

- `request_type` is closed (finite enumeration).
- `payload` corresponds directly to `request_type`.
- No "other" or "custom" variants exist.

---

### BridgeRequestType

Supported request types:

- `ValidateManifest`
- `ValidateCapabilityRequest`
- `ValidateLifecycleTransition`
- `ValidateTargetCompatibility`

This set is **exhaustive**.

Any operation not representable here is not supported.

---

### BridgePayload

Payload specific to each request type:

- `ManifestValidation`: capabilities, target, module_type.
- `CapabilityRequest`: capability.
- `LifecycleTransition`: from_phase, to_phase.
- `TargetCompatibility`: declared_target, runtime_target.

Each payload is **closed**.  
No ambiguous optional fields exist.  
No implicit default values exist.

---

## Response Structure

### BridgeResponse

All responses are:

```
BridgeResponse {
    status: BridgeResponseStatus,
    reason: Option<String>,
}
```

**Invariants:**

- `status` is closed (finite enumeration).
- `reason` is `Some` when `status` is `Rejected` or `InvalidRequest`.
- `reason` is `None` when `status` is `Approved`.

---

### BridgeResponseStatus

Supported response states:

- `Approved`: request valid and compliant with contracts.
- `Rejected`: request violates contracts.
- `InvalidRequest`: request structurally invalid.

This set is **exhaustive**.

No partial states exist.  
No warnings exist.  
No suggestions exist.

---

## Serialization Semantics

The model is designed to be serializable in:

- JSON.
- MessagePack.
- Protocol Buffers.
- Any structured format.

It does not depend on:

- Callbacks.
- Closures.
- Mutable references.
- Ambient state.

---

## Model Invariants

### Immutability

Once constructed, a request cannot mutate.  
Once generated, a response cannot mutate.

---

### Absence of Shared State

Request and response are independent values.

There is no:

- Session state.
- Shared context.
- Decision cache.

Each request is **autonomous**.

---

### Determinism

Given the same request, the response is always identical.

It does not depend on:

- Time.
- External state.
- Prior executions.

---

## Principle of Closure

The model does not grow for convenience.

If an operation cannot be expressed here,  
it does not belong at the boundary.

