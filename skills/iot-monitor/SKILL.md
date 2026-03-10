---
name: iot-monitor
description: "Embedded Rust sensor monitoring for no_std environments and WASM edge targets. Reads from GPIO, I2C, SPI interfaces via embedded-hal traits, processes readings with zero-allocation algorithms, and publishes alerts to Telegram or MQTT without heap allocation. Designed for microcontrollers (STM32, RP2040) and Cloudflare Workers edge runtime. Cross-ported from PicoClaw's Go iot-monitor for Rust/embedded-hal ecosystem."
---

# iot-monitor

Embedded Rust sensor monitoring — no_std compatible, zero-allocation, WASM-ready.

## Usage
```
@ZeroClaw iot-monitor --target rp2040 --sensor bme280 --i2c-addr 0x76
@ZeroClaw iot-monitor --target stm32f4 --sensor thermocouple --spi-cs PA4
@ZeroClaw iot-monitor --target cloudflare-worker --endpoint sensors.example.com
@ZeroClaw iot-monitor --compile-check    # Verify no_std compatibility without flashing
```

## Embedded vs. Edge Modes

| Mode | Target | Runtime | Heap |
|------|--------|---------|------|
| `embedded` | STM32, RP2040 | bare metal / RTOS | no_std, no heap |
| `linux-edge` | RPi, LicheeRV | Linux | std, minimal |
| `wasm-edge` | Cloudflare Worker, Deno | WASM | std, restricted |

## Supported Sensors (embedded-hal drivers)

| Sensor | Interface | Measurement |
|--------|-----------|-------------|
| BME280 | I2C / SPI | Temp, humidity, pressure |
| ADS1115 | I2C | 4-channel 16-bit ADC |
| DS18B20 | 1-Wire | Temperature (-55°C to +125°C) |
| MAX31865 | SPI | PT100/PT1000 RTD thermocouple |
| HC-SR04 | GPIO | Ultrasonic distance |

## Generated Code (no_std, zero-allocation)

```rust
// Generated monitor — works on RP2040 with 264KB RAM
#![no_std]
#![no_main]

use embedded_hal::i2c::I2c;
use bme280::Bme280;

fn check_temp<I: I2c>(i2c: I, threshold: f32) -> Result<Alert, SensorError> {
    let mut sensor = Bme280::new_primary(i2c);
    let reading = sensor.measure()?;
    if reading.temperature > threshold {
        Ok(Alert::High { value: reading.temperature })
    } else {
        Ok(Alert::Normal)
    }
}
```

## Alert Delivery (zero-copy)

For embedded targets, alerts are sent via:
- UART serial to a connected PicoClaw (which forwards to Telegram)
- MQTT over lwIP stack (embassy-net)
- Direct HTTP via embedded-tls (WASM edge)

## Files Created
```
src/monitors/<sensor>_monitor.rs    # Generated monitor module
config/sensors.toml                  # Sensor config + thresholds
MONITOR_SETUP.md                     # Wiring guide + flash instructions
```

## Philosophy
ZeroClaw goes where PicoClaw can't — into microcontrollers with <300KB RAM. No OS. No heap. Just direct metal access with Rust's safety guarantees. The firmware that never crashes.
