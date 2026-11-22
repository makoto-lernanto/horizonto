#!/usr/bin/env bash
# Rifuzas la ĉinan skribon en la fonta tavolo (HORIZONTO.md H11).
# La permesitaj tradukaj mapadoj estas apartaj dosieroj; ĉi tiu kontrolo
# celas la fontan deponejon.
# Uzado: bash tools/sen-cina.sh --kontroli
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

patrono='[\x{2E80}-\x{2EFF}\x{3000}-\x{303F}\x{3040}-\x{30FF}\x{3400}-\x{4DBF}\x{4E00}-\x{9FFF}\x{F900}-\x{FAFF}\x{AC00}-\x{D7AF}\x{FF00}-\x{FFEF}]'
trovoj=$(grep -rIPn --exclude-dir=.git --exclude-dir=target "$patrono" . || true)

if [ -n "$trovoj" ]; then
    echo "sen-cina: ĉina skribo trovita en la fonta tavolo:" >&2
    printf '%s\n' "$trovoj" >&2
    exit 1
fi

echo "sen-cina: neniu ĉina skribo en la fonta tavolo"
