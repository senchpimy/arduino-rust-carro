# 🚗 Arduino Avoider Bot

![Badge](https://img.shields.io/badge/Platform-Arduino-blue) ![Badge](https://img.shields.io/badge/Language-Rust-orange) ![Badge](https://img.shields.io/badge/License-MIT-green)

Este proyecto implementa un pequeño robot autónomo que avanza hacia adelante y evita obstáculos utilizando un sensor ultrasónico. El código está escrito completamente en Rust usando [`arduino-hal`](https://crates.io/crates/arduino-hal), y es compatible con placas basadas en el **ATmega328P** (como el Arduino Uno).

---

## 🧠 Características

* Navegación automática hacia adelante
* Detección de obstáculos con sensor ultrasónico (HC-SR04)
* Giros automáticos al detectar un obstáculo
* Escrito en Rust (`no_std`, `no_main`)
* Compilación y flasheo simplificados con un script `flash`

---

## ⚙️ Requisitos

* Placa Arduino con microcontrolador ATmega328P
* Sensor ultrasónico (HC-SR04)
* 4 pines de control para motores (pueden ser conectados a un driver L298N, por ejemplo)
* Rust toolchain con soporte para AVR:

  ```sh
  rustup target add avr-atmega328p
  ```
* [`cargo-binutils`](https://github.com/rust-embedded/cargo-binutils) y `avr-gcc`, `avrdude` instalados
* Puerto de la placa (ej: `/dev/ttyACM0`)

---

## 📦 Pinout

| Función           | Pin Arduino |
| ----------------- | ----------- |
| Motor Izq. Adel.  | D4          |
| Motor Izq. Atrás  | D5          |
| Motor Der. Adel.  | D6          |
| Motor Der. Atrás  | D7          |
| Trigger ultrasón. | D12         |
| Echo ultrasónico  | D13         |

---

## 🚀 Compilación y carga

Puedes flashear tu programa al Arduino ejecutando:

```sh
./flash.sh avr-atmega328p.json
```

Este script hace lo siguiente:

1. Compila el código con `cargo build`
2. Usa `avrdude` para flashear el binario `.elf` al microcontrolador

> Asegúrate de tener permisos para acceder al puerto serie o ejecuta con `sudo`.

---

## 🧾 Lógica de funcionamiento

1. El robot avanza continuamente.
2. Envía un pulso ultrasónico y espera el rebote.
3. Si la distancia detectada es menor a `10 cm`, se detiene y gira a la izquierda.
4. Repite el ciclo.

---

## 🛠️ Personalización

Puedes ajustar los siguientes parámetros en el código:

```rust
const MINIMAL_DISTANCE: u16 = 10u16;     // Distancia mínima en cm para girar
const TRIGGER_UP_TIME: u16 = 10u16;      // Duración del pulso TRIG
const TURNING_TIME: u16 = 700u16;        // Tiempo que gira al detectar obstáculo
const WAIT_BETWEEN_ACTIONS: u16 = 1000;  // Delay después de girar
```

---

## 🧪 Debug

En caso de errores:

* Verifica la conexión de los pines del sensor y motores.
* Asegúrate de que el puerto en el script `flash` coincide con el de tu Arduino (`/dev/ttyACM0`).
* Usa `dmesg` o `ls /dev/tty*` para identificar el puerto correcto.

---

## 📄 Licencia

Este proyecto está bajo la licencia MIT. Puedes usarlo, modificarlo y distribuirlo libremente.

