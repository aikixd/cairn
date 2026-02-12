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
1. **Tag adherence rate**: Do agents remember to tag new code?
2. **Tag quality**: Are tags meaningful and properly formatted?
3. **Discovery flow**: Do agents successfully use `--help` → `--llm-skill`?
4. **Workflow integration**: Do agents actually modify their agent.md/workflows?
5. **Invariant effectiveness**: Do invariant tags prevent violations?

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
