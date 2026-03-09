# Anuntech Desktop

Aplicativo desktop para a plataforma [Anuntech](https://anun.tech), construído com [Tauri v2](https://v2.tauri.app).

## Download

Baixe a versão mais recente na página de [Releases](https://github.com/anuntech/anuntech-desktop-updater/releases).

| Plataforma | Arquivo |
|---|---|
| macOS (Apple Silicon) | `Anuntech_x.x.x_aarch64.dmg` |
| macOS (Intel) | `Anuntech_x.x.x_x64.dmg` |
| Windows | `Anuntech_x.x.x_x64-setup.exe` |
| Linux (Debian/Ubuntu) | `Anuntech_x.x.x_amd64.deb` |
| Linux (AppImage) | `Anuntech_x.x.x_amd64.AppImage` |

## Funcionalidades

- Janela nativa sem barra de navegação do browser
- Links externos abrem no navegador do sistema
- Atualizações automáticas via GitHub Releases

## Desenvolvimento

### Pré-requisitos

- [Node.js](https://nodejs.org) 18+
- [Rust](https://rustup.rs)
- Dependências do Tauri para seu SO — veja o [guia oficial](https://v2.tauri.app/start/prerequisites/)

### Setup

```bash
npm install
npm run dev
```

### Build

```bash
TAURI_SIGNING_PRIVATE_KEY="$(cat ~/.tauri/anuntech.key)" \
TAURI_SIGNING_PRIVATE_KEY_PASSWORD="" \
npm run build
```

## Release

Para lançar uma nova versão:

1. Atualize `version` em `src-tauri/tauri.conf.json`
2. Commit e push
3. Crie uma tag: `git tag vX.X.X && git push origin vX.X.X`
4. O GitHub Actions builda automaticamente para macOS, Windows e Linux e cria o Release

## Tecnologias

- [Tauri v2](https://v2.tauri.app) — Framework desktop multiplataforma
- [Rust](https://www.rust-lang.org) — Backend nativo
- [tauri-plugin-updater](https://github.com/tauri-apps/plugins-workspace) — Atualizações automáticas
