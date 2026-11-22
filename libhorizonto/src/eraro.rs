//! La komunaj eraroj de `libhorizonto`.
//!
//! La eraroj estas malgrandaj, kunigeblaj kaj sen teksto, do ili povas uziĝi
//! en `no_std`-kodo sen `alloc`. La teksta priskribo venas de [`Eraro`] mem.

/// Eraro de protokolo, manifesta analizo aŭ geometria regulo.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eraro {
    /// Bufro estas pli mallonga ol la postulata kapo aŭ strukturo.
    MalgrandaMesaĝo,
    /// La mesaĝo superas la fiksan supran limon de 64 bajtoj.
    TroLongaMesaĝo,
    /// La protokola versio ne estas la subtenata.
    NekonataProtokolaVersio,
    /// La mesaĝa tipo ne apartenas al la fiksa tabelo.
    NekonataMesaĝoTipo,
    /// Ortangulo havas nulan grandon aŭ ne estas ene de la surfaco.
    NevalidaOrtangulo,
    /// La damaĝaro jam plenas kaj la nova ortangulo ne kunfandiĝas.
    PlenaDamaĝaro,
    /// La fenestra indekso estas ekster la permesata tabelo.
    NevalidaFenestraIndekso,
    /// Generacio nulo ne estas permesata; generacioj ekas ĉe unu.
    NevalidaGeneracio,
    /// La generacio atingis sian supran limon kaj la indekso ne reuzeblas.
    GeneracioElĉerpita,
    /// La surfaca priskribo malobservas D03 aŭ D04.
    NevalidaSurfaco,
    /// La eniga evento havas nekonatan specon aŭ nevalidan kampon.
    NevalidaEnigo,
    /// Manifesto mankas devigan kampon.
    MankasKampo,
    /// Manifesto enhavas nekonatan kampon.
    NekonataKampo,
    /// Manifesto enhavas nekonatan sekcion.
    NekonataSekcio,
    /// Manifesto enhavas nekonatan permeson.
    NekonataPermeso,
    /// Manifesto enhavas nekonatan arkon.
    NekonataArko,
    /// La kampo `gu` ne estas `fluoU`.
    MalkongruaGu,
    /// La kampo `fluo` ne kongruas kun la subtenata FluoU-versio.
    MalkongruaFluo,
    /// La versio ne havas la formon `ĉefo.minoro.riparo`.
    NevalidaVersio,
    /// La nomo estas malplena aŭ enhavas regsignon.
    NevalidaNomo,
    /// La dosieruja permeso ne estas `~`-radika kaj sen `..`.
    NevalidaVojo,
    /// Valoro superas la fiksan buferon de la manifesto.
    TroLongaValoro,
    /// La dosierujaj permesoj superas la fiksan supran limon.
    TroMultajDosierujoj,
    /// La sama kampo aperas dufoje.
    DuoblaKampo,
    /// Plata kaj sekcia manifestaj formoj estas miksitaj.
    MiksitajFormoj,
    /// La linio ne estas valida TOML-subaro.
    NevalidaSintakso,
    /// La valoro ne havas la atenditan tipon aŭ formon.
    NevalidaValoro,
}

impl core::fmt::Display for Eraro {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let teksto = match self {
            Self::MalgrandaMesaĝo => "bufro tro mallonga",
            Self::TroLongaMesaĝo => "mesaĝo tro longa",
            Self::NekonataProtokolaVersio => "nekonata protokola versio",
            Self::NekonataMesaĝoTipo => "nekonata mesaĝa tipo",
            Self::NevalidaOrtangulo => "nevalida ortangulo",
            Self::PlenaDamaĝaro => "plena damaĝaro",
            Self::NevalidaFenestraIndekso => "nevalida fenestra indekso",
            Self::NevalidaGeneracio => "nevalida generacio",
            Self::GeneracioElĉerpita => "generacio elĉerpita",
            Self::NevalidaSurfaco => "nevalida surfaco",
            Self::NevalidaEnigo => "nevalida eniga evento",
            Self::MankasKampo => "mankas deviga kampo",
            Self::NekonataKampo => "nekonata kampo",
            Self::NekonataSekcio => "nekonata sekcio",
            Self::NekonataPermeso => "nekonata permeso",
            Self::NekonataArko => "nekonata arko",
            Self::MalkongruaGu => "gu ne estas fluoU",
            Self::MalkongruaFluo => "fluo ne kongruas",
            Self::NevalidaVersio => "nevalida versio",
            Self::NevalidaNomo => "nevalida nomo",
            Self::NevalidaVojo => "nevalida dosieruja vojo",
            Self::TroLongaValoro => "valoro tro longa",
            Self::TroMultajDosierujoj => "tro multaj dosierujoj",
            Self::DuoblaKampo => "duobla kampo",
            Self::MiksitajFormoj => "miksitaj manifestaj formoj",
            Self::NevalidaSintakso => "nevalida sintakso",
            Self::NevalidaValoro => "nevalida valoro",
        };
        f.write_str(teksto)
    }
}

impl core::error::Error for Eraro {}
