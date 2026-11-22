//! La fenestra protokolo: kapo, mesaĝaj tipoj kaj totala legado.
//!
//! Ĉiu mesaĝo havas fiksan 8-bajtan kapon, skribitan en little-endian ordo:
//!
//! ```text
//! deŝovo 0: versio (u16)
//! deŝovo 2: tipo   (u16)
//! deŝovo 4: longo  (u32, inkluzive la kapon)
//! ```
//!
//! La tuta mesaĝo neniam superas 64 bajtojn. `Mesaĝo::analizi` estas totala:
//! ĝi rifuzas ĉiun mallongan, tro longan aŭ nekonatan enigon anstataŭ paniki.

use crate::eraro::Eraro;

/// Longo de la mesaĝa kapo en bajtoj.
pub const KAPA_LONGO: usize = 8;

/// La supra limo de la tuta mesaĝo, inkluzive la kapon.
pub const MAKSA_MESAĜA_LONGO: usize = 64;

/// La supra limo de la datumaro, inkluzive la kapon.
pub const MAKSA_DATUMO: usize = MAKSA_MESAĜA_LONGO - KAPA_LONGO;

/// La nuna protokola versio. Nekonata versio estas rifuzata, ne ignorata.
pub const PROTOKOLA_VERSIO: u16 = 1;

/// Fiksa mesaĝa tipo de la fenestra protokolo.
///
/// Petoj havas la numerojn 1–9; eventoj havas 16–20. La numeroj estas parto de
/// la kontrakto kaj ne ŝanĝiĝas sen nova protokola versio.
#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MesaĝoTipo {
    /// Peto krei fenestron.
    Krei = 1,
    /// Peto montri fenestron.
    Monstri = 2,
    /// Peto kaŝi fenestron.
    Kaŝi = 3,
    /// Peto fermi fenestron.
    Fermi = 4,
    /// Peto movi fenestron.
    Movi = 5,
    /// Peto grandigi fenestron.
    Grandigi = 6,
    /// Peto fokusigi fenestron.
    Fokusi = 7,
    /// Peto difini la titolon.
    DifiniTitolon = 8,
    /// Peto difini la damaĝon.
    DifiniDamaĝon = 9,
    /// Evento: la fenestro kreiĝis.
    Kreita = 16,
    /// Evento: la fenestro fermiĝis.
    Fermita = 17,
    /// Evento: la fenestro fokusiĝis.
    Fokusita = 18,
    /// Evento: enigo alvenis.
    Enigo = 19,
    /// Evento: la fenestra procezo paneis.
    Paneita = 20,
}

impl MesaĝoTipo {
    /// Redonas la tipon por la fiksa numero, aŭ `None` por nekonata numero.
    #[must_use]
    pub const fn el_numero(numero: u16) -> Option<Self> {
        match numero {
            1 => Some(Self::Krei),
            2 => Some(Self::Monstri),
            3 => Some(Self::Kaŝi),
            4 => Some(Self::Fermi),
            5 => Some(Self::Movi),
            6 => Some(Self::Grandigi),
            7 => Some(Self::Fokusi),
            8 => Some(Self::DifiniTitolon),
            9 => Some(Self::DifiniDamaĝon),
            16 => Some(Self::Kreita),
            17 => Some(Self::Fermita),
            18 => Some(Self::Fokusita),
            19 => Some(Self::Enigo),
            20 => Some(Self::Paneita),
            _ => None,
        }
    }

    /// Redonas la fiksan numeron de la tipo.
    #[must_use]
    pub const fn numero(self) -> u16 {
        self as u16
    }

    /// Ĉu la tipo estas peto de la aplikaĵo al la kompozitoro?
    #[must_use]
    pub const fn estas_peto(self) -> bool {
        matches!(
            self,
            Self::Krei
                | Self::Monstri
                | Self::Kaŝi
                | Self::Fermi
                | Self::Movi
                | Self::Grandigi
                | Self::Fokusi
                | Self::DifiniTitolon
                | Self::DifiniDamaĝon
        )
    }

    /// Ĉu la tipo estas evento de la kompozitoro al la aplikaĵo?
    #[must_use]
    pub const fn estas_evento(self) -> bool {
        !self.estas_peto()
    }
}

/// La fiksa kapo de mesaĝo.
///
/// La kampoj estas publika por legado, sed oni konstruu la kapon per
/// [`Kapo::nova`] aŭ [`Kapo::legi`]; [`Kapo::skribi`] validigas la kampojn
/// antaŭ ol skribi.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Kapo {
    /// Protokola versio.
    pub versio: u16,
    /// Mesaĝa tipo.
    pub tipo: u16,
    /// Longo de la tuta mesaĝo, inkluzive la kapon.
    pub longo: u32,
}

impl Kapo {
    /// Konstruas kapon por tipo kun donita datumlongo.
    ///
    /// # Eraroj
    ///
    /// `Eraro::TroLongaMesaĝo` se la tuta mesaĝo superas 64 bajtojn.
    pub fn nova(tipo: MesaĝoTipo, datumo_longo: usize) -> Result<Self, Eraro> {
        let longo = KAPA_LONGO
            .checked_add(datumo_longo)
            .ok_or(Eraro::TroLongaMesaĝo)?;
        if longo > MAKSA_MESAĜA_LONGO {
            return Err(Eraro::TroLongaMesaĝo);
        }
        Ok(Self {
            versio: PROTOKOLA_VERSIO,
            tipo: tipo.numero(),
            longo: longo as u32,
        })
    }

    /// Legas kaj validigas kapon el la komenco de bufro.
    ///
    /// # Eraroj
    ///
    /// La funkcio rifuzas tro mallongajn bufrojn, nekonatan version, nekonatan
    /// tipon kaj longon ekster la limoj.
    pub fn legi(buf: &[u8]) -> Result<Self, Eraro> {
        if buf.len() < KAPA_LONGO {
            return Err(Eraro::MalgrandaMesaĝo);
        }
        let versio = legi_u16_le(buf, 0)?;
        if versio != PROTOKOLA_VERSIO {
            return Err(Eraro::NekonataProtokolaVersio);
        }
        let tipo = legi_u16_le(buf, 2)?;
        if MesaĝoTipo::el_numero(tipo).is_none() {
            return Err(Eraro::NekonataMesaĝoTipo);
        }
        let longo = legi_u32_le(buf, 4)?;
        let longo = usize::try_from(longo).map_err(|_| Eraro::TroLongaMesaĝo)?;
        if longo < KAPA_LONGO {
            return Err(Eraro::MalgrandaMesaĝo);
        }
        if longo > MAKSA_MESAĜA_LONGO {
            return Err(Eraro::TroLongaMesaĝo);
        }
        if longo > buf.len() {
            return Err(Eraro::MalgrandaMesaĝo);
        }
        Ok(Self {
            versio,
            tipo,
            longo: longo as u32,
        })
    }

    /// Skribas la kapon en la unuajn ok bajtojn de la bufro.
    ///
    /// # Eraroj
    ///
    /// La funkcio rifuzas bufron pli mallongan ol la kapo, nevalidan version,
    /// nekonatan tipon kaj longon ekster la limoj.
    pub fn skribi(&self, buf: &mut [u8]) -> Result<(), Eraro> {
        if self.versio != PROTOKOLA_VERSIO {
            return Err(Eraro::NekonataProtokolaVersio);
        }
        if MesaĝoTipo::el_numero(self.tipo).is_none() {
            return Err(Eraro::NekonataMesaĝoTipo);
        }
        let longo = usize::try_from(self.longo).map_err(|_| Eraro::TroLongaMesaĝo)?;
        if !(KAPA_LONGO..=MAKSA_MESAĜA_LONGO).contains(&longo) {
            return Err(Eraro::TroLongaMesaĝo);
        }
        skribi_u16_le(buf, 0, self.versio)?;
        skribi_u16_le(buf, 2, self.tipo)?;
        skribi_u32_le(buf, 4, self.longo)?;
        Ok(())
    }
}

/// Mesaĝo konsistanta el kapo kaj pruntita datumaro.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mesaĝo<'a> {
    kapo: Kapo,
    datumo: &'a [u8],
}

impl<'a> Mesaĝo<'a> {
    /// Analizas unu mesaĝon el la komenco de la bufro.
    ///
    /// La legado estas totala: nek mallonga bufro nek arbitraj bajtoj kaŭzas
    /// panikon aŭ legadon ekster la bufro. Se la bufro enhavas pli ol unu
    /// mesaĝon, la resto estas ignorata; `longo()` indikas la konsumitan parton.
    pub fn analizi(buf: &'a [u8]) -> Result<Self, Eraro> {
        let kapo = Kapo::legi(buf)?;
        let fino = usize::try_from(kapo.longo).map_err(|_| Eraro::TroLongaMesaĝo)?;
        let datumo = buf.get(KAPA_LONGO..fino).ok_or(Eraro::MalgrandaMesaĝo)?;
        Ok(Self { kapo, datumo })
    }

    /// Redonas la analizitan kapon.
    #[must_use]
    pub const fn kapo(&self) -> &Kapo {
        &self.kapo
    }

    /// Redonas la datumaron sen la kapo.
    #[must_use]
    pub const fn datumo(&self) -> &'a [u8] {
        self.datumo
    }

    /// Redonas la longon de la tuta mesaĝo.
    #[must_use]
    pub const fn longo(&self) -> usize {
        self.kapo.longo as usize
    }
}

/// Skribas tutan mesaĝon en bufron kaj redonas ĝian longon.
///
/// # Eraroj
///
/// La funkcio rifuzas datumaron, kiu kun la kapo superas 64 bajtojn, kaj
/// bufron pli mallongan ol la mesaĝo.
pub fn skribi_mesaĝon(tipo: MesaĝoTipo, datumo: &[u8], buf: &mut [u8]) -> Result<usize, Eraro> {
    let longo = KAPA_LONGO
        .checked_add(datumo.len())
        .ok_or(Eraro::TroLongaMesaĝo)?;
    if longo > MAKSA_MESAĜA_LONGO {
        return Err(Eraro::TroLongaMesaĝo);
    }
    if buf.len() < longo {
        return Err(Eraro::MalgrandaMesaĝo);
    }
    skribi_u16_le(buf, 0, PROTOKOLA_VERSIO)?;
    skribi_u16_le(buf, 2, tipo.numero())?;
    skribi_u32_le(buf, 4, longo as u32)?;
    buf[KAPA_LONGO..longo].copy_from_slice(datumo);
    Ok(longo)
}

/// Legas `u16` en little-endian ordo.
///
/// # Eraroj
///
/// `Eraro::MalgrandaMesaĝo` se la bufro ne enhavas la tutan valoron.
pub fn legi_u16_le(buf: &[u8], deŝovo: usize) -> Result<u16, Eraro> {
    let fino = deŝovo.checked_add(2).ok_or(Eraro::MalgrandaMesaĝo)?;
    let ero = buf.get(deŝovo..fino).ok_or(Eraro::MalgrandaMesaĝo)?;
    Ok(u16::from_le_bytes([ero[0], ero[1]]))
}

/// Legas `u32` en little-endian ordo.
///
/// # Eraroj
///
/// `Eraro::MalgrandaMesaĝo` se la bufro ne enhavas la tutan valoron.
pub fn legi_u32_le(buf: &[u8], deŝovo: usize) -> Result<u32, Eraro> {
    let fino = deŝovo.checked_add(4).ok_or(Eraro::MalgrandaMesaĝo)?;
    let ero = buf.get(deŝovo..fino).ok_or(Eraro::MalgrandaMesaĝo)?;
    Ok(u32::from_le_bytes([ero[0], ero[1], ero[2], ero[3]]))
}

/// Skribas `u16` en little-endian ordo.
///
/// # Eraroj
///
/// `Eraro::MalgrandaMesaĝo` se la bufro ne havas lokon por la valoro.
pub fn skribi_u16_le(buf: &mut [u8], deŝovo: usize, valoro: u16) -> Result<(), Eraro> {
    let fino = deŝovo.checked_add(2).ok_or(Eraro::MalgrandaMesaĝo)?;
    let ero = buf.get_mut(deŝovo..fino).ok_or(Eraro::MalgrandaMesaĝo)?;
    ero.copy_from_slice(&valoro.to_le_bytes());
    Ok(())
}

/// Skribas `u32` en little-endian ordo.
///
/// # Eraroj
///
/// `Eraro::MalgrandaMesaĝo` se la bufro ne havas lokon por la valoro.
pub fn skribi_u32_le(buf: &mut [u8], deŝovo: usize, valoro: u32) -> Result<(), Eraro> {
    let fino = deŝovo.checked_add(4).ok_or(Eraro::MalgrandaMesaĝo)?;
    let ero = buf.get_mut(deŝovo..fino).ok_or(Eraro::MalgrandaMesaĝo)?;
    ero.copy_from_slice(&valoro.to_le_bytes());
    Ok(())
}

#[cfg(kani)]
mod kani_pruvoj {
    //! Kani-pruvo pri la totala legado de protokola mesaĝo (S04).
    //!
    //! La pruvo generas arbitrajn bajtojn kaj arbitran longon ĝis la fiksa
    //! limo; `Mesaĝo::analizi` rajtas rifuzi, sed neniam paniki aŭ legi ekster
    //! la bufro. Ĉiu sukcesa rezulto plenumas la dokumentitajn limojn.

    use super::{KAPA_LONGO, MAKSA_MESAĜA_LONGO, Mesaĝo, MesaĝoTipo, PROTOKOLA_VERSIO};

    /// Arbitraj bajtoj neniam eliras la bufron, kaj sukcesa analizo respektas
    /// la longon, la version kaj la tipon.
    #[kani::proof]
    fn totala_legado() {
        let bajtoj: [u8; MAKSA_MESAĜA_LONGO] = kani::any();
        let longo: usize = kani::any();
        kani::assume(longo <= MAKSA_MESAĜA_LONGO);

        if let Ok(mesago) = Mesaĝo::analizi(&bajtoj[..longo]) {
            assert!(mesago.longo() <= longo);
            assert!(mesago.longo() <= MAKSA_MESAĜA_LONGO);
            assert_eq!(mesago.datumo().len(), mesago.longo() - KAPA_LONGO);
            assert_eq!(mesago.kapo().versio, PROTOKOLA_VERSIO);
            assert!(MesaĝoTipo::el_numero(mesago.kapo().tipo).is_some());
        }
    }
}

#[cfg(test)]
mod testoj {
    use super::*;

    #[test]
    fn kapo_estas_ok_bajtoj() {
        assert_eq!(core::mem::size_of::<Kapo>(), 8);
        assert_eq!(core::mem::align_of::<Kapo>(), 4);
    }

    #[test]
    fn mesaĝo_skribiĝas_kaj_legiĝas() {
        let datumo = [1u8, 2, 3, 4];
        let mut buf = [0u8; MAKSA_MESAĜA_LONGO];
        let longo = skribi_mesaĝon(MesaĝoTipo::Krei, &datumo, &mut buf).unwrap();
        assert_eq!(longo, KAPA_LONGO + datumo.len());
        assert_eq!(&buf[..4], &[1, 0, 1, 0]);
        assert_eq!(&buf[4..8], &[12, 0, 0, 0]);

        let mesaĝo = Mesaĝo::analizi(&buf[..longo]).unwrap();
        assert_eq!(mesaĝo.kapo().tipo, MesaĝoTipo::Krei.numero());
        assert_eq!(mesaĝo.datumo(), &datumo);
        assert_eq!(mesaĝo.longo(), longo);
    }

    #[test]
    fn mallonga_mesaĝo_estas_rifuzata() {
        let buf = [0u8; 4];
        assert_eq!(Mesaĝo::analizi(&buf), Err(Eraro::MalgrandaMesaĝo));
    }

    #[test]
    fn nekonata_versio_estas_rifuzata() {
        let mut buf = [0u8; KAPA_LONGO];
        skribi_u16_le(&mut buf, 0, PROTOKOLA_VERSIO + 1).unwrap();
        skribi_u16_le(&mut buf, 2, MesaĝoTipo::Krei.numero()).unwrap();
        skribi_u32_le(&mut buf, 4, KAPA_LONGO as u32).unwrap();
        assert_eq!(Mesaĝo::analizi(&buf), Err(Eraro::NekonataProtokolaVersio));
    }

    #[test]
    fn nekonata_tipo_estas_rifuzata() {
        let mut buf = [0u8; KAPA_LONGO];
        skribi_u16_le(&mut buf, 0, PROTOKOLA_VERSIO).unwrap();
        skribi_u16_le(&mut buf, 2, 999).unwrap();
        skribi_u32_le(&mut buf, 4, KAPA_LONGO as u32).unwrap();
        assert_eq!(Mesaĝo::analizi(&buf), Err(Eraro::NekonataMesaĝoTipo));
    }

    #[test]
    fn longaj_kaj_mallongaj_kapoj_estas_rifuzataj() {
        let mut buf = [0u8; MAKSA_MESAĜA_LONGO];
        skribi_u16_le(&mut buf, 0, PROTOKOLA_VERSIO).unwrap();
        skribi_u16_le(&mut buf, 2, MesaĝoTipo::Krei.numero()).unwrap();
        for longo in [0u32, 7, 65, 1000] {
            skribi_u32_le(&mut buf, 4, longo).unwrap();
            assert!(Mesaĝo::analizi(&buf).is_err());
        }
    }

    #[test]
    fn mesaĝo_pli_longa_ol_la_bufro_estas_rifuzata() {
        let mut buf = [0u8; KAPA_LONGO];
        skribi_u16_le(&mut buf, 0, PROTOKOLA_VERSIO).unwrap();
        skribi_u16_le(&mut buf, 2, MesaĝoTipo::Krei.numero()).unwrap();
        skribi_u32_le(&mut buf, 4, 16).unwrap();
        assert_eq!(Mesaĝo::analizi(&buf), Err(Eraro::MalgrandaMesaĝo));
    }

    #[test]
    fn skribo_rifuzas_tro_longan_datumaron() {
        let datumo = [0u8; MAKSA_DATUMO + 1];
        let mut buf = [0u8; MAKSA_MESAĜA_LONGO];
        assert_eq!(
            skribi_mesaĝon(MesaĝoTipo::Krei, &datumo, &mut buf),
            Err(Eraro::TroLongaMesaĝo)
        );
    }

    #[test]
    fn legado_estas_totala_por_ĉiu_longo() {
        let buf = [0xA5u8; MAKSA_MESAĜA_LONGO];
        for longo in 0..=MAKSA_MESAĜA_LONGO {
            let _ = Mesaĝo::analizi(&buf[..longo]);
        }
    }

    #[test]
    fn direktoj_de_tipoj() {
        assert!(MesaĝoTipo::Krei.estas_peto());
        assert!(MesaĝoTipo::Kreita.estas_evento());
        assert!(!MesaĝoTipo::Kreita.estas_peto());
        for numero in 0..=u16::MAX {
            if let Some(tipo) = MesaĝoTipo::el_numero(numero) {
                assert_eq!(tipo.numero(), numero);
            }
        }
    }
}
