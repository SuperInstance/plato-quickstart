# plato-quickstart

**Bootstrap a Plato room in 30 seconds** — sensor configurations, alarm rules, and fleet manifests for real-time physical-space monitoring.

`plato-quickstart` is a CLI tool that scaffolds the JSON configuration files needed to run a "room" in the Plato monitoring system. Each room defines sensors (temperature, RPM, pressure), actuators (alarms, limiters), and alarm rules with cooldown semantics. A fleet manifest ties multiple rooms together under a coordinator.

## Why It Matters

Instrumented spaces — engine rooms, server halls, greenhouses — require careful configuration of sensor ranges, alarm thresholds, and response actions. Doing this by hand is error-prone and inconsistent. `plato-quickstart` generates validated, reproducible configurations in seconds, ensuring every room follows the same schema and alarm semantics.

## How It Works

### Room Model

A room is defined by:

$$\mathcal{R} = (\mathcal{S}, \mathcal{A}, \mathcal{K})$$

where $\mathcal{S} = \{s_1, \ldots, s_n\}$ is the sensor set, $\mathcal{A} = \{a_1, \ldots, a_m\}$ is the actuator set, and $\mathcal{K} = \{k_1, \ldots, k_p\}$ is the alarm rule set.

Each sensor $s_i$ has a range $[s_i^{\min}, s_i^{\max}]$ and a normal operating range $[s_i^{\text{lo}}, s_i^{\text{hi}}]$. An alarm $k_j$ fires when its boolean condition $c_j(\mathcal{S})$ evaluates true and its cooldown timer has expired.

### Alarm Cooldown

When alarm $k_j$ fires at tick $t$, it cannot fire again until tick $t + \delta_j$ where $\delta_j$ is the cooldown in ticks. This prevents alarm storms:

$$\text{fire}(k_j, t) = c_j(\mathcal{S}(t)) \wedge (t - t_{\text{last}}(k_j) \geq \delta_j)$$

### Tick Simulation

The room runs at a configurable frequency $f$ (Hz). Each tick evaluates all sensor values against alarm conditions. The bundled simulation demonstrates an overheat scenario:

1. **Ticks 1–20**: Normal operation (~82°C, ~1800 RPM)
2. **Tick 40**: Temperature crosses 95°C → `engine_overheat` fires
3. **Tick 41**: Actions execute: `alarm_bell = true`, `rpm_limit = 1500`
4. **Ticks 50–70**: Cooldown, temperature stabilizes and recovers
5. **Ticks 80–100**: Return to normal

### Fleet Topology

A fleet manifest assigns each room a host, port, and tick frequency. Rooms can run at different rates — the polyrythmic example runs a galley at 0.017 Hz (once per minute) alongside a backdeck at 2 Hz.

**Big-O complexity**: All commands are $O(1)$ — they write/read fixed-size JSON templates. The `validate` command is $O(n + m)$ where $n$ is the number of required fields and $m$ is the number of sensors/alarms.

## Quick Start

```bash
cargo run -- init          # Creates room.json with engine room defaults
cargo run -- validate      # Validates room.json
cargo run -- simulate      # Runs 100-tick simulation
cargo run -- fleet         # Generates fleet.json manifest
```

## API

| Command | Description |
|---------|-------------|
| `plato init` | Write `room.json` with 3 sensors, 2 actuators, 2 alarms |
| `plato validate [FILE]` | Check required fields in room config |
| `plato simulate` | Print 100-tick simulated overheat scenario |
| `plato fleet` | Write `fleet.json` with multi-room manifest |
| `plato version` | Print version |
| `plato help` | Print usage |

## Architecture Notes

This tool is the entry point to the Plato ecosystem. The configuration it generates follows the **γ + η = C** conservation pattern: γ (generation of sensor data) combined with η (evaluation of alarm rules) must equal C (consistent room state). Every tick must produce a complete snapshot — no partial updates, no dropped alarms.

The fleet manifest supports polyrythmic ticking: different rooms at different frequencies, coordinated by a watchdog agent. This maps to the conservation law $\sum_i \gamma_i \cdot f_i = C_{\text{fleet}}$ — the sum of all room processing rates must match the fleet's aggregate throughput capacity.

## References

- **Plato Runtime Kernel** — `plato-runtime-kernel` crate for the execution engine
- **Room Topology** — `room-topology` crate for spatial relationships
- **Ternary Bridge** — `plato-ternary-bridge` for {-1, 0, +1} state integration
- **SCADA/ICS Patterns** — ISA-95 hierarchy: enterprise → site → area → unit
- **Alarm Management** — ISA-18.2 standard for alarm system management

## License

MIT
