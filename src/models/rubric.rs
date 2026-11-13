use serde::Serialize;
use crate::models::diagnostics::{Diagnostic, RuleAnchor, Severity};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Section8Rubric(pub u8);

impl Default for Section8Rubric {
    fn default() -> Self {
        Self::new()
    }
}

impl Section8Rubric {
    pub const C1_AXIOMS_FIXED:       u8 = 1 << 0;
    pub const C2_PURE_MATH:          u8 = 1 << 1;
    pub const C3_CHOICES_EXPLICIT:   u8 = 1 << 2;
    pub const C4_NO_EXTERNAL_LAWS:   u8 = 1 << 3;
    pub const C5_DEMOS_ISOLATED:     u8 = 1 << 4;
    pub const C6_FALSIFICATION_TIED: u8 = 1 << 5;
    pub const C7_LABELS_PRECISE:     u8 = 1 << 6;
    pub const C8_RF_RESPECTED:       u8 = 1 << 7;

    pub fn new() -> Self {
        Self(0b1111_1111)
    }

    pub fn is_converged(&self) -> bool {
        self.0 == 0b1111_1111
    }

    pub fn to_bitstring(&self) -> String {
        format!("{:08b}", self.0)
    }

    pub fn apply_diagnostics(&mut self, diagnostics: &[Diagnostic]) {
        for d in diagnostics {
            if d.severity == Severity::Error || d.severity == Severity::Fatal {
                match d.rule {
                    RuleAnchor::AF1AxiomsOnce => self.0 &= !Self::C1_AXIOMS_FIXED,
                    RuleAnchor::AF2PureMath => self.0 &= !Self::C2_PURE_MATH,
                    RuleAnchor::AF3SpecialiseChoice | RuleAnchor::Sec7ExplicitChoice => {
                        self.0 &= !Self::C3_CHOICES_EXPLICIT
                    }
                    RuleAnchor::AF4NoMidstreamImports => self.0 &= !Self::C4_NO_EXTERNAL_LAWS,
                    RuleAnchor::AF7Falsification => self.0 &= !Self::C6_FALSIFICATION_TIED,
                    RuleAnchor::Sec7PreciseAnchors | RuleAnchor::Sec7UniqueLabels => {
                        self.0 &= !Self::C7_LABELS_PRECISE
                    }
                    RuleAnchor::RF1DistinctionPrecedes
                    | RuleAnchor::RF2Closure
                    | RuleAnchor::RF3NoAbsolutes
                    | RuleAnchor::RF4InformationBookkeeping
                    | RuleAnchor::RF5NoElevation => self.0 &= !Self::C8_RF_RESPECTED,
                    _ => {}
                }
            }
        }
    }
}
