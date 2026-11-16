//! The semantic AF questions are not discharged by lexical diagnostics.
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Assessment {
    NotChecked,
}

#[derive(Debug, Clone, Serialize)]
pub struct RubricQuestion {
    pub criterion: &'static str,
    pub question: &'static str,
    pub status: Assessment,
}

#[derive(Debug, Clone, Serialize)]
pub struct Section8Rubric {
    pub questions: Vec<RubricQuestion>,
}

impl Default for Section8Rubric {
    fn default() -> Self {
        Self::new()
    }
}

impl Section8Rubric {
    pub fn new() -> Self {
        let questions = [
            (
                "c1",
                "Are axioms stated once with defined symbols and fixed scope?",
            ),
            (
                "c2",
                "Do immediate consequences follow from axioms and mathematics?",
            ),
            ("c3", "Are modelling choices explicit?"),
            ("c4", "Do corollaries avoid undeclared external premises?"),
            ("c5", "Do demonstrations instantiate earlier steps?"),
            ("c6", "Are falsifiers tied to specific claims?"),
            (
                "c7",
                "Are labels, notation and units precise and consistent?",
            ),
            ("c8", "Are RF1–RF5 respected?"),
        ]
        .into_iter()
        .map(|(criterion, question)| RubricQuestion {
            criterion,
            question,
            status: Assessment::NotChecked,
        })
        .collect();
        Self { questions }
    }
}
