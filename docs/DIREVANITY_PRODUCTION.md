# Direvanity: the long campaign and living interface

Accepted direction: 25 November 2026. **Direvanity** is the game's name. The canonical repository
remains [Herbertofury/Dreadmarch](https://github.com/Herbertofury/Dreadmarch); engine crate names,
legacy save keys, source history and old release identities remain compatible.

## The experience we are building

An expansive offline, character-led action RPG: the world curiosity and interlocking lives of the
original Divine Divinity, the immediate combat language and class identity of Diablo, and the build
experimentation and encounter expression already recorded in our other inspirations. The world must
invite another adventure because its people, places and decisions are interesting. Repeating chores
or making enemies take longer to kill is not a substitute for more game.

The target is **100+ hours of main campaign, 100+ hours of substantial optional stories, and 100+
hours of endgame discovery and mastery**, with room to keep growing. There is no 100-hour cap.
The planning envelope below exceeds those floors, but it is **not measured playtime, a release
promise, or a claim that those hours currently exist**. A timed, independent player study must earn
any future duration claim. Production models, animation, voice, sound, environmental storytelling,
boss encounters and repeated player review must earn any AAA quality claim.

## A budgetable content plan

The machine-readable authority is [long_campaign.json](../data/production/long_campaign.json).
It contains original narrative premises, pivotal decisions, distinctive boss designs, dependencies,
region groups, faction arcs and endgame disciplines. It stays separate from playable definitions.
Run `python3 scripts/validate_content_plan.py` to verify its identities, dependency graph, estimates
and explicit distinction between production targets and shipped content.

| Content lane | Planned authored structure | Active-play planning envelope | Current evidence boundary |
| --- | --- | --- | --- |
| Main campaign | 12 acts, 36 regional groups | 134–182 hours | Two playable chapter loops across six story regions; the complete 12-act production campaign remains to be authored and tested |
| Optional stories | 48 substantial arcs across eight factions, four episodes targeted per arc | 108–168 hours | Four playable side quests; the 48 larger arcs remain production designs |
| Endgame | Eight complementary disciplines | 144–246 hours to first mastery | One functional expedition discipline with three objectives, seeded layout variation and three modifiers; seven disciplines remain production designs |

Do not add these estimates together and advertise the result as a finished game's length. These
are separate production budgets, subject to route and overlap audits. Mutually exclusive choices,
idle time, repeated clears, loading and compulsory backtracking do not count as new authored hours.

### Main campaign spine

| Act | Title | Its central dramatic question |
| --- | --- | --- |
| I | The Names Beneath the Ash | Can the Concord protect the living without imprisoning the forgotten? |
| II | The Velvet Famine | How do you end a miraculous harvest when the region depends on its hidden cost? |
| III | A Kingdom of Other Faces | Who has the right to rule when an heir's identity lives in other people? |
| IV | The Mercy of Wolves | What does peace mean when the winter containing a war is killing its survivors? |
| V | The Cathedral That Walks | Can you save a wandering holy city from the engine that feeds it? |
| VI | Where the Dead Keep House | What place can a living society make for citizens who have already died? |
| VII | The Sun Beneath the Mountain | Who should decide which possible futures power the present? |
| VIII | A Crown of Living Thorns | Can a conscious landscape and the people living on it share authority? |
| IX | The War of Unspoken Things | How do you resist a power that turns language into obedience? |
| X | The Witness at the End of Night | What happens when mortals discover why their gods need them? |
| XI | The Siege of Tomorrow | Which allies and institutions have your choices made worth defending? |
| XII | The Last Uncrowned | Can the world survive freedom from the divine vanity it helped create? |

Each act must have a change in pace: town relationships, investigation, exploration, traversal,
combat, a signature encounter, and a playable consequence. Neither every problem nor every class
solution should reduce to killing everything in a circle. The final act includes a return journey
through changed places. Endgame continues from that world rather than deleting its history.

### Optional stories that deserve their length

The eight factions are the Lamplighters, Inkbound Archive, Free Caravan, Ashen Order, Green Covenant,
Salt Court, Cinder Guild and Uncrowned Wayfarers. Each has six named arcs in the catalogue. Their
conflicts concern shelter, knowledge, trade, duty, ecology, memory, invention and belonging.

An arc must have a discovery, an investigation or complication, a consequential resolution and a
later response. At least two outcomes must be feasible; class, learned talent and earlier choices
can open alternatives. A player who misses an arc must still be able to complete the main story.
Faction progression unlocks relationships, services and build alternatives without making a single
faction compulsory for every class. The journal records what the character knows, where evidence
came from and why a branch is closed. A clear map helps the player orient without revealing every
secret before discovery.

The current four quests are deliberately small implementation proofs. They do not validate the
quality or length of these future multi-episode arcs.

### Endgame with multiple reasons to return

| Discipline | Why it can stay interesting | Protection against busywork |
| --- | --- | --- |
| Shattered Concord expeditions | Choose routes and respond to hunt, seal or purge contracts with different hazards | Earned loot survives a return to town; clear objectives and one-time rewards per run |
| World Reclamation | Restore regions with outcomes reflecting campaign choices | Persistent improvements rather than repeat repair chores |
| Living Reliquaries | Learn dungeon ecologies, secrets and route combinations | Shortcuts, legible exits and authored guardians |
| Oathbound Nemeses | Follow rivals with history, allies and different resolutions | Bounded, readable adaptations instead of counters that invalidate a build |
| Class Mastery Pilgrimages | Prove the distinctive skills of a chosen class | Solo viability and sidegrades rather than compulsory power gates |
| Boss Covenants | Select deliberate modifications to excellent authored encounters | Named target rewards and visible progress toward them |
| The Great Works | Build a rare item or transformation through varied activities | Preview recipes and prevent unlimited randomness from blocking completion |
| World Chronicles | Explore alternate histories in separate records | Preserve the main character's world and avoid mandatory seasonal resets |

The present expedition system is a foundation, not eight completed modes. Depth alone is not
unlimited content. Difficulty, enemy composition, authored objectives, encounter mechanics, rewards
and choices must develop together. Solo/offline play, pause and local save ownership remain primary.

## An interface with class identity

The design language is weathered dark metal, warm ivory type, restrained gold, deep life glass,
and a class-specific resource material. Ornamental framing establishes the fantasy; readable
numbers, icon shapes, text labels and predictable navigation carry the information.

Both clients consume the same Rust class profile and progression state. The existing `focus` save
field remains the actual numeric reservoir for compatibility; its display name, capacity, recovery
cadence, kill gain, color and material come from the class definition. A resource is not only a
recolored mana bar. The current foundation differentiates recovery and capacity, while more advanced
generators, combo rules, dual reservoirs and signature transformations remain class-combat work.

| Family | Visual behavior | Gameplay evidence the HUD must expose |
| --- | --- | --- |
| Fury / Wrath | Heat and embers within the reservoir | Pressure recovery, current amount and affordability |
| Mana / Arcane Power | Rotating arcane light and flowing glass | Larger reserves, recovery and spell cost |
| Conviction / Resolve | Radiant crest or sun geometry | Stable protective identity and combat recovery |
| Essence / spirit resources | Mist, leaves or soul light | Kill gain or pressure recovery and numeric capacity |
| Energy / Momentum / Flow | Quick sparks or wave motion | Fast cadence and current skill availability |
| Blood / Pact / Resonance | Blood glass, void or harmonic motion | Class-specific rhythm without obscuring exact values |

### Interaction requirements

- Life and resource indicators respond to authoritative changes immediately; interpolation affects
  appearance only. Numbers never wait for animation to finish.
- Learned active skills remain available. Cooldowns, insufficient resource, selection and locked
  prerequisites have different text or shape cues. Animation never blocks input.
- The main map shows the overworld first, then local details and discovered destinations. A local
  minimap shows actual geometry, the hero and points of interest.
- Inventory browsing includes the complete inventory, search, sorting, comparison and real equip
  actions. No viewport or quantity cap removes items from availability.
- Journal entries come from Rust quest state. Completed, available, return, active and locked states
  must remain distinct. The tracker advances when the player completes the current chapter.
- Panels preserve the preceding pause state, support keyboard closure and keep focus inside the
  modal. Reduced motion keeps numerical feedback and all controls fully functional.
- Saving, loading and importing expose real errors. Legacy saves are migrated by additive defaults;
  browser storage keys remain compatible and native fallback never overwrites the original file.

## Production acceptance gates

### Gate A: mechanically complete regional slice

Complete both story loops with every class, both Concord choices, all four side quests, every
expedition objective and every modifier. Prove save/reload during travel, combat and expedition
return, reward uniqueness, failed-command atomicity and old-save migration. Both clients must expose
and execute the same commands. Exercise the final browser and native packages, not only unit tests.

### Gate B: production-quality regional slice

Deliver original rigged characters and equipment, authored terrain and buildings, expressive
locomotion and attacks, enemy anticipation and reactions, real skill delivery, layered VFX, sound,
music, dialogue and one multi-phase boss. Review all creator combinations, resource states and
accessibility settings. Record frame-time, input and loading traces on Windows and Linux hardware.
Software rendering is useful functional evidence and cannot certify a gaming-GPU performance target.

### Gate C: content tools and authoring throughput

A designer can author and validate quests, dialogue conditions, encounters, items, consequences and
region transitions without changing client code. Validators detect missing references, unreachable
objectives, contradictory prerequisites, repeated rewards, unstable IDs and impossible save states.
Measure the effort to create and polish one distinct hour before extrapolating the full campaign.

### Gate D: campaign production and player evidence

Promote an act only after its complete critical path, optional branches, state migrations and
class-specific coverage pass. Record actual active-play distributions across unfamiliar players,
routes and difficulty. Review pacing and comprehension; cut repetitive internal work while preserving
substantive player content. A longer game is valuable only while its time remains worthwhile.

### Gate E: release quality

Run long-lived save migration, corruption recovery, controller/rebinding, localization, accessibility,
full inventory, worst-case effects and cross-vendor graphics checks. Publish the source revision,
artifact hashes, exact exercised workflows and remaining production scope together. Aspirational
hours and AAA/AAAA language cannot replace this evidence.

## Current technology decision

The existing Rust 1.100.0 / Bevy 0.20.1 baseline remains current for this change. Bevy's latest
release was rechecked against its [official release](https://github.com/bevyengine/bevy/releases/tag/v0.20.1).
Native gradients, retained UI entities and targeted interpolation fit the HUD without an additional
animation dependency; the [official gradient example](https://bevy.org/examples/ui-user-interface/gradients/)
provides the load-bearing API reference. [bevy_tweening](https://github.com/djeedai/bevy_tweening) remains
a candidate for authored multi-stage timelines when it provides a demonstrated advantage over this
small presentation layer. No new upstream game code or art was imported. Existing source-backed
research is preserved rather than repeated.
