# Horizonto — esceptoj al la kvalita konstitucio

> **Versio:** 0.1.0 · **Normo:** `HORIZONTO.md` §6.5 punkto 7

Neniu nova `#[allow]`, `unsafe`, malŝaltita testo aŭ malstreĉita pruvo ekzistas
en H1. Ĉi tiu dosiero estas la sola loko, kie tia escepto povas esti
registrita; sen vico ĉi tie la pordego estas malverda.

## Registro

| ID | Dosiero / loko | Escepto | Kialo | Aprobo | Fino |
| --- | --- | --- | --- | --- | --- |
| — | — | Neniu | — | — | — |

## Reguloj

- `libhorizonto` uzas `#![forbid(unsafe_code)]`; `unsafe` ne estas esceptebla
  en tiu kesto.
- `#[cfg(test)]` ne estas malŝaltita testo: la testoj ruliĝas per
  `cargo test --workspace`.
- Kani-pruvo ne rajtas esti forigita aŭ malŝaltita por igi la matricon verda.
  Riparu la kaŭzon laŭ [`PRUVOJ.md`](PRUVOJ.md).
- La malpermeso ruli `cargo kani` loke (Q02) estas regulo de la konstitucio,
  ne escepto; ĝi ne apartenas al ĉi tiu registro.
