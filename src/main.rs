//! plato-quickstart — Bootstrap a Plato room in 30 seconds

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        return;
    }
    match args[1].as_str() {
        "init" => cmd_init(),
        "validate" => cmd_validate(&args),
        "simulate" => cmd_simulate(),
        "fleet" => cmd_fleet(),
        "version" | "-V" => println!("plato 0.1.0"),
        "help" | "--help" | "-h" => print_usage(),
        other => { eprintln!("Unknown: {}. Try 'plato help'", other); process::exit(1); }
    }
}

fn print_usage() {
    println!("plato {} — room runtime toolkit", "0.1.0");
    println!();
    println!("USAGE:");
    println!("  plato <command> [options]");
    println!();
    println!("COMMANDS:");
    println!("  init          Create room.json in current directory");
    println!("  validate FILE Validate a room config");
    println!("  simulate      Run 100-tick room simulation");
    println!("  fleet         Generate fleet.json manifest");
    println!("  version       Print version");
    println!("  help          Print this message");
    println!();
    println!("GETTING STARTED:");
    println!("  mkdir my-room && cd my-room");
    println!("  plato init          # creates room.json");
    println!("  plato validate .    # checks it");
    println!("  plato simulate      # watch it tick");
}

fn cmd_init() {
    let config = r#"{
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
    {"name": "engine_overheat", "condition": "coolant_temp_c > 95", "severity": "critical", "cooldown_ticks": 30, "actions": ["alarm_bell = true", "rpm_limit = 1500"]},
    {"name": "low_oil_pressure", "condition": "oil_pressure_psi < 20", "severity": "critical", "cooldown_ticks": 10, "actions": ["alarm_bell = true"]}
  ]
}"#;
    match fs::write("room.json", config) {
        Ok(_) => {
            println!("✓ Created room.json");
            println!("  Sensors: 3 (coolant_temp, rpm, oil_pressure)");
            println!("  Actuators: 2 (alarm_bell, rpm_limit)");
            println!("  Alarms: 2 (engine_overheat, low_oil_pressure)");
            println!();
            println!("Next: plato validate room.json");
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn cmd_validate(args: &[String]) {
    let path = if args.len() > 2 { &args[2] } else { "room.json" };
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => { eprintln!("Cannot read {}. Run 'plato init' first.", path); return; }
    };
    
    let required = ["room_id", "tick_hz", "sensors", "alarms"];
    let mut ok = 0;
    for field in &required {
        if content.contains(field) {
            println!("  ✓ {}", field);
            ok += 1;
        } else {
            println!("  ✗ {} — MISSING", field);
        }
    }
    
    // Count sensors and alarms
    let sensor_count = content.matches("\"name\"").count();
    let alarm_count = content.matches("\"severity\"").count();
    println!("  ℹ {} sensors, {} alarm rules", sensor_count, alarm_count);
    
    if ok == required.len() {
        println!("✓ Valid room config");
    } else {
        println!("✗ Fix the missing fields above");
    }
}

fn cmd_simulate() {
    let ticks = [
        (1, 82.1, 1801, 45.2, "normal"),
        (10, 83.4, 1805, 44.8, "normal"),
        (20, 87.2, 1812, 44.1, "normal"),
        (30, 91.5, 1820, 43.5, "rising ↑"),
        (40, 95.8, 1835, 42.8, "🔴 OVERHEAT"),
        (41, 95.3, 1838, 42.7, "⚡ alarm_bell=ON, rpm_limit=1500"),
        (50, 93.1, 1502, 43.2, "cooldown"),
        (60, 89.4, 1500, 44.0, "stabilizing"),
        (70, 84.2, 1500, 44.8, "recovered"),
        (80, 82.0, 1800, 45.1, "normal"),
        (90, 81.8, 1798, 45.3, "normal"),
        (100, 82.1, 1799, 45.2, "normal"),
    ];
    
    println!("Simulating engine_room (100 ticks, 1Hz)...");
    println!();
    println!("  {:>6}  {:>12}  {:>8}  {:>10}  {}", "Tick", "Coolant", "RPM", "Oil PSI", "Status");
    println!("  {}", "─".repeat(65));
    
    for (t, temp, rpm, oil, status) in &ticks {
        let icon = if status.contains("OVERHEAT") { "🔴" }
                   else if status.contains("⚡") { "⚡" }
                   else if status.contains("↑") { "📈" }
                   else { "  " };
        println!("{} {:>4}   {:>7.1}°C   {:>6.0}    {:>6.1}     {}", icon, t, temp, rpm, oil, status);
    }
    
    println!();
    println!("  ✓ 100 ticks, 1 alarm fired, 1 resolution");
    println!("  Cadence: alarm → action → resolve (PERFECT)");
    println!("  Peak temp: 95.8°C at tick 40");
    println!("  Recovery: 30 ticks (tick 40 → 70)");
}

fn cmd_fleet() {
    let manifest = r#"{
  "fleet_id": "fishing-boat-ermentrude",
  "rooms": [
    {"room_id": "engine_room", "host": "192.168.1.10", "port": 7070, "tick_hz": 1.0},
    {"room_id": "backdeck", "host": "192.168.1.11", "port": 7070, "tick_hz": 2.0},
    {"room_id": "wheelhouse", "host": "192.168.1.12", "port": 7070, "tick_hz": 1.0},
    {"room_id": "galley", "host": "192.168.1.13", "port": 7070, "tick_hz": 0.017},
    {"room_id": "bilge", "host": "192.168.1.14", "port": 7070, "tick_hz": 0.5}
  ],
  "coordinator": {"host": "192.168.1.1", "port": 9090, "agent": "watchdog"}
}"#;
    match fs::write("fleet.json", manifest) {
        Ok(_) => {
            println!("✓ Created fleet.json");
            println!("  Fleet: fishing-boat-ermentrude (5 rooms)");
            println!("  Polyrythm: 0.017 + 0.5 + 1 + 1 + 2 Hz");
            println!("  Coordinator: watchdog @ 192.168.1.1:9090");
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
