


https://github.com/user-attachments/assets/cdfc4924-c141-4fd3-aa7e-841cee36493f


## Instalación y requisitos
- Requiere Rust (edición 2024) y Cargo.
- Dependencias principales: `raylib` (bindings a Raylib 5), `image`.
- Este proyecto fue desarrollado usando arch linux

## Controles
- Menú principal:
  - Flechas Arriba/Abajo: Navegar niveles.
  - Enter: Iniciar nivel seleccionado.
  - Esc: Salir.
- En juego:
  - W/A/S/D: Movimiento (adelante/izquierda/atrás/derecha).
  - Flechas Izquierda/Derecha: Girar cámara.
  - Mantener clic izquierdo + mover mouse: Girar con el ratón.
  - Esc: Salir.

## Estructura del proyecto
- `src/` código fuente principal:
  - `main.rs`: punto de entrada.
  - `game.rs`: bucle principal, estados (Menú/Juego/Completado), entrada, audio, tiempo.
  - `renderer.rs`: renderizado por raycasting, minimapa, sprites.
  - `level.rs`: carga y parseo de niveles ASCII.
  - `player.rs`: movimiento y colisiones del jugador.
  - `events.rs`: lectura de input.
  - `texture.rs`: carga de texturas con fallbacks de color si no existen.
  - `framebuffer.rs`: utilidades (no usadas en runtime actual).
- `levels/`: niveles ASCII (`level1.txt`, `level2.txt`, `maze.txt`).
- `textures/`: imágenes: `floor.jpg`, `wall1.jpg`, `wall2.jpg`, `guard1.png`, `guard2.png`, `folder.jpg`, `background.png`.
- `music/`: audio: `fondo.mp3` (música), `sonido.mp3` (SFX de recoger folder).

## Lógica del juego
- Estados:
  - Menú: selección de nivel.
  - Jugando: movimiento, recolección de folders, detección de meta.
  - Completado: pantalla de fin de nivel.
- Recolección: al pisar un tile `Folder`, se suma al contador y se reproduce un sonido.
- Meta: al pisar un tile `Goal`, se considera el nivel completado.
- Spawns: el jugador aparece en el primer `Floor` encontrado.


## Sprites y animaciones
- Sprites del mundo: `Folder`, `Guard1` (G), `Guard2` (H).
- Ordenados por distancia para dibujar de atrás hacia adelante.
- Oclusión: se evita dibujar sprites ocultos por paredes.
- Animaciones (basadas en tiempo acumulado):
  - Folder: pulso de escala y bobbing vertical (más notorio para resaltar recolección).
  - Guardias: respiración sutil y bobbing ligero.

## Audio (música y SFX)
- Música de fondo en loop (Ready for it? - Taylor swift - Instrumental): `music/fondo.mp3`.
- SFX de recolección: `music/sonido.mp3` al tomar un folder.
- Volumen de música ajustado a 0.45 por defecto.

## Niveles (formato y progresión)
- Archivos ASCII donde cada carácter representa una celda:
  - Espacio (` `): piso (`Floor`).
  - `+`, `-`, `|`: pared (`Wall`).
  - `g`: meta (`Goal`).
  - `f`: folder/coleccionable (`Folder`).
  - `G`: guardia tipo 1 (`Guard1`).
  - `H`: guardia tipo 2 (`Guard2`).
- Progresión sugerida en este repo:
  - Nivel 1: sin guardias, foco en exploración y recolección.
  - Nivel 2: con 1 guardia (G), diseño renovado.
  - Laberinto: con ambos guardias (G y H) y mayor complejidad.

## Cómo crear/editar niveles
- Edita o crea archivos en `levels/` siguiendo el formato ASCII.
- Reglas útiles:
  - Cierra el mapa con paredes para evitar salir de límites.
  - Coloca al menos una `g` (meta) y opcionalmente `f`, `G`, `H`.
- Para registrar un nuevo nivel en el menú, agrega una entrada en el vector `levels` en `game.rs` (sección `Game::new`).


