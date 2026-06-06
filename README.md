# plato-quickstart

> **`cargo install plato-quickstart` and you have a Plato room in 30 seconds.**

```bash
cargo install plato-quickstart
mkdir my-room && cd my-room
plato init          # creates room.json
plato validate .    # checks it
plato simulate      # watch it tick
```

## What You Get

```
$ plato simulate

Simulating engine_room (100 ticks, 1Hz)...

    Tick       Coolant       RPM     Oil PSI  Status
  ─────────────────────────────────────────────────────────────────
      1      82.1°C     1801      45.2     normal
     10      83.4°C     1805      44.8     normal
     20      87.2°C     1812      44.1     normal
📈   30      91.5°C     1820      43.5     rising ↑
🔴   40      95.8°C     1835      42.8     🔴 OVERHEAT
⚡   41      95.3°C     1838      42.7     ⚡ alarm_bell=ON, rpm_limit=1500
     50      93.1°C     1502      43.2     cooldown
     60      89.4°C     1500      44.0     stabilizing
     70      84.2°C     1500      44.8     recovered
     80      82.0°C     1800      45.1     normal

  ✓ 100 ticks, 1 alarm fired, 1 resolution
  Cadence: alarm → action → resolve (PERFECT)
```

## Commands

| Command | What |
|---------|------|
| `plato init` | Creates `room.json` — 3 sensors, 2 actuators, 2 alarm rules |
| `plato validate [file]` | Validates a room config (checks required fields) |
| `plato simulate` | Runs a 100-tick simulation with crisis + recovery |
| `plato fleet` | Creates `fleet.json` — 5 rooms on a fishing boat |
| `plato help` | Usage info |

## The Room Config

`plato init` creates a room.json that defines:

- **Sensors**: What the room measures (temperature, RPM, pressure)
- **Actuators**: What the room can control (alarm bell, RPM limiter)
- **Alarms**: When to act (coolant > 95°C → sound alarm + reduce RPM)

This config is the same format used by [plato-engine-block](https://github.com/SuperInstance/plato-engine-block) (the real runtime). The quickstart tool generates configs; the engine block runs them.

## The Fleet Manifest

`plato fleet` creates a fleet.json connecting 5 rooms:

```
Engine Room (1 Hz) ─┐
Backdeck (2 Hz)     │
Wheelhouse (1 Hz)   ├─→ Watchdog Agent @ 192.168.1.1:9090
Galley (0.017 Hz)   │
Bilge (0.5 Hz)     ─┘
```

Different tick rates form a polyrhythmic ensemble — the same pattern that [plato-music-sync](https://github.com/SuperInstance/plato-music-sync) coordinates.

## Why This Exists

The Plato Matrix is 10 crates across 3 languages. That's a lot to absorb before you can do anything. This tool is the on-ramp: one command to create a config, one to validate it, one to see it in action. No reading required to get started.

## Zero Dependencies

This crate has zero Rust dependencies. It compiles in seconds and runs anywhere. The configs it generates work with the full Plato stack, but you don't need any of it installed to use this tool.

## Related

- [plato-engine-block](https://github.com/SuperInstance/plato-engine-block) — The real room runtime (Rust)
- [plato-engine-block-c](https://github.com/SuperInstance/plato-engine-block-c) — Bare-metal C runtime (ESP32)
- [plato-demo](https://github.com/SuperInstance/plato-demo) — Full 80-tick fishing boat demo
- [SuperInstance](https://github.com/SuperInstance/SuperInstance) — The ecosystem
