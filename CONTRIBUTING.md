# CONTRIBUTING
## Contribution Policy for Pollux Polyglot Native Bridge

This repository is **public and open source**.

Openness does not imply:

- Architectural flexibility.
- Collaborative decision-making.
- Negotiation of design principles.
- Adaptation to external convenience.

This document defines what contributions are acceptable and under what conditions.

---

## Guiding Principle

Pollux Polyglot Native Bridge is a **governance boundary**, not a community project.

Contributions are welcome only if they:

- Strengthen the boundary.
- Do not weaken isolation.
- Do not expand external authority.
- Do not introduce ambiguity.

---

## Acceptable Contributions

### Defect Correction

Accepted corrections:

- Resolve failures in contract validation.
- Correct translation errors.
- Repair validation leaks.
- Restore boundary invariants.

Rejected "corrections":

- Relax validation.
- Introduce default values.
- Add tolerance for invalid inputs.
- Soften rejection.

---

### Robustness Improvements

Accepted improvements:

- Reinforce fail-fast behavior.
- Add missing validation.
- Harden parsing.
- Eliminate structural ambiguity.

Rejected "improvements":

- Add convenience features.
- Introduce adaptive logic.
- Permit validation bypass.
- Relax constraints.

---

### Documentation Correction

Accepted documentation changes:

- Clarify invariants.
- Correct incorrect descriptions.
- Reinforce boundary nature.
- Eliminate ambiguity.

Rejected documentation changes:

- Add tutorials.
- Introduce usage examples.
- Soften language.
- Suggest flexibility.

---

### Dependency Updates

Accepted updates:

- Fix vulnerabilities.
- Maintain API compatibility.
- Do not expand surface.
- Do not introduce new behavior.

All updates require formal validation.

---

## Explicitly Rejected Contributions

### Surface Expansion

Rejected contributions that:

- Add new request types.
- Expand payload models.
- Introduce new response states.
- Add boundary operations.

The boundary surface is closed.

---

### Convenience Abstractions

Rejected contributions that:

- Add helpers.
- Introduce builders.
- Simplify request construction.
- Provide DSLs.

The boundary does not seek ergonomics.

---

### Language-Specific Logic

Rejected contributions that:

- Add support for specific languages.
- Introduce ecosystem-specific adapters.
- Implement custom serialization.
- Add convenience traits.

The boundary is language-agnostic.

---

### Premature Optimization

Rejected contributions that:

- Add caching.
- Introduce pooling.
- Optimize for specific cases.
- Compromise clarity for performance.

The boundary prioritizes correctness over speed.

---

## Contribution Process

### 1. Admissibility Check

Before working on a contribution:

- Open an issue describing the problem.
- Await confirmation that the problem is valid.
- Await confirmation that the solution is acceptable.

Do not begin work without prior approval.

---

### 2. Implementation Requirements

All contributions must:

- Include tests demonstrating the problem.
- Include tests validating the fix.
- Maintain 100% coverage on modified code.
- Introduce no warnings.
- Not degrade clarity.

---

### 3. Pull Request Requirements

All PRs must:

- Reference the approved issue.
- Include precise technical description.
- Explain why the change strengthens the boundary.
- Demonstrate that it does not relax restrictions.
- Include evidence of passing tests.

PRs without approved issue will be closed without review.

---

### 4. Review

All PRs will be reviewed by:

- Project architects.
- Pollux Runtime maintainers.
- Governance committee (if applicable).

Review will evaluate:

- Technical correctness.
- Architectural alignment.
- Boundary impact.
- Compatibility with bindings.

---

### 5. Final Decision

Maintainers have final authority over:

- Acceptance or rejection.
- Change requests.
- Closure without merge.

Architectural decisions are non-negotiable.

---

## Contributor Expectations

### Principle Compliance

All contributors must:

- Read and understand Pollux `design-principles.md`.
- Respect boundary invariants.
- Not propose authority expansions.
- Not attempt to relax restrictions.

---

### Communication

All communication must be:

- Technical.
- Precise.
- Respectful.
- Free of political or social pressure.

Arguments based on:

- "This is easier"
- "Other projects do this"
- "The community expects this"

Are invalid.

---

### Respect for Decisions

Architectural decisions are:

- Final.
- Not subject to voting.
- Not subject to popularity.
- Not subject to pressure.

Contributors who disrespect final decisions will be excluded.

---

## Contribution Licensing

By contributing to this project, you agree to:

- License your contribution under MIT License.
- Cede modification and redistribution rights.
- Impose no additional restrictions.
- Claim no ownership over integrated code.

---

## Code of Conduct

This project does not adopt external codes of conduct.

Expectations are:

- Professional technical communication.
- Respect for architectural decisions.
- Absence of disruptive behavior.

Violations will result in permanent exclusion.

---

## Contact

For inquiries about potential contributions:

- Open an issue describing the proposal.
- Await maintainer response.
- Do not send PRs without prior discussion.

No informal discussion channel exists.  
No Slack, Discord, or community forums.

---

## Principle of Closure

This project values:

- Correctness over convenience.
- Governance over popularity.
- Architecture over ergonomics.

Contributions not sharing these values will be rejected.

