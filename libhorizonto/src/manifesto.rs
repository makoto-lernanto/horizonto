//! Manifesta analizo kaj validigo sen `alloc`.
//!
//! La manifesta formo sekvas `FLUOU.md` §9.1 kaj `HORIZONTO.md` §4.4. Akceptiĝas
//! du samsignifaj skriboj: la plata formo (ĉiuj kampoj ĉe la supro, `permesoj`
//! kiel enlinia tabelo) kaj la sekcia formo (`[aplikaĵo]` kaj `[permesoj]`).
//! En unu dosiero oni uzu nur unu formon; miksado estas rifuzata.
//!
//! La analizilo subtenas nur la fiksan TOML-subaron de la manifesto: unu kampo
//! en unu linio, ĉenoj sen eskapaĵoj, buleaj valoroj kaj tabeloj de ĉenoj.
//! Nekonata kampo, sekcio aŭ permeso estas rifuzata (S17, H14).

use crate::eraro::Eraro;

/// La maksimuma longo de la aplikaka nomo.
pub const MAKSA_NOMO: usize = 64;

/// La maksimuma longo de la versia ĉeno.
pub const MAKSA_VERSIO: usize = 16;

/// La maksimuma longo de la kampo `gu`.
pub const MAKSA_GU: usize = 8;

/// La maksimuma longo de la kampo `fluo`.
pub const MAKSA_FLUO: usize = 8;

/// La maksimuma nombro de deklaritaj arkoj.
pub const MAKSAJ_ARKOJ: usize = 2;

/// La maksimuma nombro de dosierujaj permesoj.
pub const MAKSAJ_DOSIERUJOJ: usize = 8;

/// La maksimuma longo de unu dosieruja radiko.
pub const MAKSA_VOJO: usize = 64;

/// La subtenata FluoU-versio; kongruas kun la protokola versio (Q15).
pub const FLUO_VERSIO: &str = "0.1";

const GU_VALORO: &str = "fluoU";

/// Subtenata arko de la manifesto.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arko {
    /// La `x86_64`-arkitekturo.
    X86_64,
    /// La `arm64`-arkitekturo.
    Arm64,
}

impl Arko {
    /// Mapas la manifestan nomon al arko, aŭ `None` por nekonata nomo.
    #[must_use]
    pub fn el_nomo(nomo: &str) -> Option<Self> {
        match nomo {
            "x86_64" => Some(Self::X86_64),
            "arm64" => Some(Self::Arm64),
            _ => None,
        }
    }

    /// La manifesta nomo de la arko.
    #[must_use]
    pub const fn nomo(self) -> &'static str {
        match self {
            Self::X86_64 => "x86_64",
            Self::Arm64 => "arm64",
        }
    }
}

/// Fiksa teksto kun kopio, sen `alloc`.
#[derive(Clone, Copy, PartialEq, Eq)]
struct Teksto<const N: usize> {
    bajtoj: [u8; N],
    longo: u8,
}

impl<const N: usize> Teksto<N> {
    const fn malplena() -> Self {
        Self {
            bajtoj: [0; N],
            longo: 0,
        }
    }

    fn nova(teksto: &str) -> Result<Self, Eraro> {
        let mut ujo = [0u8; N];
        let longo = kopii(teksto.as_bytes(), &mut ujo, N)?;
        Ok(Self {
            bajtoj: ujo,
            longo: longo as u8,
        })
    }

    fn kiel_str(&self) -> &str {
        core::str::from_utf8(&self.bajtoj[..self.longo as usize]).unwrap_or("")
    }
}

/// Kopias bajtojn en fiksan buferon, se ili envenas la limojn.
fn kopii(src: &[u8], dst: &mut [u8], maks: usize) -> Result<usize, Eraro> {
    if src.len() > maks || src.len() > dst.len() || src.len() > u8::MAX as usize {
        return Err(Eraro::TroLongaValoro);
    }
    let mut indekso = 0;
    while indekso < src.len() {
        dst[indekso] = src[indekso];
        indekso += 1;
    }
    Ok(src.len())
}

/// Valida manifesta priskribo.
///
/// Oni konstruu ĝin per [`Manifesto::analizi`], kiu validigas ĉiujn kampojn.
#[derive(Clone, PartialEq, Eq)]
pub struct Manifesto {
    nomo: Teksto<MAKSA_NOMO>,
    versio: Teksto<MAKSA_VERSIO>,
    gu: Teksto<MAKSA_GU>,
    fluo: Teksto<MAKSA_FLUO>,
    arkoj: [Arko; MAKSAJ_ARKOJ],
    arkoj_nombro: usize,
    fenestro: bool,
    reto: bool,
    dosieroj: [Teksto<MAKSA_VOJO>; MAKSAJ_DOSIERUJOJ],
    dosieroj_nombro: usize,
}

impl Manifesto {
    const fn malplena() -> Self {
        Self {
            nomo: Teksto::malplena(),
            versio: Teksto::malplena(),
            gu: Teksto::malplena(),
            fluo: Teksto::malplena(),
            arkoj: [Arko::X86_64; MAKSAJ_ARKOJ],
            arkoj_nombro: 0,
            fenestro: false,
            reto: false,
            dosieroj: [Teksto::malplena(); MAKSAJ_DOSIERUJOJ],
            dosieroj_nombro: 0,
        }
    }

    /// Analizas kaj validigas manifeston el teksto.
    ///
    /// # Eraroj
    ///
    /// La funkcio rifuzas sintaksan eraron, mankantan aŭ nekonatan kampon,
    /// nekonatan permeson aŭ sekcion, malkongruan `gu` aŭ `fluo`, nevalidan
    /// nomon, version aŭ vojon, tro longan valoron kaj miksitajn formojn.
    pub fn analizi(teksto: &str) -> Result<Self, Eraro> {
        let mut analizilo = Analizilo::nova();
        for linio in teksto.lines() {
            let linio = sen_komento(linio).trim();
            if linio.is_empty() {
                continue;
            }
            if linio.as_bytes()[0] == b'[' {
                analizilo.sekcio(linio)?;
            } else {
                analizilo.kampo(linio)?;
            }
        }
        analizilo.fini()
    }

    /// La aplikaka nomo.
    #[must_use]
    pub fn nomo(&self) -> &str {
        self.nomo.kiel_str()
    }

    /// La aplikaka versio.
    #[must_use]
    pub fn versio(&self) -> &str {
        self.versio.kiel_str()
    }

    /// La grafika marko; ĉiam `fluoU` por valida manifesto.
    #[must_use]
    pub fn gu(&self) -> &str {
        self.gu.kiel_str()
    }

    /// La deklarita FluoU-versio.
    #[must_use]
    pub fn fluo(&self) -> &str {
        self.fluo.kiel_str()
    }

    /// La deklaritaj arkoj.
    #[must_use]
    pub fn arkoj(&self) -> &[Arko] {
        let fino = self.arkoj_nombro.min(MAKSAJ_ARKOJ);
        &self.arkoj[..fino]
    }

    /// Ĉu la manifesto petas fenestran kapablon?
    #[must_use]
    pub const fn fenestro(&self) -> bool {
        self.fenestro
    }

    /// Ĉu la manifesto petas retan kapablon?
    #[must_use]
    pub const fn reto(&self) -> bool {
        self.reto
    }

    /// La nombro de dosierujaj permesoj.
    #[must_use]
    pub const fn dosierujoj_nombro(&self) -> usize {
        self.dosieroj_nombro
    }

    /// La dosieruja radiko ĉe la indekso, aŭ `None`.
    #[must_use]
    pub fn dosierujo(&self, indekso: usize) -> Option<&str> {
        if indekso < self.dosieroj.len() && indekso < self.dosieroj_nombro {
            Some(self.dosieroj[indekso].kiel_str())
        } else {
            None
        }
    }

    /// Validigas la konstruaĵon denove; servoj uzas ĉi tion antaŭ lanĉo.
    ///
    /// # Eraroj
    ///
    /// La funkcio rifuzas malplenan aŭ tro longan nomon, nevalidan version,
    /// malkongruan `gu` aŭ `fluo`, malplenan aŭ tro grandan arkan tabelon,
    /// duoblajn arkojn, tro multajn aŭ nevalidajn dosierujojn kaj duoblajn
    /// dosierujojn.
    pub fn validi(&self) -> Result<(), Eraro> {
        if self.nomo.kiel_str().is_empty() || !nomo_valida(self.nomo.kiel_str()) {
            return Err(Eraro::NevalidaNomo);
        }
        if !versio_valida(self.versio.kiel_str()) {
            return Err(Eraro::NevalidaVersio);
        }
        if self.gu.kiel_str() != GU_VALORO {
            return Err(Eraro::MalkongruaGu);
        }
        if self.fluo.kiel_str() != FLUO_VERSIO {
            return Err(Eraro::MalkongruaFluo);
        }
        if self.arkoj_nombro == 0 || self.arkoj_nombro > MAKSAJ_ARKOJ {
            return Err(Eraro::MankasKampo);
        }
        let mut indekso = 0;
        while indekso < self.arkoj_nombro {
            let mut alia = indekso + 1;
            while alia < self.arkoj_nombro {
                if self.arkoj[indekso] == self.arkoj[alia] {
                    return Err(Eraro::DuoblaKampo);
                }
                alia += 1;
            }
            indekso += 1;
        }
        if self.dosieroj_nombro > MAKSAJ_DOSIERUJOJ {
            return Err(Eraro::TroMultajDosierujoj);
        }
        indekso = 0;
        while indekso < self.dosieroj_nombro {
            let vojo = self.dosieroj[indekso].kiel_str();
            if !vojo_valida(vojo) {
                return Err(Eraro::NevalidaVojo);
            }
            let mut alia = indekso + 1;
            while alia < self.dosieroj_nombro {
                if vojo == self.dosieroj[alia].kiel_str() {
                    return Err(Eraro::DuoblaKampo);
                }
                alia += 1;
            }
            indekso += 1;
        }
        Ok(())
    }

    fn aldoni_arkon(&mut self, nomo: &str) -> Result<(), Eraro> {
        let arko = Arko::el_nomo(nomo).ok_or(Eraro::NekonataArko)?;
        let mut indekso = 0;
        while indekso < self.arkoj_nombro {
            if self.arkoj[indekso] == arko {
                return Err(Eraro::DuoblaKampo);
            }
            indekso += 1;
        }
        if self.arkoj_nombro == MAKSAJ_ARKOJ {
            return Err(Eraro::NevalidaValoro);
        }
        self.arkoj[self.arkoj_nombro] = arko;
        self.arkoj_nombro += 1;
        Ok(())
    }

    fn aldoni_dosierujon(&mut self, vojo: &str) -> Result<(), Eraro> {
        if !vojo_valida(vojo) {
            return Err(Eraro::NevalidaVojo);
        }
        let mut indekso = 0;
        while indekso < self.dosieroj_nombro {
            if self.dosieroj[indekso].kiel_str() == vojo {
                return Err(Eraro::DuoblaKampo);
            }
            indekso += 1;
        }
        if self.dosieroj_nombro == MAKSAJ_DOSIERUJOJ {
            return Err(Eraro::TroMultajDosierujoj);
        }
        self.dosieroj[self.dosieroj_nombro] = Teksto::nova(vojo)?;
        self.dosieroj_nombro += 1;
        Ok(())
    }
}

fn nomo_valida(nomo: &str) -> bool {
    !nomo.is_empty() && nomo.len() <= MAKSA_NOMO && !nomo.chars().any(char::is_control)
}

fn versio_valida(versio: &str) -> bool {
    let mut partoj = 0usize;
    let mut ciferoj = 0usize;
    for bajto in versio.bytes() {
        if bajto == b'.' {
            if ciferoj == 0 {
                return false;
            }
            partoj += 1;
            ciferoj = 0;
        } else if bajto.is_ascii_digit() {
            ciferoj += 1;
            if ciferoj > 6 {
                return false;
            }
        } else {
            return false;
        }
    }
    partoj == 2 && ciferoj > 0
}

fn vojo_valida(vojo: &str) -> bool {
    if vojo == "~" || vojo == "~/" {
        return true;
    }
    let Some(resto) = vojo.strip_prefix("~/") else {
        return false;
    };
    if resto.is_empty() {
        return true;
    }
    if resto.bytes().any(|b| b == 0 || b < 0x20) {
        return false;
    }
    let mut parto = 0usize;
    let mut indekso = 0usize;
    let bajtoj = resto.as_bytes();
    while indekso <= bajtoj.len() {
        if indekso == bajtoj.len() || bajtoj[indekso] == b'/' {
            let peco = &resto[parto..indekso];
            if peco.is_empty() || peco == "." || peco == ".." {
                return false;
            }
            parto = indekso + 1;
        }
        indekso += 1;
    }
    true
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Formo {
    Neniu,
    Plata,
    Sekcia,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Sekcio {
    Neniu,
    Aplikajxo,
    Permesoj,
}

const VIDITA_NOMO: u16 = 1 << 0;
const VIDITA_VERSIO: u16 = 1 << 1;
const VIDITA_GU: u16 = 1 << 2;
const VIDITA_FLUO: u16 = 1 << 3;
const VIDITA_ARKOJ: u16 = 1 << 4;
const VIDITA_PERMESOJ: u16 = 1 << 5;
const VIDITA_FENESTRO: u16 = 1 << 6;
const VIDITA_RETO: u16 = 1 << 7;
const VIDITA_DOSIEROJ: u16 = 1 << 8;
const DEVAJ_KAMPOJ: u16 = VIDITA_NOMO | VIDITA_VERSIO | VIDITA_GU | VIDITA_FLUO | VIDITA_ARKOJ;

struct Analizilo {
    m: Manifesto,
    formo: Formo,
    sekcio: Sekcio,
    viditaj: u16,
    aplikajxa_sekcio: bool,
    permesa_sekcio: bool,
}

impl Analizilo {
    fn nova() -> Self {
        Self {
            m: Manifesto::malplena(),
            formo: Formo::Neniu,
            sekcio: Sekcio::Neniu,
            viditaj: 0,
            aplikajxa_sekcio: false,
            permesa_sekcio: false,
        }
    }

    fn marki(&mut self, flago: u16) -> Result<(), Eraro> {
        if self.viditaj & flago != 0 {
            return Err(Eraro::DuoblaKampo);
        }
        self.viditaj |= flago;
        Ok(())
    }

    fn fini(self) -> Result<Manifesto, Eraro> {
        if self.viditaj & DEVAJ_KAMPOJ != DEVAJ_KAMPOJ {
            return Err(Eraro::MankasKampo);
        }
        let m = self.m;
        m.validi()?;
        Ok(m)
    }

    fn sekcio(&mut self, linio: &str) -> Result<(), Eraro> {
        if !linio.ends_with(']') || linio.len() < 3 {
            return Err(Eraro::NevalidaSintakso);
        }
        if self.formo == Formo::Plata {
            return Err(Eraro::MiksitajFormoj);
        }
        self.formo = Formo::Sekcia;
        let nomo = &linio[1..linio.len() - 1];
        match nomo {
            "aplikaĵo" => {
                if self.aplikajxa_sekcio {
                    return Err(Eraro::DuoblaKampo);
                }
                self.aplikajxa_sekcio = true;
                self.sekcio = Sekcio::Aplikajxo;
            }
            "permesoj" => {
                if self.permesa_sekcio {
                    return Err(Eraro::DuoblaKampo);
                }
                self.permesa_sekcio = true;
                self.sekcio = Sekcio::Permesoj;
            }
            _ => return Err(Eraro::NekonataSekcio),
        }
        Ok(())
    }

    fn kampo(&mut self, linio: &str) -> Result<(), Eraro> {
        let Some(posicio) = linio.find('=') else {
            return Err(Eraro::NevalidaSintakso);
        };
        let kampo = linio[..posicio].trim();
        let valoro = linio[posicio + 1..].trim();
        if !klavo_valida(kampo) || valoro.is_empty() {
            return Err(Eraro::NevalidaSintakso);
        }
        match self.sekcio {
            Sekcio::Neniu => {
                if self.formo == Formo::Sekcia {
                    return Err(Eraro::NevalidaSintakso);
                }
                self.formo = Formo::Plata;
                self.plata_kampo(kampo, valoro)
            }
            Sekcio::Aplikajxo => self.aplika_kampo(kampo, valoro),
            Sekcio::Permesoj => self.permesa_kampo(kampo, valoro),
        }
    }

    fn plata_kampo(&mut self, kampo: &str, valoro: &str) -> Result<(), Eraro> {
        if kampo == "permesoj" {
            self.marki(VIDITA_PERMESOJ)?;
            return self.permesoj_inline(valoro);
        }
        self.aplika_kampo(kampo, valoro)
    }

    fn aplika_kampo(&mut self, kampo: &str, valoro: &str) -> Result<(), Eraro> {
        match kampo {
            "nomo" => {
                self.marki(VIDITA_NOMO)?;
                let teksto = valora_ŝnuro(valoro)?;
                if !nomo_valida(teksto) {
                    return Err(Eraro::NevalidaNomo);
                }
                self.m.nomo = Teksto::nova(teksto)?;
            }
            "versio" => {
                self.marki(VIDITA_VERSIO)?;
                let teksto = valora_ŝnuro(valoro)?;
                if !versio_valida(teksto) {
                    return Err(Eraro::NevalidaVersio);
                }
                self.m.versio = Teksto::nova(teksto)?;
            }
            "gu" => {
                self.marki(VIDITA_GU)?;
                let teksto = valora_ŝnuro(valoro)?;
                if teksto != GU_VALORO {
                    return Err(Eraro::MalkongruaGu);
                }
                self.m.gu = Teksto::nova(teksto)?;
            }
            "fluo" => {
                self.marki(VIDITA_FLUO)?;
                let teksto = valora_ŝnuro(valoro)?;
                if teksto != FLUO_VERSIO {
                    return Err(Eraro::MalkongruaFluo);
                }
                self.m.fluo = Teksto::nova(teksto)?;
            }
            "arkoj" => {
                self.marki(VIDITA_ARKOJ)?;
                let mut legilo = Legilo::nova(valoro);
                legilo.listo(|nomo| self.m.aldoni_arkon(nomo))?;
                legilo.fino()?;
            }
            _ => return Err(Eraro::NekonataKampo),
        }
        Ok(())
    }

    fn permesa_kampo(&mut self, kampo: &str, valoro: &str) -> Result<(), Eraro> {
        match kampo {
            "fenestro" => {
                self.marki(VIDITA_FENESTRO)?;
                self.m.fenestro = valora_buleo(valoro)?;
            }
            "reto" => {
                self.marki(VIDITA_RETO)?;
                self.m.reto = valora_buleo(valoro)?;
            }
            "dosieroj" => {
                self.marki(VIDITA_DOSIEROJ)?;
                let mut legilo = Legilo::nova(valoro);
                legilo.listo(|vojo| self.m.aldoni_dosierujon(vojo))?;
                legilo.fino()?;
            }
            _ => return Err(Eraro::NekonataPermeso),
        }
        Ok(())
    }

    fn permesoj_inline(&mut self, valoro: &str) -> Result<(), Eraro> {
        let mut legilo = Legilo::nova(valoro);
        legilo.tabelo(|kampo, valoro| match kampo {
            "fenestro" => {
                self.marki(VIDITA_FENESTRO)?;
                self.m.fenestro = valoro.buleo()?;
                Ok(())
            }
            "reto" => {
                self.marki(VIDITA_RETO)?;
                self.m.reto = valoro.buleo()?;
                Ok(())
            }
            "dosieroj" => {
                self.marki(VIDITA_DOSIEROJ)?;
                valoro.listo(|vojo| self.m.aldoni_dosierujon(vojo))
            }
            _ => Err(Eraro::NekonataPermeso),
        })?;
        legilo.fino()
    }
}

fn klavo_valida(kampo: &str) -> bool {
    !kampo.is_empty()
        && kampo
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_')
}

fn sen_komento(linio: &str) -> &str {
    let mut en_ŝnuro = false;
    for (indekso, signo) in linio.char_indices() {
        match signo {
            '"' => en_ŝnuro = !en_ŝnuro,
            '#' if !en_ŝnuro => return &linio[..indekso],
            _ => {}
        }
    }
    linio
}

fn valora_ŝnuro(valoro: &str) -> Result<&str, Eraro> {
    let mut legilo = Legilo::nova(valoro);
    let teksto = legilo.ŝnuro()?;
    legilo.fino()?;
    Ok(teksto)
}

fn valora_buleo(valoro: &str) -> Result<bool, Eraro> {
    let mut legilo = Legilo::nova(valoro);
    let buleo = legilo.buleo()?;
    legilo.fino()?;
    Ok(buleo)
}

struct Legilo<'a> {
    teksto: &'a str,
    indekso: usize,
}

impl<'a> Legilo<'a> {
    fn nova(teksto: &'a str) -> Self {
        Self { teksto, indekso: 0 }
    }

    fn bajto(&self) -> Option<u8> {
        self.teksto.as_bytes().get(self.indekso).copied()
    }

    fn spacoj(&mut self) {
        while matches!(self.bajto(), Some(b' ' | b'\t')) {
            self.indekso += 1;
        }
    }

    fn preni(&mut self, atendita: u8) -> Result<(), Eraro> {
        self.spacoj();
        if self.bajto() == Some(atendita) {
            self.indekso += 1;
            Ok(())
        } else {
            Err(Eraro::NevalidaSintakso)
        }
    }

    fn preni_se(&mut self, atendita: u8) -> bool {
        self.spacoj();
        if self.bajto() == Some(atendita) {
            self.indekso += 1;
            true
        } else {
            false
        }
    }

    fn ŝnuro(&mut self) -> Result<&'a str, Eraro> {
        self.preni(b'"')?;
        let komenco = self.indekso;
        while let Some(bajto) = self.bajto() {
            if bajto == b'"' {
                let teksto = &self.teksto[komenco..self.indekso];
                self.indekso += 1;
                return Ok(teksto);
            }
            if bajto == b'\\' || bajto < 0x20 {
                return Err(Eraro::NevalidaSintakso);
            }
            self.indekso += 1;
        }
        Err(Eraro::NevalidaSintakso)
    }

    fn buleo(&mut self) -> Result<bool, Eraro> {
        self.spacoj();
        let komenco = self.indekso;
        while let Some(bajto) = self.bajto() {
            if bajto.is_ascii_alphabetic() {
                self.indekso += 1;
            } else {
                break;
            }
        }
        match &self.teksto[komenco..self.indekso] {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(Eraro::NevalidaValoro),
        }
    }

    fn listo<F>(&mut self, mut ago: F) -> Result<(), Eraro>
    where
        F: FnMut(&'a str) -> Result<(), Eraro>,
    {
        self.preni(b'[')?;
        loop {
            if self.preni_se(b']') {
                return Ok(());
            }
            let teksto = self.ŝnuro()?;
            ago(teksto)?;
            if self.preni_se(b',') {
                continue;
            }
            self.preni(b']')?;
            return Ok(());
        }
    }

    fn tabelo<F>(&mut self, mut ago: F) -> Result<(), Eraro>
    where
        F: FnMut(&'a str, &mut Legilo<'a>) -> Result<(), Eraro>,
    {
        self.preni(b'{')?;
        loop {
            if self.preni_se(b'}') {
                return Ok(());
            }
            let kampo = self.klavo()?;
            self.preni(b'=')?;
            ago(kampo, self)?;
            if self.preni_se(b',') {
                continue;
            }
            self.preni(b'}')?;
            return Ok(());
        }
    }

    fn klavo(&mut self) -> Result<&'a str, Eraro> {
        self.spacoj();
        let komenco = self.indekso;
        while let Some(bajto) = self.bajto() {
            if bajto.is_ascii_alphanumeric() || bajto == b'_' {
                self.indekso += 1;
            } else {
                break;
            }
        }
        if self.indekso == komenco {
            Err(Eraro::NevalidaSintakso)
        } else {
            Ok(&self.teksto[komenco..self.indekso])
        }
    }

    fn fino(&mut self) -> Result<(), Eraro> {
        self.spacoj();
        if self.indekso == self.teksto.len() {
            Ok(())
        } else {
            Err(Eraro::NevalidaSintakso)
        }
    }
}

#[cfg(kani)]
mod kani_pruvoj {
    //! Kani-pruvoj pri la manifestaj limoj (S16–S22).
    //!
    //! La pruvoj montras, ke la kopio neniam skribas ekster la fiksa bufero kaj
    //! ke la dosieruja tabelo respektas sian kapaciton. La kopio estas
    //! kontrolata por ĉiu longo; la tabelo por ĉiu permesata nombro.

    use super::{
        FLUO_VERSIO, GU_VALORO, MAKSA_NOMO, MAKSA_VOJO, MAKSAJ_DOSIERUJOJ, Manifesto, Teksto, kopii,
    };

    fn baza_manifesto() -> Manifesto {
        let mut m = Manifesto::malplena();
        if let Ok(nomo) = Teksto::<MAKSA_NOMO>::nova("Notilo") {
            m.nomo = nomo;
        }
        if let Ok(versio) = Teksto::<16>::nova("1.0.0") {
            m.versio = versio;
        }
        if let Ok(gu) = Teksto::<8>::nova(GU_VALORO) {
            m.gu = gu;
        }
        if let Ok(fluo) = Teksto::<8>::nova(FLUO_VERSIO) {
            m.fluo = fluo;
        }
        m.arkoj_nombro = 1;
        m
    }

    const VOJOJ: [&str; MAKSAJ_DOSIERUJOJ] =
        ["~/a", "~/b", "~/c", "~/d", "~/e", "~/f", "~/g", "~/h"];

    /// La kopio respektas la buferon por ĉiu longo ĝis unu pli ol la maksimumo.
    #[kani::proof]
    #[kani::unwind(66)]
    fn manifesta_kopio() {
        let fonto = [0xA5u8; MAKSA_NOMO + 1];
        let mut longo = 0;
        while longo <= MAKSA_NOMO + 1 {
            let mut celo = [0x5Au8; MAKSA_NOMO];
            let rezulto = kopii(&fonto[..longo], &mut celo, MAKSA_NOMO);
            if longo <= MAKSA_NOMO {
                assert!(rezulto.is_ok());
                assert_eq!(celo[..longo], fonto[..longo]);
                if longo < MAKSA_NOMO {
                    assert_eq!(celo[longo], 0x5A);
                }
            } else {
                assert!(rezulto.is_err());
                assert_eq!(celo, [0x5Au8; MAKSA_NOMO]);
            }
            longo += 1;
        }
    }

    /// Plena tabelo validas, troa nombro estas rifuzata, kaj plena tabelo
    /// rifuzas plian radikon sen ŝanĝi sian nombron.
    #[kani::proof]
    #[kani::unwind(12)]
    fn manifesta_kapacito() {
        let mut m = baza_manifesto();
        let mut indekso = 0;
        while indekso < MAKSAJ_DOSIERUJOJ {
            if let Ok(vojo) = Teksto::<MAKSA_VOJO>::nova(VOJOJ[indekso]) {
                m.dosieroj[indekso] = vojo;
            }
            indekso += 1;
        }

        let mut nombro = 0;
        while nombro <= MAKSAJ_DOSIERUJOJ {
            m.dosieroj_nombro = nombro;
            assert!(m.validi().is_ok());
            nombro += 1;
        }

        m.dosieroj_nombro = MAKSAJ_DOSIERUJOJ + 1;
        assert!(m.validi().is_err());

        m.dosieroj_nombro = MAKSAJ_DOSIERUJOJ;
        assert!(m.validi().is_ok());
        assert!(m.aldoni_dosierujon("~/i").is_err());
        assert_eq!(m.dosieroj_nombro, MAKSAJ_DOSIERUJOJ);
    }
}

#[cfg(test)]
mod testoj {
    use super::*;

    const PLATA: &str = "
nomo = \"dosierujo\"
versio = \"0.4.0\"
gu = \"fluoU\"
fluo = \"0.1\"
arkoj = [\"x86_64\", \"arm64\"]
permesoj = { fenestro = true, dosieroj = [\"~/\"], reto = false }
";

    const SEKCIA: &str = "
[aplikaĵo]
nomo = \"Notilo\"
versio = \"1.0.0\"
gu = \"fluoU\"
fluo = \"0.1\"
arkoj = [\"x86_64\", \"arm64\"]

[permesoj]
fenestro = true
dosieroj = [\"~/dokumentoj:legi\"]
reto = false
";

    #[test]
    fn plata_manifesto_estas_valida() {
        let m = Manifesto::analizi(PLATA).unwrap();
        assert_eq!(m.nomo(), "dosierujo");
        assert_eq!(m.versio(), "0.4.0");
        assert_eq!(m.gu(), "fluoU");
        assert_eq!(m.fluo(), "0.1");
        assert_eq!(m.arkoj(), &[Arko::X86_64, Arko::Arm64]);
        assert!(m.fenestro());
        assert!(!m.reto());
        assert_eq!(m.dosierujoj_nombro(), 1);
        assert_eq!(m.dosierujo(0), Some("~/"));
        assert_eq!(m.dosierujo(1), None);
    }

    #[test]
    fn sekcia_manifesto_estas_valida() {
        let m = Manifesto::analizi(SEKCIA).unwrap();
        assert_eq!(m.nomo(), "Notilo");
        assert_eq!(m.dosierujo(0), Some("~/dokumentoj:legi"));
    }

    #[test]
    fn permesoj_mankantaj_signifas_ne() {
        let teksto = "
nomo = \"X\"
versio = \"1.0.0\"
gu = \"fluoU\"
fluo = \"0.1\"
arkoj = [\"arm64\"]
";
        let m = Manifesto::analizi(teksto).unwrap();
        assert!(!m.fenestro());
        assert!(!m.reto());
        assert_eq!(m.dosierujoj_nombro(), 0);
    }

    #[test]
    fn devigaj_kampoj_mankas() {
        let teksto = "nomo = \"X\"\nversio = \"1.0.0\"\n";
        assert_eq!(Manifesto::analizi(teksto).err(), Some(Eraro::MankasKampo));
    }

    #[test]
    fn malkongrua_gu_kaj_fluo() {
        let gu = PLATA.replace("fluoU", "io");
        assert_eq!(Manifesto::analizi(&gu).err(), Some(Eraro::MalkongruaGu));
        let fluo = PLATA.replace("0.1", "0.2");
        assert_eq!(Manifesto::analizi(&fluo).err(), Some(Eraro::MalkongruaFluo));
    }

    #[test]
    fn nekonata_permeso_kaj_kampo() {
        let permeso = PLATA.replace("reto = false", "suno = true");
        assert_eq!(
            Manifesto::analizi(&permeso).err(),
            Some(Eraro::NekonataPermeso)
        );
        let kampo = PLATA.replace("versio = \"0.4.0\"", "versio = \"0.4.0\"\nkoloro = 1");
        assert_eq!(Manifesto::analizi(&kampo).err(), Some(Eraro::NekonataKampo));
    }

    #[test]
    fn nekonata_sekcio() {
        let teksto = "[agordoj]\n";
        assert_eq!(
            Manifesto::analizi(teksto).err(),
            Some(Eraro::NekonataSekcio)
        );
    }

    #[test]
    fn nekonata_arko_kaj_malplena_listo() {
        let arko = PLATA.replace("\"arm64\"", "\"mips\"");
        assert_eq!(Manifesto::analizi(&arko).err(), Some(Eraro::NekonataArko));
        let malplena = PLATA.replace("[\"x86_64\", \"arm64\"]", "[]");
        assert_eq!(
            Manifesto::analizi(&malplena).err(),
            Some(Eraro::MankasKampo)
        );
    }

    #[test]
    fn duoblaj_kampoj_kaj_arkoj() {
        let duobla = format!("{PLATA}\nnomo = \"Alia\"\n");
        assert_eq!(Manifesto::analizi(&duobla).err(), Some(Eraro::DuoblaKampo));
        let arkoj = PLATA.replace("[\"x86_64\", \"arm64\"]", "[\"arm64\", \"arm64\"]");
        assert_eq!(Manifesto::analizi(&arkoj).err(), Some(Eraro::DuoblaKampo));
    }

    #[test]
    fn nevalidaj_nomo_versio_kaj_vojo() {
        let nomo = PLATA.replace("nomo = \"dosierujo\"", "nomo = \"\"");
        assert_eq!(Manifesto::analizi(&nomo).err(), Some(Eraro::NevalidaNomo));
        let versio = PLATA.replace("versio = \"0.4.0\"", "versio = \"0.4\"");
        assert_eq!(
            Manifesto::analizi(&versio).err(),
            Some(Eraro::NevalidaVersio)
        );
        for vojo in ["/absoluta", "~/../sekreto", "~//duobla", "~/fin/"] {
            let teksto = PLATA.replace("[\"~/\"]", &format!("[\"{vojo}\"]"));
            assert_eq!(
                Manifesto::analizi(&teksto).err(),
                Some(Eraro::NevalidaVojo),
                "vojo: {vojo}"
            );
        }
    }

    #[test]
    fn tro_multaj_dosierujoj() {
        let listo =
            "[\"~/a\", \"~/b\", \"~/c\", \"~/d\", \"~/e\", \"~/f\", \"~/g\", \"~/h\", \"~/i\"]";
        let teksto = PLATA.replace("[\"~/\"]", listo);
        assert_eq!(
            Manifesto::analizi(&teksto).err(),
            Some(Eraro::TroMultajDosierujoj)
        );
    }

    #[test]
    fn miksitaj_formoj_kaj_sintaksoj() {
        let miksita = format!("{PLATA}\n[aplikaĵo]\n");
        assert_eq!(
            Manifesto::analizi(&miksita).err(),
            Some(Eraro::MiksitajFormoj)
        );
        assert_eq!(
            Manifesto::analizi("nomo = \"X\" sen egala signo").err(),
            Some(Eraro::NevalidaSintakso)
        );
        assert_eq!(
            Manifesto::analizi("nomo = \"nefinita").err(),
            Some(Eraro::NevalidaSintakso)
        );
    }

    #[test]
    fn komentoj_kaj_blankaj_linioj() {
        let teksto = "
# komento
nomo = \"X\" # malantaŭa komento
versio = \"1.0.0\"
gu = \"fluoU\"
fluo = \"0.1\"
arkoj = [\"x86_64\"]
";
        assert!(Manifesto::analizi(teksto).is_ok());
    }

    #[test]
    fn nekonata_bulea_valoro() {
        let teksto = PLATA.replace("fenestro = true", "fenestro = jes");
        assert_eq!(
            Manifesto::analizi(&teksto).err(),
            Some(Eraro::NevalidaValoro)
        );
    }
}
