# 🌀 Vortex CLI

[![Tachyon-Tex](https://img.shields.io/badge/Engine-Tectonic-blueviolet?style=for-the-badge)](https://latex.taptapp.xyz)
[![Latencia](https://img.shields.io/badge/Latency-%3C1s-green?style=for-the-badge)](https://latex.taptapp.xyz)

Ultra-fast LaTeX compilation CLI. Compila tus documentos LaTeX en la nube con latencia insignificante y feedback instantáneo.

## 🚀 Características

- **Ultra-Fast**: Compilación basada en Rust (Tectonic) optimizada para velocidad moonshot.
- **Zero-Config**: No necesitas instalar TeXLive, MikTeX ni dependencias pesadas localmente.
- **Modo Watch**: Detecta cambios en tus archivos y recompila automáticamente en milisegundos.
- **Validación Inteligente**: Feedback preventivo sobre errores de sintaxis y estructura antes de la compilación.
- **Multi-archivo**: Soporte completo para proyectos complejos, bibliografías (`.bib`), estilos (`.sty`) e imágenes.

## 📦 Instalación

Para instalar globalmente:

```bash
git clone https://github.com/srsergio/vortex.git
cd vortex
npm install
npm link
```

## 🛠️ Uso

### Compilación Simple
Especifíca la carpeta que contiene tu archivo `.tex` raíz:

```bash
vortex ./my-paper
```

### Configurar Archivo de Salida
Por defecto genera `output.pdf`, pero puedes personalizarlo:

```bash
vortex ./my-paper --output final_report.pdf
```

### Modo Watch (Desarrollo en Tiempo Real)
Compila cada vez que guardas un archivo en el directorio:

```bash
vortex ./my-paper --watch
```

## 🧠 Workflow del Sistema

1. **Escaneo Local**: La CLI identifica todos los assets necesarios en el directorio.
2. **Pre-flight Audit**: Se envía una versión ligera al endpoint de `/validate` para detectar errores comunes (llaves perdidas, entornos mal cerrados).
3. **Optimized Compilation**: Se envían los archivos vía multipart al motor en la nube.
4. **Instant Sync**: El PDF resultante se descarga y se guarda localmente de inmediato.

## 📡 API Integrada

Esta CLI es el cliente oficial para la infraestructura **Tachyon-Tex**:
- **Endpoint**: `https://latex.taptapp.xyz`
- **Engine**: Tectonic (Rust)
- **Status**: Efímero y Stateless (Máxima privacidad).

---
Hecho con ❤️ por el equipo de **Tachyon-Tex**.
