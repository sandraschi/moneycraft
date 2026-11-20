//! Embedded domain skills (`skills/*/SKILL.md`) and workstation help manual (`HELP.md`).
#![forbid(unsafe_code)]

use serde::Serialize;
use serde_json::{json, Value};

/// Full workstation help manual embedded at compile time from `HELP.md`.
pub const HELP_MD: &str = include_str!("../../../HELP.md");

/// Metadata and content for a bundled MoneyCraft skill.
pub struct SkillSrc {
    pub name: &'static str,
    pub description: &'static str,
    pub uri: &'static str,
    pub content: &'static str,
}

/// Serializable summary of a bundled MoneyCraft skill.
#[derive(Debug, Clone, Serialize)]
pub struct SkillSummary {
    pub name: &'static str,
    pub description: &'static str,
    pub uri: &'static str,
    pub bytes: usize,
}

/// All bundled MoneyCraft skills embedded into the binary.
pub const SKILLS: &[SkillSrc] = &[
    SkillSrc {
        name: "moneycraft-workflow",
        description: "End-to-end MoneyCraft market discovery, charting, multi-asset comparison, personal wealth simulation, backtesting, and paper-trading workflow using moneycraft-mcp tools.",
        uri: "skill://moneycraft-workflow/SKILL.md",
        content: include_str!("../../../skills/moneycraft-workflow/SKILL.md"),
    },
    SkillSrc {
        name: "moneycraft-ledger",
        description: "Local-first personal portfolio ledger, FIFO lot accounting, broker/bank CSV import, multi-currency valuation, and Austrian KESt (27.5%) tax estimation in MoneyCraft.",
        uri: "skill://moneycraft-ledger/SKILL.md",
        content: include_str!("../../../skills/moneycraft-ledger/SKILL.md"),
    },
];

/// Return summaries of all bundled skills.
#[must_use]
pub fn skills_list() -> Vec<SkillSummary> {
    SKILLS
        .iter()
        .map(|s| SkillSummary {
            name: s.name,
            description: s.description,
            uri: s.uri,
            bytes: s.content.len(),
        })
        .collect()
}

/// Read a bundled skill by name and return its structured JSON payload.
pub fn skill_read(name: &str) -> Result<Value, String> {
    let found = SKILLS
        .iter()
        .find(|s| s.name == name)
        .ok_or_else(|| format!("unknown skill '{name}'"))?;
    Ok(json!({
        "success": true,
        "name": found.name,
        "uri": found.uri,
        "description": found.description,
        "content": found.content,
    }))
}

/// Look up a bundled skill by its `skill://...` resource URI.
#[must_use]
pub fn skill_by_uri(uri: &str) -> Option<&'static SkillSrc> {
    SKILLS.iter().find(|s| s.uri == uri)
}

/// Return MCP `resources/list` entries for all bundled skills.
#[must_use]
pub fn mcp_resources() -> Vec<Value> {
    SKILLS
        .iter()
        .map(|s| {
            json!({
                "uri": s.uri,
                "name": s.name,
                "description": s.description,
                "mimeType": "text/markdown",
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundled_skills_are_non_empty_and_readable() {
        assert!(!HELP_MD.is_empty());
        assert_eq!(SKILLS.len(), 2);
        for summary in skills_list() {
            assert!(summary.bytes > 0);
            assert!(skill_by_uri(summary.uri).is_some());
            assert!(skill_read(summary.name).is_ok());
        }
        assert!(skill_read("nonexistent-skill").is_err());
        assert_eq!(mcp_resources().len(), 2);
    }
}
