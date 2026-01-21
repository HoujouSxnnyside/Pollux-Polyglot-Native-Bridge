# Language Binding Contract
*(Contract for Language Bindings)*

---

## Nature of the Contract

Pollux Polyglot Native Bridge defines the contract that **all** language bindings
must comply with.

It does not define implementation.  
It defines **boundary**.

---

## Binding Responsibilities

Every language binding (JS, .NET, etc.) **must**:

1. Translate language intent → `BridgeRequest`.
2. Invoke `PolyglotBridge::process_request`.
3. Translate `BridgeResponse` → language representation.
4. Propagate errors without silencing.
5. Not implement authorization logic.
6. Not cache decisions.
7. Not adapt behavior by context.

---

## Prohibited Responsibilities

Every language binding **must not**:

- Access Pollux Runtime directly.
- Access Pollux Contracts directly.
- Access Pollux Capabilities directly.
- Implement validation logic.
- Infer default values.
- Negotiate compatibility.
- Soften rejection.
- Hide error states.

---

## Interaction Model

### Unidirectionality

Binding → Bridge → Pollux

Never:

Pollux → Bridge → Binding

No callbacks exist.  
No reverse notifications exist.  
No event push exists.

---

### State Isolation

Each invocation is independent.

There is no:

- Session state.
- Authorization cache.
- Persistent context.

---

### Fail-Fast

On any failure, the binding must:

- Propagate the error explicitly.
- Not retry automatically.
- Not silence.
- Not interpret.

---

## Binding Compatibility

### Versioning

Bindings must:

- Specify supported Bridge version.
- Reject invocations if Bridge is incompatible.
- Not assume implicit backward compatibility.

---

### Evolution

If Bridge changes its model:

- Bindings must update.
- No automatic adaptation exists.
- No version negotiation exists.

---

## Principle of Subordination

Bindings **serve** the Bridge.

The Bridge does not serve bindings.

If a binding requires unsupported functionality,  
the binding is poorly designed.

---

## Principle of Closure

The boundary does not adapt.

Bindings adapt.

