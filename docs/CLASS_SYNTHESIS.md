# Original class synthesis

This is the design bridge from the source-backed atlases to original Dreadmarch gameplay. Source abilities are studied as mechanics; Dreadmarch uses its own names, balance, visual language, execution code and world. Every source edition retains its own coverage ledger. A historical rune is not silently presented as a current skill, and a data record is not called an implemented mechanic.

## The class roster stays broad

All 19 researched class identities remain selectable. Closely related traditions share engine primitives without losing their character. Sorcerer/Sorceress is one identity; the creator independently controls appearance. The current branches below match `data/gameplay/classes.json` exactly. They are original prototype structures, not quoted source-game trees.

| Class | Current original branches | Mechanical identity to preserve |
| --- | --- | --- |
| Warrior | Arms / Guard / Duelist | Weapon mastery, shield timing and field survival. Preserve the original Warrior's straightforward martial identity; do not turn every warrior into a rage-driven Barbarian. |
| Barbarian | Berserker / Warcries / Earthbreaker | Fury, weapon aggression, warcries and physical disruption. Spend or build momentum through committed attacks; armor and shouts support close combat without becoming a Paladin. |
| Paladin | Oaths / Consecration / Judgment | Oaths, auras, blessings, consecration and judgment. Auras modify the hero or chosen allies in measurable ways; melee commitment and protective timing remain central. |
| Crusader | Bulwark / Zeal / Wrath | Heavy shield play, flail-like martial delivery, siege-like commitment and radiant retaliation. Defense-to-offense conversion distinguishes the Crusader from the Paladin's sustained oath/aura identity. |
| Amazon | Spears / Javelins / Archery | Bow and spear/javelin disciplines, evasive battlecraft and storm-infused weapon skills. Weapon selection changes delivery and positioning, not only damage color. |
| Rogue | Marksman / Shadow / Venom | Precision archery, opportunistic melee, shadow movement and poison. Original Divine Divinity's Survivor skills support stealth, traps and exploration; Assassin remains the specialist in combos and prepared devices. |
| Assassin | Martial Arts / Traps / Shadow Arts | Martial sequences, traps and shadow techniques. Preparation, combo conversion and tactical entry/exit distinguish her from the Rogue's opportunistic precision. |
| Demon Hunter | Ballistics / Devices / Vengeance | Ranged arsenals, devices, sentries, evasive execution and vengeance. Prepared firing lanes and deployables differ from the Amazon's martial weapon disciplines. |
| Monk | Fists / Mantras / Spirit | Martial rhythm, mobility, spirit expenditure and supportive mantras. Preserve Hellfire's legacy identity while adapting later Monk combat expression through authored timing. |
| Sorcerer | Fire / Frost / Lightning | Elemental fire, frost and lightning, with readable element-specific status interactions. Sorceress is the same class identity and does not restrict the creator's body options. |
| Wizard | Arcana / Displacement / Illusion | Arcane manipulation, spatial/temporal control, illusions and magical convergence. Pure elemental mastery belongs primarily to Sorcerer; Wizard manipulates rules of delivery and space. |
| Necromancer | Bone / Blood / Undead | Bone, blood, curses, corpses and commanded dead. Corpses and minion roles should be explicit resources; poison alone must not erase the Witch Doctor distinction. |
| Witch Doctor | Hexcraft / Spirits / Fetishes | Hexes, spirits, fetishes and plague ecology. Spirit bargains, fear/confusion and indirect harm should behave differently from the Necromancer's corpses and undead army. |
| Druid | Storm / Wildshape / Companions | Nature magic, shapeshifting and animal companions. Transformations require real forms, movesets and animation; elemental spells and companion behavior must interact with form choice. |
| Spiritborn | Jaguar / Eagle / Gorilla / Centipede | Jaguar, Eagle, Gorilla and Centipede remain four visible spirit traditions. Aggression, mobility, protection and poison/sustain can combine through clearly stated spirit contracts. |
| Blood Knight | Blood / Shadow / Transfiguration | Blood siphoning, polearm aggression, night/shadow movement and a transformation state. Recovery is earned through exposed combat, with explicit costs and counterplay. |
| Tempest | Gales / Flow / Depths | Wind/water martial flow and mobile zephyr-like companions. Alternating delivery and movement creates a tempo identity distinct from a lightning Sorcerer or weather-focused Druid. |
| Warlock | Demonic / Eldritch / Chaos | Demon pacts, eldritch manipulation and chaotic power. Pact costs and demonic control differ from corpse-based Necromancy and the Witch Doctor's spirit/hex ecology. |
| Bard | Songs / Blades / Discord | Dual blades, identification/lore and original performance mechanics. Hellfire supplies the legacy dual-wield/Identify basis; the song systems are explicitly original extensions. |

## Combining abilities where it makes sense

1. **Inventory before synthesis.** Keep every sourced ability, rune/variant, passive and tree position in its versioned research catalogue. Missing evidence gets a named gap, not a fabricated connection. Duplicate names across games remain separate source records.
2. **Separate delivery from effect.** A projectile, melee sweep, ground area, chain, beam, persistent summon and self transformation have different targeting and timing contracts. Reusing damage/status primitives must not make them the same action.
3. **Merge genuine mechanical relatives.** Closely related elemental bolts can share an authored base with meaningful delivery variants. A persistent aura, a timed shield, an invulnerability window and healing remain different mechanics despite a shared defensive role.
4. **Preserve class signatures.** Resource generation/spending, weapon requirements, transformations, combo rules, pet commands and passive interactions define class identity. These cannot be replaced by a universal damage bonus.
5. **Use compatible modifiers.** A projectile modifier requires a projectile delivery contract. A summon modifier requires an owned persistent entity. Effect tags and explicit validation reject incompatible combinations before play.
6. **Expose tradeoffs.** A stronger variant can alter cost, recovery, targeting, risk or setup. Progression should create new decisions, not only larger numbers.
7. **Prove behavioral distinction.** Combat fixtures must demonstrate differences in target selection, timing, resource flow, status, movement or persistent world state. Visual color changes alone do not count.

## Original Divine Divinity: freedom outside the class tree

The original game allows cross-Way learning. Dreadmarch should preserve that sense of an adventurer who can learn broadly without making every combat class identical. Add a shared adventuring layer for identification, lore, repair/crafting, trading, locks, traps, stealth and environmental interaction. Its costs, quest rewards and alternative routes must be authored and tested alongside combat. See the Divinity atlas for the exact original 88 ordinary + 8 Divine + 3 innate breakdown.

The late Divine path suggests a major story-earned progression layer. An original Dreadmarch ascension can change traversal, perception, spirit interaction or high-impact combat after a consequential campaign event. It should not be an unexplained second passive grid that appears before the story earns it.

Later Divinity contributes explicit surfaces, status interactions, crafted hybrid skills and conditional dialogue. Introduce a small, internally complete interaction set in the first production region, then expand it through content validation. Do not copy turn-based encounter pacing directly into real-time action.

## Lost Ark and Path of Exile: expression and composition

Lost Ark informs class resource loops, skill timing, branching action variants and readable bosses. Adapt those ideas into solo-accessible progression with no required daily attendance, raid group or service economy. The exact Western class roster and edition boundaries are in the systems atlas.

Path of Exile informs typed supports, passive specialization, item interactions and meaningful experimentation. Its full passive graph is research, not a template to transplant wholesale. A manageable original graph must explain prerequisites, conversion order, trigger conditions and stacking rules. Infinite feedback loops require explicit semantics and termination analysis; that is distinct from silently capping a legitimate population or dropping effects.

## Current executable coverage

The playable catalogue has **174 original nodes**: 18 classes with three three-node branches and Spiritborn with four. Each branch currently has an entry active, a damage mastery passive and a later active. The engine implements 11 effect families: strike, bolt, area nova, chain-labelled area hit, frost slow, poison over time, heal, guard, dash, summon and life drain.

**Prototype boundaries:** the current bolt/chain presentations and several class actions reuse area-hit execution; there are no authored projectile flights, ordered chain hops, class-specific resource engines or transformations yet. Summons use generic allied silhouettes, and mastery passives currently add damage. The production goals above require replacing these simplified behaviors. This is why the prototype nodes are explicitly tagged original adaptations, and why the source-game catalogue is not counted as implemented features.

The shared Rust implementation already owns prerequisite checks, learning costs, cooldown/focus validation, timed slow/poison effects, healing/guard, summons, progression, inventory, equipment, quest rewards, persistent regions and save validation. Both clients consume the same commands and snapshots.

## Engine contracts for the production ability layer

| Contract | Data / behavior | Acceptance evidence |
| --- | --- | --- |
| Identity | Stable ID, class/weapon/form requirements, tags, localized display key, provenance | Duplicate/reference checks and versioned migrations |
| Targeting | Self, entity, ground, direction, cone, chain or area; reach and line of sight | Invalid targets rejected atomically; targeting matches telegraph |
| Cost | Class resource, charge, health, corpse, item or conditional reservation | Rejection leaves state unchanged; resource traces match rules |
| Timeline | Windup, commit, hit windows, recovery, interrupt and cancel rules | Input-to-action traces and animation event alignment |
| Delivery | Projectile, sweep, beam, area, trap, summon or transformation | Actual distinct motion/selection/lifetime behavior |
| Effects | Typed damage/heal/status/movement/surface/resource operations | Ordering, stacking, immunity and dispel fixtures |
| Modifiers | Validated support/rune-like transformations and item hooks | Incompatible combinations rejected; legitimate combinations preserved |
| Persistence | Learned state, cooldowns, charges, persistent entities, migrations | Exact save/resume future-state equality |
| Presentation | Animation/VFX/audio/camera/accessibility references | Authored capture and readability review, separate from simulation proof |

## Next implementation boundary

Build the production ability timeline and one complete class combat loop in the authored vertical-slice region, while preserving all 19 creator choices and catalogue entries. Complete the same contracts for every class through explicit coverage tasks. The first polished class is an implementation order, not a reduction of the full roster or research scope.
