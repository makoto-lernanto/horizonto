//! Fenestra identigilo kun indekso kaj generacio.
//!
//! La kompozitoro reuzas indekson nur kun nova generacio (S05). Malnova
//! identigilo neniam reviviĝas: [`FenestraIdentigilo::akceptas`] komparas
//! ambaŭ kampojn, do la malnova generacio restas rifuzata post la antaŭenigo.

use crate::eraro::Eraro;

/// La maksimuma nombro de fenestraj indeksoj (la minimuma limo de D10).
pub const MAKSAJ_FENESTROJ: u32 = 64;

/// La identigilo de fenestro: indekso en la fenestra tabelo kaj generacio.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FenestraIdentigilo {
    /// Indekso en la fenestra tabelo, malpli ol [`MAKSAJ_FENESTROJ`].
    pub indekso: u32,
    /// Generacio, kiu ekas ĉe unu kaj neniam revenas al antaŭa valoro.
    pub generacio: u32,
}

impl FenestraIdentigilo {
    /// Konstruas identigilon por ekzistanta fenestra loko.
    ///
    /// # Eraroj
    ///
    /// `Eraro::NevalidaFenestraIndekso` se la indekso estas ekster la tabelo;
    /// `Eraro::NevalidaGeneracio` se la generacio estas nulo.
    pub const fn nova(indekso: u32, generacio: u32) -> Result<Self, Eraro> {
        if indekso >= MAKSAJ_FENESTROJ {
            return Err(Eraro::NevalidaFenestraIndekso);
        }
        if generacio == 0 {
            return Err(Eraro::NevalidaGeneracio);
        }
        Ok(Self { indekso, generacio })
    }

    /// Ĉu la identigiloj estas egala en ambaŭ kampoj?
    #[must_use]
    pub const fn sama(&self, alia: &Self) -> bool {
        self.indekso == alia.indekso && self.generacio == alia.generacio
    }

    /// Ĉu la kompozitoro akceptu ĉi tiun identigilon kiel la nunan?
    ///
    /// La funkcio estas la sola akcepta regulo: malnova generacio ĉiam estas
    /// rifuzata, eĉ se la indekso estis reuzita.
    #[must_use]
    pub const fn akceptas(&self, nuna: &Self) -> bool {
        self.sama(nuna)
    }

    /// Redonas identigilon kun la sekva generacio por la sama indekso.
    ///
    /// # Eraroj
    ///
    /// `Eraro::GeneracioElĉerpita` se la generacio atingis `u32::MAX`; tiam la
    /// indekso ne plu reuzeblas.
    pub fn sekva_generacio(&self) -> Result<Self, Eraro> {
        let generacio = self
            .generacio
            .checked_add(1)
            .ok_or(Eraro::GeneracioElĉerpita)?;
        Self::nova(self.indekso, generacio)
    }
}

#[cfg(kani)]
mod kani_pruvoj {
    //! Kani-pruvo pri la generacia reuzo (S05).
    //!
    //! Por arbitra valida identigilo, la sekva generacio ŝanĝas la identigilon
    //! kaj la malnova estas rifuzata. Generacio nulo kaj indekso ekster la
    //! tabelo ĉiam estas rifuzataj.

    use super::{FenestraIdentigilo, MAKSAJ_FENESTROJ};

    /// Malnova identigilo neniam egalas al la posteula identigilo.
    #[kani::proof]
    fn generacia_reuzo() {
        let indekso: u32 = kani::any();
        let generacio: u32 = kani::any();
        kani::assume(indekso < MAKSAJ_FENESTROJ);
        kani::assume(generacio >= 1);

        if let Ok(malnova) = FenestraIdentigilo::nova(indekso, generacio) {
            if let Ok(nova) = malnova.sekva_generacio() {
                assert_eq!(nova.indekso, malnova.indekso);
                assert!(nova.generacio > malnova.generacio);
                assert!(!nova.akceptas(&malnova));
            } else {
                assert_eq!(generacio, u32::MAX);
            }
        }
        assert!(FenestraIdentigilo::nova(indekso, 0).is_err());
        assert!(FenestraIdentigilo::nova(MAKSAJ_FENESTROJ, 1).is_err());
    }
}

#[cfg(test)]
mod testoj {
    use super::*;

    #[test]
    fn identigilo_estas_ok_bajtoj() {
        assert_eq!(core::mem::size_of::<FenestraIdentigilo>(), 8);
        assert_eq!(core::mem::align_of::<FenestraIdentigilo>(), 4);
    }

    #[test]
    fn limoj_de_indekso_kaj_generacio() {
        assert_eq!(
            FenestraIdentigilo::nova(0, 0),
            Err(Eraro::NevalidaGeneracio)
        );
        assert_eq!(
            FenestraIdentigilo::nova(MAKSAJ_FENESTROJ, 1),
            Err(Eraro::NevalidaFenestraIndekso)
        );
        assert!(FenestraIdentigilo::nova(MAKSAJ_FENESTROJ - 1, 1).is_ok());
    }

    #[test]
    fn sekva_generacio_rifuzas_la_malnovan() {
        let malnova = FenestraIdentigilo::nova(3, 7).unwrap();
        let nova = malnova.sekva_generacio().unwrap();
        assert_eq!(nova.indekso, 3);
        assert_eq!(nova.generacio, 8);
        assert!(nova.akceptas(&nova));
        assert!(!nova.akceptas(&malnova));
        assert!(!malnova.akceptas(&nova));
    }

    #[test]
    fn generacio_ellasiĝas_ĉe_la_supro() {
        let lasta = FenestraIdentigilo::nova(0, u32::MAX).unwrap();
        assert_eq!(lasta.sekva_generacio(), Err(Eraro::GeneracioElĉerpita));
    }
}
