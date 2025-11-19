# Astrological Scheduling Formulas

## Priority Calculation

```
final_priority = base_priority × planetary_influence × element_boost
```

### Base Priorities
```
Critical: 1000  |  System: 200  |  Interactive: 150
Desktop: 120    |  CPU/Network: 100  |  Memory: 80
```

### Planetary Rulerships
- **Mars** → CPU-Intensive
- **Mercury** → Network, Interactive
- **Jupiter** → Memory-Heavy
- **Saturn** → System
- **Venus** → Desktop/UI
- **Sun** → Critical tasks

## Planetary Influence

**Retrograde**: `-1.0` (applies 50% time slice penalty)

**Direct** (by element of zodiac sign):
- Fire: `1.3` | Air: `1.2` | Earth: `1.1` | Water: `1.0`

## Element Boost/Debuff

**BOOSTED (1.3-1.5x)**
- Fire × CPU: 1.5
- Air × Network: 1.5
- Earth × System: 1.4
- Water × Memory: 1.3

**DEBUFFED (0.6-0.7x)** - Opposing elements
- Water × CPU: 0.6 (dampens fire)
- Earth × Network: 0.6 (blocks air)
- Air × System: 0.7 (disrupts earth)
- Fire × Memory: 0.7 (evaporates water)

**Neutral**: 1.0 (all other combinations)

## Retrograde Detection

```rust
delta = longitude_tomorrow - longitude_today

retrograde = if delta > 180.0: true      // crossed 360° backward
             else if delta < -180.0: false  // crossed 360° forward
             else: delta < 0.0              // normal backward motion
```

Sun and Moon never retrograde.

## Example

**rustc (CPU task), Mars in Scorpio (Water), direct:**
```
100 × 1.0 × 0.6 = 60  → DEBUFFED
```

**rustc (CPU task), Mars in Aries (Fire), direct:**
```
100 × 1.3 × 1.5 = 195  → BOOSTED
```

Positions cached 5min. Calculated via `astro` crate (real ephemeris data).
