# SOL-Splitter (4 Wallets)

- `generate-wallets.mjs` erzeugt 4 Keypairs -> `.env` (gitignored, chmod 600)
  - `PROGRAM_SPLITTER_*`: Program-ID-Keypair (damit deployst du das Programm)
  - `RECIPIENT_1..3_*`: die drei Empfänger-Wallets
- `program/`: Rust-Programm. Verteilt das Vault-Guthaben (PDA `["vault"]`) in 3 gleiche Teile.

## Wichtig
Smart Contracts auf Solana können **nicht von selbst** reagieren, wenn SOL eintrifft.
Jemand (du, ein Bot, ein Cron) muss die `distribute`-Instruction aufrufen.

## Deploy (lokal, mit Solana CLI)
```
cd program && cargo build-sbf
# Keypair-Datei aus .env (PROGRAM_SPLITTER_KEYPAIR_JSON) als program-keypair.json speichern
solana program deploy target/deploy/sol_splitter.so --program-id program-keypair.json
```
Zum Deployen braucht die Fee-Payer-Wallet ca. 2-3 SOL (Rent für das Programm).
Das Programm wurde **nicht kompiliert/getestet** – bitte zuerst auf devnet probieren.
