//! Enigaj eventoj: klavo kaj muso en fiksa 20-bajta aranĝo.
//!
//! La aranĝo estas little-endian kaj komuna por ambaŭ arkitekturoj:
//!
//! ```text
//! deŝovo  0: speco     (u16)
//! deŝovo  2: modifiloj (u16)
//! deŝovo  4: klavo     (u32)
//! deŝovo  8: x         (i32)
//! deŝovo 12: y         (i32)
//! deŝovo 16: butonoj   (u8)
//! deŝovo 17: stato     (u8)
//! deŝovo 18: rezervo   (2 bajtoj, nulaj)
//! ```
//!
//! La rezervaj bajtoj devas esti nulaj; nekonata valoro estas rifuzata
//! anstataŭ ignorata (H14).

use crate::eraro::Eraro;
use crate::protokolo::{legi_u16_le, legi_u32_le, skribi_u16_le, skribi_u32_le};

/// Evento pri klavo.
pub const SPECO_KLAVO: u16 = 1;

/// Evento pri muso.
pub const SPECO_MUSO: u16 = 2;

/// Modifilo: `Shift`.
pub const MODIFILO_ŜOVO: u16 = 1 << 0;

/// Modifilo: `Ctrl`.
pub const MODIFILO_CTRL: u16 = 1 << 1;

/// Modifilo: `Alt`.
pub const MODIFILO_ALT: u16 = 1 << 2;

/// Modifilo: la `Super`-klavo, estonte `Mito`.
pub const MODIFILO_SUPER: u16 = 1 << 3;

/// La bitmaska limo de la subtenataj modifiloj.
pub const MAKSAJ_MODIFILOJ: u16 = MODIFILO_ŜOVO | MODIFILO_CTRL | MODIFILO_ALT | MODIFILO_SUPER;

/// Musbutono: maldekstra.
pub const BUTONO_MALDEKSTRA: u8 = 1 << 0;

/// Musbutono: meza.
pub const BUTONO_MEZA: u8 = 1 << 1;

/// Musbutono: dekstra.
pub const BUTONO_DEKSTRA: u8 = 1 << 2;

/// La bitmaska limo de la subtenataj musbutonoj.
pub const MAKSAJ_BUTONOJ: u8 = BUTONO_MALDEKSTRA | BUTONO_MEZA | BUTONO_DEKSTRA;

/// Stato: la klavo aŭ butono estas liberigita.
pub const STATO_LIBERIGITA: u8 = 0;

/// Stato: la klavo aŭ butono estas premita.
pub const STATO_PREMITA: u8 = 1;

/// Longo de la eniga evento en bajtoj.
pub const MAKSA_ENIGA_LONGO: usize = 20;

/// Unu eniga evento.
///
/// Por klavo, `stato` indikas ĉu la klavo estas premita, kaj `butonoj`, `x`
/// kaj `y` estas nulaj. Por muso, `klavo` estas nula, `butonoj` estas la
/// bitmaska stato, kaj `stato` estas premita se iu butono estas premita.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnigaEvento {
    /// `SPECO_KLAVO` aŭ `SPECO_MUSO`.
    pub speco: u16,
    /// Bitmaska kombinaĵo de la `MODIFILO_*` konstantoj.
    pub modifiloj: u16,
    /// Klavokodo por `SPECO_KLAVO`; nulo por muso.
    pub klavo: u32,
    /// X-koordinato por muso; nulo por klavo.
    pub x: i32,
    /// Y-koordinato por muso; nulo por klavo.
    pub y: i32,
    /// Bitmaska kombinaĵo de la `BUTONO_*` konstantoj por muso.
    pub butonoj: u8,
    /// `STATO_LIBERIGITA` aŭ `STATO_PREMITA`.
    pub stato: u8,
    /// Rezervitaj bajtoj; devas esti nulaj.
    pub rezervo: [u8; 2],
}

impl EnigaEvento {
    /// Konstruas klavan eventon.
    ///
    /// # Eraroj
    ///
    /// `Eraro::NevalidaEnigo` se la modifiloj enhavas nekonatan biton.
    pub const fn klavo(klavo: u32, premita: bool, modifiloj: u16) -> Result<Self, Eraro> {
        if modifiloj & !MAKSAJ_MODIFILOJ != 0 {
            return Err(Eraro::NevalidaEnigo);
        }
        Ok(Self {
            speco: SPECO_KLAVO,
            modifiloj,
            klavo,
            x: 0,
            y: 0,
            butonoj: 0,
            stato: if premita {
                STATO_PREMITA
            } else {
                STATO_LIBERIGITA
            },
            rezervo: [0; 2],
        })
    }

    /// Konstruas musan eventon.
    ///
    /// # Eraroj
    ///
    /// `Eraro::NevalidaEnigo` se la modifiloj aŭ butonoj enhavas nekonatan
    /// biton.
    pub const fn muso(x: i32, y: i32, butonoj: u8, modifiloj: u16) -> Result<Self, Eraro> {
        if modifiloj & !MAKSAJ_MODIFILOJ != 0 || butonoj & !MAKSAJ_BUTONOJ != 0 {
            return Err(Eraro::NevalidaEnigo);
        }
        Ok(Self {
            speco: SPECO_MUSO,
            modifiloj,
            klavo: 0,
            x,
            y,
            butonoj,
            stato: if butonoj == 0 {
                STATO_LIBERIGITA
            } else {
                STATO_PREMITA
            },
            rezervo: [0; 2],
        })
    }

    /// Validigas ĉiujn kampojn laŭ la fiksa aranĝo.
    ///
    /// # Eraroj
    ///
    /// `Eraro::NevalidaEnigo` por nekonata speco, nekonata modifila aŭ butona
    /// bito, nevalida stato aŭ ne-nula rezervo. Klavo devas havi nulajn musajn
    /// kampojn; muso devas havi nulan klavokodon kaj kongruan staton.
    pub const fn validi(&self) -> Result<(), Eraro> {
        if self.speco != SPECO_KLAVO && self.speco != SPECO_MUSO {
            return Err(Eraro::NevalidaEnigo);
        }
        if self.modifiloj & !MAKSAJ_MODIFILOJ != 0 {
            return Err(Eraro::NevalidaEnigo);
        }
        if self.stato != STATO_LIBERIGITA && self.stato != STATO_PREMITA {
            return Err(Eraro::NevalidaEnigo);
        }
        if self.rezervo[0] != 0 || self.rezervo[1] != 0 {
            return Err(Eraro::NevalidaEnigo);
        }
        if self.speco == SPECO_KLAVO {
            if self.butonoj != 0 || self.x != 0 || self.y != 0 {
                return Err(Eraro::NevalidaEnigo);
            }
        } else if self.klavo != 0
            || self.butonoj & !MAKSAJ_BUTONOJ != 0
            || (self.stato == STATO_PREMITA) != (self.butonoj != 0)
        {
            return Err(Eraro::NevalidaEnigo);
        }
        Ok(())
    }

    /// Skribas la eventon en la unuajn 20 bajtojn de la bufro.
    ///
    /// # Eraroj
    ///
    /// La eraroj de [`EnigaEvento::validi`] kaj `Eraro::MalgrandaMesaĝo` se la
    /// bufro estas pli mallonga ol 20 bajtoj.
    pub fn skribi(&self, buf: &mut [u8]) -> Result<usize, Eraro> {
        self.validi()?;
        if buf.len() < MAKSA_ENIGA_LONGO {
            return Err(Eraro::MalgrandaMesaĝo);
        }
        skribi_u16_le(buf, 0, self.speco)?;
        skribi_u16_le(buf, 2, self.modifiloj)?;
        skribi_u32_le(buf, 4, self.klavo)?;
        skribi_u32_le(buf, 8, self.x as u32)?;
        skribi_u32_le(buf, 12, self.y as u32)?;
        buf[16] = self.butonoj;
        buf[17] = self.stato;
        buf[18] = 0;
        buf[19] = 0;
        Ok(MAKSA_ENIGA_LONGO)
    }

    /// Legas unu eventon el la komenco de la bufro.
    ///
    /// # Eraroj
    ///
    /// `Eraro::MalgrandaMesaĝo` se la bufro estas tro mallonga, kaj la eraroj
    /// de [`EnigaEvento::validi`] por arbitraj nevalidaj kampoj.
    pub fn legi(buf: &[u8]) -> Result<Self, Eraro> {
        let evento = Self {
            speco: legi_u16_le(buf, 0)?,
            modifiloj: legi_u16_le(buf, 2)?,
            klavo: legi_u32_le(buf, 4)?,
            x: legi_u32_le(buf, 8)? as i32,
            y: legi_u32_le(buf, 12)? as i32,
            butonoj: *buf.get(16).ok_or(Eraro::MalgrandaMesaĝo)?,
            stato: *buf.get(17).ok_or(Eraro::MalgrandaMesaĝo)?,
            rezervo: [
                *buf.get(18).ok_or(Eraro::MalgrandaMesaĝo)?,
                *buf.get(19).ok_or(Eraro::MalgrandaMesaĝo)?,
            ],
        };
        evento.validi()?;
        Ok(evento)
    }
}

#[cfg(test)]
mod testoj {
    use super::*;

    #[test]
    fn evento_estas_20_bajtoj() {
        assert_eq!(core::mem::size_of::<EnigaEvento>(), 20);
        assert_eq!(core::mem::align_of::<EnigaEvento>(), 4);
    }

    #[test]
    fn klavo_skribiĝas_kaj_legiĝas() {
        let evento = EnigaEvento::klavo(65, true, MODIFILO_ŜOVO | MODIFILO_CTRL).unwrap();
        let mut buf = [0u8; MAKSA_ENIGA_LONGO];
        assert_eq!(evento.skribi(&mut buf).unwrap(), MAKSA_ENIGA_LONGO);
        assert_eq!(EnigaEvento::legi(&buf).unwrap(), evento);
    }

    #[test]
    fn muso_skribiĝas_kaj_legiĝas() {
        let evento = EnigaEvento::muso(-12, 34, BUTONO_MALDEKSTRA, MODIFILO_ALT).unwrap();
        let mut buf = [0u8; MAKSA_ENIGA_LONGO];
        evento.skribi(&mut buf).unwrap();
        assert_eq!(EnigaEvento::legi(&buf).unwrap(), evento);
    }

    #[test]
    fn mallonga_bufro_estas_rifuzata() {
        let buf = [0u8; MAKSA_ENIGA_LONGO - 1];
        assert_eq!(EnigaEvento::legi(&buf), Err(Eraro::MalgrandaMesaĝo));
    }

    #[test]
    fn nekonataj_kampoj_estas_rifuzataj() {
        assert_eq!(
            EnigaEvento::klavo(1, true, 0x8000),
            Err(Eraro::NevalidaEnigo)
        );
        assert_eq!(EnigaEvento::muso(0, 0, 0x80, 0), Err(Eraro::NevalidaEnigo));

        let mut buf = [0u8; MAKSA_ENIGA_LONGO];
        EnigaEvento::klavo(1, true, 0)
            .unwrap()
            .skribi(&mut buf)
            .unwrap();
        buf[18] = 1;
        assert_eq!(EnigaEvento::legi(&buf), Err(Eraro::NevalidaEnigo));

        buf[18] = 0;
        skribi_u16_le(&mut buf, 0, 99).unwrap();
        assert_eq!(EnigaEvento::legi(&buf), Err(Eraro::NevalidaEnigo));

        skribi_u16_le(&mut buf, 0, SPECO_MUSO).unwrap();
        buf[17] = STATO_LIBERIGITA;
        assert_eq!(EnigaEvento::legi(&buf), Err(Eraro::NevalidaEnigo));
    }

    #[test]
    fn legado_de_arbitraj_longoj_ne_panikas() {
        let buf = [0x5Au8; MAKSA_ENIGA_LONGO];
        for longo in 0..=MAKSA_ENIGA_LONGO {
            let _ = EnigaEvento::legi(&buf[..longo]);
        }
    }
}
