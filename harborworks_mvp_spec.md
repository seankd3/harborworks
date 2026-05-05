
# Harborworks MVP Specification
## A code-only high-performance engineering sandbox for building, testing, breaking, repairing, and using custom marine rescue/work boats

**Version:** 0.1  
**Date:** 2026-05-05  
**Primary target:** PC desktop prototype  
**Primary development model:** AI-assisted implementation with strict module boundaries, tests, schemas, and deterministic simulation harnesses  
**Working title:** Harborworks  
**Genre:** Vehicle engineering sandbox, systems simulation, mission-based career prototype  
**MVP vehicle class:** Small marine rescue/work boats, approximately 6 m to 15 m  
**Long-term fantasy:** The ultimate vehicle builder for boats, ships, sailboats, cars, trucks, helicopters, planes, submarines, and spacecraft

---

## 0. Design intent

This document specifies the entire MVP for a game whose long-term dream is a richly engineered vehicle builder: not block-based, but built from beams, sheet metal, composite panels, wiring, pipes, mechanical systems, fluids, controls, damage, repair, and mission use. The MVP is intentionally scoped to a single vehicle family: small marine rescue/work boats. This family exercises the core simulation systems while avoiding the overwhelming scope of aircraft aerodynamics, helicopter rotor dynamics, orbital mechanics, or full automotive suspension simulation.

The MVP must prove the following sentence:

> A player can design a boat from structural members and panels, install engines and subsystems, route wires and pipes, test it in water, discover failures, repair/redesign it, and complete meaningful missions.

This game should feel like a game-native CAD-inspired engineering playground. It should not feel like professional CAD software with a campaign stapled on. The player should be able to build with precision, but the interface should be playful, tactile, and readable.

The MVP is not “real CAD.” It is a game system that gives players convincing engineering consequences.

The guiding tradeoff:

- **Simulate what creates interesting design consequences.**
- **Approximate what creates only complexity.**
- **Visualize every failure in a way the player can understand.**

---

# Part I: Product specification

---

## 1. Product pillars

### 1.1 Engineering feel without professional CAD friction

Players should feel like they are constructing a real machine. They should place beams, shape panels, choose materials, route systems, and diagnose failures. But every operation must be friendly enough for a gamepad/mouse player, not a trained mechanical engineer.

Design implications:

- Use handles, snapping, symmetry, templates, smart fill tools, and contextual suggestions.
- Do not expose constraint solvers, NURBS, full sketch history, or industrial CAD terminology unless it creates fun.
- Prefer visible cause and effect over hidden numeric correctness.
- Let players be approximate, then let simulation teach them.

### 1.2 Built machines have readable consequences

A design should succeed or fail for understandable reasons.

Examples:

- The boat capsizes because the center of mass is too high.
- The engine does not start because the starter is not wired.
- The pump does not drain water because the outlet is missing or below waterline.
- The hull floods because a compartment was not sealed.
- The propeller cavitates or loses efficiency because it is badly sized.
- The engine overheats because cooling is insufficient.
- The battery drains because lights and pumps exceed alternator output.
- A beam or panel takes damage because impacts exceed its material strength.

The MVP should prioritize feedback loops that produce “I know what to fix” moments.

### 1.3 One core loop, repeated quickly

The core loop:

1. Build or modify vehicle.
2. Validate systems.
3. Launch into test water.
4. Observe performance/failures.
5. Repair or redesign.
6. Complete mission.
7. Earn money/reputation.
8. Unlock parts and take harder jobs.

The loop should work even with ugly vehicles. A floating bathtub with a lawnmower engine should be funny and useful enough to teach.

### 1.4 Data-driven expansion

All parts, materials, missions, tutorials, UI categories, and system behaviors should be data-driven where practical. The first vehicle class is boats, but the architecture must not hardcode “boat forever.” Future expansions should add new physics modules and part categories, not rewrite the vehicle builder.

### 1.5 High-performance simulation through simplification

Performance comes from choosing the right level of representation:

- The editable vehicle may have hundreds or thousands of visual parts.
- The simulation vehicle should compile to a smaller runtime proxy:
  - one primary rigid body,
  - a set of colliders,
  - buoyancy sample points,
  - drag sample points,
  - propulsor force points,
  - compartment volumes,
  - system graphs,
  - damage zones.

Do not simulate every beam as a separate rigid body during normal gameplay. That path creates jitter, low frame rates, and unmanageable edge cases.

---

## 2. MVP summary

### 2.1 MVP fantasy

The player runs a tiny coastal fabrication and rescue outfit. They design custom boats to solve practical maritime jobs: delivery, towing, pumping, rescue, salvage, and night operations. They start with basic tools and parts, then improve their designs and reputation.

### 2.2 MVP vehicle class

**Small marine rescue/work boats** with these characteristics:

- Length: 6 m to 15 m.
- Hull: monohull only for MVP.
- Propulsion: diesel shaft drive and/or simple electric motor.
- Systems: electrical, fuel, bilge/water, control links.
- Construction: beams, panels, bulkheads, compartments.
- Environment: coastal harbor and nearshore water.
- Missions: cargo, rescue, towing, pumping, navigation.

### 2.3 MVP excludes

The MVP explicitly excludes:

- Aircraft, helicopters, cars, submarines, spacecraft.
- Multiplayer.
- Steam Workshop.
- Full procedural open world.
- Player walking/interior avatars.
- Complex crew simulation.
- Full programmable logic.
- True finite element analysis.
- True computational fluid dynamics.
- Complex AC electrical simulation.
- Full thermodynamics.
- Arbitrary CAD sketching.
- Soft-body deformation.
- Real-time per-beam rigid body simulation.
- Large ships above 15 m.
- Sail physics and hydrofoils.

These exclusions are not “never.” They are “not until the core is fun.”

---

## 3. Target player experience

### 3.1 First 10 minutes

The player should:

1. Open the workshop.
2. See a half-finished starter hull.
3. Add a missing panel.
4. Install a battery and pump.
5. Route a wire.
6. Launch the boat.
7. Watch it float.
8. Drive through a small buoy gate.
9. See a clear reward.

The first experience should be successful, not punishing.

### 3.2 First hour

The player should:

1. Build a simple hull from scratch using guided tools.
2. Add engine, fuel tank, propeller, rudder, helm, battery, switch, pump, and light.
3. Diagnose at least one intentionally created problem.
4. Complete a cargo run or rescue pickup.
5. Return to the workshop with an idea for improvement.

### 3.3 First failure

The first failure should be funny, legible, and recoverable.

Good first failures:

- Boat sits too low.
- Engine will not start.
- Pump has no power.
- Rudder not linked.
- Cargo tips boat to one side.
- Small leak causes slow flooding.

Bad first failures:

- Immediate unexplained explosion.
- Boat flips because of invisible physics bug.
- UI says “invalid” without explanation.
- Player loses progress.
- Simulation becomes unrecoverable.

---

## 4. Core game modes

### 4.1 Workshop mode

The Workshop is the main build editor.

Primary player actions:

- Place nodes.
- Place beams between nodes.
- Place panels on beam loops.
- Add bulkheads and compartments.
- Add functional components.
- Route wires, pipes, and control links.
- Inspect mass, buoyancy, power, fuel flow, and warnings.
- Save blueprint.
- Send to test mode or mission mode.

Workshop UI panels:

- Left: Parts library.
- Center: 3D viewport.
- Top: Build category tabs and transform tools.
- Right: Analysis and selected part properties.
- Bottom: Context-sensitive tool ribbon.
- Optional overlays: structure, electrical, fuel, bilge, compartments, mass, buoyancy, stress, errors.

### 4.2 Test mode

Test mode launches the current boat into a controlled harbor test area.

Primary player actions:

- Drive the boat.
- Toggle systems.
- Load cargo.
- Spawn waves.
- Intentionally breach panels.
- Run auto-tests.
- Record telemetry.
- Return to workshop.

Test mode metrics:

- Speed.
- Acceleration.
- Turning radius.
- Fuel use.
- Battery state.
- Pump flow.
- Waterline.
- Stability index.
- Flooding rate.
- Structural damage.
- Mission readiness.

### 4.3 Mission mode

Mission mode places the boat into a mission scenario.

MVP mission types:

1. Sea Trial.
2. Cargo Delivery.
3. Rescue Pickup.
4. Tow Disabled Boat.
5. Flood Response.
6. Optional Night Beacon Repair.

Mission mode should be less editor-heavy and more gameplay-heavy, but diagnostics remain available.

### 4.4 Repair mode

Repair mode is a workshop sub-mode after damage.

Primary player actions:

- Inspect damaged components.
- Replace panels/beams.
- Repair wires or pipes.
- Patch leaks.
- Drain compartments.
- Run system tests.
- Certify for mission.

MVP repair is not a full separate game mode. It is a structured set of workshop actions and UI overlays.

---

# Part II: Game mechanics specification

---

## 5. Vehicle representation

### 5.1 Vehicle concept

A vehicle is a compiled graph of editable entities.

Editable entities:

- Nodes.
- Beams.
- Panels.
- Bulkheads.
- Compartments.
- Functional components.
- Ports.
- Routes.
- Controls.
- Labels.
- Metadata.

Runtime simulation proxies:

- Rigid body mass/inertia.
- Collider set.
- Buoyancy sample set.
- Hydrodynamic drag sample set.
- Propulsion force points.
- Compartments with volumes and leak states.
- Electrical network graph.
- Fluid network graph.
- Control signal graph.
- Damage zones.

### 5.2 Vehicle coordinate system

Recommended coordinate convention:

- +X: right/starboard.
- +Y: up.
- +Z: forward/bow.
- Origin: builder grid center, preferably near designed centerline.
- Units: meters, kilograms, seconds, newtons, watts, liters.

Reasons:

- +Y up is common for many render/game frameworks.
- Forward as +Z makes viewport conventions intuitive.
- Starboard/port language maps cleanly to marine vehicles.

### 5.3 Blueprint identity

Each vehicle blueprint has:

- `blueprint_id`: stable UUID.
- `display_name`.
- `author`.
- `version`.
- `created_at`.
- `modified_at`.
- `vehicle_class`: `marine_small`.
- `schema_version`.
- `parts`.
- `routes`.
- `materials`.
- `paint`.
- `analysis_cache`.
- `thumbnail`.

### 5.4 Runtime compile pipeline

When a blueprint is loaded or edited, it compiles through these stages:

1. **Parse**
   - Load schema.
   - Validate IDs and references.
2. **Normalize**
   - Convert units if necessary.
   - Resolve default values.
   - Sort stable arrays for deterministic builds.
3. **Validate topology**
   - Check beam endpoints.
   - Check panel loops.
   - Check ports and routes.
   - Check component dependencies.
4. **Generate geometry**
   - Beam meshes.
   - Panel meshes.
   - Component meshes.
   - Collision proxies.
5. **Compute mass**
   - Sum parts.
   - Compute center of mass.
   - Compute inertia tensor approximation.
6. **Detect compartments**
   - Identify sealed regions.
   - Compute volumes.
   - Link pumps/drains/leaks.
7. **Generate buoyancy samples**
   - Interior/hull volume sampling.
   - Compartment-specific samples.
8. **Build system graphs**
   - Electrical connected components.
   - Fluid networks.
   - Control links.
9. **Generate runtime vehicle**
   - One main rigid body.
   - Colliders.
   - Force points.
   - Damage zones.
   - State arrays.
10. **Report warnings**
   - Missing controls.
   - Open compartments.
   - Disconnected systems.
   - Overloaded electrical circuits.
   - Insufficient buoyancy.
   - Structural weak points.

Dirty flags determine which stages need recomputation.

---

## 6. Structural builder

### 6.1 Purpose

The structural builder is the core differentiator. Instead of block placement, the player builds a frame and skin system.

The player should be able to say:

> “This boat has a keel, frames, stringers, deck beams, panels, compartments, and a cabin.”

### 6.2 Structural nodes

Nodes are editable points in 3D space.

Node properties:

- `node_id`.
- `position`.
- `flags`:
  - mirrored,
  - locked,
  - generated,
  - user_placed,
  - symmetry_master,
  - symmetry_slave.
- `snap_group`.
- `label`.

Node tools:

- Add node.
- Move node.
- Snap node.
- Merge nodes.
- Split node.
- Mirror node.
- Lock node.

Node constraints:

- Minimum distance between nodes.
- Maximum beams per node may be soft-limited for usability.
- Nodes may be welded, bolted, or temporary construction references.

### 6.3 Beams

Beams connect two nodes.

Beam types:

- Tube.
- Box beam.
- I-beam.
- Angle beam.
- Flat bar.
- Keel beam.
- Stringer.
- Deck beam.
- Mount rail.

Beam properties:

- `beam_id`.
- `node_a`.
- `node_b`.
- `profile_id`.
- `material_id`.
- `cross_section`.
- `wall_thickness`.
- `connection_type`.
- `mass`.
- `strength_rating`.
- `damage_state`.
- `paint/material override`.
- `symmetry_link`.

Beam placement tools:

- Draw beam.
- Repeat beam.
- Parallel beam.
- Mirror beam.
- Offset beam.
- Split beam.
- Join beam.
- Replace profile.
- Change material.
- Auto-stringer along hull.
- Auto-frame at spacing.

Beam gameplay attributes:

- Mass contribution.
- Structural strength.
- Collision resistance.
- Attachment support for panels/components.
- Damage threshold.
- Repair cost.

MVP beam simulation:

- Beams are not independent rigid bodies in gameplay.
- Beams contribute to aggregate structure and local damage zones.
- Beams can be visually bent/broken after damage.
- Damage affects attached panels/components and structural integrity scores.

### 6.4 Panels

Panels are skins, decks, windows, hatches, and surfaces attached to structural loops.

Panel types:

- Flat sheet metal.
- Curved sheet metal.
- Fiberglass panel.
- Composite sandwich panel.
- Deck plate.
- Glass/window.
- Hatch panel.
- Door panel.
- Rubber/fender panel.

Panel properties:

- `panel_id`.
- `boundary_nodes`.
- `support_beams`.
- `material_id`.
- `thickness`.
- `curvature_mode`.
- `seal_rating`.
- `damage_state`.
- `paint`.
- `cutouts`.
- `normal_direction`.
- `compartment_boundary`.

Panel placement tools:

- Fill selected loop.
- Auto-skin hull side.
- Bend panel.
- Trim panel.
- Cut hatch.
- Cut window.
- Flip normal.
- Set thickness.
- Replace material.
- Mirror.
- Delete.

MVP panel restrictions:

- Prefer triangular and quad panels.
- Allow n-gons only if internally triangulated.
- Allow mild curvature using generated subdivisions.
- Avoid arbitrary NURBS.
- Cutouts are modular inserts, not arbitrary boolean operations in MVP.
- Panels must reference a valid boundary loop or generated attachment frame.

Panel gameplay attributes:

- Mass.
- Drag surface contribution.
- Buoyancy hull boundary.
- Watertightness.
- Damage/leak probability.
- Repair cost.
- Material cost.
- Structural support rating.

### 6.5 Bulkheads and compartments

Bulkheads create internal watertight divisions.

Bulkhead types:

- Structural bulkhead.
- Watertight bulkhead.
- Partial partition.
- Deck/cabin wall.
- Tank wall.

Compartment detection:

A compartment is a connected enclosed volume bounded by panels/bulkheads/deck surfaces.

Compartment properties:

- `compartment_id`.
- `volume_m3`.
- `air_volume_m3`.
- `water_volume_m3`.
- `is_watertight`.
- `leak_sources`.
- `pump_connections`.
- `drain_connections`.
- `cargo_slots`.
- `center`.
- `buoyancy_samples`.

MVP detection options:

Option A: Player-defined compartments.

- The player places a `Compartment Volume` marker.
- The game infers nearby boundaries.
- If boundaries are missing, it reports leaks.
- Easier to implement.

Option B: Voxel flood-fill.

- Voxelize hull interior.
- Flood-fill sealed spaces.
- More automatic but more complex.

Recommendation for MVP:

- Use a hybrid.
- Let players define compartment markers.
- Use voxel or ray-based validation to check sealing.
- Report clear sealing issues.

### 6.6 Structural analysis

The MVP should not perform finite element analysis. It should calculate engineering-style heuristics.

Structural metrics:

- Global integrity score.
- Hull strength.
- Deck strength.
- Engine mount strength.
- Panel support score.
- Impact resistance.
- Fatigue estimate.
- Safety factor estimate.
- Weak point list.

Heuristic inputs:

- Material strength.
- Beam profile strength.
- Beam span length.
- Number of load paths.
- Panel unsupported area.
- Component mass attached to local supports.
- Cargo load distribution.
- Collision impulse history.
- Wave impact history.
- Existing damage.

Example rule:

```text
panel_support_score = clamp01(1 - unsupported_span / allowed_span(material, thickness))
```

Example weak point:

```text
Engine mount unsupported: engine mass 420 kg, mount rating 280 kg. Add two cross-beams or change material.
```

The UI must label warnings as estimates, but in game language:

- “Weak.”
- “Marginal.”
- “Good.”
- “Excellent.”

---

## 7. Materials

### 7.1 Material goals

Materials should create meaningful tradeoffs:

- Steel: strong, cheap, heavy.
- Aluminum: lighter, corrosion-resistant, moderately strong, more expensive.
- Fiberglass: light, weak in impact, good watertight skin.
- Composite sandwich: light and stiff, expensive, harder to repair.
- Rubber: fenders/seals.
- Glass/acrylic: windows, fragile.

### 7.2 MVP materials

Recommended MVP material list:

1. Mild steel.
2. Marine aluminum.
3. Fiberglass.
4. Composite sandwich.
5. Rubber.
6. Acrylic/glass.
7. Wood/plywood optional.

Material properties:

- density_kg_m3.
- yield_strength_mpa.
- impact_toughness.
- corrosion_resistance.
- repair_difficulty.
- cost_per_kg.
- watertight_rating.
- thermal_rating optional.
- electrical_conductivity optional.
- display color.
- PBR material reference.

### 7.3 Material gameplay effects

Mass and center of mass:

- Materials directly affect mass.
- Heavy materials lower or raise COM depending on placement.

Damage:

- Steel dents.
- Aluminum deforms.
- Fiberglass cracks/delaminates.
- Composite delaminates.
- Glass shatters.
- Rubber absorbs impact.

Repair:

- Steel/aluminum: weld/replace.
- Fiberglass/composite: patch/cure/replace.
- Glass: replace only.
- Rubber: replace.

Cost:

- Material choice affects vehicle cost and mission economics.

---

## 8. Functional components

### 8.1 Component definition

Components are functional parts with physical presence and system ports.

Component categories:

- Propulsion.
- Fuel.
- Electrical.
- Fluid/water.
- Controls.
- Navigation/lights.
- Cargo/mission.
- Safety.
- Sensors.
- Cosmetic/detail.

Every functional component should have:

- A visual model.
- A mass and cost.
- Attachment requirements.
- One or more ports.
- Damage/repair state.
- Runtime behavior.

### 8.2 Ports

Ports are typed connection endpoints.

Port types:

- `electric_in`.
- `electric_out`.
- `signal_in`.
- `signal_out`.
- `fuel_in`.
- `fuel_out`.
- `water_in`.
- `water_out`.
- `shaft_in`.
- `shaft_out`.
- `mechanical_control_in`.
- `mechanical_control_out`.

Port properties:

- `port_id`.
- `component_id`.
- `type`.
- `local_position`.
- `direction`.
- `capacity`.
- `voltage`.
- `pressure`.
- `flow_limit`.
- `compatible_route_types`.
- `required`.
- `label`.

Connection rule:

- Ports can connect only through compatible route types.
- The UI must prevent impossible connections.
- Soft warnings can allow dangerous connections if gameplay benefits.

### 8.3 Attachment rules

Components must attach to valid structure.

Examples:

- Engine must attach to engine mount beam or deck reinforced area.
- Fuel tank must attach to deck/compartment.
- Battery must attach to bracket or deck.
- Rudder must attach to stern transom or rudder mount.
- Propeller shaft must pass through hull via shaft seal.
- Pump must be inside compartment or mounted on deck.
- Light must attach to panel/beam.

MVP simplification:

- Use snap sockets and surface attachment validation.
- Do not require bolt-by-bolt installation.
- Provide warnings for under-supported components.

---

## 9. Electrical system

### 9.1 Purpose

The electrical system lets players wire power and control systems. It should be simple enough for non-electricians but deep enough that design mistakes matter.

### 9.2 MVP electrical parts

- Small battery.
- Large battery.
- Alternator/generator attached to engine.
- Fuse box.
- Switch.
- Push button/starter switch.
- Navigation light.
- Floodlight.
- Cabin light.
- Bilge pump electric input.
- Radio/beacon.
- Dashboard gauge.
- Electric motor optional.
- Junction connector.
- Wire/cable route.
- Bus bar.

### 9.3 Electrical graph

The electrical system is a graph.

Nodes:

- Sources: battery, alternator.
- Distribution: fuse box, bus bar, connector.
- Controls: switch, button.
- Loads: lights, pumps, starter, radio.
- Sensors/gauges.

Edges:

- Wires/cables with gauge, max current, length, resistance estimate, damage state.

Network solving:

1. Build connected components.
2. Identify sources.
3. Identify loads.
4. Apply switches/fuses.
5. Compute available voltage.
6. Compute total load.
7. Compare load against source capacity and wire/fuse limits.
8. Update battery charge and alternator output.
9. Trigger failures if limits exceeded.

MVP formulas:

```text
load_current_a = load_power_w / network_voltage_v

wire_capacity_ok = current_a <= wire.max_current_a

battery_delta_wh = (charge_power_w - load_power_w) * dt_hours
```

Network states:

- Off.
- Powered.
- Under-voltage.
- Overload.
- Fuse tripped.
- Shorted.
- Water damaged.
- Disconnected.

### 9.4 Electrical failures

Failures:

- Wire cut.
- Wire short.
- Fuse trip.
- Battery drained.
- Component water damage.
- Overloaded wire overheats.
- Alternator failure due to engine damage.

MVP failure rules:

- Water in compartment has chance to damage unprotected electrical components.
- Current above wire max for too long causes overheat.
- Shorted wire trips fuse if fuse exists.
- No fuse may damage battery or component.

### 9.5 Electrical UI

Workshop overlays:

- Show powered networks in yellow.
- Show signal/control lines in purple.
- Show disconnected segments in gray.
- Show overloads in red.
- Show fuses and switches.
- Show current draw.

Diagnostics panel:

- Total draw.
- Source capacity.
- Battery charge.
- Network voltage.
- Largest loads.
- Warnings.
- Suggested fixes.

Example messages:

- “Bilge Pump 1 has no power source.”
- “Navigation Lights exceed fuse rating.”
- “Wire gauge too small for starter current.”
- “Battery will drain in 14 minutes at current load.”
- “Starter switch not connected to engine starter.”

---

## 10. Fluid systems

### 10.1 MVP fluid types

MVP fluids:

- Fuel.
- Water.

Future fluids:

- Coolant.
- Hydraulic fluid.
- Air/pneumatic.
- Oil.

### 10.2 Fuel system

Fuel parts:

- Fuel tank.
- Fuel pipe.
- Fuel valve.
- Fuel filter optional.
- Fuel pump optional.
- Engine fuel input.
- Deck fill cap optional.
- Vent optional.

Fuel graph:

- Sources: fuel tanks.
- Consumers: engines.
- Controls: valves.
- Edges: pipes.

Fuel state:

- Quantity.
- Flow available.
- Leak.
- Valve open/closed.
- Pressure/flow estimate.
- Engine starvation.

MVP fuel solver:

1. Find path from tank to engine.
2. Check valve states.
3. Check pipe damage.
4. Check tank quantity.
5. Compute max flow based on pipe size and pump/gravity simplification.
6. Supply engine demand if possible.
7. Reduce tank fuel.

Engine consequence:

- If insufficient fuel: engine sputters, loses power, stops.
- If tank empty: engine stops.
- If pipe damaged: fuel leak and fire risk optional later.

### 10.3 Bilge/water system

Water parts:

- Bilge pump.
- Water pipe/hose.
- Intake.
- Outlet.
- Check valve optional.
- Compartment drain optional.

Water graph:

- Sources: compartments with water.
- Consumers/outlets: sea outlet, tank, drain.
- Active movers: pumps.

Pump behavior:

- Requires electrical power.
- Has flow rate.
- Has head pressure limit.
- Can fail or clog.
- Outlet must be valid.

MVP flooding loop:

1. Leak adds water to compartment.
2. Pump removes water if:
   - powered,
   - connected to compartment,
   - outlet valid,
   - not damaged.
3. Water increases compartment mass and reduces buoyancy.
4. If water reaches electrical components, damage chance increases.

### 10.4 Fluid UI

Workshop overlays:

- Fuel lines orange.
- Bilge/water lines blue.
- Pumps with flow arrows.
- Tank levels.
- Compartment water levels.
- Leak points.

Diagnostics:

- Fuel remaining.
- Fuel flow to engine.
- Pump flow.
- Water ingress rate.
- Net water removal rate.
- Compartment flood status.

Example messages:

- “Engine has no fuel path.”
- “Fuel valve closed.”
- “Bilge pump powered but no outlet.”
- “Pump flow 40 L/min, leak ingress 65 L/min. Boat will flood.”
- “Outlet below waterline may backflow without check valve.”

---

## 11. Control system

### 11.1 Purpose

The control system maps player inputs and cockpit components to vehicle actions.

### 11.2 MVP control parts

- Helm wheel.
- Throttle lever.
- Starter button.
- Toggle switch.
- Dashboard panel.
- Rudder servo or mechanical linkage.
- Engine throttle input.
- Pump switch.
- Light switch.

### 11.3 Control links

Control links are signal routes.

Types:

- Mechanical cable.
- Electrical signal.
- Direct abstract link for tutorial/easy mode.

Signals:

- Boolean.
- Float 0..1.
- Float -1..1.
- Momentary pulse.
- Enum.

Control graph examples:

- Helm wheel → rudder.
- Throttle lever → engine throttle.
- Starter button → engine starter.
- Switch → pump.
- Switch → lights.
- Sensor → gauge.

MVP rules:

- Controls do not need realistic cable friction or backlash.
- The link must exist unless easy mode auto-connects.
- Control latency is optional.
- Damage to control line can disable function.

### 11.4 Control UI

Control assignment mode:

- Select source.
- Select target.
- Choose signal type.
- Test control.
- Highlight linked route.

Diagnostic messages:

- “Rudder has no control input.”
- “Throttle has no engine target.”
- “Pump switch has no powered pump.”
- “Gauge has no sensor.”

---

## 12. Propulsion and movement

### 12.1 MVP propulsion parts

- Small diesel engine.
- Medium diesel engine.
- Electric motor optional.
- Prop shaft.
- Shaft seal.
- Small propeller.
- Medium propeller.
- Rudder.
- Engine mount.
- Fuel tank.
- Cooling simplified or optional.

### 12.2 Engine behavior

Engine state:

- Off.
- Starting.
- Running.
- Stalled.
- Starved.
- Overheated.
- Damaged.

Inputs:

- Starter signal.
- Throttle signal.
- Fuel availability.
- Electrical starter power.
- Damage state.

Outputs:

- Shaft power.
- Alternator power.
- Heat.
- Fuel consumption.
- Sound/visual effects.

MVP engine curve:

Use a simple curve table:

- rpm.
- torque_nm.
- fuel_consumption_g_s.
- heat_w.

Engine output:

```text
power_w = torque_nm * rpm * 2π / 60
```

Simplified engine model:

- Target RPM from throttle.
- Load from propeller.
- RPM responds with inertia.
- Fuel consumption scales with torque and throttle.
- Engine stops if fuel unavailable or damage too high.

### 12.3 Propeller thrust

MVP thrust model:

Inputs:

- Shaft RPM.
- Prop diameter.
- Prop pitch.
- Boat forward speed.
- Water density.
- Prop efficiency.
- Damage/cavitation.

Output:

- Thrust force applied at propeller location.

Simplified formula style:

```text
thrust_n = prop_efficiency * k_thrust * rpm^2 * diameter^4
```

Then reduce by speed and cavitation/ventilation factors.

This does not need to be naval architecture perfect. It must produce understandable tradeoffs:

- Bigger prop = more thrust but more load.
- More engine power = faster until drag dominates.
- Bad prop placement = reduced thrust.
- Damaged prop = lower thrust.
- Too much weight = slower acceleration.

### 12.4 Rudder steering

Rudder force:

Inputs:

- Rudder angle.
- Water speed at rudder.
- Rudder area.
- Rudder position.
- Prop wash factor.
- Damage.

Output:

- Side force and yaw torque.

MVP rules:

- Rudder must be in water.
- Rudder behind prop gets prop wash bonus.
- Rudder too small gives weak steering.
- Damaged rudder loses authority.

### 12.5 Hydrodynamic drag

Approximate drag:

- Forward hull drag.
- Side drag.
- Angular damping.
- Planing bonus/penalty optional.
- Wave/wind drag optional.

Simplified:

```text
drag_force = -normalize(v) * 0.5 * rho_water * speed^2 * drag_area * cd
```

Use multiple drag sample points for stability and yaw effects.

### 12.6 Vehicle rigid body

The boat is simulated as one main rigid body.

State:

- position.
- orientation.
- linear velocity.
- angular velocity.
- mass.
- inertia tensor.

Forces:

- gravity.
- buoyancy.
- water drag.
- propeller thrust.
- rudder force.
- wave forces.
- collision impulses.
- towing forces.
- cargo forces.

Integration:

- Fixed timestep.
- Semi-implicit Euler is acceptable for MVP.
- Use substeps if needed for stability.
- Clamp extreme forces.

---

## 13. Buoyancy and flooding

### 13.1 Buoyancy goal

Players must be able to build hulls that float, sit low, list, capsize, flood, recover, or sink.

### 13.2 Buoyancy samples

Generate buoyancy samples per compartment.

Each sample:

- local position.
- volume contribution.
- compartment_id.
- hull normal optional.
- drag coefficients.
- active if compartment not fully flooded.

For each fixed update:

1. Transform sample to world.
2. Query water height at sample position.
3. Compute submerged depth.
4. Apply buoyant force proportional to displaced volume.
5. Add damping/drag if underwater.

Basic force:

```text
buoyancy_force = water_density * gravity * sample_volume * submerged_fraction
```

### 13.3 Stability

Stability comes naturally from distributed buoyancy points and center of mass.

Additional metrics:

- Center of mass.
- Center of buoyancy.
- Metacentric-style stability estimate optional.
- Roll stability index.
- Reserve buoyancy.

UI should show:

- Waterline.
- COM marker.
- COB marker.
- Stability warning.

### 13.4 Flooding

Compartment water volume changes over time.

Water ingress sources:

- Panel breach.
- Open hatch below water.
- Failed shaft seal.
- Collision damage.
- Backflow through outlet.

Ingress rate estimate:

```text
flow_lps = leak_area_m2 * sqrt(2 * g * pressure_head_m) * coefficient * 1000
```

Pumps remove water:

```text
removed_liters = min(water_volume_liters, pump_flow_lps * dt)
```

Water affects:

- Mass.
- Center of mass.
- Buoyancy of compartment.
- Electrical damage risk.
- Mission failure if critical.

### 13.5 Sinking

A boat sinks if:

- Buoyancy force can no longer balance weight.
- Compartments flood beyond reserve buoyancy.
- Stability flips/capsizes and remains unrecoverable.
- Mission failure conditions trigger.

Sinking should be dramatic and diagnostic:

- Water overlay.
- Leak labels.
- Pump status.
- Flooded compartments.
- Suggested fixes after failure.

---

## 14. Damage and repair

### 14.1 Damage philosophy

Failure should be a teacher. Damage must create stories and improvement loops.

Damage should be:

- Local.
- Understandable.
- Recoverable.
- Connected to systems.
- Visible in both 3D and UI.

### 14.2 Damageable entities

- Panels.
- Beams.
- Components.
- Wires.
- Pipes.
- Pumps.
- Engines.
- Rudders.
- Propellers.
- Compartments.
- Batteries.
- Fuel tanks.

### 14.3 Damage causes

- Collision with rocks/docks/objects.
- Excessive wave impact.
- Overheating engine.
- Electrical overload.
- Water damage.
- Fuel starvation and rough running optional.
- Towing overload.
- Cargo impact.
- Mission hazards.

### 14.4 Damage model

Each damageable entity has:

- health 0..1.
- damage type flags.
- repair state.
- leak area if breached.
- deformation visual.
- functionality threshold.

Damage types:

- impact.
- puncture.
- crack.
- bend.
- burn.
- short.
- leak.
- flood.
- wear.

Example thresholds:

- Panel health < 0.6: visually damaged.
- Panel health < 0.35: leak possible.
- Panel health < 0.1: large breach.
- Pump health < 0.4: reduced flow.
- Wire health < 0.5: intermittent.
- Wire health < 0.2: disconnected/shorted.

### 14.5 Collision damage

Collision pipeline:

1. Collision occurs in physics/collision system.
2. Determine impulse and contact point.
3. Map contact point to nearest damage zones.
4. Apply damage based on material, panel thickness, beam support.
5. Spawn visual damage decal/deformation.
6. Update compartments/leaks if hull breached.
7. Update diagnostics.

MVP simplification:

- Collision uses simplified colliders.
- Damage zones map to panels/beams by bounding volumes.
- Visual deformation can be decals and precomputed dent meshes initially.

### 14.6 Repair actions

Repair actions:

- Patch panel.
- Replace panel.
- Replace beam.
- Repair wire.
- Replace wire.
- Repair pipe.
- Replace pump.
- Drain compartment.
- Refill fuel.
- Recharge battery.
- Certify systems.

Repair costs:

- Material cost.
- Labor time.
- Part replacement.
- Reputation penalty optional.
- Mission downtime optional.

MVP repair UI:

- Select damaged item.
- Show cause and consequence.
- Show repair options.
- Click repair if in workshop and funds available.
- Apply visual and functional fix.

---

## 15. Missions and economy

### 15.1 Career premise

The player owns a small fabrication/rescue yard in a coastal region. Jobs pay money and reputation. Better reputation unlocks harder contracts and better parts.

### 15.2 Currency

Resources:

- Money.
- Reputation.
- Part unlocks.
- Workshop capacity optional.
- Materials inventory optional later.

Money sources:

- Mission rewards.
- Bonus objectives.
- Salvage.
- Efficiency bonuses.
- Emergency contracts.

Money sinks:

- Parts.
- Materials.
- Repairs.
- Fuel.
- Upgrades.
- Insurance/penalties optional.

### 15.3 Mission scoring

Mission completion should score:

- Objective success.
- Time.
- Damage taken.
- Fuel consumed.
- Cargo condition.
- Passenger safety.
- Optional objectives.

Example reward:

```text
base_reward = 2000
time_bonus = max(0, 1000 - mission_time_seconds * 2)
damage_penalty = damage_cost * 0.25
fuel_cost = fuel_used_liters * fuel_price
final_reward = base_reward + bonuses - penalties
```

### 15.4 MVP mission list

#### Mission 1: Sea Trial

Goal:

- Reach a buoy.
- Maintain 15 knots for 10 seconds.
- Return to dock.

Tests:

- Engine.
- Fuel.
- Steering.
- Hull stability.

Failure cases:

- Engine does not start.
- Boat cannot reach speed.
- Boat sinks/capsizes.
- Boat leaves test area.

#### Mission 2: Cargo Run

Goal:

- Carry 300 kg cargo to outpost dock.
- Return.

Tests:

- Payload capacity.
- Stability.
- Deck/cargo placement.
- Fuel range.

Optional:

- Deliver under time limit.
- No cargo damage.

#### Mission 3: Rescue Pickup

Goal:

- Navigate to life raft.
- Approach slowly.
- Load rescued person/cargo module.
- Return.

Tests:

- Handling at low speed.
- Deck access.
- Stability with offset load.

Optional:

- Use searchlight at night.
- Maintain passenger health.

#### Mission 4: Tow Disabled Boat

Goal:

- Attach tow line.
- Tow small disabled craft to dock.

Tests:

- Engine torque.
- Hull strength.
- Steering under load.
- Towing attachment.

Failure cases:

- Tow hook breaks.
- Engine overheats.
- Tow target collides or sinks.

#### Mission 5: Flood Response

Goal:

- Reach leaking barge.
- Use onboard pump system to drain it or transfer water.
- Return.

Tests:

- Pump capacity.
- Battery/alternator capacity.
- Hose routing.
- Stability during operation.

Optional:

- Complete before barge sinks.

#### Mission 6: Night Beacon Repair

Goal:

- Navigate at night to beacon.
- Carry repair crate.
- Use lights/radio/beacon.

Tests:

- Electrical capacity.
- Navigation lights.
- Battery endurance.
- Cargo.

### 15.5 Mission map

MVP map: compact coastal harbor.

Required locations:

- Workshop/dock.
- Test buoy course.
- Cargo dock.
- Life raft spawn area.
- Disabled boat spawn.
- Leaking barge spawn.
- Rocky inlet hazard.
- Night beacon.
- Fuel station.
- Repair berth.

Environmental conditions:

- Calm water.
- Light waves.
- Moderate waves.
- Wind optional.
- Day/night cycle optional for missions.

---

# Part III: User interface and UX specification

---

## 16. Editor UI principles

### 16.1 Always show cause and effect

When the player changes a design:

- Mass updates.
- COM marker moves.
- Buoyancy estimate changes.
- System errors update.
- Cost changes.
- Warnings appear/disappear.

### 16.2 Three-level information density

The UI should support:

1. Casual level:
   - “Good / Warning / Critical.”
2. Builder level:
   - “Reserve buoyancy 18%, battery draw 42 W.”
3. Expert level:
   - Detailed breakdowns, graphs, per-component values.

Do not force expert numbers on beginners.

### 16.3 Modes, not chaos

Primary editor modes:

- Structure.
- Panels.
- Components.
- Systems.
- Controls.
- Paint/labels.
- Analysis.
- Repair.

Only show relevant tools per mode.

---

## 17. Workshop UI layout

### 17.1 Top bar

Tabs:

- Vehicle.
- Structure.
- Panels.
- Components.
- Systems.
- Controls.
- Analysis.
- Test.
- Save.

Vehicle class tabs may show future categories but locked:

- Boats.
- Ships locked.
- Sailboats locked.
- Cars locked.
- Trucks locked.
- Aircraft locked.
- Helicopters locked.
- Spacecraft locked.

This preserves the dream without overpromising in MVP.

### 17.2 Left parts library

Features:

- Search.
- Categories.
- Favorite parts.
- Recently used.
- Filters by material/system.
- Hover details.
- Drag-and-place.

Categories:

- Structural.
- Panels.
- Materials.
- Propulsion.
- Fuel.
- Electrical.
- Water/Bilge.
- Controls.
- Mission.
- Detail.

### 17.3 Center viewport

Viewport features:

- Orbit camera.
- Pan/zoom.
- Grid.
- Centerline.
- Symmetry plane.
- Ghost preview.
- Transform gizmo.
- Snapping.
- Measurement tool.
- Selection outlines.
- Layer visibility.
- Cross-section view.
- X-ray view.

### 17.4 Right analysis/properties panel

When nothing selected:

- Vehicle summary.
- Mass.
- Cost.
- Part count.
- COM.
- Buoyancy.
- Reserve buoyancy.
- Electrical status.
- Fuel status.
- Leak risk.
- Warnings.
- Mission readiness.

When part selected:

- Part name.
- Material.
- Mass.
- Cost.
- Health.
- Attachments.
- Ports.
- Routes.
- Parameters.
- Replace/repair buttons.

### 17.5 Bottom tool ribbon

Context-sensitive tools.

Structure mode:

- Select.
- Node.
- Beam.
- Frame.
- Mirror.
- Split.
- Merge.
- Material.
- Auto-frame.
- Measure.

Panel mode:

- Fill.
- Bend.
- Trim.
- Cut hatch.
- Cut window.
- Flip.
- Thickness.
- Material.
- Seal.

Systems mode:

- Route.
- Splice.
- Bundle.
- Clamp.
- Label.
- Test.
- Hide/show network.
- Auto-route optional.

Controls mode:

- Link.
- Test input.
- Assign.
- Invert.
- Curve.
- Label.

Repair mode:

- Inspect.
- Patch.
- Replace.
- Rewire.
- Drain.
- Test.
- Certify.

---

## 18. Build tools

### 18.1 Symmetry

Symmetry is critical.

Features:

- Toggle symmetry across centerline X=0.
- Build mirrored nodes/beams/panels/components.
- Edit master updates slave.
- Break symmetry option.
- Visual mirror plane.

MVP must implement symmetry early.

### 18.2 Snapping

Snap types:

- Grid snap.
- Node snap.
- Beam midpoint.
- Beam endpoint.
- Surface snap.
- Port snap.
- Centerline snap.
- Angle snap.
- Height snap.

Snap settings:

- Distance increment.
- Angle increment.
- Snap enable/disable.
- Smart suggestions.

### 18.3 Undo/redo

Undo/redo is non-negotiable.

Requirements:

- All editor operations are commands.
- Commands support apply/revert.
- Save file dirty state.
- Undo stack survives mode switching.
- Does not need to survive game restart for MVP.

Command examples:

- AddNodeCommand.
- MoveNodeCommand.
- AddBeamCommand.
- ChangeMaterialCommand.
- AddRouteCommand.
- DeleteSelectionCommand.
- RepairPartCommand.

### 18.4 Selection

Selection types:

- Single.
- Multi-select.
- Box select.
- Select connected.
- Select material.
- Select damaged.
- Select network.
- Select mirrored pair.

Selection UI:

- Outline selected objects.
- Show manipulator.
- Show property inspector.
- Show dependency warnings before delete.

### 18.5 Validation

Validation must run continuously but cheaply.

Validation categories:

- Build topology.
- Structural warnings.
- Systems warnings.
- Mission readiness.
- Repair warnings.

Severity levels:

- Info.
- Warning.
- Critical.
- Blocking.

Blocking errors should be rare. Let players build dangerous things.

---

## 19. Testing UI

### 19.1 Launch flow

In Workshop:

- Click Test.
- Game compiles vehicle.
- Shows readiness checklist.
- Player can “Launch anyway” unless fatal.
- Vehicle appears at ramp/water.

Readiness checklist:

- Floats.
- Engine connected.
- Fuel connected.
- Controls connected.
- Battery charge.
- Pump status.
- No critical missing hull panels.

### 19.2 Telemetry overlay

Telemetry:

- Speed.
- RPM.
- Fuel.
- Battery.
- Pump state.
- Waterline.
- List/heel angle.
- Damage.
- Mission objective.

Expert overlay:

- COM/COB.
- Buoyancy samples.
- Drag vectors.
- Thrust vector.
- Rudder force.
- Flood compartments.
- Electrical networks.
- Fuel flow.

### 19.3 Auto-tests

Auto-tests:

- Float test.
- Engine start test.
- Control response test.
- Pump test.
- Cargo load test.
- Leak test.
- Range estimate.

Auto-tests should produce plain-language results.

---

# Part IV: Technical architecture

---

## 20. Engine strategy

### 20.1 Requirement: code-only engine

The project should avoid Unity/Unreal for the MVP because the user wants a code-only AI-built stack and high control over physics/simulation. The engine should be a custom game/runtime built from lower-level libraries, not a visual editor dependency.

“Code-only engine” does not mean writing every primitive from scratch. It means:

- No dependency on a proprietary visual game editor.
- Game objects, systems, assets, physics, and UI are defined in source/data.
- Build and iteration are scriptable.
- AI coding agents can inspect and modify the whole project.
- The engine has deterministic tests and command-line simulation.

### 20.2 Recommended primary stack

Recommended stack:

- Language: Rust.
- Window/input: winit or SDL3 bindings.
- Renderer: wgpu.
- UI: egui integrated into custom renderer.
- Math: glam.
- ECS/data model: custom ECS or lightweight ECS crate; avoid over-coupling.
- Physics/collision: custom marine rigid body + optional Rapier/Jolt for collision.
- Job system: rayon/custom work stealing.
- Serialization: serde + JSON/RON for blueprints; binary cache later.
- Asset loading: glTF for meshes; custom material definitions.
- Audio: kira/rodio initially.
- Profiling: tracy or puffin.
- Testing: Rust unit tests, integration tests, property tests, golden blueprint tests.

Why Rust:

- Good fit for AI-assisted development because compile errors are precise.
- Memory safety helps reduce hard-to-debug crashes.
- No garbage collector.
- Strong module and type boundaries.
- Good data serialization ecosystem.
- Can still write high-performance code and use unsafe only in isolated places.

### 20.3 Alternative stack

Alternative stack if C++ expertise is preferred:

- Language: C++20/23.
- ECS: Flecs.
- Physics: Jolt.
- Renderer: bgfx, Vulkan, or wgpu-native.
- UI: Dear ImGui.
- Serialization: nlohmann/json or simdjson + custom binary.
- Math: glm or DirectXMath.
- Build: CMake.
- Testing: Catch2/doctest.
- Profiling: Tracy.

C++ integrates naturally with Jolt and many game libraries. Rust is preferable for AI-generated code safety unless the project demands C++ library integration from day one.

### 20.4 Physics architecture recommendation

The MVP should **not** rely on a general rigid body engine to solve the entire boat. The game should own the vehicle simulation.

Use:

- Custom single-rigid-body vehicle physics.
- Custom buoyancy, drag, prop, rudder, flooding, and system solvers.
- Collision library for environment contact and broadphase/narrowphase.
- Rigid body library only where useful.

This lets the game prioritize the exact physics that matter: buoyancy, flooding, systems, damage, and readable vehicle behavior.

---

## 21. High-level engine modules

Recommended crate/module layout:

```text
harborworks/
  Cargo.toml
  crates/
    hw_app/
    hw_core/
    hw_math/
    hw_platform/
    hw_render/
    hw_ui/
    hw_assets/
    hw_ecs/
    hw_editor/
    hw_vehicle/
    hw_vehicle_compile/
    hw_sim/
    hw_physics/
    hw_water/
    hw_systems/
    hw_damage/
    hw_missions/
    hw_save/
    hw_audio/
    hw_tools/
    hw_tests/
  assets/
    parts/
    materials/
    missions/
    maps/
    meshes/
    textures/
    shaders/
  schemas/
    blueprint_v1.schema.json
    part_v1.schema.json
    material_v1.schema.json
    mission_v1.schema.json
  tools/
    validate_assets/
    generate_schemas/
    sim_replay/
    blueprint_lint/
```

### 21.1 `hw_core`

Shared types:

- IDs.
- Handles.
- Time.
- Units.
- Error types.
- Logging.
- Configuration.
- Feature flags.

### 21.2 `hw_math`

Math wrappers and utilities:

- Vec2/Vec3/Quat/Mat4 aliases.
- Transform.
- AABB.
- OBB.
- Plane.
- Ray.
- Segment.
- Triangle.
- Mesh helpers.
- Curve/spline utilities.
- Numeric tolerances.
- Stable hashing for geometry references.

### 21.3 `hw_platform`

Platform abstraction:

- Window.
- Input.
- File paths.
- Clipboard.
- DPI scaling.
- Gamepad.
- Hotkeys.
- Timing.
- OS events.

### 21.4 `hw_render`

Rendering:

- GPU initialization.
- Render graph.
- PBR materials.
- Mesh instances.
- Editor lines/gizmos.
- Selection outlines.
- X-ray overlays.
- Transparent panels.
- UI composition.
- Debug visualizations.
- Screenshot capture.

### 21.5 `hw_ui`

UI integration:

- egui integration.
- Panels.
- Widgets.
- Icons.
- Theme.
- Property inspectors.
- Graph visualization.
- Editor dock layout.
- Tooltip system.

### 21.6 `hw_assets`

Asset system:

- Load JSON/RON part definitions.
- Load glTF meshes.
- Load textures.
- Material definitions.
- Hot reload in editor.
- Asset registry.
- Missing asset fallbacks.
- Asset validation.

### 21.7 `hw_editor`

Editor commands and tools:

- Selection.
- Gizmos.
- Snapping.
- Symmetry.
- Undo/redo.
- Tool modes.
- Viewport camera.
- Drag/drop.
- Command history.
- Validation display.

### 21.8 `hw_vehicle`

Editable vehicle model:

- Nodes.
- Beams.
- Panels.
- Components.
- Ports.
- Routes.
- Controls.
- Compartments.
- Paint.
- Blueprint metadata.
- Editing APIs.

### 21.9 `hw_vehicle_compile`

Vehicle compiler:

- Topology validation.
- Geometry generation.
- Mass/inertia calculation.
- Collider generation.
- Compartment analysis.
- Buoyancy sample generation.
- System graph compilation.
- Damage zone generation.
- Runtime proxy creation.

### 21.10 `hw_sim`

Simulation runtime:

- Fixed timestep.
- Vehicle state.
- World state.
- System update scheduler.
- Mission state.
- Telemetry recording.
- Replay.
- Deterministic test harness.

### 21.11 `hw_physics`

Physics primitives:

- Rigid body integrator.
- Force accumulation.
- Collision interface.
- Contact resolution wrapper.
- Towing constraints.
- Cargo constraints.
- Damage impulse routing.

### 21.12 `hw_water`

Water system:

- Water height queries.
- Wave models.
- Buoyancy sampling.
- Hydrodynamic drag.
- Wake effects optional.
- Debug overlays.
- Environment parameters.

### 21.13 `hw_systems`

System graph solvers:

- Electrical.
- Fuel.
- Bilge/water.
- Controls.
- Diagnostic messages.
- Network tracing.
- Runtime states.

### 21.14 `hw_damage`

Damage and repair:

- Damageable state arrays.
- Impact damage.
- Leak creation.
- Electrical water damage.
- Repair actions.
- Visual damage mapping.
- Repair cost/time estimation.

### 21.15 `hw_missions`

Missions:

- Mission definitions.
- Objectives.
- Triggers.
- Rewards.
- Failure conditions.
- NPC/target entities.
- Mission UI.
- Progression.

### 21.16 `hw_save`

Persistence:

- Blueprint save/load.
- Profile/career save.
- Replay save.
- Version migration.
- Checksums.
- Autosave.

### 21.17 `hw_tools`

Command-line tools:

- Asset validator.
- Blueprint validator.
- Simulation replay runner.
- Performance benchmark.
- Schema generator.
- Headless mission test.

---

## 22. Data model in detail

### 22.1 Strong IDs

Use typed IDs instead of raw integers.

Examples:

```rust
struct NodeId(u32);
struct BeamId(u32);
struct PanelId(u32);
struct ComponentId(u32);
struct PortId(u32);
struct RouteId(u32);
struct CompartmentId(u32);
```

Use generational IDs if deletions are common.

Benefits:

- Prevent accidental cross-ID bugs.
- Better compiler errors.
- Easier AI code review.
- Safer serialization.

### 22.2 Editable vehicle storage

Recommended:

```rust
struct VehicleBlueprint {
    meta: BlueprintMeta,
    nodes: SlotMap<NodeId, Node>,
    beams: SlotMap<BeamId, Beam>,
    panels: SlotMap<PanelId, Panel>,
    components: SlotMap<ComponentId, ComponentInstance>,
    routes: SlotMap<RouteId, Route>,
    control_links: Vec<ControlLink>,
    compartments: SlotMap<CompartmentId, CompartmentMarker>,
    paint: PaintData,
}
```

Use stable serialization order:

- Convert maps to sorted arrays on save.
- Store IDs explicitly.
- Include schema version.

### 22.3 Runtime vehicle storage

Runtime should be data-oriented.

Example:

```rust
struct RuntimeVehicle {
    body: BodyState,
    mass: MassProperties,
    colliders: Vec<ColliderProxy>,
    buoyancy_samples: Vec<BuoyancySample>,
    drag_samples: Vec<DragSample>,
    propulsors: Vec<PropulsorState>,
    rudders: Vec<RudderState>,
    compartments: Vec<RuntimeCompartment>,
    electrical: ElectricalRuntime,
    fluids: FluidRuntime,
    controls: ControlRuntime,
    damage: DamageRuntime,
    telemetry: VehicleTelemetry,
}
```

The runtime should not constantly chase editor IDs for hot simulation loops. It should store arrays and small indices.

Maintain mapping:

```rust
RuntimeMapping {
    panel_to_damage_zone: HashMap<PanelId, DamageZoneIndex>,
    component_to_runtime: HashMap<ComponentId, RuntimeComponentIndex>,
}
```

### 22.4 Dirty flags

Editing operations set dirty flags.

Dirty categories:

- topology.
- geometry.
- mass.
- colliders.
- buoyancy.
- compartments.
- electrical.
- fluid.
- controls.
- visuals.
- cost.
- validation.

Example:

- Moving a node dirties geometry, mass, colliders, buoyancy, panels, compartments.
- Changing battery capacity dirties electrical, mass, cost.
- Routing a wire dirties electrical only.
- Painting a panel dirties visuals only.

---

## 23. Simulation loop

### 23.1 Fixed timestep

Use fixed simulation timestep independent of rendering.

Recommended:

- Physics timestep: 1/120 s.
- Systems timestep: 1/20 s or 1/30 s.
- Mission logic: 1/10 s or event-driven.
- Rendering: variable, target 60+ fps.

Loop:

```text
while accumulator >= fixed_dt:
    gather_inputs()
    update_controls(fixed_dt)
    update_systems_if_due()
    update_vehicle_forces(fixed_dt)
    integrate_physics(fixed_dt)
    resolve_collisions(fixed_dt)
    update_damage(fixed_dt)
    update_missions(fixed_dt)
    record_telemetry()
    accumulator -= fixed_dt

render(interpolation_alpha)
```

### 23.2 Determinism

Full cross-platform determinism is not required for MVP, but deterministic tests are strongly recommended.

Practices:

- Use fixed timestep.
- Avoid random without seeded RNG.
- Store replay inputs.
- Sort collections before deterministic processing.
- Use stable IDs.
- Keep headless sim tests.
- Accept minor floating-point differences for visual gameplay.

### 23.3 Multithreading

Threading targets:

- Render thread/main thread.
- Simulation update maybe single thread initially for correctness.
- Parallelizable jobs:
  - buoyancy samples,
  - drag samples,
  - diagnostics,
  - mesh generation,
  - asset loading,
  - validation,
  - telemetry analysis.

Start simple:

- Single-thread fixed sim.
- Multi-thread expensive compile/analysis tasks.
- Then parallelize sample force calculations.

### 23.4 Performance budgets

Target budget on mid-range PC:

- Render frame: 16.6 ms for 60 fps.
- Simulation step: < 4 ms at 120 Hz.
- Vehicle compile after edit: < 100 ms for common edits.
- Heavy geometry rebuild: < 500 ms acceptable if progress shown.
- Systems graph update: < 1 ms.
- Buoyancy force update: < 1 ms for 256 samples.
- Editor UI: < 2 ms typical.
- Save/load: < 1 second.

MVP limits:

- Visual parts per vehicle: 1000.
- Functional components: 100.
- Routes: 200.
- Buoyancy samples: 64 to 512.
- Compartments: 1 to 16.
- Colliders: 16 to 128 simplified proxies.

---

## 24. Custom physics implementation

### 24.1 Body state

```rust
struct BodyState {
    position: Vec3,
    orientation: Quat,
    linear_velocity: Vec3,
    angular_velocity: Vec3,
    force_accum: Vec3,
    torque_accum: Vec3,
}
```

### 24.2 Mass properties

```rust
struct MassProperties {
    mass_kg: f32,
    inv_mass: f32,
    center_of_mass_local: Vec3,
    inertia_local: Mat3,
    inv_inertia_local: Mat3,
}
```

### 24.3 Applying force

```rust
fn apply_force_at_point(body: &mut BodyState, force_world: Vec3, point_world: Vec3) {
    body.force_accum += force_world;
    let r = point_world - body.world_center_of_mass();
    body.torque_accum += r.cross(force_world);
}
```

### 24.4 Integration

Semi-implicit Euler:

```rust
linear_velocity += acceleration * dt
position += linear_velocity * dt

angular_velocity += angular_acceleration * dt
orientation = normalize(delta_rotation(angular_velocity * dt) * orientation)
```

Add damping/clamping:

- Clamp angular velocity.
- Clamp extreme buoyancy impulses.
- Add sleep threshold for inactive bodies.
- Use substeps for high-speed impacts.

### 24.5 Collision

MVP collision options:

Option A: Use Rapier/Jolt for world collisions.

- Vehicle represented as compound collider.
- Engine handles contact manifolds.
- Custom vehicle physics applies impulses or syncs body state.

Option B: Custom simple collision.

- AABB/OBB/convex hull tests.
- Enough for rocks/docks?
- Harder than it sounds.

Recommendation:

- Use an existing collision/rigid body library for environment collision and contact detection.
- Keep custom vehicle forces and systems.
- Treat external rigid bodies like cargo/towed boats with simplified physics.

### 24.6 Cargo

Cargo objects:

- Rigid body boxes/crates.
- Can be attached to cargo slots.
- Contribute mass.
- Can shift if unsecured optional later.

MVP:

- Cargo can be “mounted” to vehicle at slots.
- It adds mass and mission state.
- Loose cargo optional.

### 24.7 Towing

Tow line:

- Constraint between boat tow point and target boat.
- Has length, stiffness, damping, max tension.
- Can break.

Simplified force:

```text
if distance > rope_length:
    tension = stiffness * (distance - rope_length) + damping * relative_speed_along_rope
```

Apply equal/opposite forces.

---

## 25. Water simulation

### 25.1 Water height

MVP water height sources:

- Flat water.
- Gerstner waves or sinusoidal wave layers.
- Mission-defined wave conditions.

Water query API:

```rust
struct WaterSample {
    height: f32,
    normal: Vec3,
    velocity: Vec3,
}

trait WaterSurface {
    fn sample(&self, xz: Vec2, time: f32) -> WaterSample;
}
```

### 25.2 Buoyancy sample generation

Given compiled hull/compartments:

1. Generate local-space sample points.
2. Assign volume weights.
3. Group by compartment.
4. Store local position and volume.

Simplified methods:

- Use bounding boxes per compartment.
- Fill with grid points.
- Remove points outside approximate hull using ray tests.
- Weight by total compartment volume / sample_count.

MVP acceptable shortcut:

- For starter hulls, use generated hull cross-section volumes.
- For custom hulls, use voxel approximation.

### 25.3 Drag samples

Drag sample properties:

- local_position.
- area.
- normal.
- forward_drag_cd.
- side_drag_cd.
- angular_damping_factor.

Use velocity at point:

```text
point_velocity = linear_velocity + angular_velocity × r
relative_water_velocity = point_velocity - water_velocity
```

Apply drag opposite relative velocity when underwater.

### 25.4 Wave impacts

MVP wave impact:

- If bow/side sample hits wave above speed threshold, apply impulse.
- Damage chance if impact energy exceeds panel strength.
- Show splash and stress warning.

---

## 26. Systems solvers

### 26.1 Update frequency

Systems do not need 120 Hz.

Recommended:

- Controls: 120 Hz.
- Electrical: 20 Hz.
- Fuel: 20 Hz.
- Bilge/water: 20 Hz.
- Diagnostics: 5 Hz.
- UI smoothing: render frame.

### 26.2 Electrical runtime

Data-oriented structures:

```rust
struct ElectricalRuntime {
    nodes: Vec<ElectricalNode>,
    edges: Vec<ElectricalEdge>,
    networks: Vec<ElectricalNetwork>,
    sources: Vec<SourceIndex>,
    loads: Vec<LoadIndex>,
}
```

Electrical node types:

- Source.
- Load.
- Switch.
- Fuse.
- Connector.
- Ground/reference optional.

Solver steps:

1. Update switch states from control graph.
2. Build active graph.
3. Find connected components.
4. For each component:
   - sum sources,
   - sum loads,
   - detect shorts,
   - compute voltage,
   - update source/load states,
   - trip fuses if needed.
5. Emit diagnostics.

### 26.3 Fluid runtime

```rust
struct FluidRuntime {
    fuel_networks: Vec<FluidNetwork>,
    water_networks: Vec<FluidNetwork>,
    tanks: Vec<TankState>,
    pumps: Vec<PumpState>,
    pipes: Vec<PipeState>,
}
```

Solver:

- Graph connectivity.
- Source/sink demand.
- Pump capacity.
- Valve state.
- Pipe damage.
- Flow allocation.

MVP flow allocation can be simple:

```text
available_flow = min(source_available, pipe_capacity, pump_capacity)
consumer_receives = min(consumer_demand, available_flow)
```

### 26.4 Controls runtime

```rust
struct ControlRuntime {
    inputs: Vec<ControlInput>,
    links: Vec<ControlLink>,
    outputs: Vec<ControlOutput>,
}
```

Control update:

- Read player input.
- Read cockpit input states.
- Apply curves/inversion.
- Route to outputs.
- Store output commands:
  - throttle,
  - rudder angle,
  - switch states,
  - starter pulse,
  - pump toggle.

---

## 27. Rendering architecture

### 27.1 Render goals

The MVP should not chase photorealism first. It should chase clarity and enough beauty to sell the fantasy.

Visual priorities:

1. Clear construction elements.
2. Readable system overlays.
3. Good water.
4. Solid PBR materials.
5. Damage visibility.
6. Workshop atmosphere.

### 27.2 Render pipeline

Suggested passes:

- Depth prepass optional.
- Opaque PBR.
- Transparent/x-ray panels.
- Water.
- Decals/damage.
- Lines/gizmos/routes.
- Selection outlines.
- Part previews/ghosts.
- UI.

### 27.3 Geometry

Visual geometry sources:

- Procedural beams.
- Procedural panels.
- Component mesh instances.
- Route tubes/wires.
- Generated damage decals.
- Environment meshes.

Optimization:

- Instance repeated beam profiles/components.
- Combine static panel meshes per vehicle section.
- Rebuild only dirty mesh groups.
- Use LOD for environment.
- Render routes as GPU line/tube instances.

### 27.4 Editor overlays

Overlays:

- COM/COB markers.
- Waterline.
- Buoyancy samples.
- Stress heatmap.
- Electrical networks.
- Fuel networks.
- Bilge networks.
- Damage labels.
- Snap guides.
- Symmetry plane.

Overlays should be toggleable and not overwhelm the player.

---

## 28. Asset and content pipeline

### 28.1 Part definition schema

Each part definition:

```json
{
  "id": "small_diesel_engine",
  "display_name": "Small Diesel Engine",
  "category": "propulsion",
  "tags": ["engine", "marine", "starter"],
  "mass_kg": 420.0,
  "cost": 8500,
  "mesh": "meshes/parts/small_diesel_engine.glb",
  "attach_rules": [
    { "type": "requires_surface", "surface_tags": ["engine_mount", "deck_reinforced"] }
  ],
  "ports": [
    {
      "id": "fuel_in",
      "type": "fuel_in",
      "local_position": [0.2, 0.1, -0.4],
      "capacity": 1.0
    },
    {
      "id": "starter_in",
      "type": "electric_in",
      "voltage": 24,
      "max_current_a": 80
    },
    {
      "id": "shaft_out",
      "type": "shaft_out",
      "local_position": [0, -0.1, -0.8]
    }
  ],
  "behavior": {
    "type": "diesel_engine",
    "max_power_kw": 180,
    "max_rpm": 3600,
    "idle_rpm": 700,
    "fuel_type": "diesel"
  }
}
```

### 28.2 Material definition schema

```json
{
  "id": "marine_aluminum_5083",
  "display_name": "Marine Aluminum",
  "density_kg_m3": 2660,
  "yield_strength_mpa": 215,
  "cost_per_kg": 5.2,
  "watertight_rating": 0.95,
  "impact_toughness": 0.65,
  "repair_difficulty": 0.35,
  "pbr_material": "materials/marine_aluminum.mat.json"
}
```

### 28.3 Mission definition schema

```json
{
  "id": "cargo_run_01",
  "display_name": "Harbor Cargo Run",
  "description": "Deliver 300 kg of supplies to the outer dock.",
  "required_vehicle_class": "marine_small",
  "start_location": "workshop_dock",
  "objectives": [
    { "type": "load_cargo", "cargo_id": "supplies_300kg" },
    { "type": "reach_zone", "zone_id": "outer_dock" },
    { "type": "unload_cargo", "zone_id": "outer_dock" },
    { "type": "return_to_zone", "zone_id": "workshop_dock" }
  ],
  "rewards": {
    "money": 2500,
    "reputation": 120
  },
  "failure_conditions": [
    { "type": "vehicle_sunk" },
    { "type": "cargo_destroyed" }
  ]
}
```

### 28.4 Asset validation

Every asset must pass:

- JSON schema validation.
- Referenced mesh exists.
- Referenced material exists.
- Ports have valid types.
- Behaviors have required fields.
- Mass/cost non-negative.
- Attach rules valid.
- Mission objectives valid.

This is critical for AI-assisted content generation.

---

## 29. Save/load

### 29.1 Blueprint save

Blueprint save format:

- Human-readable JSON or RON in MVP.
- Include schema version.
- Include stable IDs.
- Include optional cached analysis.
- Include checksum.

Blueprint file sections:

- meta.
- nodes.
- beams.
- panels.
- components.
- routes.
- control links.
- compartments.
- paint.
- camera thumbnail optional.

### 29.2 Career save

Career save:

- profile id.
- money.
- reputation.
- unlocked parts.
- completed missions.
- active vehicle id.
- inventory optional.
- damaged vehicles.
- world state minimal.

### 29.3 Migration

Schema migration is required from the start.

Approach:

- Every save has version.
- Migration functions:
  - v1 → v2,
  - v2 → v3.
- Test old sample saves.

---

## 30. AI-assisted development process

### 30.1 Why this matters

If the project is built heavily with AI, architecture must prevent AI from creating a giant tangled stew. The project needs small modules, explicit contracts, tests, and schemas.

### 30.2 AI coding principles

Rules for AI-generated code:

1. Every module has a README with responsibilities and non-responsibilities.
2. Public APIs are small and typed.
3. No module may directly access unrelated module internals.
4. Every feature gets tests before integration.
5. Data schemas are validated.
6. No hidden global mutable state.
7. Simulation code must be deterministic under fixed inputs where practical.
8. Debug views are created alongside simulation systems.
9. All generated code must compile with strict warnings.
10. Any unsafe Rust must be isolated, documented, and reviewed.

### 30.3 Promptable work units

AI tasks should be small.

Good task:

> Implement `ElectricalNetworkBuilder` that takes compiled component ports and wire routes and returns connected electrical networks. Include unit tests for disconnected load, fuse trip, and two independent networks.

Bad task:

> Build the whole electrical system.

Good task:

> Create the JSON schema for material definitions and add a validator CLI that prints missing fields and invalid ranges.

Bad task:

> Make content pipeline.

### 30.4 Test-first scaffolding

For each system, create:

- Unit tests.
- Integration tests.
- Golden data tests.
- Visualization/debug examples.
- Benchmark tests for hot paths.

Examples:

- `test_boat_floats_symmetric_hull`.
- `test_engine_starves_without_fuel_route`.
- `test_pump_removes_water_when_powered`.
- `test_wire_overload_trips_fuse`.
- `test_compartment_flooding_changes_mass`.
- `test_blueprint_save_load_roundtrip`.
- `test_mission_cargo_completion`.

### 30.5 Headless simulation

Create a command-line tool:

```text
cargo run -p sim_replay -- --blueprint examples/basic_boat.json --mission sea_trial --seconds 120
```

Outputs:

- pass/fail.
- telemetry CSV.
- warnings.
- performance stats.
- final vehicle state.

This makes AI iteration vastly easier because agents can run tests without graphics.

### 30.6 Documentation contracts

Every crate/module should include:

- Purpose.
- Public API summary.
- Data ownership.
- Threading assumptions.
- Determinism assumptions.
- Error handling.
- Tests.
- Known limitations.

---

# Part V: Implementation plan

---

## 31. Milestone 0: Foundation

### Goal

A working custom app skeleton with rendering, UI, input, assets, and test harness.

### Features

- Window opens.
- Basic renderer draws grid and cube.
- egui panels render.
- Camera controls.
- Asset loader reads JSON.
- Basic project structure.
- Unit tests run.
- CI checks build/test/format.
- Headless app mode exists.

### Acceptance criteria

- App runs at 60 fps with empty scene.
- UI panel can toggle debug grid.
- Command-line test runner works.
- Asset validation CLI reads a material file.
- No game logic yet.

---

## 32. Milestone 1: Editable structure prototype

### Goal

Player can build a crude frame from nodes and beams.

### Features

- Add/move/delete nodes.
- Add/delete beams.
- Beam mesh generation.
- Beam profiles.
- Material assignment.
- Symmetry.
- Snapping.
- Undo/redo.
- Save/load blueprint.
- Mass calculation.

### Acceptance criteria

- Player can build a rectangular frame.
- Mass/COM updates.
- Symmetry works.
- Save/load roundtrip preserves structure.
- 100 beams render at 60 fps.

---

## 33. Milestone 2: Panels and hull

### Goal

Player can create a watertight hull shape from panels.

### Features

- Fill loop with panel.
- Panel mesh generation.
- Panel material/thickness.
- Simple curved panel.
- Hull validation.
- Compartment marker.
- Basic buoyancy volume estimation.

### Acceptance criteria

- Player can build a boxy hull.
- System identifies open seams.
- Hull has approximate volume.
- Blueprint saves/loads panels.
- Visual panels can be transparent/x-ray.

---

## 34. Milestone 3: Custom boat physics

### Goal

A built hull can float and move as a rigid body.

### Features

- Runtime vehicle compiler.
- Mass/inertia calculation.
- Buoyancy samples.
- Drag samples.
- Water surface.
- Launch test scene.
- Basic rigid body integration.
- Debug COM/COB/waterline.
- Sinking if overweight.

### Acceptance criteria

- Simple hull floats.
- Heavy cargo makes it sit lower.
- High COM can capsize.
- Removing a panel can cause flooding placeholder.
- Simulation stable for 5 minutes.

---

## 35. Milestone 4: Propulsion and controls

### Goal

Player-built boat can be driven.

### Features

- Engine component.
- Fuel tank component placeholder.
- Propeller/shaft.
- Rudder.
- Helm/throttle controls.
- Control links.
- Engine thrust.
- Steering force.
- Basic UI telemetry.

### Acceptance criteria

- Boat starts and moves.
- Rudder steers.
- Throttle affects speed.
- Bad rudder placement gives weak steering.
- Speed and RPM displayed.

---

## 36. Milestone 5: Electrical and fuel graphs

### Goal

Functional systems require routing.

### Features

- Component ports.
- Wire route tool.
- Fuel pipe route tool.
- Battery.
- Fuse box.
- Switch.
- Lights.
- Starter.
- Fuel tank to engine.
- Electrical graph solver.
- Fuel graph solver.
- Diagnostic warnings.

### Acceptance criteria

- Engine will not start without starter power and fuel route.
- Pump/light requires power.
- Disconnected load warnings appear.
- Fuse trips on overload.
- Fuel valve closed stops engine.
- Routes render clearly.

---

## 37. Milestone 6: Flooding and pumps

### Goal

Hull breaches and pumps interact with compartments.

### Features

- Leak creation.
- Compartment water volume.
- Water mass affects physics.
- Bilge pump.
- Pump requires power.
- Pump water route/outlet.
- Flooding diagnostics.
- Repair patch action.

### Acceptance criteria

- Damaged panel leaks.
- Boat gets heavier and lower as it floods.
- Pump can reduce water if powered/routed.
- Pump without outlet fails.
- Boat can sink.
- Repair patch stops leak.

---

## 38. Milestone 7: Mission vertical slice

### Goal

The MVP becomes a game.

### Features

- Harbor map.
- Sea Trial mission.
- Cargo Run mission.
- Rescue Pickup mission.
- Rewards.
- Career save.
- Mission UI.
- Return to workshop.
- Repair costs.

### Acceptance criteria

- New player can build/modify starter boat and complete Sea Trial.
- Cargo weight affects stability.
- Mission completion grants money/reputation.
- Damage carries back to workshop.
- Player can repair and retry.

---

## 39. Milestone 8: MVP polish

### Goal

A coherent public prototype / internal demo.

### Features

- Tutorial flow.
- 60-100 parts.
- 5 missions.
- Better water and environment art.
- Better UI theme.
- Sound.
- Debug/telemetry tools.
- Performance optimization.
- Crash-safe autosaves.
- Settings menu.
- Accessibility basics.

### Acceptance criteria

- Player can finish tutorial in 30 minutes.
- New player can complete first mission without external help.
- 60 fps target on mid-range hardware.
- No known blueprint corruption bugs.
- All core systems have tests.

---

# Part VI: MVP content specification

---

## 40. Structural parts

### Beams

- Tube beam small.
- Tube beam medium.
- Box beam small.
- Box beam medium.
- Angle beam.
- Flat bar.
- Keel beam.
- Deck beam.
- Engine mount rail.
- Tow hook reinforcement.

### Panels

- Flat sheet panel.
- Curved hull panel.
- Deck plate.
- Fiberglass skin.
- Composite cabin panel.
- Hatch.
- Door.
- Window.
- Rubber fender.

### Structural modules

- Bulkhead.
- Watertight bulkhead.
- Cabin frame.
- Transom frame.
- Bow frame.
- Cargo deck plate.
- Compartment marker.

---

## 41. Propulsion parts

- Small diesel engine.
- Medium diesel engine.
- Small electric motor optional.
- Fuel tank small.
- Fuel tank medium.
- Prop shaft.
- Shaft seal.
- Small propeller.
- Medium propeller.
- Rudder small.
- Rudder medium.
- Throttle lever.
- Helm wheel.

---

## 42. Electrical parts

- 12 V battery.
- 24 V battery.
- Alternator module.
- Fuse box.
- Bus bar.
- Switch.
- Push button.
- Wire small gauge.
- Wire medium gauge.
- Connector.
- Navigation light red/green.
- White mast light.
- Floodlight.
- Cabin light.
- Radio beacon.
- Dashboard gauge.

---

## 43. Fluid parts

- Fuel pipe small.
- Fuel pipe medium.
- Fuel valve.
- Fuel filter optional.
- Bilge pump small.
- Bilge pump medium.
- Water hose.
- Through-hull outlet.
- Check valve.
- Drain fitting.
- Hose connector.

---

## 44. Mission parts

- Cargo crate mount.
- Tow hook.
- Rescue ladder.
- Rescue stretcher/cargo placeholder.
- Searchlight.
- Beacon repair crate.
- Pump hose connector.

---

## 45. Starter blueprints

Starter blueprints:

1. Empty frame tutorial.
2. Simple skiff.
3. Rescue dinghy.
4. Cargo workboat.
5. Towboat starter.

Blueprints should intentionally have fixable flaws in tutorials.

---

# Part VII: Testing and quality

---

## 46. Automated tests

### 46.1 Save/load tests

- Empty blueprint roundtrip.
- Basic hull roundtrip.
- Systems blueprint roundtrip.
- Version migration.
- Invalid schema rejection.

### 46.2 Geometry tests

- Beam mesh length correct.
- Panel triangulation valid.
- Mirrored geometry symmetrical.
- No NaNs.
- Collider generation completes.

### 46.3 Physics tests

- Symmetric hull floats level.
- Added mass changes waterline.
- High COM reduces stability.
- Pumped water changes mass.
- Prop thrust accelerates vehicle.
- Rudder produces yaw.

### 46.4 Systems tests

Electrical:

- Disconnected load unpowered.
- Fuse trip.
- Battery drains.
- Alternator charges.
- Water damage disconnects.

Fuel:

- Engine receives fuel through valid path.
- Closed valve blocks fuel.
- Broken pipe leaks.
- Empty tank stops engine.

Water:

- Leak fills compartment.
- Pump removes water.
- Outlet below waterline backflow rule.

### 46.5 Mission tests

- Sea trial completes.
- Cargo delivery completes.
- Rescue target pickup completes.
- Sinking fails mission.
- Reward applied once only.

### 46.6 Performance tests

- 1000 beam blueprint compile.
- 500 panel mesh generation.
- 256 buoyancy samples at 120 Hz.
- 200 routes diagnostic update.
- Save/load large blueprint.

---

## 47. Debug tools

Debug tools are mandatory.

### 47.1 Visual debug overlays

- Physics colliders.
- Buoyancy samples.
- Drag vectors.
- Thrust vector.
- Rudder force.
- COM/COB.
- Electrical network IDs.
- Fuel flow arrows.
- Water ingress arrows.
- Damage zones.
- Compartment volumes.

### 47.2 Logs

Structured logs:

- Vehicle compile warnings.
- Systems solver warnings.
- Physics instability warnings.
- Mission events.
- Save/load events.
- Performance spikes.

### 47.3 Telemetry

Record:

- Time.
- Position.
- Velocity.
- Orientation.
- Fuel.
- Battery.
- Water per compartment.
- Engine RPM.
- Pump flow.
- Damage events.
- Mission progress.

Telemetry export:

- CSV.
- JSON.
- In-game graph.

---

## 48. Performance strategy

### 48.1 Avoid hot-path allocations

Hot loops:

- Physics.
- Buoyancy.
- Drag.
- Systems update.
- Rendering instance updates.

Use preallocated vectors and arenas.

### 48.2 Compile proxies

Never simulate directly from editor graph when in gameplay.

Blueprint:

- editable,
- rich,
- human-readable.

Runtime:

- compact,
- array-based,
- cache-friendly.

### 48.3 Caching

Cache:

- Meshes.
- Colliders.
- Mass properties.
- Buoyancy samples.
- System graph connectivity.
- Validation results.
- Part definitions.

Invalidate with dirty flags.

### 48.4 Level of detail

Vehicle LOD:

- Full detail in workshop.
- Route details hidden at distance.
- Panel details combined.
- Damage decals limited.
- System overlays disabled in mission unless toggled.

### 48.5 GPU use

Use GPU for:

- Rendering.
- Instanced beams/routes.
- Water shading.
- Heatmap overlays.

Avoid GPU compute for MVP unless absolutely needed. CPU simulation is easier to test and debug.

---

# Part VIII: Future expansion compatibility

---

## 49. Future vehicle classes

### Cars/trucks

Required additions:

- Wheel/tire physics.
- Suspension geometry.
- Engine drivetrain.
- Gearbox/differential.
- Brake model.
- Terrain traction.

Reuse:

- Structure.
- Panels.
- Electrical.
- Fuel.
- Controls.
- Damage.
- Missions.

### Aircraft

Required additions:

- Aerodynamic surfaces.
- Lift/drag/stall.
- Flight controls.
- Stability.
- Prop/jet models.
- Airframe stress.
- Landing gear.

Reuse:

- Structure.
- Panels.
- Electrical.
- Fuel.
- Controls.
- Damage.
- Systems routing.

### Helicopters

Required additions:

- Rotor model.
- Collective/cyclic control.
- Anti-torque.
- Rotor damage.
- Flight stability helpers.

### Sailboats

Required additions:

- Sail aerodynamics.
- Rigging.
- Keel/foil hydrodynamics.
- Wind model.
- Crew/ballast maybe.

### Submarines

Required additions:

- Ballast tanks.
- Pressure hull.
- Depth pressure.
- Oxygen/power endurance.
- Underwater drag.

### Spacecraft

Required additions:

- Orbital mechanics.
- Rocket engines.
- Staging.
- RCS.
- Vacuum/atmospheric modes.

---

## 50. Architecture hooks for expansion

Keep these abstraction points:

- VehicleClass.
- PhysicsModule.
- EnvironmentModule.
- ComponentBehavior.
- PortType.
- RouteType.
- MissionObjective.
- DamageType.
- MaterialBehavior.
- ControlSignal.

Do not hardcode boat-specific behavior into generic vehicle builder modules. Put marine-specific code in `hw_marine` or `hw_water`.

---

# Part IX: Key risks and mitigations

---

## 51. Risk: Builder complexity overwhelms player

Mitigation:

- Guided starter hulls.
- Smart templates.
- Auto-frame tools.
- Simple mode/expert mode.
- Clear warnings.
- Good tutorial.

## 52. Risk: Panel generation becomes CAD nightmare

Mitigation:

- Restrict panel topology.
- Use loop fill with triangulation.
- Modular cutouts.
- Mild curvature only.
- Improve gradually.

## 53. Risk: Physics instability

Mitigation:

- One main rigid body per vehicle.
- Simplified colliders.
- Force clamps.
- Fixed timestep.
- Substeps.
- Extensive debug overlays.
- Avoid per-beam rigid bodies.

## 54. Risk: Systems become too abstract

Mitigation:

- Visible wires/pipes.
- Port-based routing.
- Clear diagnostic messages.
- Failures tied to actual routes.
- Test buttons.

## 55. Risk: AI codebase becomes tangled

Mitigation:

- Modular contracts.
- Strict tests.
- Small task prompts.
- Schema validation.
- CI from day one.
- No giant god objects.
- Headless replay tests.

## 56. Risk: Scope creep

Mitigation:

- MVP is boats only.
- Every new feature must support core loop.
- Backlog future vehicles explicitly.
- Do not build aircraft until marine loop works.

---

# Part X: Definition of done

---

## 57. MVP is done when

The MVP is done when:

1. A player can create a small boat from beams and panels.
2. The boat compiles into a runtime simulation proxy.
3. The boat floats, lists, capsizes, floods, or sinks based on design.
4. The player can install engine, fuel tank, propeller, rudder, helm, battery, fuse box, lights, and pump.
5. The player can route fuel, electrical, water, and control connections.
6. The diagnostics identify missing or broken systems.
7. The player can launch into a harbor test scene.
8. The player can complete at least five missions.
9. Damage can create leaks and system failures.
10. The player can repair or redesign after failure.
11. Blueprints and career state save/load reliably.
12. The simulation has automated tests and headless replays.
13. Performance is acceptable on a mid-range PC.
14. The game teaches the player through understandable feedback.

---

# Appendix A: Example blueprint skeleton

```json
{
  "schema_version": 1,
  "meta": {
    "blueprint_id": "8e3a5d4e-0000-4000-9000-boat00000001",
    "display_name": "Starter Rescue Skiff",
    "vehicle_class": "marine_small"
  },
  "nodes": [
    { "id": 1, "position": [0.0, 0.0, 0.0] },
    { "id": 2, "position": [1.0, 0.0, 0.0] }
  ],
  "beams": [
    {
      "id": 1,
      "node_a": 1,
      "node_b": 2,
      "profile_id": "tube_small",
      "material_id": "marine_aluminum_5083"
    }
  ],
  "panels": [],
  "components": [],
  "routes": [],
  "control_links": []
}
```

---

# Appendix B: Example diagnostic messages

## Structural

- “Forward compartment is not sealed. Add panels or mark as open.”
- “Panel P-042 unsupported over 1.8 m span. Add stringer.”
- “Engine mount load exceeds rating by 32%.”
- “Center of mass is high. Capsize risk increased.”
- “Reserve buoyancy is below recommended mission minimum.”

## Electrical

- “Bilge Pump 1 has no powered network.”
- “Starter wire gauge too small.”
- “Fuse F1 trips when floodlights and pump run together.”
- “Battery will drain in 9 minutes at current load.”
- “Navigation lights are required for night missions.”

## Fuel

- “Engine 1 has no fuel path.”
- “Fuel valve V2 is closed.”
- “Fuel tank empty.”
- “Damaged pipe leaking fuel.”

## Water

- “Compartment 2 flooding: ingress 42 L/min.”
- “Pump 1 removing 30 L/min. Net flooding remains 12 L/min.”
- “Outlet below waterline may backflow.”
- “Pump has power but no water inlet.”

## Mission

- “Cargo load exceeds safe payload.”
- “Tow hook not installed.”
- “Rescue ladder missing.”
- “Night mission requires navigation lights.”

---

# Appendix C: Recommended AI task sequence

1. Create repository and module skeleton.
2. Implement math and ID types.
3. Implement asset schema validation.
4. Implement window/render grid.
5. Implement egui panels.
6. Implement editable nodes/beams.
7. Implement beam mesh generation.
8. Implement undo/redo.
9. Implement blueprint save/load.
10. Implement panel loop fill.
11. Implement mass calculation.
12. Implement runtime compiler.
13. Implement buoyancy samples.
14. Implement rigid body physics.
15. Implement water surface.
16. Implement test launch scene.
17. Implement propulsion.
18. Implement controls.
19. Implement ports.
20. Implement electrical graph.
21. Implement fuel graph.
22. Implement bilge/flooding.
23. Implement damage.
24. Implement repair.
25. Implement missions.
26. Polish UI/tutorial.

---

# Appendix D: Non-negotiable early tests

- Save/load does not lose IDs.
- Undo/redo works after delete and restore.
- Symmetry does not create orphan references.
- Compiled vehicle has finite mass/inertia.
- No NaNs in physics after 10 minutes.
- Basic hull floats.
- Overweight hull sinks.
- Engine needs fuel.
- Pump needs power.
- Damaged panel leaks.
- Mission reward applies once.
- Invalid asset fails validation clearly.

---

# Appendix E: MVP north-star demo script

1. Open workshop.
2. Load starter hull.
3. Add missing forward panel.
4. Add two stringers to fix support warning.
5. Place diesel engine and fuel tank.
6. Route fuel pipe.
7. Place battery, fuse box, switch, and bilge pump.
8. Route power wire to pump and starter.
9. Link helm to rudder and throttle to engine.
10. Run readiness checklist.
11. Launch into harbor.
12. Boat floats but lists due to tank placement.
13. Move tank lower/center.
14. Relaunch, boat handles well.
15. Take cargo mission.
16. Hit a rock, forward compartment leaks.
17. Pump slows flooding but cannot keep up.
18. Return to workshop.
19. Patch panel, add bigger pump.
20. Complete mission and earn reward.

That is the MVP in one dramatic little voyage.
