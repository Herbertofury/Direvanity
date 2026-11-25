# ARPG systems research: Lost Ark and Path of Exile

Research snapshot: **25 November 2026**. Design status: **research and original design proposals**, with implementation tracked separately in the game and engine. Diablo, Diablo II, Diablo III, Diablo IV, Diablo Immortal, Lost Ark, Divine Divinity, the wider Divinity series, Path of Exile and Path of Exile II are major inspirations for Dreadmarch. The specific contribution of this document is Lost Ark and Path of Exile; the companion Diablo and Divinity research supplies their class and ability histories.

The intended experience is a substantial, personal, single-player action RPG: create a character, choose a distinct class, explore a persistent world, resolve quests, discover equipment and develop a build through a complete campaign. Lost Ark contributes expressive action and class-specific mechanics. Path of Exile contributes composable abilities, consequential specialization and equipment that changes a build. These ideas support an original game with its own setting, abilities, encounter design, art and code.

## 1. What is actually catalogued

The reproducible inventory is [arpg-comparison.json](../../data/research/arpg-comparison.json). Its source receipts contain the exact public URL, revision, byte count and SHA-256 of each input. [arpg_catalog.py](../../scripts/research/arpg_catalog.py) examines complete pinned tables. [arpg_design.py](../../scripts/research/arpg_design.py) contains the authored interpretation, source register and class mappings.

This is a complete extraction **of the stated input tables**. It is not a claim that every entry is currently learnable, that every regional release has identical content, or that all these abilities are implemented in Dreadmarch. Display names, stable identifiers, tags, effect categories and passive-tree connections are research facts. A name inventory is not a complete behavioral specification.

| Inventory | Expected in pinned input | Included | Excluded or unresolved |
|---|---:|---:|---|
| Lost Ark advanced class identities | 30 observed across current data and release evidence | 30 | Exact regional live parity remains subject to release verification |
| Lost Ark raw skill rows | 25,496 | 2,631 nonzero-class rows: 1,199 named records and 1,432 unnamed IDs | 22,865 class-zero rows counted separately; these include NPC, generic and item effects |
| Lost Ark tripod groups | 636 | 636 | 0 omitted groups |
| Lost Ark tripod choices | 4,966 | 4,966 | UI tier/prerequisite interpretation is not inferred from internal effect fields |
| Lost Ark Ark Passive records | 704 | 704, containing 2,657 level rows | Complete class/tier/prerequisite graph absent from this source |
| Lost Ark Ark Grid core variants | 2,232 | 2,232 | Grade variants are not counted as distinct class mechanics |
| Lost Ark Ark Grid core options / gem types / bases | 3,906 / 26 / 6 | All | Processing and fusion economy tables are counted but not imported |
| PoE1 generated gem table | 836 entries | 836: 561 active or granted entries, 275 supports | Live availability of legacy or variant entries is not established by inclusion |
| PoE1 official 3.29.1 passive export | 3,390 nodes | 3,390 | Includes the root and structural nodes; tooltip prose and numerical modifiers excluded |
| PoE1 official 3.30.0 preview | 3,462 nodes | 3,462, in a separate preview object | Never substituted for the stable baseline |
| PoE2 generated gem table | 967 source entries | 967: 400 active or granted entries, 567 supports | 966 unique source keys; one upstream duplicate retained and identified |
| PoE2 official 0.5.5 passive export | 5,153 nodes | 5,153 | Includes 241 unnamed/structural nodes and placeholder class slots |

Named skill rows are **not** unique player skill buttons. Basic attacks, identity actions, removal actions, transformed attacks, legacy rows and auxiliary events can have separate IDs or repeat a display name. The catalogue retains their identities so later reconciliation cannot silently collapse them. All tripod groups are retained independently, even when a group cannot be joined to a named player-class row.

The PoE2 source defines `Metadata/Items/Gems/SkillGemSkeletalSniper` twice. Both source entries are preserved with ordinals; the final occurrence is marked as the effective Lua-table entry. Reporting 967 distinct gem IDs would be incorrect. The published tree also contains twelve class slots, but empty class slots and unnamed ascendancies are not evidence that those classes are released.

### Version boundaries

| Source | Pinned revision | Interpretation |
|---|---|---|
| `snoww/loa-logs` | `2f89591450db347b2cf802fc0f7828768e769720` | Community-maintained extracted Lost Ark tables, observed 25 November 2026 |
| Path of Building | `5606cc64fa9f15694e69001a52b90aa968d01a86` | Generated gem inventory associated with release 2.69.0 |
| Path of Building PoE2 | `601768bd105869618c3d6581b883e5d7aeec91b2` | Generated PoE2 gem inventory at a fixed revision |
| GGG PoE1 tree | `8bd138b32ea2631455cac5935bfab089f826094f` | Official published 3.29.1 export |
| GGG PoE1 preview | `550fb34ae7fca9ec4d760a2ac845bb2ba1032ec3` | Official 3.30.0 **preview** |
| GGG PoE2 tree | `bd87e6512c92b868542eddfb1ba4ea8b6dc2da36` | Official published 0.5.5 export |

The latest Lost Ark release page checked was [Echoes of Thunder, 18 November 2026](https://www.playlostark.com/en-us/game/releases/echoes-of-thunder). [Guardianknight arrived on 4 February 2026](https://www.playlostark.com/en-us/game/releases/guardians-rage); [Dimensionalist arrived with Dimensions Unbound on 16 September 2026](https://www.playlostark.com/en-us/game/releases/dimensions-unbound). A roster copied from an older guide would miss them.

Three source labels required reconciliation: class ID `604` is called `Alchemist` in the small `PCData` lookup but corresponds to **Wildsoul**; `612` is **Dimensionalist**; `702` is **Guardianknight**. The latter two are missing from that lookup. Release evidence and the associated skill identities support these overrides. The extractor records the overrides explicitly.

## 2. Lost Ark: the full current reference roster

This table describes reusable mechanical patterns. The three skill labels per row are **illustrative examples**, not an exhaustive substitute for the complete named inventory in JSON. Historical official Academy articles explain the original class concepts; they must not be used as current numerical balance specifications. The table's named-row count comes from the pinned data and includes auxiliary records.

| Class / family | Named rows | Mechanical pattern | Example skill names |
|---|---:|---|---|
| [Berserker](https://www.playlostark.com/en-us/news/articles/warrior-advanced-classes) / Warrior | 48 | Build fury and commit to a greatsword burst state. | Whirlwind; Bloody Rush; Overdrive |
| [Destroyer](https://www.playlostark.com/en-gb/news/articles/lost-ark-academy-destroyer-class) / Warrior | 32 | Generate gravity cores, spend them on heavy releases, then enter a gravity state. | Perfect Swing; Seismic Hammer; Vortex Gravity |
| [Gunlancer](https://www.playlostark.com/en-us/news/articles/warrior-advanced-classes) / Warrior | 37 | Manage a defensive stance, shield counters and deliberate lance attacks. | Counter Gunlance; Surge Cannon; Charged Stinger |
| [Paladin](https://www.playlostark.com/en-us/news/articles/warrior-advanced-classes) / Warrior | 37 | Combine sword techniques and holy magic with an identity-state decision. | Holy Sword; Holy Aura; Sacred Executioner |
| [Slayer](https://www.playlostark.com/en-us/news/articles/lost-ark-academy-slayer) / Warrior | 34 | Build fury into an aggressive greatsword burst, with alternative sustain and finisher paths. | Bloodlust; Brutal Impact; Guillotine |
| [Valkyrie](https://www.playlostark.com/en-gb/game/releases/rise-of-the-valkyrie) / Warrior | 37 | Channel light through sword combat, with offensive and protective specialization. | Seraphic Oath; Shining Knight; Judgment Stigmata |
| [Arcanist](https://www.playlostark.com/en-us/news/articles/arcanist-academy) / Mage | 52 | Apply stacks, consume them with ruin attacks and react to card effects. | Celestial Rain; Secret Garden; Return |
| [Summoner](https://www.playlostark.com/en-us/news/articles/summoner-academy) / Mage | 57 | Build ancient energy through attacks and companions to command larger elemental summons. | Ancient Spear; Maririn; Akir |
| [Bard](https://www.playlostark.com/en-us/news/articles/mage-advanced-classes) / Mage | 43 | Use musical attacks and protection, then choose an offensive or restorative serenade. | Serenade of Courage; Serenade of Salvation; Soundholic |
| [Sorceress](https://github.com/snoww/loa-logs) / Mage | 34 | Combine elemental casting with an arcane release or mobility decision. | Doomsday; Arcane Rupture; Blink |
| [Wardancer](https://www.playlostark.com/en-us/news/articles/martial-artist-advanced-classes) / Martial Artist | 32 | Turn fast martial sequences into elemental finishers. | Wind's Whisper; Sweeping Kick; Esoteric Skill: Blast Formation |
| [Scrapper](https://www.playlostark.com/en-us/news/articles/martial-artist-advanced-classes) / Martial Artist | 34 | Alternate two interdependent energy pools for heavy gauntlet attacks. | Death Rattle; Supernova; Iron Cannon Blow |
| [Soulfist](https://www.playlostark.com/en-us/news/articles/martial-artist-advanced-classes) / Martial Artist | 39 | Weave ranged energy with martial attacks and staged power escalation. | Hype; World Decimation; Energy Blast |
| [Glaivier](https://www.playlostark.com/en-gb/news/articles/lost-ark-academy-glaivier) / Martial Artist | 39 | Build momentum in one spear stance and spend it by switching to another. | Red Dragon's Horn; Raging Dragon Slash; Dragonscale Defense |
| [Striker](https://www.playlostark.com/en-us/news/articles/martial-artist-advanced-classes) / Martial Artist | 33 | Build elemental power through mobile combos for committed esoteric attacks. | Lightning Whisper; Esoteric Skill: Lightning Tiger Strike; Esoteric Skill: Tiger Emerges |
| [Breaker](https://www.playlostark.com/en-us/news/articles/breaker-breakdown) / Martial Artist | 37 | Alternate stamina and shock while filling a separate stance resource. | Brawl King Stance; Asura Destruction; Celestial Force Barrage |
| [Deathblade](https://www.playlostark.com/en-us/news/articles/assassin-advanced-classes) / Assassin | 39 | Build a sword-combo identity state and release a decisive surge. | Deathblade Surge; Soul Absorber; Blitz Rush |
| [Shadowhunter](https://www.playlostark.com/en-us/news/articles/assassin-advanced-classes) / Assassin | 46 | Build toward demonic transformation with a distinct transformed action set. | Demonize; Demon Vision; Blood Massacre |
| [Reaper](https://www.playlostark.com/en-us/news/articles/reaper-academy) / Assassin | 38 | Choose stealth-driven persona attacks or a sustained chaos sequence. | Persona; Nightmare; Dance of Fury |
| [Souleater](https://www.playlostark.com/en-us/news/articles/souleater-academy) / Assassin | 31 | Coordinate soul stones, possession and scythe/deathlord attacks. | Soul Drain; Guillotine Swing; Deathlord Mode |
| [Sharpshooter](https://www.playlostark.com/en-us/news/articles/gunner-advanced-classes) / Gunner | 39 | Combine deliberate bow attacks with a summoned hawk. | Snipe; Charged Shot; Summon Silverhawk |
| [Deadeye](https://www.playlostark.com/en-us/news/articles/gunner-advanced-classes) / Gunner | 44 | Switch weapon sets to manage distance, timing and burst opportunities. | Shotgun Dominator; Perfect Shot; Desperado |
| [Artillerist](https://www.playlostark.com/en-us/news/articles/gunner-advanced-classes) / Gunner | 40 | Trade mobility for heavy artillery and a committed barrage state. | Homing Barrage; Barrage: Energy Cannon; Summon Turret |
| [Machinist](https://www.playlostark.com/en-us/news/articles/lost-ark-academy-machinist) / Gunner | 51 | Coordinate independent drone commands and personal attacks or spend energy in a powered suit. | Command: Raid Missile; Energy Buster; Hypersync |
| [Gunslinger](https://www.playlostark.com/en-us/news/articles/gunner-advanced-classes) / Gunner | 39 | Switch firearm sets while maintaining mobile positional combat. | Focused Shot; Target Down; Bullet Rain |
| [Artist](https://www.playlostark.com/en-us/news/articles/lost-ark-academy-artist) / Specialist | 39 | Build harmony through brush attacks and spend it on damage or restoration. | Moonfall; Sunrise; Paint: Illusion Door |
| [Aeromancer](https://www.playlostark.com/en-us/news/articles/lost-ark-academy-aeromancer) / Specialist | 33 | Blend weapon flourishes and weather spells with a sustained rain state. | Sun Shower; Piercing Wind; Rainstorm |
| [Wildsoul](https://www.playlostark.com/en-us/game/releases/call-of-the-wildsoul) / Specialist | 47 | Alternate bear and fox forms while invoking phantom beasts. | Boulder Bear; Fox Flame; Phantom Beast Awakening |
| [Dimensionalist](https://www.playlostark.com/en-us/game/releases/dimensions-unbound) / Specialist | 35 | Use spatial displacement, timing and dimensional control. | Temporal Crush; Spatial Manipulation; Timeline Divergence |
| [Guardianknight](https://www.playlostark.com/en-us/game/releases/guardians-rage) / Guardianknight | 38 | Use a halberd and guardian orb to build into an empowered incarnation. | Invoke Incarnation; Guardian's Roar; Embereth's Breath |

## 3. Lost Ark progression systems worth adapting

### Identity first, then skill customization

A class identity should define an action loop the player can feel immediately: alternate energies, build and spend a resource, choose when to transform, control a companion, or switch stance. It should affect targeting, positioning and timing. Giving every class a different-colored mana bar would lose the strongest part of the reference.

Lost Ark's [Tripod system](https://www.playlostark.com/en-gb/game/about) adds three tiers of choices to skills. The useful principle is that a familiar action can acquire different tactical roles. Dreadmarch should author modifiers that change delivery, timing, movement, defense, status interactions or summons. A wider projectile fan, a delayed detonation and a defensive conversion are meaningfully different choices. Small percentage bonuses belong mainly in passive ranks and equipment, so the skill screen does not bury the important decisions.

The engine should express an ability as typed actions: **targeting → cost → windup → delivery → effect → recovery**. Delivery can produce a projectile, melee sweep, beam, ground area or summon. Effects operate on damage, status, movement, surfaces, resources and commands. Tags and explicit event contracts determine whether a modifier is compatible. A projectile-only modification must not quietly attach to a self-ward. Trigger chains require depth and repetition limits to prevent infinite proc loops.

The upstream `SkillWindows` table contains 1,052 skill timing records and 1,251 stages; it is combat timing data, not a player skill-list screen. Its useful lesson is to treat windup, impact and recovery as authored gameplay parameters. Dreadmarch still needs its own animations, readable impact moments, cancel rules and audiovisual feedback.

### Ark Passive and Ark Grid are separate layers

| Reference layer | Meaning supported by official sources | Dreadmarch interpretation |
|---|---|---|
| **Evolution** | Shared progression and set-style combat specialization | A compact common foundation for offense, defense, resource management and movement |
| **Enlightenment** | Class-specific specialization | Three distinct class disciplines with identity-changing choices |
| **Leap** | Hyper Awakening and Hyper Awakening Skill development | A late-campaign mastery branch attached to earned signature abilities |
| **Ark Grid: Order** | Class-specific cores | Optional class relics that alter a defined action loop |
| **Ark Grid: Chaos** | Universal cores | Cross-class relics with explicit compatibility and opportunity costs |
| **Astrogems / core thresholds** | Gem allocation activates core effects at thresholds | A small, legible relic configuration puzzle unlocked through exploration |

The [Arkesia Ignited release](https://www.playlostark.com/en-us/game/releases/arkesia-ignited) documents the three Ark Passive categories. [Call of the Wildsoul](https://www.playlostark.com/en-us/game/releases/call-of-the-wildsoul) subsequently changed Ark Passive progression. [Day of Prophecy](https://www.playlostark.com/en-us/game/releases/day-of-prophecy) and the [2025 roadmap](https://www.playlostark.com/en-us/news/articles/2025-roadmap-part-4) document Ark Grid. These are successive layers, not alternative names for the same tree. Historical item-level unlock requirements must not be presented as the current rules.

For Dreadmarch, the progression sequence should be quest and mastery driven: learn a signature technique from a class quest, unlock its modification through a meaningful encounter, and discover a relic through exploration. There should be no daily/weekly lockout, required multiplayer raid, auction dependency, paid shortcut or account-wide grind needed to finish a class. Each class must provide its own offense, survival, traversal and boss tools. Optional companions can enrich encounters without becoming prerequisites.

Awakening and Hyper Awakening suggest memorable class payoffs with authored presentation. They should remain earned abilities whose targeting and timing matter, with a clear purpose in both normal combat and boss fights. They must not simply erase encounter mechanics after a long cooldown.

## 4. Path of Exile: composability with legible constraints

The official [PoE1 overview](https://www.pathofexile.com/game) describes itemized active skills, modifying supports, a shared passive tree with class-specific starting positions, keystones and equipment-driven builds. The current [PoE2 overview](https://pathofexile2.com/) likewise describes active gems modified by as many as five support gems. The Dreadmarch lesson is **composable behavior with visible tradeoffs**, rather than copying the shape or sheer size of another game's tree.

PoE2 support rules have changed during development. The official [Third Edict announcement](https://pathofexile2.com/es/edict) describes support tiers and allowing repeated copies on a character. A blanket restriction copied from an early-access launch guide would be stale. Dreadmarch should specify its own compatibility and duplicate rules explicitly and test them; it should never inherit them accidentally from a planner's current UI.

### Data availability and exactly what the index preserves

| Domain | Best retrieved source | Available facts | Boundary |
|---|---|---|---|
| PoE1 active and support gems | [Path of Building generated data](https://github.com/PathOfBuildingCommunity/PathOfBuilding) | All source entries, names, IDs, tags, variant/effect identifiers and available weapon requirements | Community-generated game data; not an official GGG gem API |
| PoE1 passives and ascendancies | [GGG official export](https://github.com/grindinggear/skilltree-export) | All node names/IDs and outgoing edges; node roles; class starts; ascendancy names; mastery effect IDs; alternate ascendancy identities | Numerical effects, art and tooltip prose excluded |
| PoE2 active and support gems | [Path of Building PoE2 generated data](https://github.com/PathOfBuildingCommunity/PathOfBuilding-PoE2) | All source entries with explicit duplicate handling; active/support classification and tags | Includes variants and potentially unused entries; not a released-content roster |
| PoE2 passives and ascendancies | [GGG official export](https://github.com/grindinggear/poe2-skilltree-export) | All node names/IDs and outgoing edges; role and constraint fields; class/ascendancy override pairs; 80 named override definitions | Export slots can be placeholders; an override is not an additional ordinary allocated node |

Every extracted outgoing edge resolves to an included node. The PoE1 stable graph contains **3,382 outgoing edges**; PoE2 contains **6,076**. These counts include structural connections. The catalogue keeps the graph as data, so design work can compare branching, identity gates and opportunity costs without manually transcribing screenshots.

The PoE1 export contains seven class records and **21 named ordinary ascendancies**, plus **16 alternate ascendancy records**. The PoE2 export contains **23 named ascendancies** across its populated class slots. These are export facts, not a promise that every exported class or historical alternate system is currently playable. The source has retained IDs whose display labels have changed, so the catalogue preserves both.

For Dreadmarch, each class should have a strong, bounded core tree. A smaller shared training layer can provide utility and controlled cross-class experimentation inspired by Divinity. Equipment and relic sockets can modify typed skills. Masteries can add a second-stage class commitment earned through the campaign. A player should understand a first build without an external planner, while a dedicated player can find substantial interactions among resource costs, delivery, ailments, summons and item effects.

A support or relic should be accepted only if its tags and requirements match the underlying skill. Its cost must be visible: reduced coverage, greater commitment, a resource conversion, a cooldown change, lost utility or a genuinely exclusive alternative. A large pool of unconditional damage multipliers would increase numerical complexity without producing proportionate build variety.

## 5. Mapping into every Diablo-derived identity

These mappings are **original design proposals**. They connect mechanical ideas, not narrative characters or imported skill implementations. The Diablo reference catalogue establishes the nineteen identities. Sorceress is treated as a naming alias of Sorcerer; Wizard remains distinct. Original Hellfire's hidden Bard remains represented in the roster ambition.

| Dreadmarch class | Lost Ark references | PoE references | Proposed disciplines | Identity to protect |
|---|---|---|---|---|
| Warrior | Gunlancer, Destroyer | Warrior, Juggernaut | Arms / Bulwark / Tactician | Guard, stance and weapon mastery; reliable physical fundamentals without a mandatory rage state. |
| Rogue | Sharpshooter, Deadeye, Reaper | Ranger, Deadeye, Trickster | Marksman / Skirmisher / Wayfinder | Precision, movement and fieldcraft; bow and agile weapon play remain viable throughout the campaign. |
| Sorcerer | Sorceress | Elementalist, Stormweaver | Flame / Frost / Storm | Innate elemental power, elemental reactions and managed overcharge; Sorceress is an alias. |
| Monk | Wardancer, Scrapper, Soulfist, Striker, Breaker | Monk, Invoker | Open Hand / Flowing Staff / Inner Flame | Rhythmic martial sequences, spirit and deliberate finishers with readable combo windows. |
| Bard | Bard, Artist | Guardian, Hierophant | War Cantor / Dirges / Harmonics | Self-sufficient offensive phrases, wards and rhythm; healing never requires another player. |
| Barbarian | Berserker, Slayer, Destroyer | Berserker, Slayer, Chieftain | Ravager / Berserker / Ancestor | Rage, heavy weapon commitment and forceful disruption; distinct from Warrior's stable guard play. |
| Amazon | Glaivier, Sharpshooter | Huntress, Amazon, Deadeye | Spear Dancer / Storm Archer / Huntress | Spear and bow expertise with stance or range transitions and a strong martial identity. |
| Assassin | Deathblade, Reaper | Assassin, Saboteur, Trickster | Shadowblade / Venom / Saboteur | Preparation, stealth, traps and payoff sequences; distinct from Rogue's sustained fieldcraft. |
| Necromancer | Summoner, Souleater | Necromancer, Witch, Lich | Bone / Grave Host / Blood Rite | Corpses, bones and commanded undead; minion decisions matter beyond passive damage. |
| Paladin | Paladin, Valkyrie | Guardian, Inquisitor, Templar | Oathkeeper / Zealot / Judicator | Oaths, auras and righteous weapon strikes; every protective path has a solo damage loop. |
| Druid | Wildsoul, Aeromancer, Summoner | Druid, Oracle, Shaman | Wildshape / Stormcaller / Grovekeeper | Persistent animal-form play, weather and living nature with meaningful form-specific actions. |
| Crusader | Gunlancer, Paladin, Valkyrie | Juggernaut, Guardian, Templar | Iron Wall / Siegebreaker / Consecrator | Heavy shield counters, armored advance and battlefield control; distinct from Paladin's oath emphasis. |
| Demon Hunter | Gunslinger, Deadeye, Machinist, Artillerist | Mercenary, Witchhunter, Ranger | Crossbow / Trapper / Night Stalker | Ranged pursuit, traps and constructed sentries; translate reference firearms into setting-appropriate weapons. |
| Witch Doctor | Summoner, Wildsoul, Arcanist | Occultist, Pathfinder, Hierophant | Hexes / Spirit Beasts / Plague | Spirits, ritual objects, poison and curses; distinct from Necromancer's corpse-and-bone command. |
| Wizard | Arcanist, Dimensionalist, Sorceress | Chronomancer, Hierophant | Arcane / Chronomancy / Conjuration | Studied arcane geometry, time, space and constructs; distinct from Sorcerer's innate elemental core. |
| Blood Knight | Shadowhunter, Souleater | Blood Mage, Lich | Blood Lance / Sanguine Veil / Night Form | Life-cost conversion, drain, reach and controlled monstrous transformation. |
| Tempest | Aeromancer, Glaivier, Soulfist | Invoker, Stormweaver | Gale / Tide / Flow | Water and wind martial motion, afterimages and flowing attacks; no animal-form dependency. |
| Spiritborn | Wildsoul, Guardianknight, Soulfist | Monk, Druid, Shaman | Jaguar / Eagle / Gorilla / Centipede | Invoke guardian aspects through martial actions; maintain a distinct identity from full Druid shapeshifting. |
| Warlock | Shadowhunter, Summoner, Souleater | Infernalist, Occultist, Warlock of the Mists | Pacts / Demonology / Malediction | Bind demons, bargain for power and exploit curse interactions through explicit pact costs. |

The class distinction must survive equipment and shared training. A universal relic can grant a defensive dash or change a projectile's behavior; it should not hand every class the defining resource, form and full summon roster of another class. Paladin emphasizes vows and empowering presence, while Crusader emphasizes shields and armored control. Sorcerer emphasizes elemental force, while Wizard emphasizes studied arcane manipulation. Witch Doctor emphasizes spirit ritual, while Necromancer emphasizes corpses and commanded undead. These differences should be visible in animation, sound, encounter solutions, resource decisions and progression.

Lost Ark's technologically advanced gunner ideas need a setting translation. Weapon-set switching can become bow/crossbow modes, drone command can become a constructed familiar or sentry, and artillery commitment can become a heavy siege device. The interaction pattern is useful even when the original weapon fiction does not belong in Dreadmarch.

## 6. A massive single-player game needs a content architecture

The following is a **proposed full-production planning floor**, not delivered content, a release date or a measured playtime claim. It makes the ambition finite enough to budget and verify. The project can raise these targets once measured production throughput and player testing support doing so.

| Full-production planning target | Quantity | Definition of meaningful completion |
|---|---:|---|
| Distinct class identities | 19 | Every class can finish the campaign alone, has its own resource/identity loop, and presents a clearly different experience |
| Disciplines | 3 per class, 4 for Spiritborn; 58 total | Different decisions and encounter solutions, not cosmetic skill recolors |
| Active abilities | 24 per class; 456 total | Each action has implemented targeting, costs, delivery, feedback, AI interaction and progression |
| Behavioral modifiers | 3 per active; 1,368 total | Authored alternatives with explicit compatibility, meaningful opportunity cost and clear UI |
| Class passive nodes | 48 per class; 912 total | A readable graph with no dead-end prerequisites or mandatory point traps |
| Signature ultimates | 2 per class; 38 total | Distinct encounter roles and authored presentation |
| Campaign acts / major regions | 6 / 12 | Authored progression, distinct environments, towns, secrets and lasting quest consequences |
| Authored dungeons / major bosses | 48 / 36 | Different spatial and combat demands, with solo solutions for every class |
| Side quest lines | 60 | Complete narrative arcs, including meaningful world-state changes and rewards |

A playable content unit should include more than combat geometry. A region needs navigation, landmarks, inhabitants, ambient storytelling, secrets, quest state, encounter populations, reward tables, travel rules and recovery behavior. A dungeon needs authored pacing, enemy combinations, resource pressure, a climax and persistent completion state. A boss needs readable attacks, clear recovery opportunities, counters that do not require a particular class, and suitable behavior for ranged characters and summoners.

The world model should persist stable IDs for regions, NPCs, quest stages, doors, containers, encounters and discoveries. Quest changes should be event driven and survive save/load. Rewards should be issued once unless an encounter is explicitly repeatable. Dialogue conditions and faction reactions must read the same authoritative state as the quest journal. A central content manifest should validate missing references, unreachable objectives, duplicate identifiers and incompatible reward definitions before release.

Loot should support a complete solo economy: predictable access to foundational build pieces, random discoveries, targetable sources, crafting with legible outcomes, vendors that remain relevant and salvage that respects player intent. Build-defining item drops should not depend on another player's trade listing. A unique item earns its place by changing an action loop or solving a distinctive problem; a different name and a slightly larger number are insufficient.

Character creation should separate **class, body, face, hair, skin, voice, identity presentation and background**. Background may change dialogue, starting context or optional quest opportunities without permanently invalidating a class build. Presets and randomization should remain editable. The creator must save the selected configuration, reproduce it after loading, allow the player to examine the result under representative lighting, and clearly identify which appearance controls are implemented in each milestone. A class-select button is not proof of a complete class kit.

### Production gates

1. **Authoritative foundation:** character creation persists; class identity is stored; movement, attacks, progression, quests, loot and saves use the same simulation rules in every client.
2. **Complete vertical slice:** one region, a settlement, a dungeon, an authored boss, a quest with a lasting consequence, and three contrasting classes with deeply implemented kits. The nineteen-class roster remains the full-production target.
3. **Class production gate:** every class has offense, survival, crowd interaction, traversal and boss tools; all disciplines complete representative solo encounters; animation, audio and VFX communicate the actual combat timing.
4. **Campaign production gate:** the complete main quest is playable from a new save to an ending; all required travel and quest paths are valid; no campaign completion depends on a party or a timed live event.
5. **Release quality gate:** measured performance on declared hardware, stable saves and migrations, accessibility and control remapping, readable combat, bounded loading, complete localization coverage, and reliable recovery from interrupted sessions.

The term “AAAA” is an ambition label. The acceptance criteria are observable: input feels immediate, impacts are convincing, enemies react coherently, traversal and camera are dependable, UI explains the rules, the world rewards curiosity, builds differ in practice, and the full game remains stable. These properties require authored content, iteration and measurement in addition to a modern Rust technology stack.

## 7. Reproduction, validation and remaining gaps

From the repository root:

```sh
python scripts/research/arpg_catalog.py --fetch
```

The default cache is `dist/research-cache/arpg`; raw input is kept out of the committed research index. To reuse an existing cache, pass `--cache /absolute/path`. The script downloads only missing pinned files, then writes the complete generated index. Its receipts make subsequent source comparisons explicit. Updating to a new game release should change the pinned revision, regenerate the catalogue, review added/removed/renamed identities and preserve a changelog of relevant design changes.

The extraction checks row accounting, parses every top-level generated gem entry, retains duplicate source keys explicitly, and verifies closure of every passive-tree outgoing edge. The review also reconciles every named Lost Ark advanced class against the thirty-row identity table and verifies the nineteen-class Dreadmarch mapping. A source update must not silently replace a stable baseline with a preview.

The most material remaining research gap is the **complete player-facing Lost Ark Ark Passive graph**. The extracted table supplies all 704 records and their level labels, but does not provide reliable class/tier/prerequisite relationships. The [official Lost Ark API](https://developer-lostark.game.onstove.com/changelog) documents character skill and Ark Passive endpoints, with service and authorization requirements; it is not an anonymous full western-game catalogue. A supplemental community index did not supply a complete verified graph during this pass. No edges or class assignments have been invented to fill that gap.

Further behavioral design needs a versioned decision for each planned Dreadmarch skill: its gameplay purpose, reference mechanic, original implementation, required engine primitives, resource/cancel rules, animation needs, progression interactions and verification encounter. The current inventory deliberately does not reproduce long tooltips or numerical damage formulae. The complete name/graph inventory must not be reported as completed behavioral implementation.

The PoE character-tree index does not enumerate every possible item-generated cluster configuration or the separate Atlas progression graphs. Those are separate domains, discoverable through the official [PoE1 Atlas tree](https://www.pathofexile.com/atlas-skill-tree) and PoE2's official endgame material. They are relevant to an optional endgame system, rather than prerequisites for the initial class and campaign implementation.

The research sources are not interchangeable with engine foundations. GGG's published tree exports are official game data; Path of Building and `loa-logs` are community projects, with their own attribution and licensing context. This repository records factual research and implements original systems. It does not import those games' art, audio, source engines or proprietary campaign content.

## 8. Source register

The machine-readable register in `design_synthesis.sources` distinguishes official releases, official exports and community extracted data. Historical sources establish a mechanic's provenance and are explicitly dated; current unlock thresholds and balance must come from the selected release.

| ID | Source | Provenance / version scope |
|---|---|---|
| `la-current` | [Echoes of Thunder](https://www.playlostark.com/en-us/game/releases/echoes-of-thunder) | official release; 2026-11-18 |
| `la-dimensionalist` | [Dimensions Unbound / Dimensionalist](https://www.playlostark.com/en-us/game/releases/dimensions-unbound) | official release; 2026-09-16 |
| `la-guardianknight` | [Guardian's Rage / Guardianknight](https://www.playlostark.com/en-us/game/releases/guardians-rage) | official release; 2026-02-04 |
| `la-guardianknight-design` | [Guardianknight design](https://www.playlostark.com/en-us/news/articles/guardianknight-class-behind-the-scenes) | official developer article; 2026 |
| `la-valkyrie` | [Rise of the Valkyrie](https://www.playlostark.com/en-gb/game/releases/rise-of-the-valkyrie) | official release; 2025 |
| `la-wildsoul` | [Call of the Wildsoul / Ark Passive overhaul](https://www.playlostark.com/en-us/game/releases/call-of-the-wildsoul) | official release; 2025 |
| `la-passive` | [Arkesia Ignited / Evolution, Enlightenment, Leap](https://www.playlostark.com/en-us/game/releases/arkesia-ignited) | official release; 2024; historical introduction |
| `la-grid` | [Day of Prophecy / Ark Grid](https://www.playlostark.com/en-us/game/releases/day-of-prophecy) | official release; 2025-11-19 |
| `la-grid-explainer` | [Order and Chaos cores / Astrogems](https://www.playlostark.com/en-us/news/articles/2025-roadmap-part-4) | official roadmap; 2025 |
| `la-tripod` | [Tripod customization](https://www.playlostark.com/en-gb/game/about) | official overview; observed 2026-11-25 |
| `la-warrior` | [Warrior advanced classes](https://www.playlostark.com/en-us/news/articles/warrior-advanced-classes) | official class article; 2021; historical mechanics |
| `la-mage` | [Mage advanced classes](https://www.playlostark.com/en-us/news/articles/mage-advanced-classes) | official class article; 2021; historical mechanics |
| `la-assassin` | [Assassin advanced classes](https://www.playlostark.com/en-us/news/articles/assassin-advanced-classes) | official class article; 2021; historical mechanics |
| `la-martial` | [Martial artist advanced classes](https://www.playlostark.com/en-us/news/articles/martial-artist-advanced-classes) | official class article; 2021; historical mechanics |
| `la-gunner` | [Gunner advanced classes](https://www.playlostark.com/en-us/news/articles/gunner-advanced-classes) | official class article; 2021; historical mechanics |
| `la-arcanist` | [Arcanist Academy](https://www.playlostark.com/en-us/news/articles/arcanist-academy) | official class article; 2022; historical mechanics |
| `la-summoner` | [Summoner Academy](https://www.playlostark.com/en-us/news/articles/summoner-academy) | official class article; 2022; historical mechanics |
| `la-reaper` | [Reaper Academy](https://www.playlostark.com/en-us/news/articles/reaper-academy) | official class article; 2022; historical mechanics |
| `la-souleater` | [Souleater Academy](https://www.playlostark.com/en-us/news/articles/souleater-academy) | official class article; 2023; historical mechanics |
| `la-breaker` | [Breaker Breakdown](https://www.playlostark.com/en-us/news/articles/breaker-breakdown) | official class article; 2024; historical mechanics |
| `la-glaivier` | [Glaivier Academy](https://www.playlostark.com/en-gb/news/articles/lost-ark-academy-glaivier) | official class article; 2022; historical mechanics |
| `la-destroyer` | [Destroyer Academy](https://www.playlostark.com/en-gb/news/articles/lost-ark-academy-destroyer-class) | official class article; 2022; historical mechanics |
| `la-slayer` | [Slayer Academy](https://www.playlostark.com/en-us/news/articles/lost-ark-academy-slayer) | official class article; 2023; historical mechanics |
| `la-machinist` | [Machinist Academy](https://www.playlostark.com/en-us/news/articles/lost-ark-academy-machinist) | official class article; 2022; historical mechanics |
| `la-artist` | [Artist Academy](https://www.playlostark.com/en-us/news/articles/lost-ark-academy-artist) | official class article; 2023; historical mechanics |
| `la-aeromancer` | [Aeromancer Academy](https://www.playlostark.com/en-us/news/articles/lost-ark-academy-aeromancer) | official class article; 2023; historical mechanics |
| `la-api` | [Open API / character skill and Ark Passive endpoints](https://developer-lostark.game.onstove.com/changelog) | official api documentation; observed 2026-11-25; Korean service |
| `la-data` | [loa-logs meter data](https://github.com/snoww/loa-logs) | community extracted data; pinned in source_receipts |
| `poe1-overview` | [Gems, supports, passive tree, keystones and items](https://www.pathofexile.com/game) | official overview; observed 2026-11-25 |
| `poe1-tree` | [GGG passive-tree export](https://github.com/grindinggear/skilltree-export) | official data export; 3.29.1 stable; 3.30.0 preview separate |
| `poe1-gems` | [Path of Building generated gem table](https://github.com/PathOfBuildingCommunity/PathOfBuilding) | community generated data; Path of Building 2.69.0 |
| `poe2-overview` | [Skill/support gems, endgame and equipment](https://pathofexile2.com/) | official overview; observed 2026-11-25 |
| `poe2-support-change` | [Support tiers and repeated support copies](https://pathofexile2.com/es/edict) | official expansion announcement; The Third Edict; historical change |
| `poe2-tree` | [GGG passive-tree export](https://github.com/grindinggear/poe2-skilltree-export) | official data export; 0.5.5 published export |
| `poe2-gems` | [Path of Building 2 generated gem table](https://github.com/PathOfBuildingCommunity/PathOfBuilding-PoE2) | community generated data; pinned in source_receipts |
