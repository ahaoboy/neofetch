for f in assets/*.ans; do
    [ -e "$f" ] || continue

    out="${f%.ans}.svg"
    ansi2 "$f" > "$out"
done