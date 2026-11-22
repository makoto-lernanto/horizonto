# Horizonto — normiga priskribo

> **Stato:** skeleto `0.1.0` · **Celo:** `1.0.0` · **Licenco:** MIT
> **Dato:** 2026-11-21 · **Deponejo:** `~/horizonto`
>
> Ĉi tiu dokumento estas la normo por la Horizonto-sistemo. La kvalita
> konstitucio de MiKerno restas la supera normo por la tuta projekto; Horizonto
> ĝin spegulas kaj aldonas la postulojn de servoj, fenestra protokolo kaj
> izolado.
>
> Devigaj fontoj:
>
> - `../mikerno/docs/MEJLOSTONOJ.md` §9.1–§9.3 (Kvalita Konstitucio), §4.7–§4.8
>   (nuba DI kaj la matrico de Kani), §8 (Difino de «1.0»).
> - `../mikerno/AGENTS.md` (lingva regulo, laborfluo, komitaj reguloj).
> - `../mikerno/docs/FLUOU.md` (la grafika kontrakto; normo por ĉio videbla).
> - `../lumo/docs/LUMO.md` (la rastrumilo; normo por pikseloj).
>
> Se du dokumentoj kontraŭdiras: por kvalito kaj DI venkas la MiKerno-dokumento;
> por videbla formo venkas `FLUOU.md`; por servoj, protokoloj kaj izolado venkas
> ĉi tiu dokumento. Kontraŭdiro estas cimo, ne bagatelo: ĝi devas esti
> registrita kaj riparita antaŭ la venonta Fazo.

## 0. Normaj vortoj

- **DEVI** — deviga; malobservo blokas la Fazon.
- **DEVI NE** — malpermesita.
- **DEVUS** — forta rekomendo; deflankiĝo bezonas skriban kialon.
- **POVAS** — permesata elekto.
- **H** — principo de la sistemo.
- **S** — regulo pri servoj kaj protokoloj.
- **D** — regulo pri determinismo kaj izolado.
- **Q** — regulo de la kvalita konstitucio.
- **K** — peto al MiKerno.
- **R** — rendimenta buĝeto.

Ĉiu regulo havas stabilan identigilon. Regulo ne malaperas; se ĝi ŝanĝiĝas,
la malnova identigilo restas kun «anstataŭigita de …» kaj la historia kialo.

## 1. Celaro kaj limoj

Horizonto estas la **uzantlanda sistemo** super MiKerno. Ĝi posedas la politikon
de la labirtablo: kiu rajtas vidi kion, kiu ricevas enigon, kiel aplikaĵo
startas kaj finiĝas, kiel pakaĵo instaliĝas, kiel la ŝelo kaj la unuaj aplikaĵoj
aspektas. Ĝi **ne** posedas la mekanismojn de la kerno kaj **ne** difinas la
grafikan kontrakton.

### 1.1 Kion Horizonto posedas

- Sesion kaj vivciklon de procezoj (`sesio`).
- Kompozitoron, fenestran protokolon, fokuson kaj eniganvojigon.
- Ŝelon `Ligo`, la paletron `Eniro` kaj la unuajn aplikaĵojn.
- Manifestojn, permesojn, SDK, pakaĵojn, instalilon kaj ĝisdatigojn.
- Uzantlandan retan stakon kaj la politikon de retpermesoj.
- La dokumentan legilon kaj la vojon al pli posta oficprogramaro.

### 1.2 Kion Horizonto ne posedas

- Paĝadon, procezan MSP, IPC, kapablojn, interrompojn kaj pelilojn — tiuj
  restas en MiKerno.
- Ĵetonojn, komponantojn, manifestan formon kaj la API-formon de FluoU — tiuj
  restas en `FLUOU.md`.
- Miksadon, rastrumadon, tiparojn kaj la ordonaron — tiuj restas en Lumo.
- La kernan retan stakon: la kerno liveras nur la pelilon kaj la kapablon; la
  protokoloj loĝas en la uzantlando.

### 1.3 Difino de «1.0» por Horizonto

Horizonto `1.0.0` estas preta kiam:

1. Sesio startas kompozitoron kaj la ŝelon sen permana helpo.
2. Pluraj fenestroj funkcias kun fokuso, enigo kaj FluoU-titola kadro.
3. Aplikaĵo povas esti konstruita per la SDK, instalita, lanĉita kaj forigita.
4. Aplikaĵo paneanta ne faligas la sesion kaj ne tuŝas aliajn fenestrojn.
5. La sama scenaro donas la saman pikselan haŝon sur `x86_64` kaj `arm64`.
6. La protokolo, la manifesta formo kaj la SDK estas frostigitaj.
7. La tuta pordego de §6 estas verda, inkluzive de la Kani-matrico, Miri,
   permesiloj, paritateco kaj izolado.
8. La dokumentaro permesas al nova programisto konstrui kaj instali aplikaĵon
   sen legi la kernon.

## 2. Rilato al MiKerno, FluoU kaj Lumo

```
            +-----------------------------+
            |        Ekosistemo           |
            |  dokumentoj, reto, eldonoj  |
            +--------------+--------------+
                           |
            +--------------v--------------+
            |          Horizonto          |
            | sesio, kompozitoro, Ligo,   |
            | aplikaĵoj, SDK, pakaĵoj     |
            +----+------------+-----------+
                 |            |
      +----------v--+      +--v---------+
      |   FluoU     |      |   Lumo     |
      | kontrakto   |      | rastrumilo |
      +-------------+      +------------+
                 \            /
            +-----v----------v-----+
            |       MiKerno        |
            | mekanismoj, peliloj  |
            +----------------------+
```

Dependodirekto estas unudirekta: Horizonto dependas de `libmikerno`, de la
FluoU-kontrakto kaj de Lumo. MiKerno, FluoU kaj Lumo **DEVI NE** dependi de
Horizonto.

| Demando | Respondo |
| --- | --- |
| Kiu posedas la frambufon? | La kompozitoro, per kapablo de la kerno. |
| Kiu rastrumas fenestran enhavon? | La aplikaĵo, per FluoU + Lumo, en sian propran surfacon. |
| Kiu rastrumas la titolan kadron kaj la finan komponaĵon? | La kompozitoro, per Lumo. |
| Kiu scias la fokuson? | La kompozitoro; la sesio nur lanĉas kaj permesas. |
| Kiu legas la klavaron? | La kerno liveras eventojn; la kompozitoro direktas ilin. |
| Kiu instalas pakaĵojn? | La sesio, laŭ subskribita manifesta kaj pakaĵa politiko. |

## 3. Arkitekturo

### 3.1 Starto

```
kerno
  └─ lanĉas sesio (Ring 3, kapabloj: procezoj, IPC, dosierujoj, frambufo)
       ├─ lanĉas kompozitoro (kapablo: frambufo; rajtas mapi ĝin)
       │    ├─ kreas eligan surfacon
       │    ├─ akceptas fenestrajn petojn
       │    └─ komponas en la frambufon
       ├─ lanĉas ligo (kapablo: fenestro, dosieroj laŭ la ŝelo)
       ├─ lanĉas eniro (supermetita paletro, ne aparta fenestro)
       └─ lanĉas aplikaĵojn laŭ manifestoj
```

La sesio estas la sola procezo rajtigita lanĉi aliajn procezojn. Aplikaĵo
**DEVI NE** povas lanĉi alian procezon sen kapablo donita de la sesio.

### 3.2 Servoj

| Servo | Crates | Tasko | Unua fazo |
| --- | --- | --- | --- |
| `sesio` | `sesio` | Iniciativo, procezoj, permesoj, pakaĵoj | H1 (kontrakto), H2 (starto) |
| `kompozitoro` | `kompozitoro` | Frambufo, fenestroj, fokuso, enigo, komponaĵo | H2 |
| `ligo` | `ligo` | Ŝelo en fenestro; tekstaj komandoj | H4 |
| `eniro` | `eniro` | Komanda paletro `Ctrl+E` | H4 |
| `dosierujo` | `dosierujo` | Dosiera administrado | H4 |
| `redaktilo` | `redaktilo` | Teksta redaktilo | H4 |
| `agordoj` | `agordoj` | Temo, movo, lingvo, reto | H4, H6 |
| `pakilo` | `pakilo` | Instalo, forigo, ĝisdatigo | H5 |
| `retagento` | `retagento` | DHCP, DNS, UDP/TCP, ĝisdatigoj | H7 |
| `legilo` | `legilo` | ODF-legado | H8 |

La nomoj estas rezervitaj; ili povas ŝanĝiĝi nur per revizio de ĉi tiu tabelo
kaj de la vortaro.

### 3.3 Kapabloj

- **H01** — Aplikaĵo **DEVI NE** ricevi rektan kapablon al la frambufo.
- **H02** — Aplikaĵo ricevas nur sian propran surfacon, siajn IPC-finpunktojn
  kaj la dosierujajn radikojn, kiujn ĝi deklaris kaj la uzanto aprobis.
- **H03** — La kompozitoro estas la sola posedanto de la frambufo.
- **H04** — La sesio disdonas kapablojn; ĝi **DEVI NE** disdonu kapablon al
  procezo, kiu ne havas validan manifeston.
- **H05** — Kapablo estas transdonita, ne eltrovita. Neniu «defaŭlta» aliro
  ekzistas.
- **H06** — Aplikaĵo **DEVI NE** povas legi la enigon de alia fenestro.
- **H07** — Aplikaĵo **DEVI NE** povas skribi en la surfacon de alia fenestro.
- **H08** — Paneo de aplikaĵo **DEVI** mortigi nur tiun procezon; la sesio kaj
  la kompozitoro restas vivaj.
- **H09** — Horizonto **DEVI NE** malfermas la kernon per rekta aparataro,
  identa mapado aŭ kaŝita `unsafe`. Se mekanismo mankas, ĝi petas ĝin de
  MiKerno (vidu §7) kaj atendas.
- **H10** — Ĉiu vidaĵo uzas FluoU-ĵetonojn kaj Lumo-ordojn; aplikaĵo **DEVI
  NE** desegnas per propra piksela buklo ekster Lumo.
- **H11** — La fonta tavolo estas Esperanta. Tradukoj loĝas en apartaj mapadaj
  dosieroj (`tradukoj/`); ili **DEVI NE** ŝanĝi identigilojn, protokolajn nomojn
  aŭ komandnomojn kaj **DEVI NE** eniri la fontan tavolon.
- **H12** — Neniu C, neniu C++, neniu ekstera binara dependaĵo. La uzantlando
  rajtas uzi reviziitajn purajn Rust-kratajn (Q08).
- **H13** — La kompozitoro uzas nur entjeran aritmetikon por geometrio kaj
  miksado. Glitkomo en la kompona vojo estas malpermesita.
- **H14** — Protokolo estas versiita, fiksa kaj finiebla. Nekonata kampo aŭ
  versio estas rifuzata, ne ignorata.
- **H15** — Unu Fazo samtempe. Ne komencu la sekvan antaŭ ol la nuna pordego
  estas verda kaj la uzanto permesas.
- **H16** — `libhorizonto` restas sen `alloc` kaj sen `unsafe`. Servoj kaj
  aplikaĵoj **POVAS** uzi `alloc` per unu reviziita alokilo super la uzanta
  kupo (K1), kun sekurec-kontrakto; neniu alia `unsafe` estas permesata.

### 3.4 Rendimentaj buĝetoj

La buĝetoj estas mezurataj en la referenca QEMU-medio (programa rastrumado,
1280 × 800) per fiksa scenaro kaj registrataj en `docs/RENDIMENTO.md`.

- **R01** — Eniga latenteco (klavo ĝis videbla ŝanĝo) ne superas 50 ms.
- **R02** — Komponaĵo de tri fenestroj ne superas 33 ms po kadro; nur damaĝo
  estas kopiita.
- **R03** — Sesio startas la kompozitoron kaj la ŝelon en 2 sekundoj.
- **R04** — Sesio, kompozitoro kaj ŝelo kune uzas ne pli ol 64 MiB da memoro,
  aparte de la kunhavaj surfacoj.

Buĝeto povas esti reviziita nur per mezuro kaj skriba kialo; ĝi **DEVI NE**
esti pligrandigita por kaŝi regreson.

## 4. Protokoloj

### 4.1 Fenestra protokolo

Ĉiu mesaĝo havas fiksan kapon:

```text
+--------+--------+--------+--------+
| versio |  tipo  |      longo      |
|  u16   |  u16   |       u32       |
+--------+--------+--------+--------+
```

Reguloj:

- **S01** — La kapo estas 8 bajtoj, little-endian, sen implica padding.
- **S02** — `longo` estas la longo de la tuta mesaĝo, inkluzive de la kapo.
  Mesaĝo pli granda ol 64 bajtoj estas rifuzata.
- **S03** — Nekonata `tipo` aŭ `versio` estas rifuzata per aparta eraro; la
  ricevanto restas viva.
- **S04** — La legado estas totala: neniu enigo povas kaŭzi `panic`, senfinan
  buklon aŭ legadon ekster la bufro.
- **S05** — Fenestra identigilo konsistas el indekso kaj generacio. Reuzo de
  indekso **DEVI** havi novan generacion; malnova identigilo estas rifuzata.
- **S06** — Petoj: `KREI`, `MONSTRI`, `KAŜI`, `FERMI`, `MOVI`, `GRANDIGI`,
  `FOKUSI`, `DIFINI_TITOLON`, `DIFINI_DAMAGXON`.
- **S07** — Eventoj: `KREITA`, `FERMITA`, `FOKUSITA`, `ENIGO`, `PANEITA`.
- **S08** — La kompozitoro **DEVI** validigi ĉiun ortangulon kontraŭ la surfaco
  kaj la ekrano antaŭ ol uzi ĝin.
- **S09** — Mesaĝoj de la kompozitoro al aplikaĵo estas ordigitaj; mesaĝoj de
  aplikaĵo al la kompozitoro estas prilaboritaj en la riceva ordo.
- **S10** — La protokolo ne portas krudan frambufan kapablon, nur surfacan
  identigilon kaj la kunhavigan tenilon.

### 4.2 Surfaco kaj damaĝo

- **D01** (decido) — Fenestra surfaco estas **kunhavigita memoro**. La aplikaĵo
  skribas en ĝin; la kompozitoro mapas ĝin nurlege. Ĉi tio anstataŭas la
  principon «neniu kunhava memoro en 1.0» de `LUMO.md` §6 K03. Antaŭ H2 oni
  revizias la Lumo-dokumenton kaj registras la decidon en ambaŭ deponejoj.
- **D02** — La kompozitoro legas nur la regionojn, kiujn la aplikaĵo markis kiel
  damaĝon; plena ekrankopio por ĉiu kadro estas malpermesita.
- **D03** — Surfaco estas `ARGB8888`, antaŭmiksita, vico laŭ pikseloj; `stride`
  estas multipliko de 4 kaj ne malpli ol `larĝo × 4`.
- **D04** — La maksimuma surfaco estas 4096 × 4096 pikseloj; pli granda peto
  estas rifuzata antaŭ ol mapi memoron.
- **D05** — La aplikaĵo devas publikigi novan damaĝon post skribo; la
  kompozitoro ne legas surfacon sen tia signo.
- **D06** — Kunhava surfaco vivas ĝis la fenestro fermiĝas aŭ la procezo mortas;
  la posedanto estas la kreinto; la kompozitoro nur pruntas la mapadon.

### 4.3 Enigo

- **S11** — Klavaraj eventoj iras nur al la fokusita fenestro.
- **S12** — Musaj eventoj iras al la fenestro sub la montrilo; dum trenado la
  kompozitoro kaptas la muson ĝis la butono liberiĝas.
- **S13** — La kompozitoro kunfandas movojn de la muso, sed **DEVI NE** forigi
  klavopremon aŭ klavoliberigon.
- **S14** — Aplikaĵo **DEVI NE** ricevi tutmondan klavaran aŭ musan fluon.
- **S15** — La fokuso **DEVI** havi videblan indikon; `Eniro` kaj modalaj
  dialogoj ricevas fokuson laŭ la reguloj de `FLUOU.md` §8 kaj P16.

### 4.4 Manifesto kaj permesoj

La manifesta formo sekvas `FLUOU.md` §9.1. Devigaj kampoj:

```toml
nomo = "dosierujo"
versio = "0.4.0"
gu = "fluoU"
fluo = "0.1"
arkoj = ["x86_64", "arm64"]
permesoj = { fenestro = true, dosieroj = ["~/"], reto = false }
```

- **S16** — `gu` **DEVI** esti `"fluoU"`; `fluo` **DEVI** esti kongrua kun la
  subtenata FluoU-versio.
- **S17** — Nekonata permeso aŭ nekonata kampo en `permesoj` kaŭzas rifuzon.
- **S18** — Defaŭlto estas malpermeso: mankanta permeso signifas «ne».
- **S19** — Aplikaĵo sen `fenestro = true` ne ricevas fenestran kapablon.
- **S20** — Aplikaĵo kun `reto = false` ne ricevas retan kapablon; krudaj
  ŝtopiloj estas malpermesitaj al ĉiuj aplikaĵoj.
- **S21** — Dosieraj permesoj estas radikoj, ne liberaj vojoj. La sesio
  validigas la vojon kaj rifuzas `..`, absolutajn vojojn ekster la radiko kaj
  simbolajn ligilojn, kiuj eskapas.
- **S22** — La manifesta validigo estas pura funkcio en `libhorizonto` kaj
  havas Kani-pruvon pri longoj kaj limoj.

## 5. Sekureco kaj izolado

- **D07** — La kompozitoro validigas ĉiun peton antaŭ ol mapi aŭ legi memoron.
- **D08** — Aplikaĵo ne povas malfermi la frambufon, eĉ se ĝi konas la
  identigilon de la kapablo; la kerno rifuzas la mapadon.
- **D09** — Paneo de aplikaĵo estas normale riparita: la kerno mortigas la
  procezon, la kompozitoro fermas la fenestron, la sesio liberigas la resursojn.
- **D10** — La kompozitoro havas limon por atendaj petoj kaj por fenestroj
  (minimuma limo: 64 fenestroj, 1024 atendaj mesaĝoj). Supero kaŭzas rifuzon,
  ne blokadon de la tuta sistemo.
- **D11** — Aplikaĵo ne povas sendi enigan eventon al alia aplikaĵo; nur la
  kompozitoro liveras enigon.
- **D12** — Pakaĵo estas subskribita; la instalilo validigas la subskribon
  antaŭ ol lanĉi ajnan kodon el la pakaĵo.
- **D13** — Ĝisdatigo estas atoma: malsukceso aŭ paneo lasas la antaŭan version
  funkcianta (aŭ revenigas ĝin).
- **D14** — Ekzistas aparta izolada testo por ĉiu el D08, D09, D10, D11, D12,
  D13; ĝi rulas en QEMU en la DI.

## 6. Kvalita Konstitucio (neintertraktebla)

Ĉi tiu sekcio spegulas `../mikerno/docs/MEJLOSTONOJ.md` §9 kaj aldonaĵojn de
Horizonto. La reguloj estas neintertrakteblaj: Fazo ne estas finita, se iu el
ili estas malverda.

| MiKerno | Horizonto | Temo |
| --- | --- | --- |
| §9.1 | Q09–Q11 | Tri neintertrakteblaj reguloj |
| §9.2 | Q01–Q03, Q07 | Pordego, sen loka Kani, Miri |
| §9.3 | Q09 | Esceptoj por `#[allow]` kaj `unsafe` |
| §4.7 | Q02 | Nuba DI anstataŭ loka Kani |
| §4.8 | Q05, Q06 | La Kani-matrico kaj la pruva protokolo |
| M18 | Q04 | `cargo deny` kaj la licenca listo |
| `AGENTS.md` | Q10, Q11 | Laborfluo, komitaj reguloj |
| `FLUOU.md` A06/P17 | Q13 | Piksela paritateco |
| §8 | §1.3 | Difino de «1.0» |

### 6.1 La tri neintertrakteblaj reguloj

- **Q09** — Neniu malfortigo de la pordego: neniu `#[allow(...)]` por
  silentigi Clippy, neniu `kani::assume` por eviti pruvon, neniu `unsafe`
  ekster la aprobita sistemo-voko, neniu malŝaltita testo. Escepto postulas
  skriban kialon, aprobon de la uzanto kaj registron en `docs/ESCEPTOJ.md`.
- **Q10** — «Preta» signifas la plenan liston de §6.5; ne «ĝi kompiliĝas» kaj
  ne «la unua scenaro funkcias».
- **Q11** — Neniu komito aŭ puŝo sen klara permeso de la uzanto. Post permeso,
  komitu nur la nunan Fazon; ne kunfandu Fazojn.

### 6.2 Loka pordego

- **Q01** — `just kontroli` **DEVI** ruli, sen Kani kaj sen reto:

  ```sh
  bash tools/versio.sh --kontroli
  bash tools/sen-cina.sh --kontroli
  cargo fmt --all -- --check
  cargo clippy --workspace --release -- -D warnings
  cargo clippy -p libhorizonto --release --target x86_64-unknown-none -- -D warnings
  cargo clippy -p libhorizonto --release --target aarch64-unknown-none -- -D warnings
  cargo test --workspace
  cargo build --workspace --release
  ```

  Kiam servoj aldoniĝos, la lastaj du linioj etendiĝos al iliaj celoj, sed la
  principo restas: la tuta laborloko, ambaŭ arkitekturoj, sen Kani.

- **Q02** — Loke oni **DEVI NE** ruli `cargo kani`. WSL2 elĉerpiĝas je memoro.
  Kani rulas nur en GitHub Actions.
- **Q03** — La loka pordego **DEVI** esti rulebla sen reto kaj sen eksteraj
  iloj krom Rust, `just` kaj la du `no_std` celoj. Se ĝi postulas Ion, ĝi ne
  estas loka pordego.

### 6.3 Nuba DI

- **Q04** — `cargo deny check` laŭ `deny.toml`. Permesataj licencoj: MIT,
  Apache-2.0, BSD-3-Clause, Unicode-DFS-2016. GPL, AGPL kaj LGPL estas
  rifuzataj. Nova licenco postulas skriban aprobon de la uzanto.
- **Q05** — La pruvoj estas registritaj en `docs/PRUVOJ.md`: identigilo,
  celata regulo, priskribo, vokado, tempolimo. Ĉiu pruvo havas unu taskon en la
  matrico.
- **Q06** — La Kani-matrico spegulas `MEJLOSTONOJ.md` §4.8: `kani-pretigi`,
  poste unu taskon por ĉiu pruvo, `timeout -k 10 600 cargo kani … --exact
  --harness <pruvo>`, `timeout-minutes: 30`, `fail-fast: false`, kaj protokolo
  kiel artefakto eĉ ĉe malsukceso.
- **Q07** — Miri rulas super la unu-testoj de `libhorizonto` (kaj poste de la
  puraj servaj moduloj) per nokta Rust. Loke `just miri`; en DI `miri-kontroli`.
- **Q08** — Dependaĵoj: nur pura Rust, sen C, sen binara blobo. Ĉiu nova
  krata estas registrita en `docs/DEPENDOJ.md` kun versio, licenco, kialo kaj
  revizia dato. La kerno restas sen eksteraj krataj; la uzantlando rajtas ilin
  uzi nur laŭ ĉi tiu regulo.

### 6.4 Paritateco, izolado kaj reala eligo

- **Q12** — Raportoj enhavas la veran eliron de la ordonoj, ne priskribon de
  la intenco. «Ĝi devus funkcii» ne estas pruvo.
- **Q13** — Por ĉiu fenestra fiksaĵo ekzistas piksela haŝo en
  `docs/FIKSAĴOJ.md`. La haŝo de `x86_64` kaj `arm64` **DEVI** esti identa;
  tolero estas nulo. Fiksaĵo sen haŝo ne validas.
- **Q14** — La izolada testo (`izolado-kontroli`) rulas la atakojn de §5 en
  QEMU: frambufa mapado, skribo ekster la surfaco, eniga-injekto, dosiera
  eskapo, paneo, mesaĝa diluvo, falsa subskribo. Ĉiu atako **DEVI** malsukcesi
  sen faligi la sesion.
- **Q15** — Protokola versio, manifesta `fluo`-versio kaj la subtenata
  `libmikerno`-ABI estas kongruaj kaj kontrolataj de la pordego. Malkongruo
  estas rifuzata antaŭ lanĉo, ne kaŝita per adaptilo.

### 6.5 Difino de preta (Fazo)

Fazo estas preta nur kiam ĉio ĉi estas vera:

1. `just kontroli` verda loke, kun la vera eligo.
2. La DI-taskoj de §6.6 verda; la Kani-matrico sen malsukceso.
3. Miri kaj `cargo deny` verda.
4. Paritateco verda ekde H2; izolado verda ekde H2; pakaĵa fumo ekde H5.
5. Ĉiu nova publika ero havas Esperantan dokumentaron; `RUSTDOCFLAGS='-D
   missing-docs' cargo doc --no-deps` verda.
6. `docs/HORIZONTO.md`, la vortaro kaj la fazoregistro (kiam ili ekzistas) kaj
   la versio kongruas; `tools/versio.sh --kontroli` verda.
7. Neniu nova `#[allow]`, `unsafe` aŭ malŝaltita testo sen escepto en
   `docs/ESCEPTOJ.md`.
8. La akceptaj kriterioj de la Fazo estas plenumitaj per reala eligo.
9. Oni haltas kaj raportas; oni ne komencas la sekvan Fazon.

### 6.6 DI-taskoj

| Tasko | Kiam | Enhavo |
| --- | --- | --- |
| `kontroli-kaj-konstrui` | H1 | `just kontroli`, plus la krucaj konstruloj |
| `permesiloj-kontroli` | H1 | `cargo deny check` |
| `miri-kontroli` | H1 | `just miri` |
| `kani-pretigi` | H1 | Instalas Kani per la agado |
| `kani-kontroli` | H1 | Matrico, unu pruvo por tasko |
| `paritateco` | H2 | Pikselaj haŝoj ambaŭ arkitekturoj |
| `izolado-kontroli` | H2 | La atakoj de §5 en QEMU |
| `paki-kontroli` | H5 | Instalo, lanĉo, forigo de fiksa pakaĵo |

Taskoj, kiuj ankoraŭ ne aplikeblas, **DEVI** esti markitaj kiel «ne aktiva» en
la DI, ne forigitaj. La pordego estas verda nur kiam ĉiuj aktivaj taskoj estas
verdaj.

## 7. Petoj al MiKerno

Horizonto **DEVI NE** anstataŭigas mankantan kernan mekanismon per uzantlanda
truko. La sekvaj petoj estas la antaŭkondiĉoj de la Fazoj; la efektivigo,
pruvoj kaj dokumentaro apartenas al MiKerno.

| Peto | Bezono | Uzata de | Noto |
| --- | --- | --- | --- |
| K1 | Kunhava memoro / `SYS_MMAP` kun kapablo | H2 | Surfacoj kaj grandaj bufroj |
| K2 | Frambufa kapablo kaj reĝima agordo | H2 | Nur la kompozitoro rajtas mapi |
| K3 | Restarigo post Ring 3-paneo | H2 | Mortigi la procezon, ne haltigi la sistemon |
| K4 | Enigaj eventoj (klavaro, muso) | H3 | `virtio-input` aŭ PS/2; kapablo |
| K5 | Reta pelilo kaj kapablo | H7 | `virtio-net`; la stako restas uzantlanda |
| K6 | Legado de tempo / RTC | H7 | Por ĝisdatigoj kaj atestiloj |
| K7 | Dosierujaj kapabloj kun radika limo | H6 | La sesio disdonas; la kerno gardas |
| K8 | Kalkulado de procezoj kaj resursoj | H2 | Por D10 kaj D09 |
| K9 | Dinamika ligilo | post 1.0 | La SDK unue konstruas statikajn bildojn |
| K10 | `fork`/COW | post 1.0 | Ne necesa por la unuaj servoj |
| K11 | GPU-akcelo (`virtio-gpu`) | post 1.0 | La unua komponaĵo estas programa |

Se peto mankas, la Fazo estas **blokita**: oni ne ŝajnigas ĝin per rekta
aparataro, identa mapado aŭ senposeda `unsafe`. La blokita Fazo raportiĝas al
la uzanto kun la ĝusta peto, ne kun laboraĵo, kiu rompos izoladon.

## 8. Fazoj

| Fazo | Versio | Celo | Antaŭkondiĉoj |
| --- | --- | --- | --- |
| H1 | 0.1.0 | Kontraktoj, tipoj, manifestoj, pordego | — |
| H2 | 0.2.0 | Kunhava memoro, unua fenestro, izolado | K1, K2, K3, K8 |
| H3 | 0.3.0 | Fenestroj, fokuso, enigo, FluoU-kadro | K4; Lumo ≥ 0.2 |
| H4 | 0.4.0 | Ligo, Eniro, unuaj aplikaĵoj | H3; ekzistanta dosiera aliro |
| H5 | 0.5.0 | SDK, pakaĵoj, instalilo | H4 |
| H6 | 0.6.0 | Dosieroj, agordoj, alirebleco, tradukoj | K7; H4 |
| H7 | 0.7.0 | Reto kaj ĝisdatigoj | K5, K6; H5 |
| H8 | 0.8.0 | ODF-legilo, eldonkandidato | H5–H7 |
| H9 | 1.0.0 | Frostigo kaj publikigo | H8; plena pordego |

Ĉiu Fazo finiĝas per haltado kaj raporto. La sekva Fazo ne komenciĝas sen
klara instrukcio de la uzanto.

## 9. Detalaj fazoj

### H1 — 0.1.0 «Kontraktoj kaj skeleto»

**Celo.** Fiksi la servan modelon, la fenestran protokolon, la manifestan formon
kaj la kvalitan pordegon antaŭ ol ekzistas fenestro. Neniu grafikaĵo en ĉi tiu
Fazo.

**Enhavo.**

1. Laborloko kun `libhorizonto`: protokolaj kapoj, fenestra identigilo kun
   generacio, damaĝaj ortanguloj, enigaj eventoj, manifestaj tipoj kaj
   permesoj. Sen `alloc`, sen `unsafe`, sen glitkomo.
2. `docs/KONTRAKTOJ.md`: la serva modelo, la starta fluo, la kapabla tabelo kaj
   la petoj de §7 al MiKerno.
3. Manifesta validigo laŭ §4.4 kaj `FLUOU.md` §9.1: devigaj kampoj, `gu`,
   `fluo`, arkoj, permesoj; rifuzo de nekonataĵoj.
4. Protokola versio kaj fiksaj aranĝoj kun `const`-asertoj pri grandeco,
   ĝisrandigo kaj deŝovoj. Little-endian skribo kaj legado.
5. Pordega skafaldo: `justfile`, `deny.toml`, `tools/versio.sh`,
   `tools/sen-cina.sh`, `tools/sen-glitkomo.sh`, `docs/PRUVOJ.md`,
   `docs/ESCEPTOJ.md`, `.github/workflows/di.yml`.
6. Pruvoj: manifestaj limoj, damaĝa kunigo, generacia reuzo, totala legado de
   protokola mesaĝo. Minimumo: kvar pruvoj.

**Evitindaj eraroj.**

- Enkonduki `String`, `Vec` aŭ `alloc` en `libhorizonto`. Uzu fiksajn tabelojn,
  `&str` kaj funkciojn, kiuj redonas longon aŭ eraron.
- Lasi la aranĝon de `struct` al la kompililo. Uzu `#[repr(C)]` aŭ eksplicitan
  bajtan aranĝon kaj asertojn; alie la protokolo rompiĝos inter arkitekturoj.
- Supozi gastigantan endianness. La protokolo estas eksplicite little-endian.
- Malfermi permesojn defaŭlte. Nekonata aŭ mankanta permeso signifas «ne».
- Reuzi fenestran indekson sen nova generacio; malnova identigilo devas esti
  rifuzata.
- Enkonduki Lumo aŭ FluoU en H1 «por provi». La Fazo estas kontrakta; vidaĵo
  venas en H2/H3.
- Krei pruvon, kiu nur pruvas `true == true`. Ĉiu pruvo celas konkretan
  regulon de §4 aŭ §5.

**Akcepto.**

1. `just kontroli` kaj la DI verda; Miri kaj `cargo deny` verda.
2. Almenaŭ kvar Kani-pruvoj en `docs/PRUVOJ.md`, ĉiu kun propra DI-tasko.
3. Unu-testoj: validaj kaj malvalidaj manifestoj, nekonata permeso, malkongrua
   `fluo`, mallonga mesaĝo, nekonata tipo, generacia reuzo.
4. La protokolaj grandecoj kaj deŝovoj estas `const`-asertitaj kaj dokumentitaj
   en `docs/KONTRAKTOJ.md`.
5. `libhorizonto` estas sen `alloc`, sen `unsafe` kaj sen glitkomo (kontrolite
   per `tools/sen-glitkomo.sh`).
6. `docs/KONTRAKTOJ.md` listigas la petojn K1–K11 kaj iliajn Fazojn.
7. La versio `0.1.0` kongruas en `Cargo.toml`, `README.md` kaj ĉi tiu dokumento.

### H2 — 0.2.0 «Kunhava memoro kaj unua fenestro»

**Antaŭkondiĉoj.** K1 (kunhava memoro), K2 (frambufa kapablo), K3 (restarigo
post Ring 3-paneo), K8 (kalkulado de resursoj). Lumo ≥ 0.2 por la titola kadro;
se Lumo ankoraŭ ne pretas, la unua fenestro uzas simplan ortangulan kadron kaj
la FluoU-kadro venas en H3. La revizio de `LUMO.md` §6 K03 (D01) **DEVI** esti
farita antaŭ la integriĝo.

**Enhavo.**

1. `kunhava`-volvaĵo en la uzantlando: krei, mapi, fermi kaj revoki kunhavigan
   surfacon; tenilo kaj generacio; nurlega mapado en la kompozitoro.
2. Servo `kompozitoro`: posedas la frambufan kapablon, kreas eligan surfacon,
   komponas unu fenestron sur fono, legas nur la damaĝon.
3. Servo `sesio`: startas la kompozitoron kaj unu demonstran aplikaĵon; validigas
   la manifeston; disdonas la kapablojn.
4. Demonstra aplikaĵo: kreas fenestron, skribas sian surfacon per Lumo (kadro,
   titolo, simpla enhavo) kaj publikigas damaĝon.
5. Restarigo: la demonstra aplikaĵo paneas intence; la kompozitoro fermas la
   fenestron, la sesio liberigas resursojn, dua aplikaĵo povas starti.
6. Pikselaj haŝoj por la unua scenaro ambaŭ arkitekturoj; `docs/FIKSAĴOJ.md`.
7. `docs/IZOLADO.md` kun la unuaj atakoj de §5.

**Evitindaj eraroj.**

- Uzo post liberigo de surfaco. La fenestra generacio kaj la posedanto devas
  esti klaraj; fermo revokas la mapadon antaŭ ol liberigi la memoron.
- Lasi la aplikaĵon skribi ekster sian surfacon. La kerno devas limigi la
  mapadon; la kompozitoro devas validigi damaĝon eĉ se la aplikaĵo mensogas.
- Legi surfacon dum la aplikaĵo ankoraŭ skribas. Uzu duoblan bufron aŭ
  publikigan signon; ne fidu hazardan ordon.
- Konfuzi `stride` kaj larĝon. La vico povas esti pli longa ol la videbla
  larĝo; ĉiu legado respektu `stride`.
- Forgesi la frambufan formaton. La eliga surfaco estas `ARGB8888`; la
  konvertiĝo al la frambufa formato okazas unu fojon, en la kompozitoro.
- Kopii la tutan ekranon por ĉiu kadro. Nur la damaĝaj ortanguloj rajtas esti
  kopiitaj (D02).
- Lasi la frambufan kapablon en la aplikaĵa kapabla aro «por testado». Ĝi
  neniam rajtas tie esti.
- Krei novan fenestron post paneo sen forigi la malnovan el la fenestra tabelo.
  Resursoj devas esti liberigitaj precize unufoje.

**Akcepto.**

1. La pordego kaj la DI verda; paritateco kaj izolado aktivaj.
2. Unu fenestro montras la fiksan scenaron; la haŝo estas registrita kaj identa
   sur `x86_64` kaj `arm64`.
3. Paneo de la fenestra procezo: la sesio restas viva, la fenestro malaperas,
   la resursoj estas liberigitaj kaj nova procezo povas starti.
4. Negativaj testoj: la aplikaĵo ne povas mapi la frambufon; ne povas mapi alian
   surfacon; skribo ekster la propra surfaco mortigas nur ĝin.
5. La kompozitoro kopias nur damaĝon; la plena ekrankopio ne aperas en la
   kompona vojo (kontrolite per la pruvo kaj per mezuro).
6. `LUMO.md` §6 K03 estas reviziita kaj la decido D01 estas registrita en ambaŭ
   deponejoj.
7. `docs/IZOLADO.md` enhavas almenaŭ kvar atakojn kaj iliajn atendatajn
   rezultojn.

### H3 — 0.3.0 «Fenestroj, enigo kaj FluoU-kadro»

**Antaŭkondiĉoj.** K4 (enigaj eventoj). Lumo ≥ 0.2 por la titola kadro. La
kompozitoro el H2 funkcias.

**Enhavo.**

1. Pluraj fenestroj: tavolordo, fokuso, aktivigo, titola kadro laŭ
   `FLUOU.md` §3 (32 DIP, tri regiloj 46 × 32, ikonoj 10 × 10, streko 1,2 DIP),
   fermo, minimumigo, maksimumigo, trenado kaj ŝanĝo de grandeco.
2. Eniga vojigo: klavaro al la fokusita fenestro; muso al la fenestro sub la
   montrilo; musa kapto dum trenado; kunfando de musaj movoj sen perdo de
   klavopremoj.
3. Fokusa ciklo per `Tab`, fermo per `Alt+F4`, fenestra menuo per `Alt+Spaco`,
   reĝimaj klavoj laŭ `FLUOU.md` §8.
4. Damaĝo kaj kaŝado: kaŝitaj fenestroj ne komponiĝas; supermetitaj fenestroj
   komponiĝas en la ĝusta ordo; la fokusa ringo estas videbla.
5. Fiksaj bildoj por tri fenestroj, fokuso, titola kadro kaj ferma butono;
   haŝoj en `docs/FIKSAĴOJ.md`.
6. `Eniro` ankoraŭ ne en ĉi tiu Fazo; ĝi venas en H4 kiel paletro, ne kiel
   fenestro (`FLUOU.md` F17).

**Evitindaj eraroj.**

- Fokusa ŝtelo: nova fenestro ne rajtas preni fokuson sen decido de la sesio aŭ
  de la uzanto.
- Miksi DIP kaj pikselojn. En 1.0 la skalo estas 1,0; tamen la kodaj unuoj
  devas esti klaraj, por ke skalado post 1.0 ne rompu geometrion.
- Perdi klavopremon aŭ klavoliberigon pro musa diluvo. Klavaj eventoj estas
  ordigitaj kaj ne kunfandeblaj.
- Detrui la fokusitan fenestron sen elekti novan fokuson. La fokuso **DEVI**
  transiri al difinita kandidato aŭ al la ŝelo.
- Komponi kaŝitajn aŭ plene kovritajn fenestrojn. La damaĝa kalkulo devas
  konsideri kaŝadon.
- Permesi al fenestro moviĝi aŭ grandiĝi ekster la ekrano sen limigo.
- Desegni la titolan kadron per propra kodo anstataŭ la FluoU-komponanto.

**Akcepto.**

1. La pordego, paritateco kaj izolado verda.
2. Scenaro kun tri fenestroj: krei, fokusigi, movi, ŝanĝi grandecon, fermi;
   la haŝo estas identa ambaŭ arkitekturoj.
3. La titola kadro kongruas kun la FluoU-fiksaĵoj de §3; la butonoj havas la
   ĝustajn dimensiojn kaj statojn.
4. Klavara testado: `Tab` ciklas, `Alt+F4` fermas, `Alt+Spaco` malfermas la
   menuon; neniu klavo perdiĝas en aro de 1000 eventoj.
5. Fona fenestro ne ricevas klavojn; aplikaĵo ne povas movi alian fenestron.
6. Paneo de la fokusita fenestro transdonas fokuson laŭ difinita regulo.
7. Eniga latenteco estas mezurita kaj registrita en `docs/RENDIMENTO.md`; ĝi
   ne superas la buĝeton de R01.

### H4 — 0.4.0 «Ŝelo kaj unuaj aplikaĵoj»

**Antaŭkondiĉoj.** H3; baza dosiera aliro per la ekzistanta `LEGI`/`SKRIBI`
kaj sesiaj permesoj por malfermi dosierojn.

**Enhavo.**

1. `ligo`: la ekzistanta ŝela logiko (listi, legi, lanĉi, fini) en fenestro;
   tekstaj eligo kaj enigo per Lumo-tiparoj; rulumado kaj historio.
2. `eniro`: paletro `Ctrl+E` kiel supermetita paletro, ne aparta fenestro; komandoj por
   lanĉi, malfermi, agordi; klavara navigado.
3. `dosierujo`: listo, navigado, malfermo per asocia aplikaĵo, bazaj operacioj
   (kopii, movi, renomi, forigi al rubujo).
4. `redaktilo`: malfermi kaj konservi UTF-8-tekston; ŝanĝa stato; demando antaŭ
   fermo; atoma konservo (provizora dosiero + renomo).
5. `agordoj`: temo (`hela`/`malhela`), movo (plena/limigita), lingvo; bazaj
   agordoj konserviĝas.
6. Sesio lanĉas la ŝelon kaj la agordojn; la uzanto lanĉas la ceterajn.
7. Permesoj: dosiera aliro per radikoj; demando de la sesio por nova radiko;
   la demando estas fidinda surfaco, ne desegnita de la aplikaĵo.

**Evitindaj eraroj.**

- Blokanta dosiera legado frostigas la fenestron. Legu en malgrandaj pecoj kaj
  redonu al la eventa buklo; aŭ uzu kunlaboran taskon, se la modelo permesas.
- Perdi nesavitajn ŝanĝojn. Fermo kun modifita bufro **DEVI** demandi aŭ
  konservi aŭtomate; neniu silenta perdo.
- Senfina ŝela eligo. Limigu la historion kaj la rulumon; malnova eligo
  malplenigu laŭ difinita regulo.
- Falsa permesa demando desegnita de la aplikaĵo. Nur la sesio aŭ la
  kompozitoro rajtas montri fidindan demandon.
- Trakti Esperantajn literojn kiel du signojn. Uzu kodpunktojn; la tiparo
  havas la necesajn glifojn.
- Konservi agordon per rekta skribo en la celan dosieron. Uzu provizoran
  dosieron kaj atoman renomon.

**Akcepto.**

1. La pordego, paritateco kaj izolado verda; sesia fiksa bildo kun la ŝelo.
2. Tri aplikaĵoj lanĉeblas, fermeblas kaj restaŭras fokuson; la tuta fluo
   funkcias per klavaro.
3. La redaktilo malfermas kaj konservas dosieron sen perdo; ŝanĝa stato kaj
   demando funkcias; atoma konservo post paneo ne koruptas la antaŭan dosieron.
4. Permesaj testoj: malpermesita dosiero ne legeblas; permesita dosiero
   legeblas; rifuzo ne faligas la aplikaĵon.
5. Fermo de aplikaĵo liberigas procezon, fenestron kaj surfacon; neniu zombio.
6. `FLUOU.md` P09, P10 kaj P16 estas kontrolitaj per fiksaĵoj aŭ testoj.
7. `Eniro` funkcias per `Ctrl+E` kaj ne aperas kiel aparta fenestro.

### H5 — 0.5.0 «SDK kaj pakaĵoj»

**Antaŭkondiĉoj.** H4; la paka formato kaj la subskriba politiko fiksitaj en
`docs/PAKAĴOJ.md`.

**Enhavo.**

1. SDK: `horizonto-sdk` (aŭ samnoma modulo) por krei projekton, generi
   manifeston, konstrui por `x86_64` kaj `arm64` kaj paki. Ŝablono kun
   FluoU + Lumo.
2. Paka formato: dosieroj + manifesta + subskribo; instalilo kaj forigilo;
   versiaj limoj kaj simplaj dependecoj.
3. Subskribo: pura Rust (kandidato: `ed25519-dalek`, laŭ Q08); publikaj ŝlosiloj
   en la sesio; kontrolo antaŭ ol lanĉi ajnan kodon el la pakaĵo. Nesubskribita
   reĝimo nur por evoluigantoj, kun klara averto kaj sen reto.
4. Loka deponejo kaj indekso; instalado sen reto.
5. `docs/DEPENDOJ.md`: ĉiu nova krata kun licenco, versio, kialo kaj revizio.
6. `docs/PAKAĴOJ.md`: formato, subskribo, versioj, migrado, forigo.

**Evitindaj eraroj.**

- Voja eskapo per `..` aŭ absoluta vojo en pakaĵo. Normaligu kaj limigu ĉiun
  vojon al la cela radiko; rifuzu simbolajn ligilojn, kiuj eskapas.
- Kontroli subskribon kaj poste malfermi alian dosieron (TOCTOU: kontrolo kaj
  uzo en malsamaj momentoj). Kontrolu la saman bajtofluon, kiun vi instalas aŭ
  lanĉas.
- Subskribi nur la manifeston kaj ne la dosierojn. La subskribo **DEVI** kovri
  la indekson kaj la enhavon laŭ la formato.
- Neatoma instalado. Ĉiu paŝo devas povi reveni; malsukceso lasas la antaŭan
  version funkcianta.
- Trokomplika dependeca solvanto. Unue precizaj versioj kaj simplaj dependecoj;
  kompleksa solvanto venas post 1.0.
- Postuli reton por la SDK aŭ por baza instalado. La loka laborfluo **DEVI**
  funkcii sen reto.
- Akcepti novan dependaĵon sen registrado kaj licenca kontrolo.

**Akcepto.**

1. La pordego, paritateco, izolado kaj `paki-kontroli` verda.
2. La SDK konstruas fiksan ekzemplan aplikaĵon por ambaŭ arkitekturoj; la
   rezulto instaliĝas, lanĉiĝas kaj foriĝas.
3. Ĝisdatigo kaj malĝisdatigo funkcias; malsukcesa ĝisdatigo revenigas la
   antaŭan version.
4. Malbonaj pakaĵoj (voja eskapo, absoluta vojo, falsa subskribo, tro granda
   dosiero) estas rifuzataj sen paneo.
5. La konstruo estas ripetebla; la SDK havas Esperantan lernilon.
6. Ĉiu nova dependaĵo estas en `docs/DEPENDOJ.md` kaj pasas `cargo deny`.

### H6 — 0.6.0 «Dosieroj, agordoj, alirebleco»

**Antaŭkondiĉoj.** K7 (dosierujaj kapabloj kun radika limo); H4.

**Enhavo.**

1. Hejma dosierujo kaj normaj subdosierujoj; rubujo kun restaŭro; dosieraj
   permesoj videblaj kaj ŝanĝeblaj per la agordoj.
2. Agordoj: temo, movo, lingvo, reto; aplikaĵaj agordoj apartaj; atoma skribo
   kaj reveno al defaŭltoj ĉe korupto.
3. Alirebleco: plena klavara navigado, alta kontrasto, limigita movo, videbla
   fokuso; kongruo kun `FLUOU.md` P08, P09, P12.
4. Traduka tavolo: Esperanta fonto + `tradukoj/`; mankanta traduko montras la
   Esperantan fonton; neniu traduko en la fonta tavolo.
5. Dosieraj asocioj kaj «malfermi per»; lastaj dosieroj.
6. `docs/ALIREBLECO.md` kun la kontroloj kaj la konataj limoj.

**Evitindaj eraroj.**

- Simbolaj ligiloj kaj voja normaligo. Kontrolu la realan celon; rifuzu
  eskapon el la permesita radiko.
- Konfuzi kapablojn kun tradiciaj dosierpermesoj. La kapablo estas la unua
  limo; la dosiera reĝimo estas dua, ne anstataŭo.
- Korupti agordojn per rekta skribo. Provizora dosiero + atoma renomo; se la
  dosiero estas rompita, uzu defaŭltojn kaj raportu.
- Hardkodi kolorojn por alta kontrasto. Uzu FluoU-ĵetonojn kaj la kontrastan
  varianton.
- Trakti kunmetitajn signojn en dosiernomoj naive. Uzu kodpunktojn kaj
  validigu UTF-8 ĉe la limo.
- Enmeti tradukan ŝnuron en la fontan kodon. La fonto estas Esperanta; la
  traduko estas mapado.

**Akcepto.**

1. La pordego, paritateco kaj izolado verda.
2. Dosieraj operacioj (kopii, movi, renomi, forigi, restaŭri) funkcias; simbolaj
   ligiloj kaj `..` ne eskapas.
3. Agordoj pluvivas restartigon; korupta dosiero revenigas defaŭltojn sen
   paneo.
4. Alta kontrasto kaj limigita movo havas fiksaĵojn; klavara navigado kovras
   la ĉefajn fluojn.
5. Traduka manko montras Esperantan tekston; la fonta tavolo restas sen aliaj
   lingvoj (kontrolite per `tools/sen-cina.sh` kaj revizio).
6. `docs/ALIREBLECO.md` listigas la kontrolojn kaj la konatajn limojn.

### H7 — 0.7.0 «Reto kaj ĝisdatigoj»

**Antaŭkondiĉoj.** K5 (reta pelilo kaj kapablo), K6 (tempo); H5 (pakaĵoj).

**Enhavo.**

1. Uzantlanda reta stako: ARP, IPv4, UDP, TCP (minimuma), DHCP, DNS; ŝtopila
   API en `libhorizonto`; la stako estas servo, ne parto de la kerno.
2. Retpermeso: defaŭlte malpermesita; krudaj ŝtopiloj malpermesitaj al ĉiuj
   aplikaĵoj; la retagento havas la kapablon.
3. Ĝisdatiga kliento: subskribita indekso, elŝuto, kontrolo, atoma instalado,
   reveno; tempo por atestiloj.
4. Retaj agordoj kaj stato en `agordoj`.
5. `docs/RETO.md`: protokoloj, limoj, permesoj, testa medio.

**Evitindaj eraroj.**

- Vivdaŭro de pakaĵaj bufroj. Klarigu posedon ĉe ĉiu tavolo; ne lasu referencon
  al bufro, kiun la pelilo jam redonis.
- Kontrolsumoj, fragmentado kaj retranssendado. Testu ilin aparte; ne fidu la
  gastigan stakon.
- Blokanta DNS aŭ HTTP en la UI-ŝtofo. Uzu tempolimojn, nuligon kaj
  kunlaboran atendon.
- Permesa bypass per kruda ŝtopilo aŭ per rekta pelila aliro. Nur la retagento
  rajtas krudan aliron; aplikaĵoj ricevas ŝtopilojn.
- Neatoma ĝisdatigo aŭ rompita subskriba ĉeno. Kontrolu ĉiun ligilon; revenu
  al la antaŭa versio ĉe malsukceso.
- Dependi de QEMU-user-reto en produkta kodo. La testa medio estas aparta.

**Akcepto.**

1. La pordego, paritateco, izolado kaj `paki-kontroli` verda.
2. En QEMU: DHCP, DNS kaj HTTP elŝuto el loka servilo funkcias; la rezulto estas
   ripetebla.
3. Aplikaĵo sen `reto = true` ne povas malfermi ŝtopilon; kun la permeso ĝi
   povas; kruda ŝtopilo estas rifuzata al ĉiuj aplikaĵoj.
4. Ĝisdatigo instaliĝas kaj povas reveni; falsa subskribo estas rifuzata.
5. La kerno liveras nur la pelilon kaj la kapablon; TCP/IP loĝas en la
   uzantlando.
6. `docs/RETO.md` priskribas la testan medion kaj la limojn.

### H8 — 0.8.0 «Dokumentoj kaj eldonkandidato»

**Antaŭkondiĉoj.** H5–H7; la dokumenta legilo kaj la eldona procedo fiksitaj.

**Enhavo.**

1. ODF-legilo por teksto: malfermi `.odt`, legi la ZIP-enhavon, analizi
   `content.xml` kaj `styles.xml`, montri bazan paĝan aranĝon per Lumo, kaj
   fermi sen modifi. Redaktado kaj konservado venas post 1.0.
2. Dosieraj asocioj por `.odt`; lastaj dosieroj; malfermo el `dosierujo` kaj
   `ligo`.
3. Eldonkandidato: instalilo, migrado de 0.7, fina dokumentaro, SDK-frostigo,
   rendimenta kaj memora revizio.
4. `docs/DOKUMENTOJ.md`: subtenata ODF-subaro, limoj, konataj mankoj.
5. LibreOffice **ne** estas postulata; kongrua tavolo venas post 1.0.

**Evitindaj eraroj.**

- ZIP-bombo aŭ XML-enteca ekspansio. Limigu la grandecon post malkompaktigo, la
  nombron de dosieroj kaj la profundecon de XML; rifuzu superon antaŭ analizi.
- Perdo de datumoj ĉe malfermo aŭ «konservo». En H8 la legilo estas nurlega;
  se oni aldonas konservon, uzu provizoran dosieron kaj atoman renomon.
- Uzi glitkomon por paĝa aranĝo. La geometrio restu entjera por paritateco.
- Fidi la unuan XML-analizilon sen limoj. Elektu puran Rust-kraton laŭ Q08 kaj
  envolvu ĝin per limoj kaj totala analizo.
- Promesi plenan ODF-kongruon. Dokumentu la subaron kaj la mankojn.
- Miksi la legilon kun retaj aŭ makroaj kapabloj. Neniu makroo, neniu reto.

**Akcepto.**

1. La pordego, paritateco, izolado kaj `paki-kontroli` verda.
2. Fiksaj `.odt`-dosieroj malfermiĝas kaj montras la atendatan unuan paĝon;
   la haŝo estas registrita ambaŭ arkitekturoj.
3. ZIP-bombo, rompita ZIP kaj tro profunda XML estas rifuzataj sen paneo.
4. La legilo ne modifas la fontan dosieron; malfermo de la sama dosiero dufoje
   donas la saman bildon.
5. Instalilo kaj migrado de 0.7 funkcias; la dokumentaro kaj la SDK estas
   frostigitaj.
6. `docs/DOKUMENTOJ.md` listigas la subtenatan subaron kaj la mankojn.

### H9 — 1.0.0 «Frostigo kaj publikigo»

**Antaŭkondiĉoj.** H8; plena pordego; neniu malfermita cimo de sekureco aŭ
datumperdo.

**Enhavo.**

1. Frostigo de la fenestra protokolo, la manifesta formo, la permesa modelo kaj
   la SDK. Post 1.0, rompaj ŝanĝoj postulas novan majoran version.
2. Eldona procedo: bildoj, instalilo, migrado de 0.8, SHA-256, subskribo.
3. Fina dokumentaro: `HORIZONTO.md`, `KONTRAKTOJ.md`, `PAKAĴOJ.md`, `RETO.md`,
   `DOKUMENTOJ.md`, `VORTARO.md`.
4. Frostigo de la DI: ĉiuj taskoj aktivaj, neniu «ne aktiva» preterlaso.
5. Etikedo `v1.0.0` nur kun klara permeso de la uzanto.

**Evitindaj eraroj.**

- Aldoni funkcion dum la frostiga Fazo. Se io mankas, ĝi estas dokumentita
  manko, ne kaŝita lasta ŝanĝo.
- Ŝanĝi protokolon sen versia altigo. La frostigo estas la punkto, ne
  deklaro.
- Publikigi bildojn konstruitajn el nepuŝita aŭ malverda kodo.
- Forgesi la migradon de 0.8 aŭ la revenan vojon.
- Malstreĉi la pordegon por «fini ĝustatempe». La pordego estas la produkto.

**Akcepto.**

1. Ĉiuj DI-taskoj verda, inkluzive de la plena Kani-matrico, Miri, permesiloj,
   paritateco, izolado kaj paki.
2. La versio estas `1.0.0` ĉie; `tools/versio.sh --kontroli` verda.
3. Migrado de 0.8 funkcias; la antaŭa versio povas reveni.
4. La protokolo, manifesta formo, permesoj kaj SDK estas dokumentitaj kiel
   frostigitaj.
5. La bildoj estas konstruitaj el la puŝita kodo, kontrolitaj kaj publikigitaj
   kun SHA-256.
6. La fina raporto listigas veran eliron, ne intencojn.

## 10. Post 1.0

- Redaktado kaj konservado de ODF; pli da ODF-subaro.
- Kongrua tavolo por OOXML kaj, laste, por LibreOffice-dokumentoj.
- Dinamika ligilo (K9), `fork`/COW (K10), pli da procezaj modeloj.
- GPU-akcelo (K11), vidbenda kaj aŭda subteno.
- Presado, skanado, pluruzanta sesio.
- Retaj sekurecoj: TLS, atestiloj, subskriba infrastrukturo.
- Real-aparata subteno preter la QEMU-celoj.
- Oficprogramaro: unue denaska ODF-verkilo, poste kongrua tavolo; LibreOffice
  restas la lasta paŝo, ne la fundamento.

## 11. Vortareto

| Esperanto | Angla / rimarko |
| --- | --- |
| Horizonto | la uzantlanda sistemo |
| MiKerno | la kerno |
| FluoU | la grafika kontrakto |
| Lumo | la rastrumilo |
| sesio | session / init |
| kompozitoro | compositor |
| fenestra protokolo | window protocol |
| surfaco | surface |
| damaĝo | damage |
| kapablo | capability |
| izolado | isolation |
| manifesta | manifest |
| permeso | permission |
| pakaĵo | package |
| subskribo | signature |
| Ligo | la ŝelo |
| Eniro | la komanda paletro (`Ctrl+E`) |
| `gu` | GUI-marko en la manifesto (`"fluoU"`) |
| `fluo` | FluoU-versio en la manifesto |
| paritateco | pixel parity |
| fiksaĵo | golden fixture |
| pordego | gate |
| Fazo | phase |
| Ring 3 | user mode |
| ABI | binary interface |

## Aldono A — Historio de la dokumento

| Versio | Dato | Ŝanĝo |
| --- | --- | --- |
| 0.1.0 | 2026-11-21 | Unua skeleto: kontraktoj, kvalita konstitucio, fazoj H1–H9 |
