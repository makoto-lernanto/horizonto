# Horizonto — Kani-pruvoj (H1)

> **Versio:** 0.1.0 · **Pordego:** GitHub Actions `kani-pretigi` + `kani-kontroli`
>
> Ĉiu pruvo havas unu taskon en la DI-matrico. Loke oni **DEVI NE** ruli
> `cargo kani` (Q02); la matrico estas la sola formala pordego. La nomoj sube
> estas la modulaj nomoj; `cargo kani list` eligas la plenajn nomojn, kiujn la
> matrico uzas.

## 1. Celo

La pruvoj celas konkretajn regulojn de `HORIZONTO.md` §4 kaj §5, ne
`true == true`. Ĉiu pruvo estas pura funkcio super fiksaj tabeloj; ĝi ne
aliras aparataron, reton aŭ la dosiersistemon.

## 2. Katalogo

| ID | Pruvo | Regulo | Priskribo | Tempolimo |
| --- | --- | --- | --- | --- |
| P1 | `protokolo::kani_pruvoj::totala_legado` | S01–S04 | Por arbitraj bajtoj kaj arbitra longo ĝis 64, `Mesaĝo::analizi` neniam legas ekster la bufron; sukceso respektas version, tipon kaj longon. | 600 s |
| P2 | `damago::kani_pruvoj::ortangula_kunigo` | S08, D02, D05 | Por du arbitraj ortanguloj ene de 16×16, la damaĝaro restas en sia kapacito, ĉiu ero eniĝas la surfacon kaj kovras la originalojn. | 600 s |
| P3 | `damago::kani_pruvoj::plena_damagxaro_rifuzas` | D02 | Plena damaĝaro rifuzas plian ortangulon kaj konservas sian nombron; neniu damaĝo estas silente forigita. | 600 s |
| P4 | `fenestro::kani_pruvoj::generacia_reuzo` | S05 | Por arbitra valida identigilo, la sekva generacio (kiam ĝi ekzistas) ŝanĝas la identigilon kaj malnova identigilo estas rifuzata; generacio nulo kaj indekso ≥ 64 estas rifuzataj. | 600 s |
| P5 | `manifesto::kani_pruvoj::manifesta_kopio` | S16–S17 | Por ĉiu longo ĝis unu pli ol la maksimumo, la fiksa kopio sukcesas nur ene de la bufero, kopias la ĝustajn bajtojn kaj lasas la ceteron netuŝita. | 600 s |
| P6 | `manifesto::kani_pruvoj::manifesta_kapacito` | S20–S22 | Plena tabelo de ok dosierujoj rifuzas plian radikon per `TroMultajDosierujoj` sen ŝanĝi sian nombron, kaj nombro naŭ estas rifuzata antaŭ tranĉo. | 600 s |

La minimumo por H1 estas kvar pruvoj; ĉi tiu Fazo havas ses.

## 3. Vokado

La DI-tasko eltrovas la plenajn nomojn per:

```sh
cargo kani -p libhorizonto -Z unstable-options list --format json
```

kaj poste rulas unu pruvon:

```sh
timeout -k 10 600 cargo kani -p libhorizonto -Z unstable-options \
  --exact --harness "<plena nomo>"
```

`--exact` malhelpas, ke prefiksa nomo elektu ankaŭ alian pruvon.
`timeout-minutes: 30`, `fail-fast: false`, kaj `kani.log` supreniĝas kiel
artefakto eĉ ĉe malsukceso.

## 4. Kiam pruvo malsukcesas

Oni riparu la kaŭzon, ne malstreĉu la pruvon. Oftaj kaŭzoj estas mempremo de
CBMC: malplenigu la datumaron, malgrandigu la tabelon aŭ `unwind`, aŭ
anstataŭigu tut-tabelan egalecon per skalaraj komparoj. Neniu pruvo povas esti
forigita aŭ malŝaltita sen escepto en [`ESCEPTOJ.md`](ESCEPTOJ.md).

## 5. Rilato al la unu-testoj

La unu-testoj en `libhorizonto` kovras la samajn regulojn per konkretaj
ekzemploj (validaj kaj malvalidaj manifestoj, nekonata permeso, malkongrua
`fluo`, mallonga mesaĝo, nekonata tipo, generacia reuzo). Kani pruvas la
limojn por arbitraj enigoj; Miri kontrolas la unu-testojn por nedifinita
konduto.
