# Horizonto

**Versio:** 0.1.0 (Cargo-laborloko)

Horizonto estas la uzantlanda sistemo super la kerno MiKerno: sesio,
kompozitoro, fenestra protokolo, ŝelo, unuaj aplikaĵoj, SDK kaj pakaĵoj.
Ĝi uzas la grafikan kontrakton FluoU kaj la rastrumilon Lumo, sed ĝi mem
posedas la servojn kaj la politikon de la labirtablo.

La normiga priskribo estas [`docs/HORIZONTO.md`](docs/HORIZONTO.md). La
kvalita konstitucio kaj la fazoj troviĝas en tiu dokumento; ĉi tiu README
nur resumas la projekton.

## Tavoloj

| Tavolo | Posedanto | Enhavo |
| --- | --- | --- |
| Mekanismoj | MiKerno | Planado, paĝoj, IPC, kapabloj, peliloj, `SYS_*` |
| Servoj kaj politiko | Horizonto | Sesio, kompozitoro, ŝelo, aplikaĵoj, SDK, pakaĵoj |
| Grafika kontrakto | FluoU | Ĵetonoj, komponantoj, manifesta kaj API-formo |
| Rastrumado | Lumo | Surfaco, ordonaro, miksado, tiparoj |
| Ekosistemo | apartaj projektoj | Dokumentoj, reto, eldonoj |

Dependodirekto: Horizonto → `libmikerno`, FluoU, Lumo. La kerno neniam
dependas de Horizonto.

## Strukturo

```
horizonto/
  Cargo.toml            # laborloko
  justfile              # konstrui kaj la loka pordego
  deny.toml             # permesiloj kaj fontoj
  libhorizonto/         # protokoloj, manifestoj, puraj reguloj
  tools/                # versio, lingvaj kontroloj
  docs/
    HORIZONTO.md        # normiga gvidilo, inkluzive de la fazoj
    KONTRAKTOJ.md       # serva modelo, protokolo, petoj al MiKerno
    PRUVOJ.md           # Kani-pruvoj kaj iliaj taskoj
    ESCEPTOJ.md         # esceptoj al la kvalita konstitucio
    DEPENDOJ.md         # eksteraj krataj kaj iliaj licencoj
```

La servoj (`sesio`, `kompozitoro`, `ligo`, `eniro`, aplikaĵoj) aldoniĝos
laŭ la fazoj. Ĉiu servo estos aparta laborloka membro; `libhorizonto` restas
sen `alloc` kaj sen `unsafe`.

## Fazoj

| Fazo | Versio | Celo |
| --- | --- | --- |
| H1 | 0.1.0 | Kontraktoj, protokolaj tipoj, manifesta validigo, kvalita pordego |
| H2 | 0.2.0 | Kunhava memoro, unua fenestro, kompozitoro, restarigo post paneo |
| H3 | 0.3.0 | Pluraj fenestroj, fokuso, enigo, FluoU-titola kadro |
| H4 | 0.4.0 | Ŝelo Ligo, Eniro, dosierujo, redaktilo, agordoj |
| H5 | 0.5.0 | SDK, subskribitaj pakaĵoj, instalilo, loka deponejo |
| H6 | 0.6.0 | Hejma dosierujo, rubujo, agordoj, alirebleco, tradukoj |
| H7 | 0.7.0 | Uzantlanda reta stako, retpermesoj, ĝisdatigoj |
| H8 | 0.8.0 | ODF-legilo, eldonkandidato, migrado |
| H9 | 1.0.0 | Frostigo de protokoloj kaj SDK, publikigo |

Detaloj, evitindaj eraroj kaj akceptaj kriterioj estas en
[`docs/HORIZONTO.md` §8–§9](docs/HORIZONTO.md#8-fazoj).

## Konstruo

```sh
just kontroli      # versio, lingvo, fmt, Clippy (ambaŭ arkoj), testoj, konstruo
just miri          # Miri super la testoj de libhorizonto (nokta ilĉeno)
just permesiloj    # cargo deny check (postulas cargo-deny)
```

`just kontroli` estas la loka pordego: ĝi ne rulas Kani kaj ne bezonas reton.
Loke oni ne rulas `cargo kani`; la pruvoj rulas en la DI-matrico de GitHub
Actions. La detaloj estas en la kvalita konstitucio de la normiga dokumento.

## Versio

La sola fonto de la versio estas `[workspace.package] version` en
`Cargo.toml`. La dokumentoj kaj la README kongruas kun ĝi kaj estas
kontrolataj de `tools/versio.sh --kontroli`.

## Licenco

MIT. Vidu [`LICENSE`](LICENSE).
