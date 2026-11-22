//! Surfaca priskribo, damaĝaj ortanguloj kaj ilia kunigo.
//!
//! La kompozitoro legas nur la regionojn, kiujn la aplikaĵo markis kiel
//! damaĝon (D02). La damaĝaro estas fiksa tabelo sen `alloc`: nova ortangulo
//! kunfandiĝas kun ĉiu ortangulo, kiun ĝi intersekcas, ĝis restas aro da
//! ne-intersekcantaj ortanguloj. La kunfandaĵo estas la plej malgranda
//! ortangulo, kiu enhavas ambaŭ originalojn.

use crate::eraro::Eraro;

/// La supra limo de ortanguloj en unu damaĝaro.
pub const MAKSAJ_DAMAĜOJ: usize = 32;

/// La maksimuma surfaca dimensio laŭ D04.
pub const MAKSA_SURFACA_DIMENSIO: u32 = 4096;

/// Ortangulo en la fenestra surfaco, kun la origino supre-maldekstre.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ortangulo {
    /// X-koordinato de la supra-maldekstra angulo.
    pub x: i32,
    /// Y-koordinato de la supra-maldekstra angulo.
    pub y: i32,
    /// Larĝo en pikseloj; devas esti pli ol nulo.
    pub larĝo: u32,
    /// Alto en pikseloj; devas esti pli ol nulo.
    pub alto: u32,
}

impl Ortangulo {
    /// Konstruas ortangulon kun nemulaj dimensioj.
    ///
    /// # Eraroj
    ///
    /// `Eraro::NevalidaOrtangulo` se la larĝo aŭ alto estas nulo.
    pub const fn nova(x: i32, y: i32, larĝo: u32, alto: u32) -> Result<Self, Eraro> {
        if larĝo == 0 || alto == 0 {
            return Err(Eraro::NevalidaOrtangulo);
        }
        Ok(Self { x, y, larĝo, alto })
    }

    /// La unua X-koordinato malantaŭ la ortangulo, kiel `i64` por eviti
    /// superfluon.
    #[must_use]
    pub const fn fino_x(&self) -> i64 {
        self.x as i64 + self.larĝo as i64
    }

    /// La unua Y-koordinato sub la ortangulo, kiel `i64` por eviti superfluon.
    #[must_use]
    pub const fn fino_y(&self) -> i64 {
        self.y as i64 + self.alto as i64
    }

    /// Ĉu la ortangulo estas tute ene de la surfaco?
    #[must_use]
    pub const fn eniĝas_en(&self, surfaco: &SurfacaPriskribo) -> bool {
        self.x >= 0
            && self.y >= 0
            && self.fino_x() <= surfaco.larĝo as i64
            && self.fino_y() <= surfaco.alto as i64
    }

    /// Ĉu la ortanguloj havas komunan areon?
    #[must_use]
    pub const fn intersekcas(&self, alia: &Self) -> bool {
        (self.x as i64) < alia.fino_x()
            && (alia.x as i64) < self.fino_x()
            && (self.y as i64) < alia.fino_y()
            && (alia.y as i64) < self.fino_y()
    }

    /// Ĉu ĉi tiu ortangulo tute enhavas la alian?
    #[must_use]
    pub const fn enhauvas(&self, alia: &Self) -> bool {
        self.x as i64 <= alia.x as i64
            && self.y as i64 <= alia.y as i64
            && alia.fino_x() <= self.fino_x()
            && alia.fino_y() <= self.fino_y()
    }

    /// La plej malgranda ortangulo, kiu enhavas ambaŭ, aŭ `None` se la
    /// rezulto ne reprezenteblas.
    #[must_use]
    pub fn kunfandi(&self, alia: &Self) -> Option<Self> {
        let x = self.x.min(alia.x);
        let y = self.y.min(alia.y);
        let fino_x = self.fino_x().max(alia.fino_x());
        let fino_y = self.fino_y().max(alia.fino_y());
        let larĝo = u32::try_from(fino_x - i64::from(x)).ok()?;
        let alto = u32::try_from(fino_y - i64::from(y)).ok()?;
        Self::nova(x, y, larĝo, alto).ok()
    }
}

/// La fiksa priskribo de fenestra surfaco.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SurfacaPriskribo {
    /// Larĝo en pikseloj, 1–4096.
    pub larĝo: u32,
    /// Alto en pikseloj, 1–4096.
    pub alto: u32,
    /// Paŝo en bajtoj; multipliko de 4 kaj ne malpli ol `larĝo × 4`.
    pub paŝo: u32,
}

impl SurfacaPriskribo {
    /// Konstruas kaj validigas surfacan priskribon laŭ D03 kaj D04.
    ///
    /// # Eraroj
    ///
    /// `Eraro::NevalidaSurfaco` se dimensio aŭ paŝo malobservas la regulojn.
    pub fn nova(larĝo: u32, alto: u32, paŝo: u32) -> Result<Self, Eraro> {
        let priskribo = Self {
            larĝo, alto, paŝo
        };
        priskribo.validi()?;
        Ok(priskribo)
    }

    /// Validigas la priskribon laŭ D03 kaj D04.
    ///
    /// # Eraroj
    ///
    /// `Eraro::NevalidaSurfaco` se dimensio estas nulo aŭ tro granda, se la
    /// paŝo ne estas multipliko de kvar, aŭ se ĝi estas pli mallonga ol
    /// `larĝo × 4`.
    pub fn validi(&self) -> Result<(), Eraro> {
        if self.larĝo == 0
            || self.alto == 0
            || self.larĝo > MAKSA_SURFACA_DIMENSIO
            || self.alto > MAKSA_SURFACA_DIMENSIO
        {
            return Err(Eraro::NevalidaSurfaco);
        }
        if !self.paŝo.is_multiple_of(4) {
            return Err(Eraro::NevalidaSurfaco);
        }
        let minimumo = self.larĝo.checked_mul(4).ok_or(Eraro::NevalidaSurfaco)?;
        if self.paŝo < minimumo {
            return Err(Eraro::NevalidaSurfaco);
        }
        Ok(())
    }
}

/// Fiksa aro da ne-intersekcantaj damaĝaj ortanguloj.
///
/// La ordo de la eroj ne estas kontrakta. La kapacito estas `N`; kiam la aro
/// plenas kaj nova ortangulo ne kunfandiĝas, [`Damaĝaro::aldonu`] redonas
/// `Eraro::PlenaDamaĝaro` anstataŭ fordoni damaĝon.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Damaĝaro<const N: usize> {
    eroj: [Ortangulo; N],
    nombro: usize,
}

/// La norma damaĝaro kun [`MAKSAJ_DAMAĜOJ`] lokoj.
pub type Damaĝaro32 = Damaĝaro<MAKSAJ_DAMAĜOJ>;

impl<const N: usize> Damaĝaro<N> {
    /// Konstruas malplenan damaĝaron.
    #[must_use]
    pub const fn nova() -> Self {
        Self {
            eroj: [Ortangulo {
                x: 0,
                y: 0,
                larĝo: 0,
                alto: 0,
            }; N],
            nombro: 0,
        }
    }

    /// Aldonas ortangulon, kunfandante ĝin kun ĉiu intersekcata ero.
    ///
    /// # Eraroj
    ///
    /// `Eraro::NevalidaOrtangulo` se la ortangulo estas nula aŭ la kunfandaĵo
    /// ne reprezenteblas; `Eraro::PlenaDamaĝaro` se ne restas loko.
    pub fn aldonu(&mut self, ortangulo: Ortangulo) -> Result<(), Eraro> {
        if ortangulo.larĝo == 0 || ortangulo.alto == 0 {
            return Err(Eraro::NevalidaOrtangulo);
        }

        let mut kandidato = ortangulo;
        let mut indekso = 0;
        while indekso < self.nombro {
            if self.eroj[indekso].intersekcas(&kandidato) {
                kandidato = self.eroj[indekso]
                    .kunfandi(&kandidato)
                    .ok_or(Eraro::NevalidaOrtangulo)?;
                self.nombro -= 1;
                self.eroj[indekso] = self.eroj[self.nombro];
                indekso = 0;
            } else {
                indekso += 1;
            }
        }

        if self.nombro == N {
            return Err(Eraro::PlenaDamaĝaro);
        }
        self.eroj[self.nombro] = kandidato;
        self.nombro += 1;
        Ok(())
    }

    /// Validigas la ortangulon kontraŭ surfaco kaj aldonas ĝin.
    ///
    /// # Eraroj
    ///
    /// La eraroj de [`Damaĝaro::aldonu`] kaj `Eraro::NevalidaOrtangulo` se la
    /// ortangulo ne estas tute ene de la surfaco.
    pub fn aldonu_en(
        &mut self,
        surfaco: &SurfacaPriskribo,
        ortangulo: Ortangulo,
    ) -> Result<(), Eraro> {
        if !ortangulo.eniĝas_en(surfaco) {
            return Err(Eraro::NevalidaOrtangulo);
        }
        self.aldonu(ortangulo)
    }

    /// La nunaj ortanguloj.
    #[must_use]
    pub fn eroj(&self) -> &[Ortangulo] {
        &self.eroj[..self.nombro]
    }

    /// La nombro de ortanguloj.
    #[must_use]
    pub const fn nombro(&self) -> usize {
        self.nombro
    }

    /// Forviŝas ĉiujn ortangulojn.
    pub fn malplenigi(&mut self) {
        self.nombro = 0;
    }
}

#[cfg(kani)]
mod kani_pruvoj {
    //! Kani-pruvo pri la damaĝa kunigo (S08, D02, D05).
    //!
    //! Por arbitraj ortanguloj ene de 16×16-surfaco, post sukcesa aldono la
    //! damaĝaro restas ene de la kapacito, kaj ĉiu aldonita ortangulo estas
    //! kovrita de almenaŭ unu ero. Plena damaĝaro rifuzas plian eron sen
    //! ŝanĝi sian nombron.

    use super::{Damaĝaro, Ortangulo, SurfacaPriskribo};

    fn en_limoj(x: i32, y: i32, larĝo: u32, alto: u32) -> bool {
        x >= 0
            && y >= 0
            && larĝo >= 1
            && alto >= 1
            && i64::from(x) + i64::from(larĝo) <= 16
            && i64::from(y) + i64::from(alto) <= 16
    }

    /// Kunigo konservas la kapaciton kaj kovras la originalojn.
    #[kani::proof]
    #[kani::unwind(12)]
    fn ortangula_kunigo() {
        let x1: i32 = kani::any();
        let y1: i32 = kani::any();
        let l1: u32 = kani::any();
        let a1: u32 = kani::any();
        kani::assume(en_limoj(x1, y1, l1, a1));

        let x2: i32 = kani::any();
        let y2: i32 = kani::any();
        let l2: u32 = kani::any();
        let a2: u32 = kani::any();
        kani::assume(en_limoj(x2, y2, l2, a2));

        let Ok(r1) = Ortangulo::nova(x1, y1, l1, a1) else {
            return;
        };
        let Ok(r2) = Ortangulo::nova(x2, y2, l2, a2) else {
            return;
        };

        let surfaco = match SurfacaPriskribo::nova(16, 16, 64) {
            Ok(surfaco) => surfaco,
            Err(_) => return,
        };
        let mut damaĝoj = Damaĝaro::<3>::nova();
        if damaĝoj.aldonu_en(&surfaco, r1).is_err() {
            return;
        }
        if damaĝoj.aldonu_en(&surfaco, r2).is_err() {
            return;
        }

        assert!(damaĝoj.nombro() <= 3);
        assert!(!damaĝoj.eroj().is_empty());

        let mut kovrita1 = false;
        let mut kovrita2 = false;
        let mut indekso = 0;
        while indekso < damaĝoj.nombro() {
            let ero = damaĝoj.eroj()[indekso];
            assert!(ero.eniĝas_en(&surfaco));
            kovrita1 = kovrita1 || ero.enhauvas(&r1);
            kovrita2 = kovrita2 || ero.enhauvas(&r2);
            indekso += 1;
        }
        assert!(kovrita1);
        assert!(kovrita2);
    }

    /// Plena damaĝaro rifuzas plian eron.
    #[kani::proof]
    #[kani::unwind(12)]
    fn plena_damagxaro_rifuzas() {
        let mut damaĝoj = Damaĝaro::<3>::nova();
        let ok1 = Ortangulo::nova(0, 0, 1, 1);
        let ok2 = Ortangulo::nova(2, 0, 1, 1);
        let ok3 = Ortangulo::nova(4, 0, 1, 1);
        let ok4 = Ortangulo::nova(6, 0, 1, 1);
        if let (Ok(r1), Ok(r2), Ok(r3), Ok(r4)) = (ok1, ok2, ok3, ok4) {
            assert!(damaĝoj.aldonu(r1).is_ok());
            assert!(damaĝoj.aldonu(r2).is_ok());
            assert!(damaĝoj.aldonu(r3).is_ok());
            assert!(damaĝoj.aldonu(r4).is_err());
            assert_eq!(damaĝoj.nombro(), 3);
        }
    }
}

#[cfg(test)]
mod testoj {
    use super::*;

    #[test]
    fn ortangulo_estas_16_bajtoj() {
        assert_eq!(core::mem::size_of::<Ortangulo>(), 16);
        assert_eq!(core::mem::align_of::<Ortangulo>(), 4);
    }

    #[test]
    fn surfaco_estas_12_bajtoj() {
        assert_eq!(core::mem::size_of::<SurfacaPriskribo>(), 12);
        assert_eq!(core::mem::align_of::<SurfacaPriskribo>(), 4);
    }

    #[test]
    fn surfaca_validigo() {
        assert!(SurfacaPriskribo::nova(640, 480, 2560).is_ok());
        assert_eq!(
            SurfacaPriskribo::nova(0, 480, 2560),
            Err(Eraro::NevalidaSurfaco)
        );
        assert_eq!(
            SurfacaPriskribo::nova(4097, 480, 16388),
            Err(Eraro::NevalidaSurfaco)
        );
        assert_eq!(
            SurfacaPriskribo::nova(640, 480, 2559),
            Err(Eraro::NevalidaSurfaco)
        );
        assert_eq!(
            SurfacaPriskribo::nova(640, 480, 2556),
            Err(Eraro::NevalidaSurfaco)
        );
        assert!(SurfacaPriskribo::nova(4096, 4096, 16384).is_ok());
    }

    #[test]
    fn ortangulo_validigo_kaj_limoj() {
        assert_eq!(Ortangulo::nova(0, 0, 0, 1), Err(Eraro::NevalidaOrtangulo));
        let surfaco = SurfacaPriskribo::nova(16, 16, 64).unwrap();
        assert!(Ortangulo::nova(0, 0, 16, 16).unwrap().eniĝas_en(&surfaco));
        assert!(!Ortangulo::nova(-1, 0, 16, 16).unwrap().eniĝas_en(&surfaco));
        assert!(!Ortangulo::nova(1, 0, 16, 16).unwrap().eniĝas_en(&surfaco));
        assert!(!Ortangulo::nova(0, 1, 16, 16).unwrap().eniĝas_en(&surfaco));
    }

    #[test]
    fn intersekco_kaj_kunfando() {
        let a = Ortangulo::nova(0, 0, 10, 10).unwrap();
        let b = Ortangulo::nova(5, 5, 10, 10).unwrap();
        let c = Ortangulo::nova(20, 20, 10, 10).unwrap();
        assert!(a.intersekcas(&b));
        assert!(!a.intersekcas(&c));
        let kun = a.kunfandi(&b).unwrap();
        assert!(kun.enhauvas(&a));
        assert!(kun.enhauvas(&b));
        assert_eq!(kun, Ortangulo::nova(0, 0, 15, 15).unwrap());
    }

    #[test]
    fn kunigo_transitiva() {
        let mut damaĝoj = Damaĝaro::<8>::nova();
        damaĝoj
            .aldonu(Ortangulo::nova(0, 0, 10, 10).unwrap())
            .unwrap();
        damaĝoj
            .aldonu(Ortangulo::nova(8, 8, 10, 10).unwrap())
            .unwrap();
        damaĝoj
            .aldonu(Ortangulo::nova(16, 16, 10, 10).unwrap())
            .unwrap();
        assert_eq!(damaĝoj.nombro(), 1);
        assert_eq!(damaĝoj.eroj()[0], Ortangulo::nova(0, 0, 26, 26).unwrap());
    }

    #[test]
    fn neintersekcantaj_restas_apartaj() {
        let mut damaĝoj = Damaĝaro::<8>::nova();
        damaĝoj
            .aldonu(Ortangulo::nova(0, 0, 2, 2).unwrap())
            .unwrap();
        damaĝoj
            .aldonu(Ortangulo::nova(4, 0, 2, 2).unwrap())
            .unwrap();
        assert_eq!(damaĝoj.nombro(), 2);
    }

    #[test]
    fn plena_damagxaro_rifuzas() {
        let mut damaĝoj = Damaĝaro::<2>::nova();
        damaĝoj
            .aldonu(Ortangulo::nova(0, 0, 1, 1).unwrap())
            .unwrap();
        damaĝoj
            .aldonu(Ortangulo::nova(2, 0, 1, 1).unwrap())
            .unwrap();
        assert_eq!(
            damaĝoj.aldonu(Ortangulo::nova(4, 0, 1, 1).unwrap()),
            Err(Eraro::PlenaDamaĝaro)
        );
        assert_eq!(damaĝoj.nombro(), 2);
    }

    #[test]
    fn kunfando_de_superplenaj_koordinatoj_estas_rifuzata() {
        let a = Ortangulo {
            x: i32::MAX,
            y: 0,
            larĝo: u32::MAX,
            alto: 1,
        };
        let b = Ortangulo {
            x: 0,
            y: 0,
            larĝo: 1,
            alto: 1,
        };
        assert!(a.kunfandi(&b).is_none());
    }

    #[test]
    fn aldonu_en_validigas_la_surfacan_limon() {
        let surfaco = SurfacaPriskribo::nova(8, 8, 32).unwrap();
        let mut damaĝoj = Damaĝaro::<4>::nova();
        assert_eq!(
            damaĝoj.aldonu_en(&surfaco, Ortangulo::nova(7, 0, 2, 1).unwrap()),
            Err(Eraro::NevalidaOrtangulo)
        );
        assert!(
            damaĝoj
                .aldonu_en(&surfaco, Ortangulo::nova(6, 0, 2, 1).unwrap())
                .is_ok()
        );
    }
}
