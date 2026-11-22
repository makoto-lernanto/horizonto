# Horizonto — dependaĵoj

> **Versio:** 0.1.0 · **Normo:** `HORIZONTO.md` §6.3 Q08

## Eksteraj krataj

Neniu. `libhorizonto` uzas nur `core` (kaj `std` nur en la testoj); la
laborloko ne havas eksteran Rust-dependaĵon, C-kodon aŭ binaran blobon.

| Krato | Versio | Licenco | Kialo | Revizia dato |
| --- | --- | --- | --- | --- |
| — | — | — | Neniu ekstera krato | — |

## Laborlokaj membroj

| Membro | Rolo | Eksteraj dependaĵoj |
| --- | --- | --- |
| `libhorizonto` | Protokoloj, fenestra identigilo, damaĝo, enigo, manifesto | Neniu |

## Reguloj

- Ĉiu nova ekstera krato bezonas vicon supre kun versio, licenco, kialo kaj
  revizia dato antaŭ ol eniri `Cargo.toml`.
- Permesataj licencoj estas la samaj kiel en `deny.toml`: MIT, Apache-2.0,
  BSD-3-Clause, Unicode-DFS-2016. Nova licenco postulas skriban aprobon de la
  uzanto.
- Nur pura Rust: neniu C, neniu C++, neniu binara blobo.
