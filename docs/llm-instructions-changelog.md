# LLM Instructions Changelog

This document tracks the evolution of instructions provided to AI agents via `--llm-skill`, documenting what worked, what didn't, and how we're iteratively improving agent guidance.

## Purpose

Track empirical observations about how different LLMs adhere to tagging workflows and what instruction patterns are most effective. This helps us:
- Identify which instruction patterns work across different models
- Document failure modes and their mitigations
- Build institutional knowledge about effective LLM guidance
- Iterate on the skill document based on real-world usage

## Log Format

Each entry should include:
- **Date**: When the change was observed/made
- **Model(s)**: Which LLM(s) exhibited the behavior
- **Observation**: What happened (positive or negative)
- **Action**: What we changed in response
- **Result**: Did it work?

---

## 2026-02-18 - Ambiguity Reduction Pass (v3)

### Context
Real-codebase trial runs (Claude-assisted) exposed ambiguity in tag selection boundaries, especially around over-tagging and `entry` vs `core` classification.

### Observations
1.  Agents still over-tagged discoverable implementation details when instructions were interpreted too literally.
2.  `entry` and `core` boundaries were sometimes conflated in library-style code (important types tagged as `entry`).
3.  Invariant tags were repeated at multiple enforcement sites instead of being anchored to the defining constraint location.

### Action Taken
Updated `llm_skill.txt` to tighten selection criteria and reduce ambiguity:

1.  Added a **Discoverability** principle to explicitly skip tags that readers naturally encounter from already-tagged landmarks.
2.  Changed implementation phase from **Tag-Always** to **Tag-When-Appropriate** to discourage mandatory tagging of every significant edit.
3.  Expanded tag type definitions with stronger boundaries:
    - `core`: story-level domain/architecture concepts, including qualifying modules.
    - `entry`: temporal invocation roots only (functions/methods/invocation points), never types/modules.
    - `invariant`: tag the defining constraint location, not every enforcement call site.
    - `recipe`: broadened to include reusable organizational/test-support patterns.
4.  Added temporary project addendum to omit `rfc=` metadata while ADR→RFC migration is incomplete.

### Expected Benefits
1.  Better map signal-to-noise ratio via fewer redundant tags.
2.  More consistent `entry` vs `core` tagging decisions.
3.  Less duplicate `invariant` coverage.

### Regression Risks To Monitor
1.  **Under-tagging**: Combining "if in doubt, do not tag" with discoverability rules may suppress useful landmarks.
2.  **Policy drift**: The temporary `rfc=` omission can conflict with existing docs/spec examples until those are updated.
3.  **Interpretation drift**: Planning language still implies tags "will require" updates while implementation is now conditional.

---

## 2026-02-13 - The Imperative Pivot (v2)

### Context
Initial real-world testing showed mixed results with the educational/tutorial style instructions.

### Observations
1.  **Summarization Loss**: Models tended to "compact" the instructions, losing critical details like specific syntax rules.
2.  **Gating Failures**: Models used the "Check if applicable" logic to opt-out of tagging (e.g., "Directory is empty, so Cairn is not active yet").
3.  **Syntax Hallucination**: Models invented their own formats (e.g., `#[derive(ChunkKind)]` instead of doc comments) because the syntax wasn't enforced strictly enough.
4.  **Over-tagging**: Without a clear philosophy, models tagged trivial helpers as `core`.

### Action Taken
Complete rewrite of `llm_skill.txt` to shift from **Educational** to **Imperative/Protocol**.

1.  **Mandate vs Guide**: Changed tone from "Here is how you use this" to "You are operating under the Cairn Protocol".
2.  **Always Active**: Removed conditional logic. The workflow is now "Always Active" to prevent opt-outs.
3.  **Concrete Heuristics**: Replaced abstract definitions with "The 5-minute Onboarding Test" for `core` tags.
4.  **Negative Constraints**: Added explicit "Forbidden Patterns" (e.g., `❌ // [nb:core]`) to block specific failure modes.
5.  **Signal/Noise Philosophy**: Added a dedicated section on "High Signal, Low Noise" to discourage spamming tags.
6.  **Planning Integration**: Added a requirement to include tagging in the *Plan* phase, not just implementation.

### Expected Result
Higher compliance, less syntax hallucination, and higher quality (curated) maps.

---

## 2026-02-12 - Initial Implementation

### Context
First implementation of `--llm-desc` and `--llm-skill` commands.

### Initial Design Decisions

**Gating Mechanism**: Chose emphatic skill instructions + workflow integration guidance
- **Rationale**: Rather than technical enforcement, rely on clear documentation that agents should add tagging to their task.md workflow
- **Risk**: Agents may forget to tag despite instructions
- **Mitigation**: Include verification checklist, emphasize in multiple places

**Workflow vs Skill Framing**
- **Decision**: Frame this as a workflow enrichment, not just a tool skill
- **Rationale**: Agents need to understand this changes their *process*, not just adds a tool
- **Implementation**: Explicitly instruct agents to add workflow steps to their agent.md/system instructions

**Tag Set**: entry, core, invariant, recipe
- **invariant**: New tag for constraints/rules - addresses critical need to document "what not to do"
- **core**: Renamed from "concept" for clarity

### Observations to Track

Monitor for:
1.  **Tag adherence rate**: Do agents remember to tag new code?
2.  **Tag quality**: Are tags meaningful and properly formatted?
3.  **Discovery flow**: Do agents successfully use `--help` → `--llm-skill`?
4.  **Workflow integration**: Do agents actually modify their agent.md/workflows?
5.  **Invariant effectiveness**: Do invariant tags prevent violations?

### Next Steps
- Implement and deploy
- Gather real-world usage data
- Update this log based on observations

---

## Future Entries Template

### YYYY-MM-DD - [Brief Title]

**Model(s)**: [GPT-4, Claude, etc.]

**Observation**:
[What was observed - specific example if possible]

**Root Cause**:
[Why did this happen? Instruction clarity? Model limitation?]

**Action Taken**:
[What we changed - quote specific instruction changes]

**Result**:
[Did it work? Evidence?]

**Lessons Learned**:
[Principles for future instruction design]
