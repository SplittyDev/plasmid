use std::fmt::Display;

use crate::{traits::*, uni::IupacNucleotide};

/// RNA Nucleobase
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RnaNucleotide {
    /// Adenine
    A,
    /// Cytosine
    C,
    /// Guanine
    G,
    /// Uracil
    U,
}

impl Nucleotide for RnaNucleotide {
    fn complement(&self) -> RnaNucleotide {
        use self::RnaNucleotide::*;
        match self {
            A => U,
            U => A,
            C => G,
            G => C,
        }
    }
}

impl Display for RnaNucleotide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use self::RnaNucleotide::*;
        match self {
            A => write!(f, "Adenine"),
            C => write!(f, "Cytosine"),
            G => write!(f, "Guanine"),
            U => write!(f, "Uracil"),
        }
    }
}

impl TryFromLetter for RnaNucleotide {
    fn try_from_letter(c: char) -> Option<Self> {
        use self::RnaNucleotide::*;
        match c.to_ascii_uppercase() {
            'A' => Some(A),
            'C' => Some(C),
            'G' => Some(G),
            'U' => Some(U),
            'Ψ' => Some(U), // Pseudouridine
            _ => None,
        }
    }
}

impl ToLetter for RnaNucleotide {
    fn to_letter(&self) -> char {
        use self::RnaNucleotide::*;
        match self {
            A => 'A',
            C => 'C',
            G => 'G',
            U => 'U',
        }
    }
}

impl ToIupac for RnaNucleotide {
    fn to_iupac(&self) -> IupacNucleotide {
        use self::RnaNucleotide::*;
        match self {
            A => IupacNucleotide::A,
            C => IupacNucleotide::C,
            G => IupacNucleotide::G,
            U => IupacNucleotide::T,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RnaNucleotide;

    #[test]
    fn test_rna_base_to_string() {
        assert_eq!(RnaNucleotide::U.to_string(), format!("Uracil"))
    }
}
