// Erzeugt 4 Solana-Keypairs und schreibt die Private Keys in .env
import { generateKeyPairSync } from "node:crypto";
import { writeFileSync, existsSync } from "node:fs";

const ALPHABET = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
function base58(buf) {
  let n = BigInt("0x" + Buffer.from(buf).toString("hex"));
  let out = "";
  while (n > 0n) { out = ALPHABET[Number(n % 58n)] + out; n /= 58n; }
  for (const b of buf) { if (b === 0) out = "1" + out; else break; }
  return out;
}
function newWallet() {
  const { privateKey, publicKey } = generateKeyPairSync("ed25519");
  const seed = privateKey.export({ format: "der", type: "pkcs8" }).subarray(-32);
  const pub = publicKey.export({ format: "der", type: "spki" }).subarray(-32);
  const secret = Buffer.concat([seed, pub]); // 64 Byte, Phantom/Solana-CLI-Format
  return { pub: base58(pub), secretBase58: base58(secret), secretJson: JSON.stringify([...secret]) };
}

if (existsSync(".env")) { console.error(".env existiert schon - abgebrochen (nichts überschrieben)."); process.exit(1); }
const names = ["PROGRAM_SPLITTER", "RECIPIENT_1", "RECIPIENT_2", "RECIPIENT_3"];
let env = "# NIEMALS committen oder teilen! Wer diese Keys hat, hat Zugriff auf die Wallets.\n";
for (const n of names) {
  const w = newWallet();
  env += `\n${n}_PUBLIC_KEY=${w.pub}\n${n}_PRIVATE_KEY=${w.secretBase58}\n${n}_KEYPAIR_JSON=${w.secretJson}\n`;
  console.log(n, w.pub);
}
writeFileSync(".env", env, { mode: 0o600 });
