# 🔒 QuietBox (CryptVault)
### Cifrador de Volúmenes Ocultos con Denegación Plausible
*Desktop Application for Hidden-Volume Container Encryption with Plausible Deniability*

[![CI](https://github.com/alfredgabriel/QuietBox/actions/workflows/ci.yml/badge.svg)](https://github.com/alfredgabriel/QuietBox/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Built with Tauri v2](https://img.shields.io/badge/Built%20with-Tauri%20v2-24C8D8?logo=tauri&logoColor=white)](https://tauri.app/)
[![Rust Crypto](https://img.shields.io/badge/RustCrypto-AES--256--GCM%20%7C%20Argon2id-orange)](https://github.com/RustCrypto)

---

## 🇪🇸 Español

### 1. Visión y Propósito
**QuietBox** es una herramienta de escritorio diseñada para activistas, periodistas, investigadores y personas que cruzan fronteras o se encuentran en situaciones de coacción.

Permite cifrar un archivo o conjunto de carpetas dentro de **dos volúmenes independientes en un único contenedor**:
- **Volumen Señuelo (Decoy)**: Contenido inofensivo que se revela al introducir la *contraseña señuelo*.
- **Volumen Oculto (Hidden)**: Contenido sensible que se revela únicamente con la *contraseña oculta*.

El contenedor resultante es **matemáticamente indistinguible de ruido aleatorio puro (entropía > 7.999 bits/byte)**. Ningún análisis forense puede demostrar la existencia de un segundo volumen oculto.

---

### 2. Arquitectura Criptográfica y Formato de Contenedor

```
[ Contenedor de Tamaño Fijo (ej. 500 MB) ]
Offset 0                                                                  Offset FINAL
│                                                                                    │
├── Cabecera Cifrada Señuelo (512 B) [Salt | Nonce | Longitud | AES-GCM(Header)]     │
├── Datos Cifrados Señuelo (ZIP uncompressed + AES-256-GCM)                          │
├── ... Relleno Aleatorio CSPRNG (OsRng) ...                                         │
│    └── Cabecera + Datos Cifrados del Volumen OCULTO (Posición no determinista)     │
└── Relleno Aleatorio hasta el tamaño total exacto                                   │
```

#### Reglas de Seguridad No Negociables
1. **Sin Bytes Mágicos**: No existen cabeceras en texto claro, firmas de archivo ni identificadores de versión expuestos.
2. **Posicionamiento No Determinista**: La posición del volumen oculto se deriva mediante hash keyed determinista (`BLAKE3`) a partir de la contraseña oculta y el tamaño del contenedor, ubicándose en la mitad superior del archivo.
3. **Tiempo de Respuesta Constante (Constant-Time Response)**: Al abrir un contenedor, la aplicación ejecuta siempre las derivaciones de clave para evitar ataques de canal lateral (*timing attacks*).
4. **Cero Retención en Memoria**: Todas las claves y estructuras sensibles implementan `ZeroizeOnDrop`.
5. **Cifrado Autenticado (AES-256-GCM)**: Cualquier manipulación de bytes se detecta automáticamente.
6. **KDF Resistente a GPU/ASIC**: Utiliza `Argon2id` con parámetros mínimos seguros (por defecto: 256 MB RAM, 3 iteraciones, 4 hilos).

---

### 3. Modo de Uso
1. **Crear Contenedor**:
   - Elige la ruta y el tamaño fijo (ej. 100 MB).
   - Añade los archivos y la contraseña del **Volumen Señuelo**.
   - (Opcional) Activa el **Volumen Oculto**, define su contraseña independiente y el tamaño reservado.
2. **Abrir Contenedor**:
   - Selecciona el contenedor y la carpeta de destino.
   - Introduce **una única contraseña** (la app no pregunta si es señuelo u oculto; detecta y extrae el volumen correspondiente automáticamente).
3. **Cierre de Pánico**:
   - Pulsa `Ctrl + Shift + Q` en cualquier momento para limpiar la memoria y recargar la aplicación al instante.

---

### 4. Avisos Críticos y Limitaciones del Modelo de Amenazas
> ⚠️ **No modifiques el volumen señuelo con herramientas externas tras crear el volumen oculto**: escribir datos en el señuelo sin respetar los límites puede sobrescribir el volumen oculto.
> 
> ⚠️ **Evita backups automáticos con historial de versiones**: herramientas como Dropbox o Google Drive guardan versiones previas, lo que podría revelar cambios en sectores internos del archivo.
> 
> ⚠️ **Fuerza de la contraseña**: Argon2id mitiga ataques por fuerza bruta, pero contraseñas cortas o comunes son vulnerables a diccionarios. Utiliza contraseñas de alta entropía.

---

## 🇬🇧 English

### 1. Vision & Purpose
**QuietBox** is a desktop application tailored for journalists, activists, and individuals under coercive environments.

It allows packing files/folders into **two independent volumes inside a single container**:
- **Decoy Volume**: Harmless files unlocked by the *decoy password*.
- **Hidden Volume**: Sensitive files unlocked only by the *hidden password*.

The resulting container is **statistically indistinguishable from pure cryptographic noise (Shannon entropy > 7.999 bits/byte)**. Even under forensic scrutiny, an adversary cannot prove that a hidden volume exists.

---

### 2. Cryptographic Stack
- **AES-256-GCM**: Authenticated encryption for headers and data payloads.
- **Argon2id**: Memory-hard key derivation function resisting GPU/ASIC attacks.
- **BLAKE3**: Keyed domain-separated offset positioning and integrity verification.
- **Zeroize**: Memory wiping on drop for all key material.
- **Subtle**: Constant-time verification comparisons.

---

### 3. Build & Test Instructions

#### Prerequisites
- [Node.js](https://nodejs.org/) (v18+)
- [Rust & Cargo](https://rustup.rs/) (v1.75+)

#### Running Tests
```bash
# Run all unit and integration tests (crypto, container, KDF, cipher)
cd src-tauri
cargo test --all-targets
```

#### Frontend Type Checking & Build
```bash
npm install
npm run check
npm run build
```

#### Running the Desktop App
```bash
npm run tauri dev
```

---

## 📜 License
Distributed under the **MIT License**. See `LICENSE` for details.
