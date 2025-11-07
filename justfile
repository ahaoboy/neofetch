default:
  just --list

bloaty-build:
  cargo build --profile bloaty
bloaty-csv:
  bloaty ./target/bloaty/neofetch -d sections,symbols -n 0 --csv > meta.csv
bloaty-json:
  bloaty-metafile meta.csv > meta.json
bloaty: bloaty-build bloaty-csv bloaty-json
