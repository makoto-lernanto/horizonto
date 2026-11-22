//! Konstantaj asertoj pri la protokolaj aranĝoj (S02, S07, S11, D03).
//!
//! Ĉi tiuj asertoj malkovras aranĝan rompon jam dum tradukado, antaŭ ol ajna
//! processo aŭ servo ruliĝas.

use core::mem::{align_of, offset_of, size_of};

use crate::damago::{Ortangulo, SurfacaPriskribo};
use crate::enigo::EnigaEvento;
use crate::fenestro::FenestraIdentigilo;
use crate::protokolo::{KAPA_LONGO, Kapo, MAKSA_DATUMO, MAKSA_MESAĜA_LONGO};

const _: () = {
    assert!(KAPA_LONGO == 8);
    assert!(MAKSA_MESAĜA_LONGO == 64);
    assert!(MAKSA_DATUMO == 56);
    assert!(MAKSA_DATUMO + KAPA_LONGO == MAKSA_MESAĜA_LONGO);

    assert!(size_of::<Kapo>() == 8);
    assert!(align_of::<Kapo>() == 4);
    assert!(offset_of!(Kapo, versio) == 0);
    assert!(offset_of!(Kapo, tipo) == 2);
    assert!(offset_of!(Kapo, longo) == 4);

    assert!(size_of::<FenestraIdentigilo>() == 8);
    assert!(align_of::<FenestraIdentigilo>() == 4);
    assert!(offset_of!(FenestraIdentigilo, indekso) == 0);
    assert!(offset_of!(FenestraIdentigilo, generacio) == 4);

    assert!(size_of::<Ortangulo>() == 16);
    assert!(align_of::<Ortangulo>() == 4);
    assert!(offset_of!(Ortangulo, x) == 0);
    assert!(offset_of!(Ortangulo, y) == 4);
    assert!(offset_of!(Ortangulo, larĝo) == 8);
    assert!(offset_of!(Ortangulo, alto) == 12);

    assert!(size_of::<SurfacaPriskribo>() == 12);
    assert!(align_of::<SurfacaPriskribo>() == 4);
    assert!(offset_of!(SurfacaPriskribo, larĝo) == 0);
    assert!(offset_of!(SurfacaPriskribo, alto) == 4);
    assert!(offset_of!(SurfacaPriskribo, paŝo) == 8);

    assert!(size_of::<EnigaEvento>() == 20);
    assert!(align_of::<EnigaEvento>() == 4);
    assert!(offset_of!(EnigaEvento, speco) == 0);
    assert!(offset_of!(EnigaEvento, modifiloj) == 2);
    assert!(offset_of!(EnigaEvento, klavo) == 4);
    assert!(offset_of!(EnigaEvento, x) == 8);
    assert!(offset_of!(EnigaEvento, y) == 12);
    assert!(offset_of!(EnigaEvento, butonoj) == 16);
    assert!(offset_of!(EnigaEvento, stato) == 17);
    assert!(offset_of!(EnigaEvento, rezervo) == 18);
};
