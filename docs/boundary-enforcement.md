# Boundary Enforcement
*(Boundary Enforcement in Pollux Polyglot)*

---

## Nature of Enforcement

Pollux Polyglot Native Bridge does not authorize.  
It does not execute.  
It does not decide.

**It validates**.

All external requests are subject to contract validation **before** any
interaction with Pollux Runtime.

---

## Validation Model

### Manifest Validation

External request provides:

- List of requested capabilities.
- Declared target.
- Module type.

The boundary:

1. Parses capabilities against canonical vocabulary (Pollux Capabilities).
2. Parses target and module type.
3. Invokes `validate_authorization` from Pollux Contracts.
4. Returns `Approved` or `Rejected` with reason.

No negotiation exists.  
No automatic retry exists.  
No progressive escalation exists.

---

### Capability Request Validation

External request provides:

- Capability string.

The boundary:

1. Parses against canonical vocabulary.
2. Returns `Approved` if exists, `InvalidRequest` if not.

Does not validate contextual authorization here.  
Only validates existence in vocabulary.

---

### Lifecycle Transition Validation

External request provides:

- Source phase.
- Destination phase.

The boundary:

1. Parses phases against canonical enumeration.
2. Invokes `validate_lifecycle_transition` from Pollux Contracts.
3. Returns `Approved` or `Rejected` with reason.

Does not modify state.  
Does not execute transition.  
Only validates admissibility.

---

### Target Compatibility Validation

External request provides:

- Declared target.
- Runtime target.

The boundary:

1. Parses targets.
2. Invokes `validate_target` from Pollux Contracts.
3. Returns `Approved` or `Rejected` with reason.

Does not adapt targets.  
Does not negotiate compatibility.

---

## Response States

### Approved

The request is valid and complies with all contracts.

Does not imply execution.  
Implies formal admissibility.

---

### Rejected

The request violates one or more contracts.

The reason is explicit.  
No automatic retry exists.  
No correction suggestion exists.

---

### InvalidRequest

The request is structurally invalid:

- Nonexistent capability.
- Invalid target.
- Invalid phase.
- Invalid module type.

Not a contract violation.  
A translation failure.

---

## Principle of Non-Interpretation

The boundary does not interpret intent.

If a request is not expressed exactly in the closed model,  
it does not exist.

There is no:

- Inference of default values.
- Translation of synonyms.
- Adaptation to external conventions.
- Automatic correction.

---

## Principle of Closure

The boundary is impermeable.

If anything can leak,  
the boundary has failed.

