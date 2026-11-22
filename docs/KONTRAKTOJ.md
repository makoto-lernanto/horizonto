# Horizonto — kontraktoj de H1

> **Versio:** 0.1.0 (Cargo-laborloko) · **Fazo:** H1 «Kontraktoj kaj skeleto»
> **Normaj fontoj:** [`HORIZONTO.md`](HORIZONTO.md) §3–§7,
> `../mikerno/docs/FLUOU.md` §9.1, `../mikerno/docs/MEJLOSTONOJ.md` §9.1–§9.3.
>
> Ĉi tiu dokumento fiksas la servan modelon, la fenestran protokolon, la
> enigajn eventojn, la manifestan formon kaj la petojn al MiKerno. La
> efektiva kodo estas `libhorizonto`; la `const`-asertoj en
> `libhorizonto/src/asertoj.rs` gardas la aranĝojn.

## 1. Serva modelo

La sesio estas la sola procezo, kiu rajtas lanĉi aliajn procezojn. Ĝi disdonas
kapablojn laŭ la manifesto kaj la aprobo de la uzanto. La starta fluo estas:

```text
kerno
  └─ sesio (Ring 3; kapabloj: procezoj, IPC, dosierujoj, frambufo)
       ├─ kompozitoro (kapablo: frambufo; la sola posedanto)
       │    ├─ eliga surfaco
       │    ├─ fenestraj petoj kaj fokuso
       │    └─ komponaĵo en la frambufon
       ├─ ligo (fenestra kapablo, dosieroj laŭ la ŝelo)
       ├─ eniro (supermetita paletro, ne aparta fenestro)
       └─ aplikaĵoj (laŭ la manifestoj)
```

H1 fiksas nur la kontraktojn kaj la purajn tipojn. Neniu servo ankoraŭ
ruliĝas; la sesio kaj la kompozitoro venas en H2. Aplikaĵo **DEVI NE** povas
lanĉi procezon sen kapablo donita de la sesio.

| Servo | Kesto | Tasko | Unua Fazo |
| --- | --- | --- | --- |
| `sesio` | `sesio` | Iniciativo, procezoj, permesoj, pakaĵoj | H2 |
| `kompozitoro` | `kompozitoro` | Frambufo, fenestroj, fokuso, enigo | H2 |
| `ligo` | `ligo` | Ŝelo en fenestro; tekstaj komandoj | H4 |
| `eniro` | `eniro` | Komanda paletro `Ctrl+E` | H4 |
| `dosierujo` | `dosierujo` | Dosiera administrado | H4 |
| `redaktilo` | `redaktilo` | Teksta redaktilo | H4 |
| `agordoj` | `agordoj` | Temo, movo, lingvo, reto | H4, H6 |
| `pakilo` | `pakilo` | Instalo, forigo, ĝisdatigo | H5 |
| `retagento` | `retagento` | DHCP, DNS, UDP/TCP, ĝisdatigoj | H7 |
| `legilo` | `legilo` | ODF-legado | H8 |

## 2. Kapabloj

| ID | Regulo |
| --- | --- |
| H01 | Aplikaĵo **DEVI NE** ricevi rektan kapablon al la frambufo. |
| H02 | Aplikaĵo ricevas nur sian surfacon, siajn IPC-finpunktojn kaj la aprobitajn dosierujajn radikojn. |
| H03 | La kompozitoro estas la sola posedanto de la frambufo. |
| H04 | La sesio disdonas kapablojn; ĝi **DEVI NE** disdonu al procezo sen valida manifesto. |
| H05 | Kapablo estas transdonita, ne eltrovita; neniu defaŭlta aliro ekzistas. |
| H06 | Aplikaĵo **DEVI NE** povas legi la enigon de alia fenestro. |
| H07 | Aplikaĵo **DEVI NE** povas skribi en la surfacon de alia fenestro. |
| H08 | Paneo de aplikaĵo **DEVI** mortigi nur tiun procezon. |
| H09 | Horizonto **DEVI NE** malfermas la kernon per rekta aparataro, identa mapado aŭ kaŝita `unsafe`. |
| H10 | Ĉiu vidaĵo uzas FluoU-ĵetonojn kaj Lumo-ordojn. |
| H11 | La fonta tavolo estas Esperanta; tradukoj loĝas en apartaj mapadaj dosieroj. |
| H12 | Neniu C, neniu C++, neniu ekstera binara dependaĵo. |
| H13 | La kompozitoro uzas entjeran aritmetikon; glitkomo en la kompona vojo estas malpermesita. |
| H14 | Protokolo estas versiita, fiksa kaj finiebla; nekonata kampo aŭ versio estas rifuzata. |
| H15 | Unu Fazo samtempe; la sekva Fazo atendas verdan pordegon kaj permeson. |
| H16 | `libhorizonto` restas sen `alloc` kaj sen `unsafe`. |

## 3. Fenestra protokolo

### 3.1 Ĝeneralaj konstantoj

| Konstanto | Valoro | Signifo |
| --- | --- | --- |
| `PROTOKOLA_VERSIO` | `1` | La nuna protokola versio (u16). |
| `KAPA_LONGO` | `8` | La fiksa kapo en bajtoj. |
| `MAKSA_MESAĜA_LONGO` | `64` | La maksimuma tuta mesaĝo. |
| `MAKSA_DATUMO` | `56` | La maksimuma datumaro (`64 − 8`). |
| `MAKSAJ_FENESTROJ` | `64` | La maksimuma fenestra indekso. |
| `MAKSAJ_DAMAĜOJ` | `32` | La kapacito de unu damaĝaro. |
| `MAKSA_SURFACA_DIMENSIO` | `4096` | La maksimuma surfaca larĝo aŭ alto. |

Ĉiuj entjeroj estas little-endian. La legado kaj skribado uzas la garditajn
helpilojn en `libhorizonto/src/protokolo.rs`; neniu strukturo estas rekte
transĵetita al bajta bufro.

### 3.2 Kapo kaj mesaĝaj tipoj

| Deŝovo | Grando | Kampo | Tipo |
| --- | --- | --- | --- |
| 0 | 2 | `versio` | u16 |
| 2 | 2 | `tipo` | u16 |
| 4 | 4 | `longo` | u32 |

`longo` estas la longo de la tuta mesaĝo, inkluzive la kapon; ĝi estas
`8 ≤ longo ≤ 64` kaj ne pli granda ol la bufro. Grando de `Kapo` estas 8,
ĝisrandigo 4.

| Numero | Tipo | Direkto |
| --- | --- | --- |
| 1 | `Krei` | peto |
| 2 | `Monstri` | peto |
| 3 | `Kaŝi` | peto |
| 4 | `Fermi` | peto |
| 5 | `Movi` | peto |
| 6 | `Grandigi` | peto |
| 7 | `Fokusi` | peto |
| 8 | `DifiniTitolon` | peto |
| 9 | `DifiniDamaĝon` | peto |
| 16 | `Kreita` | evento |
| 17 | `Fermita` | evento |
| 18 | `Fokusita` | evento |
| 19 | `Enigo` | evento |
| 20 | `Paneita` | evento |

Nekonata versio, tipo aŭ longo estas aparta `Eraro`; la ricevanto restas viva.
La legado estas totala por ĉiu enigo (S04) kaj estas pruvita per Kani.

### 3.3 Fenestra identigilo

| Deŝovo | Grando | Kampo | Tipo |
| --- | --- | --- | --- |
| 0 | 4 | `indekso` | u32 |
| 4 | 4 | `generacio` | u32 |

Grando 8, ĝisrandigo 4. `indekso < 64`; `generacio ≥ 1`. Reuzo de indekso
postulas novan generacion; `sekva_generacio` rifuzas elĉerpiĝon de `u32::MAX`.
Malnova identigilo estas rifuzata de `akceptas`.

### 3.4 Surfaco kaj damaĝo

| Deŝovo | Grando | Kampo | Tipo |
| --- | --- | --- | --- |
| 0 | 4 | `x` | i32 |
| 4 | 4 | `y` | i32 |
| 8 | 4 | `larĝo` | u32 |
| 12 | 4 | `alto` | u32 |

`Ortangulo` havas grandon 16 kaj ĝisrandigon 4; nula larĝo aŭ alto estas
nevalida. La koordinatoj estas relativaj al la surfaco.

| Deŝovo | Grando | Kampo | Tipo |
| --- | --- | --- | --- |
| 0 | 4 | `larĝo` | u32 |
| 4 | 4 | `alto` | u32 |
| 8 | 4 | `paŝo` | u32 |

`SurfacaPriskribo` havas grandon 12. `1 ≤ larĝo, alto ≤ 4096`; `paŝo` estas
multipliko de 4 kaj ne malpli ol `larĝo × 4`. Surfaco estas `ARGB8888`,
antaŭmiksita, unu vico laŭ pikseloj.

`Damaĝaro<N>` estas fiksa tabelo de ortanguloj. `aldonu` kunfandas ĉiun
intersekcantan ortangulon ĝis la fiksa punkto; `aldonu_en` rifuzas ortangulon,
kiu ne entute eniĝas la surfacon. Plena damaĝaro redonas `PlenaDamaĝaro`
anstataŭ perdi damaĝon. La kompozitoro legu nur la markitajn regionojn (D02);
la protokolo ne portas krudan frambufan kapablon (S10).

## 4. Enigo

| Deŝovo | Grando | Kampo | Tipo |
| --- | --- | --- | --- |
| 0 | 2 | `speco` | u16 |
| 2 | 2 | `modifiloj` | u16 |
| 4 | 4 | `klavo` | u32 |
| 8 | 4 | `x` | i32 |
| 12 | 4 | `y` | i32 |
| 16 | 1 | `butonoj` | u8 |
| 17 | 1 | `stato` | u8 |
| 18 | 2 | `rezervo` | [u8; 2] |

Grando 20, ĝisrandigo 4. `speco` estas `1` (klavo) aŭ `2` (muso).
`modifiloj` permesas nur `ŜOVO = 1`, `CTRL = 2`, `ALT = 4`, `SUPER = 8`.
`butonoj` permesas nur `MALDEKSTRA = 1`, `MEZA = 2`, `DEKSTRA = 4`; la stato
estas `0` (liberigita) aŭ `1` (premita). `rezervo` devas esti nulo. Klava
evento havas nulajn musajn kampojn; musa evento havas nulan `klavon` kaj
koheran `stato`-n. Nekonata bito aŭ nekohera kampo estas rifuzataj (H14).

Klavaraj eventoj iras nur al la fokusita fenestro; musaj eventoj al la
fenestro sub la montrilo, krom dum trenado (S11–S15). Aplikaĵo neniam ricevas
tutmondan enigan fluon.

## 5. Manifesto

La manifesta formo sekvas `FLUOU.md` §9.1. Akceptiĝas la plata formo de
`HORIZONTO.md` §4.4 kaj la sekcia formo de `FLUOU.md`; en unu dosiero oni uzu
nur unu formon.

```toml
nomo = "dosierujo"
versio = "0.4.0"
gu = "fluoU"
fluo = "0.1"
arkoj = ["x86_64", "arm64"]
permesoj = { fenestro = true, dosieroj = ["~/"], reto = false }
```

```toml
[aplikaĵo]
nomo = "Notilo"
versio = "1.0.0"
gu = "fluoU"
fluo = "0.1"
arkoj = ["x86_64", "arm64"]

[permesoj]
fenestro = true
dosieroj = ["~/dokumentoj:legi"]
reto = false
```

Devigaj kampoj: `nomo`, `versio`, `gu`, `fluo`, `arkoj`. `gu` **DEVI** esti
`"fluoU"`; `fluo` **DEVI** esti la subtenata `0.1`; `arkoj` enhavas almenaŭ
unu el `"x86_64"` kaj `"arm64"`, sen duoblaĵoj. La nomo estas ne malplena,
maksimume 64 bajtoj kaj sen regsignoj; la versio estas `Ĉefo.Malĉefo.Peceto`
kun ĝis ses ciferoj po parto.

| Permeso | Tipo | Defaŭlto | Signifo |
| --- | --- | --- | --- |
| `fenestro` | buleo | `false` | Rajtas ricevi `FENESTRO`-tenilon per `DONI`. |
| `reto` | buleo | `false` | Rajtas ricevi la retan kapablon en H7. |
| `dosieroj` | tabelo de ĉenoj | malplena | Radikoj, kiujn la uzanto povas aprobi (H6). |

Mankanta permeso signifas «ne» (S18). Nekonata permeso aŭ kampo estas
rifuzata (S17). Dosieruja radiko komenciĝas per `~` aŭ `~/`; absolutaj vojoj,
`..`, malplenaj segmentoj kaj regsignoj estas rifuzataj. La supralimo estas
ok radikoj po 64 bajtoj. La sufikso `:legi` aŭ `:skribi` el `FLUOU.md` restas
parto de la radika ĉeno en H1; la alirmanieroj efektiviĝas en H6, ne per
kaŝita malfermo.

La analizilo estas pura funkcio sen `alloc`: ĝi akceptas unu kampon en unu
linio, ĉenojn sen eskapaĵoj, buleojn kaj tabelojn de ĉenoj. Komentoj kaj
blankaj linioj estas ignorataj. Duobla kampo, sekcio aŭ dosierujo estas
eraro. `Manifesto::validi` revalidigas la konstruaĵon antaŭ lanĉo.

## 6. Petoj al MiKerno

La petoj estas antaŭkondiĉoj de la Fazoj; ilia efektivigo, pruvoj kaj
dokumentaro apartenas al MiKerno. H1 bezonas neniun el ili.

| Peto | Bezono | Uzata de | Fazo |
| --- | --- | --- | --- |
| K1 | Kunhava memoro / `SYS_MMAP` kun kapablo | H2 | H2 |
| K2 | Frambufa kapablo kaj reĝima agordo | H2 | H2 |
| K3 | Restarigo post Ring 3-paneo | H2 | H2 |
| K4 | Enigaj eventoj (klavaro, muso) | H3 | H3 |
| K5 | Reta pelilo kaj kapablo | H7 | H7 |
| K6 | Legado de tempo / RTC | H7 | H7 |
| K7 | Dosierujaj kapabloj kun radika limo | H6 | H6 |
| K8 | Kalkulado de procezoj kaj resursoj | H2 | H2 |
| K9 | Dinamika ligilo | post 1.0 | post 1.0 |
| K10 | `fork` / COW | post 1.0 | post 1.0 |
| K11 | GPU-akcelo (`virtio-gpu`) | post 1.0 | post 1.0 |

Se peto mankas, la Fazo estas blokita: oni ne ŝajnigas ĝin per rekta
aparataro, identa mapado aŭ senposeda `unsafe`.

## 7. Versia kongruo

`PROTOKOLA_VERSIO = 1`, `FLUO_VERSIO = "0.1"` kaj `MIKERNO_ABI = 2` estas la
solaj deklaritaj versioj en `libhorizonto`. La protokola versio kaj la
manifesta `fluo` estas malsamaj nomspacoj, sed ili priskribas la saman
kontrakton; `Manifesto::validi` rifuzas alian valoron anstataŭ adapti ĝin
(Q15). H1 ankoraŭ ne vokas `libmikerno`, do `MIKERNO_ABI` estas antaŭa
deklaro por H2; kiam la unua voko alvenos, la pordego kontrolos ĝin kontraŭ
la efektiva ABI.

## 8. Kontrolo

- `cargo test --workspace` kovras validajn kaj malvalidajn manifestojn,
  nekonatan permeson, malkongruan `fluo`, mallongan mesaĝon, nekonatan tipon
  kaj generacian reuzon.
- `docs/PRUVOJ.md` listigas la Kani-pruvojn; ĉiu havas propran DI-taskon.
- `tools/sen-glitkomo.sh` kaj `tools/sen-cina.sh` gardas la lingvajn kaj
  aritmetikajn regulojn; `tools/versio.sh --kontroli` gardas la version.
