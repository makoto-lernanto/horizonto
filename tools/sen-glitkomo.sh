#!/usr/bin/env bash
# Rifuzas glitkomon (f32/f64 kaj glitkomajn literalojn), `unsafe` kaj
# `alloc`-tipojn en `libhorizonto`. Komentoj kaj ĉenaj literaloj estas
# ignorataj, ĉar versioj kiel "0.1" ne estas glitkomo en la kodo.
# Uzado: bash tools/sen-glitkomo.sh --kontroli
set -euo pipefail

cd "$(dirname "$0")/.."

modo=${1:---kontroli}
case "$modo" in
    --kontroli) ;;
    *)
        echo "uzado: $0 [--kontroli]" >&2
        exit 2
        ;;
esac

trovoj=$(
    find libhorizonto/src -name '*.rs' -type f -print0 |
        xargs -0 awk '
            {
                lino = $0
                # Forigu linikomenton por ne kapti "0.1" en dokumentoj.
                komento = index(lino, "//")
                if (komento > 0) {
                    lino = substr(lino, 1, komento - 1)
                }
                if (lino ~ /(^|[^[:alnum:]_])unsafe([^[:alnum:]_]|$)/) {
                    printf "%s:%d: unsafe\n", FILENAME, FNR
                    next
                }
                if (lino ~ /(^|[^[:alnum:]_])(alloc|Vec|String|Box)([^[:alnum:]_]|$)/) {
                    printf "%s:%d: alloc-tipo\n", FILENAME, FNR
                    next
                }
                if (lino ~ /(^|[^[:alnum:]_])(f32|f64)([^[:alnum:]_]|$)/) {
                    printf "%s:%d: glitkoma tipo\n", FILENAME, FNR
                    next
                }
                # Ĉenaj literaloj enhavas versiojn; ne rigardu ilin.
                if (lino ~ /"/) {
                    next
                }
                if (lino ~ /(^|[^[:alnum:]_])[0-9]+\.[0-9]+([^[:alnum:]_]|$)/ ||
                    lino ~ /(^|[^[:alnum:]_])[0-9]+[eE][+-]?[0-9]+([^[:alnum:]_]|$)/) {
                    printf "%s:%d: glitkoma literalo\n", FILENAME, FNR
                }
            }
        '
)

if [ -n "$trovoj" ]; then
    echo "sen-glitkomo: glitkomo trovita:" >&2
    printf '%s\n' "$trovoj" >&2
    exit 1
fi

echo "sen-glitkomo: neniu glitkomo, unsafe aŭ alloc en libhorizonto"
