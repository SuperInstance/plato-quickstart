# Plato Quickstart

**Plato Quickstart** is a CLI tool that bootstraps a Plato room in 30 seconds — generating sensor configurations, alarm rules, and fleet manifests for IoT monitoring and control scenarios.

## Why It Matters

Setting up an IoT monitoring room from scratch involves defining sensors (temperature, RPM, pressure), actuators (alarms, limiters), and alarm rules with correct thresholds, cooldown periods, and escalation actions. Doing this by hand is tedious and error-prone. Plato Quickstart automates the entire process: `plato init` generates a complete `room.json`, `plato validate` checks it, `plato simulate` runs a 100-tick simulation, and `plato fleet` generates deployment manifests.

## How It Works

### Room Configuration Model

A Plato room is a JSON object:

```json
{
  "room_id": "engine_room",
  "tick_hz": 1.0,
  "sensors": [...],
  "actuators": [...],
  "alarms": [...]
}
```

- **Sensors**: Named values with units, min/max range, and normal operating range
- **Actuators**: Boolean or float outputs with defaults and range constraints
- **Alarms**: Condition expressions (e.g., `coolant_temp_c > 95`) with severity, cooldown ticks, and actions

### Alarm Rule Evaluation

Each tick, the room evaluates all alarm conditions:

```
for alarm in room.alarms:
    if eval(alarm.condition, sensor_values):
        if alarm.cooldown_expired():
            for action in alarm.actions:
                execute(action, actuators)
            alarm.reset_cooldown()
```

Condition evaluation: **O(1)** per alarm (simple comparison). Cooldown tracking prevents alarm flapping — once triggered, the alarm won't re-fire for `cooldown_ticks` ticks.

### Simulation

The `simulate` command runs 100 ticks with sensor values sampled from normal distributions within configured ranges, evaluating alarms and logging actions. This validates that alarm thresholds trigger correctly under realistic conditions.

### Validation

The `validate` command checks:
- JSON syntax validity
- Sensor ranges: `min < normal_range[0] ≤ normal_range[1] < max`
- Alarm conditions reference defined sensors
- Action targets reference defined actuators

Validation: **O(N + M)** where N = sensors, M = alarms.

## Quick Start

```bash
mkdir my-room && cd my-room
plato init          # creates room.json with 3 sensors, 2 actuators, 2 alarms
plato validate .    # checks config validity
plato simulate      # runs 100-tick simulation
plato fleet         # generates fleet.json manifest
```

## API

| Command | Description |
|---------|-------------|
| `plato init` | Create room.json with default engine monitoring config |
| `plato validate [file]` | Validate a room config file |
| `plato simulate` | Run 100-tick simulation with alarm evaluation |
| `plato fleet` | Generate fleet.json deployment manifest |
| `plato version` | Print version |
| `plato help` | Print usage |

## Architecture Notes

Plato Quickstart is the bootstrap tool for the Plato room runtime in the SuperInstance fleet system. In γ + η = C, rooms are the unit of γ (growth — each room monitors and controls a physical or logical space) while alarm rules implement η (avoidance — detecting and responding to dangerous conditions). Room configs integrate with `ternary-cell` for state tracking and `openmind-esp32-bridge` for hardware I/O.

See [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md) for fleet room architecture.

## References

1. Appleton, D. (2017). "Threshold-Based Alerting in Monitoring Systems." *O'Reilly Media*.
2. Nagios Enterprises (2024). "Configuration Best Practices for Alert-based Monitoring."
3. Prometheus Authors (2024). "Recording Rules and Alerting Rules." *Prometheus Documentation*.

## License

MIT
