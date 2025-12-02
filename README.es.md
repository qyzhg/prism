# Prism - Software de traducción con IA

<div align="center">

**[English](./README.md) | [中文](./README.zh.md) | [Español](./README.es.md) | [日本語](./README.ja.md) | [한국어](./README.ko.md) | [Tiếng Việt](./README.vi.md)**

🚀 Una potente aplicación de traducción multiplataforma impulsada por modelos de lenguaje avanzados y tecnología OCR.

[Descarga](#descarga) • [Funciones](#funciones) • [Inicio-rápido](#inicio-rápido) • [Documentación](#documentación)

</div>

---
<img src="icon.png" alt="Icono" width="350" height="350">

## Funciones

- **🌍 Compatibilidad multiplataforma** - Soporte completo para Windows, macOS y Linux con las mismas funciones
- **🤖 Traducción IA avanzada** - Basado en el modelo Tencent Hunyuan-MT-7B para traducciones precisas con contexto
- **📸 OCR integrado** - Extrae y traduce texto directamente desde capturas con Qwen3-VL-8B-Instruct
- **⚡ Traducción ultrarrápida** - Traducción en tiempo real con latencia mínima
- **🎯 Interfaz fácil de usar** - UI intuitiva en Vue 3 con interacciones fluidas
- **🔗 Atajos globales** - Atajos personalizables para acceso rápido (en desarrollo)
- **💾 Historial local** - Almacena traducciones en SQLite de forma local
- **🎨 Arquitectura moderna** - Construido con Tauri + Rust para máximo rendimiento y seguridad

---

## Stack tecnológico

### Frontend

- **Vue 3** (3.5.13) - Framework JavaScript progresivo y moderno
- **Vite** (6.0.3) - Herramienta de build de nueva generación
- **Componentes UI de Tauri** - Experiencia nativa en escritorio

### Backend

- **Rust** (edición 2021) - Lenguaje de sistemas de alto rendimiento
- **Tauri** (2.9.3) - Framework ligero para apps de escritorio
- **Tokio** (1.48.0) - Runtime asíncrono para concurrencia

### IA y procesamiento

- **Modelo de traducción** - Tencent Hunyuan-MT-7B
- **Modelo de OCR** - Qwen3-VL-8B-Instruct
- **Proveedor API** - SiliconFlow
- **Compatibilidad completa con APIs tipo OpenAI para modelos personalizados**

### Almacenamiento y librerías

- **SQLite** (rusqlite 0.37.0) - Base de datos local
- **Reqwest** (0.12.24) - Cliente HTTP
- **Procesamiento de imágenes** (0.25.9) - Capturas y manipulación de imágenes
- **Atajos globales** (2.3.1) - Plugin de atajos de teclado

---

## Inicio-rápido

### Requisitos previos

- Rust 1.91.0 o superior
- Node.js 18+ y pnpm
- Git

### Instalación

**1. Clonar el repositorio**
```bash
git clone https://github.com/qyzhg/prism.git
cd prism
```

**2. Instalar dependencias**
#### Dependencias frontend
```bash
pnpm install
```

#### Dependencias Rust gestionadas por Cargo

**3. Obtener API Key**
- Usa tu propia URL base compatible con OpenAI y tu API key para empezar.
- Regístrate en SiliconFlow con nuestro enlace de invitación: [https://cloud.siliconflow.cn/i/QhM7Qyuq](https://cloud.siliconflow.cn/i/QhM7Qyuq) y consigue créditos gratuitos (larga validez).

**4. Ejecutar en modo desarrollo**
```bash
pnpm tauri dev
```

**5. Compilar para producción**
```bash
pnpm tauri build
```

---

## Descarga

| Plataforma | Enlace |
|-----------|--------|
| 🪟 Windows | [Última versión](https://github.com/qyzhg/prism/releases) |
| 🍎 macOS | [Última versión](https://github.com/qyzhg/prism/releases) |
| 🐧 Linux | Próximamente |

### Notas de instalación en macOS

Prism está firmado ad-hoc (los certificados Developer ID son de pago), por lo que Gatekeeper mostrará una advertencia la primera vez.

1. Mueve `Prism.app` a `/Applications`.
2. Abre **Terminal** y ejecuta:
   ```bash
   xattr -cr /Applications/prism.app
   sudo spctl --add --label Prism /Applications/prism.app
   ```
3. Haz clic derecho en la app, elige **Open** y confirma una vez. Las próximas ejecuciones serán normales.

---

## Documentación

### Configuración

Gestiona tus preferencias en el panel de ajustes:

- Selección de idioma origen/destino por defecto
- Gestión de API keys
- Personalización de atajos (en desarrollo)

### Atajos

En desarrollo - Próximamente

### Modelos de IA

- **Modelo de traducción** - `tencent/Hunyuan-MT-7B` traducción multilingüe empresarial
- **Modelo de OCR** - `Qwen/Qwen3-VL-8B-Instruct` visión-lenguaje avanzada

---

## Hoja de ruta

- [x] Funcionalidad básica de traducción
- [x] Integración de OCR por captura
- [x] Configuración de atajos personalizados
- [ ] Memoria de traducción y gestión de glosarios
- [ ] Traducción por lotes de archivos
- [ ] Ecosistema de plugins
- [ ] App complementaria móvil

---

## Preguntas frecuentes (FAQ)

**P: ¿Puedo usarlo gratis?**  
R: Sí. Regístrate en SiliconFlow con nuestro enlace para obtener créditos gratuitos suficientes para uso prolongado.

**P: ¿Qué idiomas se soportan?**  
R: El modelo Tencent Hunyuan-MT-7B cubre varios idiomas principales (chino, inglés, japonés, coreano y más). También puedes usar el modelo que prefieras.

**P: ¿Se guarda mi información?**  
R: El historial de traducción se guarda localmente en SQLite y nunca se sube a servidores. Tu privacidad está protegida.

**P: ¿Puedo usarlo sin conexión?**  
R: Los modelos en línea requieren conexión. Con modelos locales, el uso sin conexión es posible.

**P: ¿Cuándo estarán listos los atajos?**  
R: La función de atajos está en desarrollo y llegará pronto.

---

## Contribuir

Se aceptan Issues y Pull Requests. ¡Tus contribuciones son bienvenidas!

---

## Licencia

Proyecto bajo licencia MIT - consulta [LICENSE](LICENSE) para más detalles.

---

## Agradecimientos

- Construido con [Tauri](https://tauri.app/)
- Servicios de traducción durante el desarrollo proporcionados por [SiliconFlow](https://siliconflow.cn/)
- UI basada en [Vue 3](https://vuejs.org/)

---

## Ayuda

- 🐛 Reporte de bugs: [GitHub Issues](https://github.com/qyzhg/prism/issues)

---

<div align="center">

❤️ Desarrollado por el equipo Prism@pity

**[⬆ Volver arriba](#prism---software-de-traducción-con-ia)**

</div>
