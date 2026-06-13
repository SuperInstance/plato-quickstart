# Plato Quickstart

**Plato Quickstart** is a command-line toolkit for bootstrapping Plato rooms — sensor configuration, alarm rules, and fleet manifests — letting you go from zero to a running simulation in 30 seconds with `plato init`, `plato validate`, `plato simulate`, and `plato fleet`.

## Why It Matters

Configuring a monitoring room from scratch involves defining sensors (names, units, ranges, normal operating bands), actuators (boolean toggles, float limits), and alarm rules (conditions, severity, cooldown, actions). Doing this by hand is tedious and error-prone. Plato Quickstart generates a complete room.json configuration with realistic defaults (engine coolant temperature, RPM, oil pressure), validates it for required fields, runs a 100-tick simulation showing alarm firing and recovery, and generates a fleet.json manifest for multi-room deployment — all from four commands.

## How It Works

### Room Configuration (room.json)

```json
{
  "room_id": "engine_room",
  "tick_hz": 1.0,
  "sensors": [
    {"name": "coolant_temp_c", "unit": "°C", "min": 0, "max": 120, "normal_range": [75, 95]},
    {"name": "rpm", "unit": "rpm", "min": 0, "max": 3000, "normal_range": [1200, 2200]},
    {"name": "oil_pressure_psi", "unit": "psi", "min": 0, "max": 100, "normal_range": [25, 65]}
  ],
  "actuators": [
    {"name": "alarm_bell", "type": "boolean", "default": false},
    {"name": "rpm_limit", "type": "float", "default": 2200, "range": [800, 2200]}
  ],
  "alarms": [
    {"name": "engine_overheat", "condition": "coolant_temp_c > 95", "severity": "critical",
     "cooldown_ticks": 30, "actions": ["alarm_bell = true", "rpm_limit = 1500"]}
  ]
}
```

Parsing: **O(N)** where N = JSON character count. Validation checks 4 required fields.

### Simulation Model

The simulate command runs a fixed 12-point time series representing 100 ticks at 1Hz:

```
Tick  Coolant   RPM    Oil PSI   Status
  1    82.1°C  1801    45.2     normal
 30    91.5°C  1820    43.5     rising ↑
 40    95.8°C  1835    42.8     🔴 OVERHEAT
 41    95.3°C  1838    42.7     ⚡ alarm_bell=ON, rpm_limit=1500
 70    84.2°C  1500    44.8     recovered
100    82.1°C  1799    45.2     normal
```

Alarm lifecycle: normal → rising → threshold breach → alarm+action → cooldown → recovery → normal. Total cycle: 100 ticks. Simulation output: **O(T)** where T = number of sampled ticks.

### Alarm Evaluation

```
for each alarm rule:
    if evaluate(condition, sensor_values):
        if not in cooldown_period:
            fire alarm
            execute actions (set actuator values)
            enter cooldown
    else:
        if in cooldown and cooldown_expired:
            clear alarm
```

Per-tick alarm evaluation: **O(A × C)** where A = number of alarm rules, C = condition complexity (typically O(1) per comparison).

### Fleet Manifest

```json
{
  "fleet_id": "fishing-boat-ermentrude",
  "rooms": [
    {"room_id": "engine_room", "host": "192.168.1.10", "port": 7070, "tick_hz": 1.0},
    {"room_id": "wheelhouse", "host": "192.168.1.12", "port": 7070, "tick_hz": 1.0}
  ]
}
```

Fleets support polyrhythmic tick rates: engine_room at 1Hz, galley at 0.017Hz (once per minute), backdeck at 2Hz. Manifest generation: **O(R)** for R rooms.

## Quick Start

```bash
# Create a new room
mkdir my-room && cd my-room
plato init          # creates room.json with defaults
plato validate .    # checks required fields
plato simulate      # runs 100-tick simulation
plato fleet         # generates fleet.json manifest
plato version       # prints version
```

## API

| Command | Description |
|---------|-------------|
| `plato init` | Create room.json with sensor/actuator/alarm defaults |
| `plato validate [FILE]` | Check room config for required fields (room_id, tick_hz, sensors, alarms) |
| `plato simulate` | Run 100-tick simulation with alarm firing/recovery |
| `plato fleet` | Generate fleet.json with multi-room manifest |
| `plato version` | Print version |
| `plato help` | Print usage |

## Architecture Notes

Plato Quickstart provides the room bootstrapping layer for fleet deployment in SuperInstance. In γ + η = C, sensor values represent γ (growth — operational telemetry indicating fleet activity), alarm conditions trigger η (avoidance — corrective actions preventing damage), and the cooldown period implements C (conservation — preventing alarm spam by requiring recovery time between firings). Integrates with `openmind-conductor` for AI-driven alarm response and `node-agent` for sensor data collection.

See [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md) for fleet room architecture.

## References

1. Nagios Enterprises (2024). *Nagios Core Administration Guide*. Configuration and alarm rules.
2. Prometheus Authors (2024). *Prometheus Alerting Rules Documentation*. Alert condition evaluation.
3. Bernstein, D. (2014). "Containers and Cloud: From LXC to Docker to Kubernetes." *IEEE Cloud Computing*.

## License

MIT
