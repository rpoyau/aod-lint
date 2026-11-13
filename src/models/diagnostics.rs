use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Severity {
    Fatal,
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum RuleAnchor {
    RF1DistinctionPrecedes,
    RF2Closure,
    RF3NoAbsolutes,
    RF4InformationBookkeeping,
    RF5NoElevation,

    AF1AxiomsOnce,
    AF2PureMath,
    AF3SpecialiseChoice,
    AF4NoMidstreamImports,
    AF5NoRetrofitting,
    AF6ScopeDiscipline,
    AF7Falsification,

    Sec7UniqueLabels,
    Sec7PreciseAnchors,
    Sec7NotationConsistent,
    Sec7ExplicitChoice,

    RepoHygieneAgentLeakage,
}

impl RuleAnchor {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RF1DistinctionPrecedes => "RF1: Distinction precedes relation",
            Self::RF2Closure => "RF2: Model closure (no distinction => no entity)",
            Self::RF3NoAbsolutes => "RF3: No undeclared absolute properties",
            Self::RF4InformationBookkeeping => "RF4: Information is bookkeeping, not law",
            Self::RF5NoElevation => "RF5: No elevation of limitations to universal laws",

            Self::AF1AxiomsOnce => "AF1: State axioms once, no defense essays",
            Self::AF2PureMath => "AF2: Derive with mathematics only",
            Self::AF3SpecialiseChoice => "AF3: Specialise by declaration, not postulate",
            Self::AF4NoMidstreamImports => "AF4: No midstream imports",
            Self::AF5NoRetrofitting => "AF5: No retrofitting axioms",
            Self::AF6ScopeDiscipline => "AF6: Keep proofs local and self-contained",
            Self::AF7Falsification => "AF7: Traceability and concrete falsification hooks",

            Self::Sec7UniqueLabels => "§7.1: Give important claims unique labels",
            Self::Sec7PreciseAnchors => "§7.2: References must point to a single, precise anchor",
            Self::Sec7NotationConsistent => "§7.3: Notation and units consistency",
            Self::Sec7ExplicitChoice => "§7.4: Mark modelling choices explicitly at first use",

            Self::RepoHygieneAgentLeakage => {
                "AF6 / Repo Hygiene: No agent state, private session URLs, or handoff archives in scientific source"
            }
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Diagnostic {
    pub severity: Severity,
    pub rule: RuleAnchor,
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub suggested_fix: Option<String>,
    pub hoisted: bool,
}
