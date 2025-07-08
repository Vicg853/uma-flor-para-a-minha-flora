# Uma flor para a minha Flor(a)

## Instructions
### Déjà, on commence par rust ;D
- On vérifie si tu l'as déjà installé Flo (ou si moi je te l'ai déjà fait en fait quoi)
```bash
rustup toolchain list # Si la commande est missing, donc, tu ne l'as pas, sinon, une liste de version devrait apparaître
```

- Si tu ne l'as pas
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Mtn, juste on le tourne :)
```bash
cargo run
```
Ça prend un p'tit peu de temps tqtt et ça imprime pleeiiinn des trucs

### Si tu veux une version opti et permanente
- On build le programme en optimizé (c'est encore plus long en théorie ...)
```bash
cargo build --release
```

- On installe le bundler (l'outil pour transformer l'executable dans un programme macos valide)
```bash
cargo install cargo-bundle
```

- On rend l'executable un appli (le target indique vers quel genre d'appli l'outil doit bundler : un exe windows, un .app macos, ...)
```bash
cargo bundle --release --target aarch64-apple-darwin
```

- Voiláaa, l'appli est sous : ``target/aarch64-apple-darwin/release/bundle/osx/r_macroquad.app`` et tu peux la placer sous Applications de ton PC
