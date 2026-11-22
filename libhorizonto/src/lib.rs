#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! `libhorizonto` — la komunaj protokoloj kaj manifesta logiko de Horizonto.
//!
//! La biblioteko restas sen `alloc`, sen `unsafe` kaj sen glitkomo. Ĝi entenas
//! nur purajn regulojn: la fenestran protokolon, la fenestran identigilon kun
//! generacio, la damaĝajn ortangulojn, la enigajn eventojn kaj la manifestan
//! analizon. La servoj kaj aplikaĵoj de Horizonto konsumas ĉi tiun
//! bibliotekon; ili mem rajtas uzi la uzantan kupilon, sed ne la protokolajn
//! tipojn por kaŝi posedon aŭ vivdaŭron.
//!
//! La normiga priskribo estas `docs/HORIZONTO.md`; la koncentrita kontrakto
//! estas `docs/KONTRAKTOJ.md`.

mod asertoj;
pub mod damago;
pub mod enigo;
pub mod eraro;
pub mod fenestro;
pub mod manifesto;
pub mod protokolo;

pub use damago::{
    Damaĝaro, Damaĝaro32, MAKSA_SURFACA_DIMENSIO, MAKSAJ_DAMAĜOJ, Ortangulo, SurfacaPriskribo,
};
pub use enigo::{
    BUTONO_DEKSTRA, BUTONO_MALDEKSTRA, BUTONO_MEZA, EnigaEvento, MAKSA_ENIGA_LONGO, MAKSAJ_BUTONOJ,
    MAKSAJ_MODIFILOJ, MODIFILO_ALT, MODIFILO_CTRL, MODIFILO_SUPER, MODIFILO_ŜOVO, SPECO_KLAVO,
    SPECO_MUSO, STATO_LIBERIGITA, STATO_PREMITA,
};
pub use eraro::Eraro;
pub use fenestro::{FenestraIdentigilo, MAKSAJ_FENESTROJ};
pub use manifesto::{
    Arko, FLUO_VERSIO, MAKSA_NOMO, MAKSA_VERSIO, MAKSA_VOJO, MAKSAJ_ARKOJ, MAKSAJ_DOSIERUJOJ,
    Manifesto,
};
pub use protokolo::{
    KAPA_LONGO, Kapo, MAKSA_DATUMO, MAKSA_MESAĜA_LONGO, Mesaĝo, MesaĝoTipo, PROTOKOLA_VERSIO,
};

/// Versio de la biblioteko, legata el la laborloka `Cargo.toml`.
pub const VERSIO: &str = env!("CARGO_PKG_VERSION");

/// La subtenata `libmikerno`-ABI. La regula Q15 postulas, ke ĉi tiu valoro,
/// la protokola versio kaj la manifesta `fluo`-versio restu kongruaj.
pub const MIKERNO_ABI: u16 = 2;

#[cfg(test)]
mod testoj {
    use super::*;

    #[test]
    fn versioj_kaj_abi_kongruas() {
        assert_eq!(VERSIO, "0.1.0");
        assert_eq!(PROTOKOLA_VERSIO, 1);
        assert_eq!(FLUO_VERSIO, "0.1");
        assert_eq!(MIKERNO_ABI, 2);
    }
}
