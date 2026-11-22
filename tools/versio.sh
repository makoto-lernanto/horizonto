#!/usr/bin/env bash
# Kontrolas, ke la versio kongruas en Cargo.toml, README.md kaj HORIZONTO.md.
# Uzado: bash tools/versio.sh --kontroli
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

versio_cargo=$(sed -n 's/^version *= *"\([^"]*\)"/\1/p' Cargo.toml | head -n 1)
versio_readme=$(sed -n 's/^\*\*Versio:\*\* *\([^ ]*\).*/\1/p' README.md | head -n 1)
versio_dokumento=$(sed -n 's/.*skeleto `\([^`]*\)`.*/\1/p' docs/HORIZONTO.md | head -n 1)

if [ -z "$versio_cargo" ]; then
    echo "versio: mankas [workspace.package] version en Cargo.toml" >&2
    exit 1
fi
if [ -z "$versio_readme" ]; then
    echo "versio: mankas '**Versio:** X' en README.md" >&2
    exit 1
fi
if [ -z "$versio_dokumento" ]; then
    echo "versio: mankas 'skeleto \`X\`' en docs/HORIZONTO.md" >&2
    exit 1
fi

kongruas=1
if [ "$versio_cargo" != "$versio_readme" ]; then
    echo "versio: README.md havas $versio_readme, Cargo.toml havas $versio_cargo" >&2
    kongruas=0
fi
if [ "$versio_cargo" != "$versio_dokumento" ]; then
    echo "versio: docs/HORIZONTO.md havas $versio_dokumento, Cargo.toml havas $versio_cargo" >&2
    kongruas=0
fi
if [ "$kongruas" -eq 0 ]; then
    exit 1
fi

echo "versio: $versio_cargo (Cargo.toml, README.md, HORIZONTO.md kongruas)"
