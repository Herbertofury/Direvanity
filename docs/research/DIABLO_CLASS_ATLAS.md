# Diablo class and ability atlas

Research baseline: **25 November 2026**. This is the Diablo source catalogue for Dreadmarch's massive single-player action RPG direction. It is research input for an original game; it does not describe implemented Dreadmarch combat content.

The companion [`diablo-catalog.json`](../../data/research/diablo-catalog.json) contains the complete enumerated labels, stable source identifiers, source URLs, tree relationships, rune/variant categories and coverage ledger. This atlas explains the decisions and provides a readable class index. Descriptions, art, dialogue and complete numerical balance tables remain outside the repository.

## Verified class union: 19 identities

| Game / edition | Playable class identities in that edition | Treatment |
| --- | --- | --- |
| Diablo I | Warrior, Rogue, Sorcerer | Three original classes; spellbooks are largely shared. |
| Hellfire | The three original classes, Monk, Bard, Barbarian | Monk is the expansion class; Bard and Barbarian are hidden test classes. |
| Diablo II original | Amazon, Barbarian, Necromancer, Paladin, Sorceress | Five-class subset of the legacy catalogue. |
| Lord of Destruction | The five original classes, Assassin, Druid | Seven classes, 30 skills each. |
| Diablo II: Resurrected / Reign of the Warlock | The seven LoD classes, Warlock | Eight classes, 30 skills each. |
| Diablo III / Reaper of Souls / Rise of the Necromancer | Barbarian, Crusader, Demon Hunter, Monk, Necromancer, Witch Doctor, Wizard | All seven official active/passive tables. |
| Diablo IV / Vessel of Hatred / Lord of Hatred | Barbarian, Druid, Necromancer, Paladin, Rogue, Sorcerer, Spiritborn, Warlock | Eight released classes at this baseline. |
| Diablo Immortal | Barbarian, Blood Knight, Crusader, Demon Hunter, Druid, Monk, Necromancer, Tempest, Warlock, Wizard | Ten released classes at this baseline. |

**Creator identity rules:** Sorceress is an alias of Sorcerer. Warrior and Barbarian remain separate. Wizard and Sorcerer remain separate. Paladin and Crusader remain separate. Bard remains selectable despite its hidden legacy status. Their original game-specific restrictions do not constrain Dreadmarch's character bodies or appearance options.

Roster evidence: [official D2 class guide](https://classic.battle.net/diablo2exp/classes/), [official D2R Warlock expansion announcement](https://news.blizzard.com/en-us/article/24243863/rain-annihilation-in-reign-of-the-warlock), [official D3 class guide](https://us.diablo3.blizzard.com/en-us/class/), [official D4 class page](https://diablo4.blizzard.com/en-us/), [official Lord of Hatred release guide](https://news.blizzard.com/en-us/article/24267729/prepare-for-the-reckoning-lord-of-hatred-draws-near), [official Immortal class page](https://diabloimmortal.blizzard.com/en-us/), and [official Immortal Warlock introduction](https://news.blizzard.com/en-us/article/24277443/introducing-diablo-immortals-newest-class-warlock). The [pinned DevilutionX class tables](https://github.com/diasurgical/devilutionX/tree/e1c377775a206274f73b5f01c84a967d06e74d54/mods/hf/txtdata/classes) and [Jarulf's first-hand technical guide](https://wheybags.gitlab.io/jarulfs-guide/) support the legacy classifications.

### Current versions matter

D4's researched live snapshot is **3.2.4.74043**, matching [Blizzard's October 28 patch notes](https://news.blizzard.com/en-us/article/24295395/diablo-iv-patch-notes-3-2). Patch **3.3.0** has been announced for **December 1, 2026**; it is not substituted for the November 25 live baseline. See the [official 3.3.0 notes](https://news.blizzard.com/en-us/article/24316980/diablo-iv-patch-notes) and [season announcement](https://news.blizzard.com/en-us/article/24316979/advance-the-war-against-death-in-season-of-tyraels-gambit).

The 3.0 redesign removed standalone passive nodes from D4 class skill trees. Each active now branches into modifiers, while passive progression also exists in class mechanics, Paragon and equipment. Consequently, a list copied from an old D4 calculator would misrepresent today's trees. Our extraction includes the current tree graphs, class mechanics and Paragon separately. Blizzard describes the redesign in the [Major Skill Tree Updates section](https://worldofwarcraft.blizzard.com/en-us/news/24267729).

Immortal's Warlock was released on June 17, 2026. Its official introduction and launch article establish it as the tenth class. The [official Druid introduction](https://news.blizzard.com/en-us/article/24216435/introducing-diablo-immortals-newest-class-druid), [Tempest introduction](https://news.blizzard.com/en-us/article/24096019/command-wind-and-sea-as-the-tempest), and [Blood Knight introduction](https://news.blizzard.com/en-us/article/23967323/introducing-the-newest-diablo-immortal-class-blood-knight) supply additional transformation, summon and ultimate names.

## Enumeration and source standard

The unit of coverage is explicit. An active, a passive, a rune, a modifier, a Paragon node and a legendary essence are different record types. A UI connector node is not counted as an ability. A localized label retained in game files but absent from the current tree is not counted as currently selectable. Repeated labels such as “Upgrade A” retain their owning power and relation path.

D1/Hellfire use all rows from pinned source-port tables, with unused entries marked. D2 has both the complete official 210-skill legacy index and all 240 current class rows from a pinned extraction. D3 uses official Blizzard HTML tables, including every listed rune. D4 follows every playable SkillKit and every available class Paragon board from pinned client data. Immortal exhausts the public builder database's pagination and cross-checks names against official introductions and a second datamined index.

This is complete enumeration **within these stated snapshots and record categories**. It is not a claim to include every historical patch, all item affixes, every monster/internal proc, or every follower skill. Immortal's community data has no exposed client-build identifier, so its full source enumeration does not prove live executable equivalence. That limitation is carried in the JSON rather than hidden by a broad “complete” label.

## Coverage ledger


| Source scope | Expected / discovered | Included | Unresolved / interpretation |
| --- | ---: | ---: | --- |
| D1 table | 36 rows | 36 | Five unavailable/unused rows explicitly marked; 31 usable spell/innate rows. |
| Hellfire table | 51 rows | 51 | Four unused rows; 47 spell/innate/rune-effect rows. Identify/Search have both shared and innate uses. |
| D2 LoD official index | 210 skills | 210 | All seven legacy classes; original D2 is the five-class subset. |
| D2R / Reign of the Warlock | 240 class rows | 240 | Eight classes × 30; tree grids and prerequisites retained. |
| D3 active / passive / rune | 163 / 131 / 815 | 163 / 131 / 815 | All seven official class tables; footer rows rejected. |
| D4 SkillKits | 198 active entries; 194 unlocks; 1,402 modifiers | All | 2,228 graph nodes, including 632 structural nodes. No missing current names. |
| D4 class mechanics | 122 power references | 122 | Seven Barbarian rank bonuses and 21 enchantments use derived parent labels; spirit-tier labels are marked derived. |
| D4 Paragon | 80 boards; 566 nodes; 161 glyphs | All | 12,852 occupied board cells; 427 named nodes; 139 unnamed stat nodes; one unreferenced definition retained. |
| Immortal skillbook | 1,547 rows across 1,000 + 547 pages | 1,547 | 158 base rows + 1,389 essence rows; no page cap mistaken for completion. |
| Immortal core class abilities | 20 primaries; 130 equipped skills; 20 ultimates | All | Additional form, pet and resource records make 190 class records in total. |
| Immortal Paragon | 136 records | 136 | Eight 15-skill groups plus 16 Legendary Skills; source lacks client-build ID. |

## Class synthesis mapped to the current prototype

The branch column below is loaded directly from **`data/gameplay/classes.json`**: three branches for most classes and **four for Spiritborn**. These are original Dreadmarch branch names, not a claim about source-game terminology. The mechanical connections are **research-backed design proposals for further development**, not a claim that the prototype already implements every listed mechanic. Rebuild timing, costs, damage, names, effects and progression for Dreadmarch's world and campaign. The Divinity atlas supplies additional exploration, social, crafting and environmental interactions for these identities.

| Class | Current prototype branches | Source mechanics to study and identity boundary |
| --- | --- | --- |
| Warrior | Arms / Guard / Duelist | D1's equipment-focused fighter and Item Repair support a grounded weapon master. Build directional blocks, stamina, dueling and field preparation here; reserve rage and supernatural earth attacks for Barbarian. Campaign utility is an original extension, not a hidden D1 tree. |
| Rogue | Marksman / Shadow / Venom | Combine D1 Trap Disarm with D4 bow/knife skills, Concealment, Smoke Grenade and traps. Emphasize scouting, precision and improvised advantage; avoid duplicating Demon Hunter's autonomous arsenal. |
| Sorcerer | Fire / Frost / Lightning | D2's Fire/Cold/Lightning trees and D4 enchantments provide coherent elemental specialization. Support interactions between these elements while keeping an explicit primary school. |
| Monk | Fists / Mantras / Spirit | D3's generators, Seven-Sided Strike, mantras and Mystic Ally combine with Immortal's mobile martial skills. Give its defensive and healing tools useful solo applications; it must not require a party to function. |
| Bard | Songs / Blades / Discord | Hellfire provides dual-weapon identity and Identify, not an extensive musical spellbook. Blades and lore utility draw on that evidence. Songs and Discord are deliberate original expansions into rhythm, sound and morale; do not attribute the new musical abilities to Hellfire. |
| Barbarian | Berserker / Warcries / Earthbreaker | D2 weapon masteries and Warcries, D3 Earthquake and Wrath of the Berserker, and D4 weapon expertise support distinct rage, command and terrain-impact builds. |
| Amazon | Spears / Javelins / Archery | Retain D2's Bow and Crossbow, Javelin and Spear, and Passive and Magic identities. Guided shots, lightning javelins, evasive passives and Valkyrie-style support can share a martial core without turning into Rogue. |
| Assassin | Martial Arts / Traps / Shadow Arts | D2 already supplies three coherent trees. Study charge-up/finisher flow, mobility, shadow doubles and placed mechanisms; combine these through deliberate setup and execution windows. |
| Necromancer | Bone / Blood / Undead | Combine D2 curses and summons with D3/D4 bone, corpse and blood mechanics. Curses should support each branch; corpse supply and minion control are explicit systems rather than free background damage. |
| Paladin | Oaths / Consecration / Judgment | D2 Combat Skills and auras combine with D4 oaths, holy weapons and judgment. Preserve oath-driven conviction and persistent support choices; separate this from Crusader's mounted, heavy-shield assault. |
| Druid | Storm / Wildshape / Companions | D2 shapeshifting and summons, D4 earth/storm skills and Spirit Boons, and Immortal's full replacement form bars supply a strong transformation model. Forms, companions and weather need shared rules for gear and cooldowns. |
| Crusader | Bulwark / Zeal / Wrath | D3 shield skills, Consecration and Steed Charge, plus Immortal Draw and Quarter and Holy Banner, support a durable moving front line. Shield-based area control and mounted momentum distinguish it from Paladin. |
| Demon Hunter | Ballistics / Devices / Vengeance | D3/Immortal crossbows, grenades, Sentry, traps, evasions and Vengeance support a ranged weapons engineer. Explicit ammunition patterns and deployables distinguish it from the Rogue scout. |
| Witch Doctor | Hexcraft / Spirits / Fetishes | D3 Hex, Haunt, Locust Swarm, Firebats, fetishes, gargantuans and spirit movement offer curse, attrition and spirit-pet builds. Preserve living ritual and spirit themes rather than duplicating Necromancer's corpse army. |
| Wizard | Arcana / Displacement / Illusion | D3 Arcane Orb, Arcane Torrent, Slow Time, Teleport, Mirror Image and Archon support shaped arcane forces, altered space/time and deceptive copies. Sorcerer retains direct elemental-school mastery. |
| Blood Knight | Blood / Shadow / Transfiguration | Immortal's Siphon Blood, Tendrils of Blood, polearm attacks, bats, Sanguinate and Abomination support health tradeoffs, reach and transformation. Keep self-sustain readable and resist invulnerability loops. |
| Tempest | Gales / Flow / Depths | Immortal's dual blades, wind/water repositioning and Mist-Touched Zephyr echoes support attack mirroring and flowing mobility. Treat echoes as timed action copies, distinct from permanent necromantic minions. |
| Spiritborn | Jaguar / Eagle / Gorilla / Centipede | D4's four spirit identities remain four explicit prototype branches. Preserve Jaguar offense, Eagle movement/ranged pressure, Gorilla defense and Centipede attrition rather than deleting or hiding one spirit to force symmetry. |
| Warlock | Demonic / Eldritch / Chaos | D2R Demon/Eldritch/Chaos, D4 soul shards and Greater Demons, and Immortal Soulgorger/portals/sacrifices provide pact control, destructive casting and spatial/health exchange. Preserve the distinction from undead and blood-knight systems. |

The class evidence is the complete per-game index below and its linked official or pinned sources. Dreadmarch's branch convention is a design interface, not a claim that every source game used the same structure. D1 primarily used shared spellbooks; D3 used selectable loadouts and runes; Immortal uses loadouts, essences and Paragon; current D4 adds branches within each active skill.

### Merge rules that preserve meaningful choices

1. **Keep the owning system with the ability.** A corpse skill needs corpse production and consumption; an elemental reaction needs surface/condition rules; a form skill needs a defined form bar. Importing only a damage formula loses the mechanic that made it interesting.
2. **Merge equivalent jobs, preserve different decisions.** Two point-blank fire bursts can become one original ability with variants. A delayed ground detonation, a channeled cone and a homing projectile ask different questions and should remain distinct choices even when all deal fire damage.
3. **Protect class signatures.** Necromancer owns corpse/undead command, Witch Doctor spirit/ritual attrition, Warlock demonic contracts, and Blood Knight personal predation and transformation. Shared status effects do not erase these differences.
4. **Make every class a complete solo adventurer.** Each needs an early attack, defense, movement option, resource recovery path and boss strategy. Support-oriented branches must have a solo expression without removing their support role.
5. **Separate campaign utility from combat tuning.** Identification, repair, dialogue, traversal, lock/trap interactions and crafting should deepen exploration. They should not force the player to permanently sacrifice essential combat viability for access to story content.
6. **Budget variants by changed behavior.** Projectile count, target geometry, movement, summons, resource conversion and conditional triggers make useful variant families. A long list of small percentage increases should not be mistaken for the depth shown by the research inventory.

## Reading the full index

The following sections list every base/class ability in the catalogue. D3 rune names and D4 selectable modifier names are included alongside their parent skills. Immortal's 1,389 essence rows and all D4 Paragon grid placements are in the JSON, where their relationships can be queried without duplicating a large item database into prose. Named Paragon groups/boards and class-mechanic entries are listed here. The JSON also retains all 427 named D4 Paragon node definitions, 139 unnamed stat-node definitions and 161 glyph definitions.

## Diablo I

classic table

**Spell:** Firebolt; Healing; Lightning; Flash; Identify; Fire Wall; Town Portal; Stone Curse; Infravision; Phasing; Mana Shield; Fireball; Guardian; Chain Lightning; Flame Wave; Nova; Inferno; Golem; Teleport; Apocalypse; Elemental; Charged Bolt; Holy Bolt; Resurrect; Telekinesis; Heal Other; Blood Star; Bone Spirit.

**Inactive Data Entry:** Doom Serpents; Blood Ritual; Invisibility; Rage; Etherealize.

**Class Skill:** Item Repair; Staff Recharge; Trap Disarm.

Source: https://github.com/diasurgical/devilutionX/blob/e1c377775a206274f73b5f01c84a967d06e74d54/assets/txtdata/spells/spelldat.tsv

## Diablo: Hellfire

expansion table

**Spell:** Firebolt; Healing; Lightning; Flash; Identify; Fire Wall; Town Portal; Stone Curse; Infravision; Phasing; Mana Shield; Fireball; Guardian; Chain Lightning; Flame Wave; Nova; Inferno; Golem; Teleport; Apocalypse; Elemental; Charged Bolt; Holy Bolt; Resurrect; Telekinesis; Heal Other; Blood Star; Bone Spirit; Mana; the Magi; the Jester; Lightning Wall; Immolation; Warp; Reflect; Berserk; Ring of Fire; Search.

**Inactive Data Entry:** Doom Serpents; Blood Ritual; Invisibility; Etherealize.

**Class Skill:** Rage; Item Repair; Staff Recharge; Trap Disarm.

**Rune Item Effect:** Rune of Fire; Rune of Light; Rune of Nova; Rune of Immolation; Rune of Stone.

Source: https://github.com/diasurgical/devilutionX/blob/e1c377775a206274f73b5f01c84a967d06e74d54/mods/hf/txtdata/spells/spelldat.tsv

## Diablo II: Lord of Destruction

official legacy Arreat Summit named-skill index

### Amazon

**Passive and Magic Skills · passive**

- **Avoid**.
- **Critical Strike**.
- **Dodge**.
- **Evade**.
- **Penetrate**.
- **Pierce**.

**Javelin and Spear Skills · active**

- **Charged Strike**.
- **Fend**.
- **Impale**.
- **Jab**.
- **Lightning Bolt**.
- **Lightning Fury**.
- **Lightning Strike**.
- **Plague Javelin**.
- **Poison Javelin**.
- **Power Strike**.

**Bow and Crossbow Skills · active**

- **Cold Arrow**.
- **Exploding Arrow**.
- **Fire Arrow**.
- **Freezing Arrow**.
- **Guided Arrow**.
- **Ice Arrow**.
- **Immolation Arrow**.
- **Magic Arrow**.
- **Multiple Shot**.
- **Strafe**.

**Passive and Magic Skills · active**

- **Decoy**.
- **Inner Sight**.
- **Slow Missiles**.
- **Valkyrie**.

Source entry: https://classic.battle.net/diablo2exp/skills/amazon-passive.shtml#avoid

### Sorceress

**Fire Spells · active**

- **Blaze**.
- **Enchant**.
- **Fire Bolt**.
- **Fire Wall**.
- **Fireball**.
- **Hydra**.
- **Inferno**.
- **Meteor**.

**Cold Spells · active**

- **Blizzard**.
- **Chilling Armor**.
- **Frost Nova**.
- **Frozen Armor**.
- **Frozen Orb**.
- **Glacial Spike**.
- **Ice Blast**.
- **Ice Bolt**.
- **Shiver Armor**.

**Lightning Spells · active**

- **Chain Lightning**.
- **Charged Bolt**.
- **Energy Shield**.
- **Lightning**.
- **Nova**.
- **Static Field**.
- **Telekinesis**.
- **Teleport**.
- **Thunder Storm**.

**Cold Spells · passive**

- **Cold Mastery**.

**Fire Spells · passive**

- **Fire Mastery**.
- **Warmth**.

**Lightning Spells · passive**

- **Lightning Mastery**.

Source entry: https://classic.battle.net/diablo2exp/skills/sorceress-fire.shtml#blaze

### Necromancer

**Curses · active**

- **Amplify Damage**.
- **Attract**.
- **Confuse**.
- **Decrepify**.
- **Dim Vision**.
- **Iron Maiden**.
- **Life Tap**.
- **Lower Resist**.
- **Terror**.
- **Weaken**.

**Summoning Spells · active**

- **Blood Golem**.
- **Clay Golem**.
- **Fire Golem**.
- **Iron Golem**.
- **Revive**.
- **Skeletal Mage**.

**Poison and Bone Spells · active**

- **Bone Armor**.
- **Bone Prison**.
- **Bone Spear**.
- **Bone Spirit**.
- **Bone Wall**.
- **Corpse Explosion**.
- **Poison Dagger**.
- **Poison Explosion**.
- **Poison Nova**.
- **Teeth**.

**Summoning Spells · passive**

- **Golem Mastery**.
- **Skeleton Mastery**.
- **Summon Resist**.

**Summoning · active**

- **Raise Skeleton**.

Source entry: https://classic.battle.net/diablo2exp/skills/necromancer-curses.shtml#amplifydamage

### Paladin

**Offensive Auras · aura**

- **Blessed Aim**.
- **Concentration**.
- **Conviction**.
- **Fanaticism**.
- **Holy Fire**.
- **Holy Freeze**.
- **Holy Shock**.
- **Might**.
- **Sanctuary**.
- **Thorns**.

**Combat Skills · active**

- **Blessed Hammer**.
- **Charge**.
- **Conversion**.
- **Fist of the Heavens**.
- **Holy Bolt**.
- **Holy Shield**.
- **Sacrifice**.
- **Smite**.
- **Vengeance**.
- **Zeal**.

**Defensive Auras · aura**

- **Cleansing**.
- **Defiance**.
- **Meditation**.
- **Prayer**.
- **Redemption**.
- **Resist Cold**.
- **Resist Fire**.
- **Resist Lightning**.
- **Salvation**.
- **Vigor**.

Source entry: https://classic.battle.net/diablo2exp/skills/paladin-offense.shtml#blessedaim

### Barbarian

**Combat Masteries · passive**

- **Axe Mastery**.
- **Increased Speed**.
- **Increased Stamina**.
- **Iron Skin**.
- **Mace Mastery**.
- **Natural Resistance**.
- **Polearm Mastery**.
- **Spear Mastery**.
- **Sword Mastery**.
- **Throwing Mastery**.

**Combat Skills · active**

- **Bash**.
- **Berserk**.
- **Concentrate**.
- **Double Swing**.
- **Double Throw**.
- **Frenzy**.
- **Leap**.
- **Leap Attack**.
- **Stun**.
- **Whirlwind**.

**Warcries · active**

- **Battle Command**.
- **Battle Cry**.
- **Battle Orders**.
- **Find Item**.
- **Find Potion**.
- **Grim Ward**.
- **Howl**.
- **Shout**.
- **Taunt**.
- **War Cry**.

Source entry: https://classic.battle.net/diablo2exp/skills/barbarian-combatmasteries.shtml#axemastery

### Druid

**Elemental · active**

- **Arctic Blast**.
- **Armageddon**.
- **Cyclone Armor**.
- **Firestorm**.
- **Fissure**.
- **Hurricane**.
- **Molten Boulder**.
- **Tornado**.
- **Twister**.
- **Volcano**.

**Summoning · active**

- **Carrion Vine**.
- **Heart of Wolverine**.
- **Oak Sage**.
- **Poison Creeper**.
- **Raven**.
- **Solar Creeper**.
- **Spirit of Barbs**.
- **Summon Dire Wolf**.
- **Summon Grizzly**.
- **Summon Spirit Wolf**.

**Shape Shifting · active**

- **Feral Rage**.
- **Fire Claws**.
- **Fury**.
- **Hunger**.
- **Maul**.
- **Rabies**.
- **Shock Wave**.
- **Werebear**.
- **Werewolf**.

**Shape Shifting · passive**

- **Lycanthropy**.

Source entry: https://classic.battle.net/diablo2exp/skills/druid-elemental.shtml#arcticblast

### Assassin

**Traps · active**

- **Blade Fury**.
- **Blade Sentinel**.
- **Blade Shield**.
- **Charged Bolt Sentry**.
- **Death Sentry**.
- **Fire Blast**.
- **Lightning Sentry**.
- **Shock Web**.
- **Wake of Fire**.
- **Wake of Inferno**.

**Martial Arts · active**

- **Blades of Ice**.
- **Claws of Thunder**.
- **Cobra Strike**.
- **Dragon Claw**.
- **Dragon Flight**.
- **Dragon Tail**.
- **Dragon Talon**.
- **Fists of Fire**.
- **Phoenix Strike**.
- **Tiger Strike**.

**Shadow Disciplines · active**

- **Burst of Speed**.
- **Cloak of Shadows**.
- **Fade**.
- **Mind Blast**.
- **Psychic Hammer**.
- **Shadow Master**.
- **Shadow Warrior**.
- **Venom**.

**Shadow Disciplines · passive**

- **Claw Mastery**.
- **Weapon Block**.

Source entry: https://classic.battle.net/diablo2exp/skills/assassin-traps.shtml#bladefury

## Diablo II: Resurrected / Reign of the Warlock

pinned d2data snapshot; source repository identifies D2R data, compare current official patch notes independently

### Amazon

**Bow and Crossbow Skills · active**

- **Magic Arrow**.
- **Fire Arrow**.
- **Cold Arrow**.
- **Multiple Shot**.
- **Exploding Arrow**.
- **Ice Arrow**.
- **Guided Arrow**.
- **Strafe**.
- **Immolation Arrow**.
- **Freezing Arrow**.

**Passive and Magic Skills · active**

- **Inner Sight**.
- **Slow Missiles**.
- **Decoy**.
- **Valkyrie**.

**Passive and Magic Skills · passive**

- **Critical Strike**.
- **Dodge**.
- **Avoid**.
- **Penetrate**.
- **Evade**.
- **Pierce**.

**Javelin and Spear Skills · active**

- **Jab**.
- **Power Strike**.
- **Poison Javelin**.
- **Impale**.
- **Lightning Bolt**.
- **Charged Strike**.
- **Plague Javelin**.
- **Fend**.
- **Lightning Strike**.
- **Lightning Fury**.

Source entry: https://github.com/blizzhackers/d2data/blob/51e7826e67515f4a84583ebaa882339aeeb99758/json/skills.json

### Sorceress

**Fire Spells · active**

- **Fire Bolt**.
- **Inferno**.
- **Blaze**.
- **Fire Ball**.
- **Fire Wall**.
- **Enchant**.
- **Meteor**.
- **Hydra**.

**Fire Spells · passive**

- **Warmth**.
- **Fire Mastery**.

**Lightning Spells · active**

- **Charged Bolt**.
- **Static Field**.
- **Telekinesis**.
- **Nova**.
- **Lightning**.
- **Chain Lightning**.
- **Teleport**.
- **Thunder Storm**.
- **Energy Shield**.

**Cold Spells · active**

- **Ice Bolt**.
- **Frozen Armor**.
- **Frost Nova**.
- **Ice Blast**.
- **Shiver Armor**.
- **Glacial Spike**.
- **Blizzard**.
- **Chilling Armor**.
- **Frozen Orb**.

**Lightning Spells · passive**

- **Lightning Mastery**.

**Cold Spells · passive**

- **Cold Mastery**.

Source entry: https://github.com/blizzhackers/d2data/blob/51e7826e67515f4a84583ebaa882339aeeb99758/json/skills.json

### Necromancer

**Curses · active**

- **Amplify Damage**.
- **Dim Vision**.
- **Weaken**.
- **Iron Maiden**.
- **Terror**.
- **Confuse**.
- **Life Tap**.
- **Attract**.
- **Decrepify**.
- **Lower Resist**.

**Poison and Bone Spells · active**

- **Teeth**.
- **Bone Armor**.
- **Poison Dagger**.
- **Corpse Explosion**.
- **Bone Wall**.
- **Poison Explosion**.
- **Bone Spear**.
- **Bone Prison**.
- **Poison Nova**.
- **Bone Spirit**.

**Summoning Spells · passive**

- **Skeleton Mastery**.
- **Golem Mastery**.
- **Summon Resist**.

**Summoning Spells · active**

- **Raise Skeleton**.
- **Clay Golem**.
- **Raise Skeletal Mage**.
- **Blood Golem**.
- **Iron Golem**.
- **Fire Golem**.
- **Revive**.

Source entry: https://github.com/blizzhackers/d2data/blob/51e7826e67515f4a84583ebaa882339aeeb99758/json/skills.json

### Paladin

**Combat Skills · active**

- **Sacrifice**.
- **Smite**.
- **Holy Bolt**.
- **Zeal**.
- **Charge**.
- **Vengeance**.
- **Blessed Hammer**.
- **Conversion**.
- **Holy Shield**.
- **Fist of the Heavens**.

**Offensive Auras · aura**

- **Might**.
- **Holy Fire**.
- **Thorns**.
- **Blessed Aim**.
- **Concentration**.
- **Holy Freeze**.
- **Holy Shock**.
- **Sanctuary**.
- **Fanaticism**.
- **Conviction**.

**Defensive Auras · aura**

- **Prayer**.
- **Resist Fire**.
- **Defiance**.
- **Resist Cold**.
- **Cleansing**.
- **Resist Lightning**.
- **Vigor**.
- **Meditation**.
- **Redemption**.
- **Salvation**.

Source entry: https://github.com/blizzhackers/d2data/blob/51e7826e67515f4a84583ebaa882339aeeb99758/json/skills.json

### Barbarian

**Combat Skills · active**

- **Bash**.
- **Leap**.
- **Double Swing**.
- **Stun**.
- **Double Throw**.
- **Leap Attack**.
- **Concentrate**.
- **Frenzy**.
- **Whirlwind**.
- **Berserk**.

**Combat Masteries · passive**

- **Blade Mastery**.
- **Axe Mastery**.
- **Mace Mastery**.
- **Polearm Mastery**.
- **Throwing Mastery**.
- **Spear Mastery**.
- **Increased Stamina**.
- **Iron Skin**.
- **Increased Speed**.
- **Natural Resistance**.

**Warcries · active**

- **Howl**.
- **Find Potion**.
- **Taunt**.
- **Shout**.
- **Find Item**.
- **Battle Cry**.
- **Battle Orders**.
- **Grim Ward**.
- **War Cry**.
- **Battle Command**.

Source entry: https://github.com/blizzhackers/d2data/blob/51e7826e67515f4a84583ebaa882339aeeb99758/json/skills.json

### Druid

**Summoning · active**

- **Raven**.
- **Poison Creeper**.
- **Oak Sage**.
- **Summon Spirit Wolf**.
- **Carrion Vine**.
- **Heart of Wolverine**.
- **Summon Dire Wolf**.
- **Solar Creeper**.
- **Spirit of Barbs**.
- **Summon Grizzly**.

**Shape Shifting · active**

- **Werewolf**.
- **Werebear**.
- **Feral Rage**.
- **Maul**.
- **Rabies**.
- **Fire Claws**.
- **Hunger**.
- **Shock Wave**.
- **Fury**.

**Shape Shifting · passive**

- **Lycanthropy**.

**Elemental · active**

- **Firestorm**.
- **Molten Boulder**.
- **Arctic Blast**.
- **Fissure**.
- **Cyclone Armor**.
- **Twister**.
- **Volcano**.
- **Tornado**.
- **Armageddon**.
- **Hurricane**.

Source entry: https://github.com/blizzhackers/d2data/blob/51e7826e67515f4a84583ebaa882339aeeb99758/json/skills.json

### Assassin

**Traps · active**

- **Fire Blast**.
- **Shock Web**.
- **Blade Sentinel**.
- **Charged Bolt Sentry**.
- **Wake of Fire**.
- **Blade Fury**.
- **Lightning Sentry**.
- **Wake of Inferno**.
- **Death Sentry**.
- **Blade Shield**.

**Shadow Disciplines · passive**

- **Claw Mastery**.
- **Weapon Block**.

**Shadow Disciplines · active**

- **Psychic Hammer**.
- **Burst of Speed**.
- **Cloak of Shadows**.
- **Fade**.
- **Shadow Warrior**.
- **Mind Blast**.
- **Venom**.
- **Shadow Master**.

**Martial Arts · active**

- **Tiger Strike**.
- **Dragon Talon**.
- **Fists of Fire**.
- **Dragon Claw**.
- **Cobra Strike**.
- **Claws of Thunder**.
- **Dragon Tail**.
- **Blades of Ice**.
- **Dragon Flight**.
- **Phoenix Strike**.

Source entry: https://github.com/blizzhackers/d2data/blob/51e7826e67515f4a84583ebaa882339aeeb99758/json/skills.json

### Warlock

**Demon · active**

- **Summon Goatman**.
- **Death Mark**.
- **Summon Tainted**.
- **Summon Defiler**.
- **Engorge**.
- **Blood Boil**.
- **Consume**.
- **Bind Demon**.

**Demon · passive**

- **Demonic Mastery**.
- **Blood Oath**.

**Eldritch · passive**

- **Levitation Mastery**.

**Eldritch · active**

- **Eldritch Blast**.
- **Hex: Bane**.
- **Hex: Siphon**.
- **Psychic Ward**.
- **Echoing Strike**.
- **Hex: Purge**.
- **Blade Warp**.
- **Cleave**.
- **Mirrored Blades**.

**Chaos · active**

- **Sigil: Lethargy**.
- **Ring of Fire**.
- **Miasma Bolt**.
- **Sigil: Rancor**.
- **Flame Wave**.
- **Miasma Chain**.
- **Sigil: Death**.
- **Apocalypse**.
- **Abyss**.

**Chaos · passive**

- **Enhanced Entropy**.

Source entry: https://github.com/blizzhackers/d2data/blob/51e7826e67515f4a84583ebaa882339aeeb99758/json/skills.json

## Diablo III / Reaper of Souls / Rise of the Necromancer

official web tables retrieved 2026-11-25

### Barbarian

**Primary · active**

- **Bash** — Frostbite; Onslaught; Punish; Instigation; Pulverize.
- **Cleave** — Rupture; Reaping Swing; Scattering Blast; Broad Sweep; Gathering Storm.
- **Frenzy** — Sidearm; Berserk; Vanguard; Smite; Maniac.
- **Weapon Throw** — Mighty Throw; Ricochet; Throwing Hammer; Stupefy; Balanced Weapon.

**Secondary · active**

- **Hammer of the Ancients** — Rolling Thunder; Smash; The Devil's Anvil; Thunderstrike; Birthright.
- **Rend** — Ravage; Blood Lust; Lacerate; Mutilate; Bloodbath.
- **Seismic Slam** — Stagger; Shattered Ground; Rumble; Strength from Earth; Permafrost.
- **Whirlwind** — Dust Devils; Hurricane; Blood Funnel; Wind Shear; Volcanic Eruption.
- **Ancient Spear** — Ranseur; Harpoon; Jagged Edge; Boulder Toss; Rage Flip.

**Defensive · active**

- **Ground Stomp** — Deafening Crash; Wrenching Smash; Trembling Stomp; Foot of the Mountain; Jarring Slam.
- **Leap** — Iron Impact; Launch; Toppling Impact; Call of Arreat; Death from Above.
- **Sprint** — Rush; Run Like the Wind; Marathon; Gangway; Forced March.
- **Ignore Pain** — Bravado; Iron Hide; Ignorance is Bliss; Mob Rule; Contempt for Weakness.

**Might · active**

- **Overpower** — Storm of Steel; Killing Spree; Crushing Advance; Momentum; Revel.
- **Revenge** — Blood Law; Best Served Cold; Retribution; Grudge; Provocation.
- **Furious Charge** — Battering Ram; Merciless Assault; Stamina; Cold Rush; Dreadnought.
- **Avalanche** — Volcano; Lahar; Snow-Capped Mountain; Tectonic Rift; Glacier.

**Tactics · active**

- **Threatening Shout** — Intimidate; Falter; Grim Harvest; Demoralize; Terrify.
- **Battle Rage** — Marauder's Rage; Ferocity; Swords to Ploughshares; Into the Fray; Bloodshed.
- **War Cry** — Hardened Wrath; Charge!; Invigorate; Veteran's Warning; Impunity.

**Rage · active**

- **Earthquake** — Giant's Stride; Chilling Earth; The Mountain's Call; Molten Fury; Cave-In.
- **Call of the Ancients** — The Council Rises; Duty to the Clan; Ancients' Blessing; Ancients' Fury; Together as One.
- **Wrath of the Berserker** — Arreat's Wail; Insanity; Slaughter; Striding Giant; Thrive on Chaos.

**Passive · passive**

- **Pound of Flesh**.
- **Ruthless**.
- **Nerves of Steel**.
- **Weapons Master**.
- **Inspiring Presence**.
- **Berserker Rage**.
- **Bloodthirst**.
- **Animosity**.
- **Superstition**.
- **Tough as Nails**.
- **No Escape**.
- **Relentless**.
- **Brawler**.
- **Juggernaut**.
- **Unforgiving**.
- **Boon of Bul-Kathos**.
- **Earthen Might**.
- **Sword and Board**.
- **Rampage**.

Source entry: https://us.diablo3.blizzard.com/en-us/class/barbarian/active/bash

### Crusader

**Primary · active**

- **Punish** — Roar; Celerity; Rebirth; Retaliate; Fury.
- **Slash** — Electrify; Carve; Crush; Zeal; Guard.
- **Smite** — Shatter; Shackle; Surge; Reaping; Shared Fate.
- **Justice** — Burst; Crack; Hammer of Pursuit; Sword of Justice; Holy Bolt.

**Secondary · active**

- **Shield Bash** — Shattered Shield; One on One; Shield Cross; Crumble; Pound.
- **Sweep Attack** — Blazing Sweep; Trip Attack; Holy Shock; Gathering Sweep; Inspiring Sweep.
- **Blessed Hammer** — Burning Wrath; Thunderstruck; Limitless; Brute Force; Dominion.
- **Blessed Shield** — Staggering Shield; Combust; Divine Aegis; Shattering Throw; Piercing Shield.
- **Fist of the Heavens** — Divine Well; Heaven's Tempest; Fissure; Reverberation; Retribution.

**Defensive · active**

- **Shield Glare** — Divine Verdict; Uncertainty; Zealous Glare; Emblazoned Shield; Subdue.
- **Iron Skin** — Reflective Skin; Steel Skin; Explosive Skin; Charged Up; Flash.
- **Consecration** — Bathed in Light; Bed of Nails; Aegis Purgatory; Shattered Ground; Fearful.
- **Judgment** — Penitence; Mass Verdict; Deliberation; Resolved; Debilitate.

**Utility · active**

- **Provoke** — Cleanse; Flee Fool; Too Scared to Run; Charged Up; Hit Me.
- **Steed Charge** — Spiked Barding; Nightmare; Rejuvenation; Endurance; Draw and Quarter.
- **Condemn** — Vacuum; Unleashed; Eternal Retaliation; Shattering Explosion; Reciprocate.
- **Phalanx** — Bowmen; Shield Charge; Stampede; Shield Bearers; Bodyguard.

**Laws · active**

- **Laws of Valor** — Invincible; Frozen in Terror; Critical; Unstoppable Force; Answered Prayer.
- **Laws of Justice** — Protect the Innocent; Immovable Object; Faith's Armor; Decaying Strength; Bravery.
- **Laws of Hope** — Wings of Angels; Eternal Hope; Hopeful Cry; Faith's Reward; Promise of Faith.

**Conviction · active**

- **Falling Sword** — Superheated; Part the Clouds; Rise Brothers; Rapid Descent; Flurry.
- **Akarat's Champion** — Fire Starter; Embodiment of Power; Rally; Prophet; Hasteful.
- **Heaven's Fury** — Blessed Ground; Ascendancy; Split Fury; Thou Shalt Not Pass; Fires of Heaven.
- **Bombardment** — Barrels of Spikes; Annihilate; Mine Field; Impactful Bombardment; Targeted.

**Passive · passive**

- **Heavenly Strength**.
- **Fervor**.
- **Vigilant**.
- **Righteousness**.
- **Insurmountable**.
- **Fanaticism**.
- **Indestructible**.
- **Holy Cause**.
- **Wrathful**.
- **Divine Fortress**.
- **Lord Commander**.
- **Hold Your Ground**.
- **Long Arm of the Law**.
- **Iron Maiden**.
- **Renewal**.
- **Finery**.
- **Blunt**.
- **Towering Shield**.

Source entry: https://us.diablo3.blizzard.com/en-us/class/crusader/active/punish

### Demon Hunter

**Primary · active**

- **Hungering Arrow** — Puncturing Arrow; Serrated Arrow; Shatter Shot; Devouring Arrow; Spray of Teeth.
- **Entangling Shot** — Chain Gang; Shock Collar; Heavy Burden; Justice is Served; Bounty Hunter.
- **Bolas** — Volatile Explosives; Thunder Ball; Freezing Strike; Bitter Pill; Imminent Doom.
- **Evasive Fire** — Hardened; Parting Gift; Covering Fire; Focus; Surge.
- **Grenade** — Tinkerer; Cluster Grenades; Grenade Cache; Stun Grenade; Cold Grenade.

**Secondary · active**

- **Impale** — Impact; Chemical Burn; Overpenetration; Ricochet; Grievous Wounds.
- **Rapid Fire** — Withering Fire; Frost Shots; Fire Support; High Velocity; Bombardment.
- **Chakram** — Twin Chakrams; Serpentine; Razor Disk; Boomerang; Shuriken Cloud.
- **Elemental Arrow** — Ball Lightning; Frost Arrow; Immolation Arrow; Lightning Bolts; Nether Tentacles.

**Defensive · active**

- **Caltrops** — Hooked Spines; Torturous Ground; Jagged Spikes; Carved Stakes; Bait the Trap.
- **Smoke Screen** — Displacement; Lingering Fog; Healing Vapors; Special Recipe; Vanishing Powder.
- **Shadow Power** — Night Bane; Blood Moon; Well of Darkness; Gloom; Shadow Glide.

**Hunting · active**

- **Vault** — Action Shot; Rattling Roll; Tumble; Acrobatics; Trail of Cinders.
- **Preparation** — Invigoration; Punishment; Battle Scars; Focused Mind; Backup Plan.
- **Companion** — Spider Companion; Bat Companion; Boar Companion; Ferret Companion; Wolf Companion.
- **Marked for Death** — Contagion; Valley of Death; Grim Reaper; Mortal Enemy; Death Toll.

**Devices · active**

- **Fan of Knives** — Pinpoint Accuracy; Bladed Armor; Knives Expert; Fan of Daggers; Assassin's Knives.
- **Spike Trap** — Echoing Blast; Custom Trigger; Impaling Spines; Lightning Rod; Scatter.
- **Sentry** — Spitfire Turret; Impaling Bolt; Chain of Torment; Polar Station; Guardian Turret.
- **Vengeance** — Personal Mortar; Dark Heart; Side Cannons; Seethe; From the Shadows.

**Archery · active**

- **Strafe** — Icy Trail; Drifting Shadow; Stinging Steel; Rocket Storm; Demolition.
- **Multishot** — Fire at Will; Wind Chill; Suppression Fire; Full Broadside; Arsenal.
- **Cluster Arrow** — Dazzling Arrow; Shooting Stars; Maelstrom; Cluster Bombs; Loaded for Bear.
- **Rain of Vengeance** — Dark Cloud; Shade; Stampede; Anathema; Flying Strike.

**Passive · passive**

- **Thrill of the Hunt**.
- **Tactical Advantage**.
- **Blood Vengeance**.
- **Steady Aim**.
- **Cull the Weak**.
- **Night Stalker**.
- **Brooding**.
- **Hot Pursuit**.
- **Archery**.
- **Numbing Traps**.
- **Perfectionist**.
- **Custom Engineering**.
- **Grenadier**.
- **Sharpshooter**.
- **Ballistics**.
- **Leech**.
- **Ambush**.
- **Awareness**.
- **Single Out**.

Source entry: https://us.diablo3.blizzard.com/en-us/class/demon-hunter/active/hungering-arrow

### Monk

**Primary · active**

- **Fists of Thunder** — Thunderclap; Wind Blast; Static Charge; Quickening; Bounding Light.
- **Deadly Reach** — Piercing Trident; Searing Grasp; Scattered Blows; Strike from Beyond; Foresight.
- **Crippling Wave** — Mangle; Concussion; Rising Tide; Tsunami; Breaking Wave.
- **Way of the Hundred Fists** — Hands of Lightning; Blazing Fists; Fists of Fury; Assimilation; Windforce Flurry.

**Secondary · active**

- **Lashing Tail Kick** — Vulture Claw Kick; Sweeping Armada; Spinning Flame Kick; Scorpion Sting; Hand of Ytar.
- **Tempest Rush** — Northern Breeze; Tailwind; Flurry; Electric Field; Bluster.
- **Wave of Light** — Wall of Light; Explosive Light; Empowered Wave; Shattering Light; Pillar of the Ancients.

**Defensive · active**

- **Blinding Flash** — Self Reflection; Mystifying Light; Replenishing Light; Crippling Light; Faith in the Light.
- **Breath of Heaven** — Circle of Scorn; Circle of Life; Blazing Wrath; Infused with Light; Zephyr.
- **Serenity** — Peaceful Repose; Unwelcome Disturbance; Tranquility; Ascension; Instant Karma.
- **Inner Sanctuary** — Sanctified Ground; Safe Haven; Temple of Protection; Intervene; Forbidden Palace.

**Techniques · active**

- **Dashing Strike** — Way of the Falling Star; Blinding Speed; Quicksilver; Radiance; Barrage.
- **Exploding Palm** — The Flesh is Weak; Strong Spirit; Impending Doom; Shocking Grasp; Essence Burn.
- **Sweeping Wind** — Master of Wind; Blade Storm; Fire Storm; Inner Storm; Cyclone.

**Focus · active**

- **Cyclone Strike** — Eye of the Storm; Implosion; Sunburst; Wall of Wind; Soothing Breeze.
- **Seven-Sided Strike** — Sudden Assault; Incinerate; Pandemonium; Sustained Attack; Fulminating Onslaught.
- **Mystic Ally** — Water Ally; Fire Ally; Air Ally; Enduring Ally; Earth Ally.
- **Epiphany** — Desert Shroud; Ascendance; Soothing Mist; Insight; Inner Fire.

**Mantras · active**

- **Mantra of Salvation** — Hard Target; Divine Protection; Wind through the Reeds; Perseverance; Agility.
- **Mantra of Retribution** — Retaliation; Transgression; Indignation; Against All Odds; Collateral Damage.
- **Mantra of Healing** — Sustenance; Circular Breathing; Boon of Inspiration; Heavenly Body; Time of Need.
- **Mantra of Conviction** — Overawe; Intimidation; Dishearten; Annihilation; Submission.

**Passive · passive**

- **Resolve**.
- **Fleet Footed**.
- **Exalted Soul**.
- **Transcendence**.
- **Chant of Resonance**.
- **Seize the Initiative**.
- **The Guardian's Path**.
- **Sixth Sense**.
- **Determination**.
- **Relentless Assault**.
- **Beacon of Ytar**.
- **Alacrity**.
- **Harmony**.
- **Combination Strike**.
- **Near Death Experience**.
- **Unity**.
- **Momentum**.
- **Mythic Rhythm**.

Source entry: https://us.diablo3.blizzard.com/en-us/class/monk/active/fists-of-thunder

### Necromancer

**Primary · active**

- **Bone Spikes** — Sudden Impact; Bone Pillars; Frost Spikes; Path of Bones; Blood Spikes.
- **Grim Scythe** — Execution; Dual Scythes; Cursed Scythe; Frost Scythe; Blood Scythe.
- **Siphon Blood** — Blood Sucker; Suppress; Power Shift; Purity of Essence; Drain Life.

**Secondary · active**

- **Bone Spear** — Blighted Marrow; Teeth; Crystallization; Shatter; Blood Spear.
- **Skeletal Mage** — Gift of Death; Contamination; Skeleton Archer; Singularity; Life Support.
- **Death Nova** — Unstable Compound; Tendril Nova; Blight; Bone Nova; Blood Nova.

**Corpses · active**

- **Corpse Explosion** — Bloody Mess; Close Quarters; Shrapnel; Dead Cold; Final Embrace.
- **Corpse Lance** — Shredding Splinters; Brittle Touch; Ricochet; Visceral Impact; Blood Lance.
- **Devour** — Satiated; Ruthless; Devouring Aura; Voracious; Cannibalize.
- **Revive** — Personal Army; Horrific Return; Purgatory; Recklessness; Oblation.

**Reanimation · active**

- **Command Skeletons** — Enforcer; Frenzy; Dark Mending; Freezing Grasp; Kill Command.
- **Command Golem** — Flesh Golem; Ice Golem; Bone Golem; Decay Golem; Blood Golem.
- **Army of the Dead** — Blighted Grasp; Death Valley; Unconventional Warfare; Frozen Army; Dead Storm.
- **Land of the Dead** — Frozen Lands; Plaguelands; Shallow Graves; Invigoration; Land of Plenty.

**Curses · active**

- **Decrepify** — Dizzying Curse; Enfeeblement; Opportunist; Wither; Borrowed Time.
- **Leech** — Transmittable; Osmosis; Blood Flask; Sanguine End; Cursed Ground.
- **Frailty** — Scent of Blood; Volatile Death; Aura of Frailty; Harvest Essence; Early Grave.

**Blood & Bone · active**

- **Bone Armor** — Vengeful Armaments; Dislocation; Limited Immunity; Harvest of Anguish; Thy Flesh Sustained.
- **Bone Spirit** — Astral Projection; Panic Attack; Poltergeist; Unfinished Business; Possession.
- **Blood Rush** — Potency; Transfusion; Molting; Hemostasis; Metabolism.
- **Simulacrum** — Cursed Form; Reservoir; Self Sacrifice; Blood Debt; Blood and Bone.

**Passive · passive**

- **Life from Death**.
- **Fueled by Death**.
- **Stand Alone**.
- **Bone Prison**.
- **Swift Harvesting**.
- **Commander of the Risen Dead**.
- **Extended Servitude**.
- **Rigor Mortis**.
- **Overwhelming Essence**.
- **Dark Reaping**.
- **Spreading Malediction**.
- **Eternal Torment**.
- **Final Service**.
- **Grisly Tribute**.
- **Draw Life**.
- **Serration**.
- **Aberrant Animator**.
- **Blood for Blood**.
- **Blood is Power**.
- **Rathma's Shield**.

Source entry: https://us.diablo3.blizzard.com/en-us/class/necromancer/active/bone-spikes

### Witch Doctor

**Primary · active**

- **Poison Dart** — Splinters; Numbing Dart; Spined Dart; Flaming Dart; Snake to the Face.
- **Corpse Spiders** — Leaping Spiders; Spider Queen; Widowmakers; Medusa Spiders; Blazing Spiders.
- **Plague of Toads** — Explosive Toads; Piercing Toads; Rain of Toads; Addling Toads; Toad Affinity.
- **Firebomb** — Flash Fire; Roll the Bones; Fire Pit; Pyrogeist; Ghost Bomb.

**Secondary · active**

- **Grasp of the Dead** — Unbreakable Grasp; Groping Eels; Death Is Life; Desperate Grasp; Rain of Corpses.
- **Firebats** — Dire Bats; Vampire Bats; Plague Bats; Hungry Bats; Cloud of Bats.
- **Haunt** — Consuming Spirit; Resentful Spirits; Lingering Spirit; Poisoned Spirit; Draining Spirit.
- **Locust Swarm** — Pestilence; Devouring Swarm; Cloud of Insects; Diseased Swarm; Searing Locusts.

**Defensive · active**

- **Summon Zombie Dogs** — Rabid Dogs; Chilled to the Bone; Life Link; Burning Dogs; Leeching Beasts.
- **Horrify** — Phobia; Stalker; Face of Death; Frightening Aspect; Ruthless Terror.
- **Spirit Walk** — Jaunt; Honored Guest; Umbral Shock; Severance; Healing Journey.
- **Hex** — Hedge Magic; Jinx; Angry Chicken; Toad of Hugeness; Unstable Form.

**Terror · active**

- **Soul Harvest** — Swallow Your Soul; Siphon; Languish; Soul to Waste; Vengeful Spirit.
- **Sacrifice** — Black Blood; Next of Kin; Pride; For the Master; Provoke the Pack.
- **Mass Confusion** — Unstable Realm; Devolution; Mass Hysteria; Paranoia; Mass Hallucination.

**Decay · active**

- **Zombie Charger** — Pile On; Undeath; Lumbering Cold; Explosive Beast; Zombie Bears.
- **Spirit Barrage** — The Spirit Is Willing; Well of Souls; Phantasm; Phlebotomize; Manitou.
- **Acid Cloud** — Acid Rain; Lob Blob Bomb; Slow Burn; Kiss of Death; Corpse Bomb.
- **Wall of Death** — Ring of Poison; Wall of Zombies; Surrounded by Death; Fire Wall; Communing with Spirits.
- **Piranhas** — Bogadile; Zombie Piranhas; Piranhado; Wave of Mutilation; Frozen Piranhas.

**Voodoo · active**

- **Gargantuan** — Humongoid; Restless Giant; Wrathful Protector; Big Stinker; Bruiser.
- **Big Bad Voodoo** — Jungle Drums; Rain Dance; Slam Dance; Ghost Trance; Boogie Man.
- **Fetish Army** — Fetish Ambush; Devoted Following; Legion of Daggers; Tiki Torchers; Head Hunters.

**Passive · passive**

- **Jungle Fortitude**.
- **Circle of Life**.
- **Spiritual Attunement**.
- **Gruesome Feast**.
- **Blood Ritual**.
- **Bad Medicine**.
- **Zombie Handler**.
- **Pierce the Veil**.
- **Spirit Vessel**.
- **Fetish Sycophants**.
- **Rush of Essence**.
- **Vision Quest**.
- **Fierce Loyalty**.
- **Grave Injustice**.
- **Tribal Rites**.
- **Confidence Ritual**.
- **Creeping Death**.
- **Swampland Attunement**.
- **Midnight Feast**.

Source entry: https://us.diablo3.blizzard.com/en-us/class/witch-doctor/active/poison-dart

### Wizard

**Primary · active**

- **Magic Missile** — Charged Blast; Glacial Spike; Split; Seeker; Conflagrate.
- **Shock Pulse** — Explosive Bolts; Fire Bolts; Piercing Orb; Power Affinity; Living Lightning.
- **Spectral Blade** — Flame Blades; Siphoning Blade; Thrown Blade; Barrier Blades; Ice Blades.
- **Electrocute** — Chain Lightning; Forked Lightning; Lightning Blast; Surge of Power; Arc Lightning.

**Secondary · active**

- **Ray of Frost** — Cold Blood; Numb; Black Ice; Sleet Storm; Snow Blast.
- **Arcane Orb** — Obliteration; Arcane Orbit; Spark; Scorch; Frozen Orb.
- **Arcane Torrent** — Flame Ward; Death Blossom; Arcane Mines; Static Discharge; Cascade.
- **Disintegrate** — Convergence; Volatility; Entropy; Chaos Nexus; Intensify.

**Defensive · active**

- **Frost Nova** — Shatter; Cold Snap; Frozen Mist; Deep Freeze; Bone Chill.
- **Diamond Skin** — Crystal Shell; Prism; Sleek Shell; Enduring Skin; Diamond Shards.
- **Slow Time** — Time Shell; Exhaustion; Time Warp; Point of No Return; Stretch Time.
- **Teleport** — Safe Passage; Wormhole; Reversal; Fracture; Calamity.

**Force · active**

- **Wave of Force** — Impactful Wave; Debilitating Force; Arcane Attunement; Static Pulse; Heat Wave.
- **Energy Twister** — Mistral Breeze; Gale Force; Raging Storm; Wicked Wind; Storm Chaser.
- **Hydra** — Arcane Hydra; Lightning Hydra; Blazing Hydra; Frost Hydra; Mammoth Hydra.
- **Meteor** — Thunder Crash; Star Pact; Comet; Meteor Shower; Molten Impact.
- **Blizzard** — Lightning Storm; Frozen Solid; Snowbound; Apocalypse; Unrelenting Storm.

**Conjuration · active**

- **Ice Armor** — Chilling Aura; Crystallize; Jagged Ice; Ice Reflect; Frozen Storm.
- **Storm Armor** — Reactive Armor; Power of the Storm; Thunder Storm; Scramble; Shocking Aspect.
- **Magic Weapon** — Electrify; Force Weapon; Conduit; Ignite; Deflection.
- **Familiar** — Sparkflint; Icicle; Ancient Guardian; Arcanot; Cannoneer.
- **Energy Armor** — Absorption; Pinpoint Barrier; Energy Tap; Force Armor; Prismatic Armor.

**Mastery · active**

- **Explosive Blast** — Unleashed; Flash; Short Fuse; Obliterate; Chain Reaction.
- **Mirror Image** — Hard Light; Duplicates; Mocking Demise; Extension of Will; Mirror Mimics.
- **Archon** — Combustion; Teleport; Pure Power; Slow Time; Improved Archon.
- **Black Hole** — Supermassive; Absolute Zero; Event Horizon; Blazar; Spellsteal.

**Passive · passive**

- **Power Hungry**.
- **Blur**.
- **Evocation**.
- **Glass Cannon**.
- **Prodigy**.
- **Astral Presence**.
- **Illusionist**.
- **Cold Blooded**.
- **Conflagration**.
- **Paralysis**.
- **Galvanizing Ward**.
- **Temporal Flux**.
- **Dominance**.
- **Arcane Dynamo**.
- **Unstable Anomaly**.
- **Unwavering Will**.
- **Audacity**.
- **Elemental Exposure**.

Source entry: https://us.diablo3.blizzard.com/en-us/class/wizard/active/magic-missile

## Diablo IV / Vessel of Hatred / Lord of Hatred

3.2.4.74043 (live snapshot; 3.3.0 announced for December 1, 2026)

### Barbarian

**Basic · active**

- **Bash** — Violent Swings; Atomic Blast; Heavy Knuckles; Resolve; Clobbering; Overpower; Fury Generation.
- **Frenzy** — Throwing; Barrage; Raging; Frenzy Stacks; Fury Generation; Ferocity; Unhindered.
- **Lunging Strike** — Whirling; Impaling; Skewer; Range; Fury Generation; Healing; Berserking.
- **Flay** — Anemia; Hemorrhaging; The Flayer; Slow; Duration Bonus; Weaken; Healing.

**Ultimate · active**

- **Call of the Ancients** — The Chosen; Empowered; Highlander; Damage Bonus; Damage Reduction; Ancient Attack Speed; Attack Speed and Damage Bonus.
- **Iron Maelstrom** — Mastered Weapon; Hidden Technique; Armageddon; Cooldown Reduction; Global Cooldown Reduction; Weaken; Additional Effect.
- **Wrath of the Berserker** — Maximum Carnage; Deep Impact; Full Throttle; Overpower; Duration Bonus; Berserking; Fury Generation.

**Defensive · active**

- **Challenging Shout** — Talic; Bawl; Raging; Damage Bonus; Fury Generation; Weaken; Duration Bonus.
- **Ground Stomp** — Seismic; Stomping Grounds; Breach; Fury Generation; Additional Charges; Size Bonus; Damage Bonus.
- **Iron Skin** — Endure; Juggernaut; Retaliation; Movement Speed; Weaken; Resolve; Maximum Life.
- **Rallying Cry** — Booming; Madawc; The World; Duration Bonus; Fortify; Fury Gain; Potency.

**Brawling · active**

- **Charge** — Phalanx; Merciless Assault; Battering Ram; Unstoppable; Distance Damage Bonus; Cooldown Reduction; Enemy Damage Bonus.
- **Kick** — Crimson; Power; Madness; Increase Damage; Vulnerable; Collide; Cooldown Reduction.
- **Leap** — Death From Above; Earthshatter; Mosh; Arsenal Bonus; Fury Generation; Pull; Damage Bonus and Crowd Control.
- **War Cry** — Mighty Roar; Korlic; The Scream; Ferocity; Slow; Duration Increase; Vulnerable.

**Weapon_Mastery · active**

- **Death Blow** — Consecutive; Sonic Boom; Blast; Execute; Overpower; Fury Generation; Charges.
- **Rupture** — Blood Pool; Gore; All For One; Size Bonus; Damage Bonus; Cooldown Reduction; Healing.
- **Steel Grasp** — Chain Whip; Devil's Chain; Solo Chain; Berserking; Extra Charges; Weaken; Vulnerable.
- **Mighty Throw** — Spares; Rapid; Mightiest; Barrier; Size Bonus; Ferocity; Overpower.

**Core · active**

- **Double Swing** — Twister; Flurry; Slice and Dice; Fortify; Chance to Hit Twice; Fury Cost Reduction; Damage Increase.
- **Hammer of the Ancients** — Explosive; Fissure; Ancestral; Critical Strike; Size and Damage Bonus; Fury Generation; Overpower.
- **Rend** — Scorching; Blood Sworn; Flailing; Cast Speed; Healing; Moving Damage Bonus; Vulnerable Damage Bonus.
- **Upheaval** — Suppress; Iron Burst; Trailblazing; Damage Increase; Fury Cost Reduction; Overpower; Free Cast.
- **Whirlwind** — Bursting; Tornado; Weaken; Channel Damage Bonus; Cost Damage Bonus; Movement Speed; Winding.

Source entry: https://github.com/DiabloTools/d4data/blob/75e619f0d90a0951945747f45feef59c076a5db0/json/base/meta/Power/Barbarian_Bash.pow.json

**Class mechanic entries:** One-Handed Axe Expertise; One-Handed Axe Expertise — rank 10 bonus; One-Handed Mace Expertise; One-Handed Mace Expertise — rank 10 bonus; One-Handed Sword Expertise; One-Handed Sword Expertise — rank 10 bonus; Two-Handed Axe Expertise; Two-Handed Axe Expertise — rank 10 bonus; Two-Handed Mace Expertise; Two-Handed Mace Expertise — rank 10 bonus; Polearm Expertise; Polearm Expertise — rank 10 bonus; Two-Handed Sword Expertise; Two-Handed Sword Expertise — rank 10 bonus.

**Paragon boards:** Start; Hemorrhage; Blood Rage; Carnage; Decimator; Bone Breaker; Flawless Technique; Warbringer; Weapons Master; Force of Nature.

### Druid

**Defensive · active**

- **Blood Howl** — Movement Speed; Unstoppable; Werewolf; Dodge Chance; Hungering Howl; Berserking Howl; Spirit Howl; Healing.
- **Cyclone Armor** — Versatile; Damage Bonus; Increased Radius; Resistance; Greater Cyclone Armor; Vulnerable; Slipstream; Reversal.
- **Debilitating Roar** — Werebear; Maximum Life; Booming Roar; Survival Instincts; Roar of Resolve; Slow; Increased Duration; Cooldown.
- **Earthen Bulwark** — Versatile; Travertine Bulwark; Earthen Shrapnel; Reconstruction; Unstoppable; Overpower; Fortify and Healing; Barrier Duration and Bonus.

**Wrath · active**

- **Boulder** — Human; Werebear; Critical Strike Chance; Additional Boulder; Fortify; Crushing Impact; Slow; Dolmen's Will; Landslip.
- **Hurricane** — Slow; Critical Strike Chance; Versatile; Weaken; Rapid Intensification; Movement Speed; Derecho; Twin Storms.
- **Rabies** — Cost Reduction; Healing; Spread Distance; Poisoning Speed; Malaise; Relapse; Superinfection; Werewolf; Werebear.
- **Trample** — Stun; Exsanguination; Trampled Earth; Brunt Force; Spirit; Cooldown Reduction; Unstoppable; Werewolf; Werebear.

**Ultimate · active**

- **Cataclysm** — Slow; Duration; Ferocity; Vulnerable; Severe Storm; Terrible Twisters; Tectonic Spikes; Human; Werewolf.
- **Grizzly Rage** — Werebear; Unstoppable; Immunity; Cornered Beast; Avatar of Nature; Armor Bonus; Cooldown Reduction; Fortify.
- **Lacerate** — Ferocity; Damage Bonus to Multiple; Damage Bonus to Target; Additional Strikes; Werewolf; Toxin Outburst; Hunter Killer; Temperance Training.
- **Petrify** — Enduring Consequences; Seismic Shift; Greater Petrify; Fortify; Resolve; Spirit; Duration; Werebear; Human.

**Basic · active**

- **Claw** — Critical Strike Chance; Cast Speed; Ferocity; Spirit; Dash; Lightning Claw; Viper Strikes; Werebear; Werewolf.
- **Earth Spike** — Overpower; Fortify; Critical Strike Chance; Seismic Shift; Aftershock; Megalith Stone; Spirit; Werewolf; Werebear; Human.
- **Maul** — Reckless Rage; Stone Fists; Wide Sweeps; Resolve; Berserking; Overpower; Spirit; Werebear; Werewolf.
- **Storm Strike** — Storm Hammer; Focused Strike; Additional Chain; Chain Reaction; Spirit; Armor; Ferocity; Werewolf; Werebear; Human.
- **Wind Shear** — Solar Wind; Wind Swept; Boomerang; Spirit; Barrier; Vulnerable; Cast Speed; Werewolf; Werebear; Human.

**Core · active**

- **Lightning Storm** — Werewolf; Human; Hero of the Storm; Omnibolt; Supercell; Movement Speed; Minimum Strikes; Additional Strikes; Damage Bonus.
- **Pulverize** — Mega Bear Punch; Greater Pulverize; Shockwave; Cost Reduction; Damage Bonus; Initial Damage Bonus; Weaken; Werebear.
- **Shred** — Roundhouse; Critical Shred; Storm Shred; Healing; Berserking; Spirit; Cast Speed; Werewolf.
- **Stone Burst** — Living Stones; Rumbling Stones; Armor; Epicenter; Fortify; Spirit Cost Reduction; Growth Speed; Human; Werebear.
- **Tornado** — Cast Speed; Slow; Greater Tornado; Swelling Storm; Explosive Tornado; Increased Duration; Additional Tornado; Werewolf; Human.
- **Landslide** — Continental Shift; Vengeance of Earth; Land Mine; Additional Pillar; Initial Damage Bonus; Critical Strike Chance; Resolve; Human; Werebear.

**Companion · active**

- **Ravens** — Versatile; Raven Flock; Stormcrow; Migration; Active Damage Bonus; Vulnerable; Additional Raven; Resolve.
- **Poison Creeper** — Active Poisoning Duration; Versatile; Toxic Vegetation; Carnivorous; Germinate; Additional Poison Creeper; Fortify; Cooldown Reduction.
- **Wolves** — Versatile; Berserking; Werewolf Pack; Thrill of the Hunt; Damage Bonus; Stun; Attack Speed; Additional Wolf.

Source entry: https://github.com/DiabloTools/d4data/blob/75e619f0d90a0951945747f45feef59c076a5db0/json/base/meta/Power/Druid_BloodHowl.pow.json

**Class mechanic entries:** Prickleskin; Gift of the Stag; Wariness; Advantageous Beast; Scythe Talons; Iron Feather; Swooping Attacks; Avian Wrath; Pack Leader; Energize; Bolster; Calamity; Obsidian Slam; Overload; Masochistic; Calm Before the Storm.

**Paragon boards:** Start; Thunderstruck; Earthen Devastation; Survival Instincts; Lust for Carnage; Heightened Malice; Inner Beast; Constricting Tendrils; Ancestral Guidance; Untamed.

### Necromancer

**Ultimate · active**

- **Army of the Dead** — Damage Bonus; Cooldown Reduction; Dead Cold; Pile the Bodies; Unyielding Commander; Passive Bonus; Corpse Generation.
- **Blood Wave** — Overpower; Blood Orb; Hematolagnia; Path of Darkness; Tides of Blood; Damage Reduction; Damage Bonus.
- **Bone Storm** — Damage Reduction; Duration Increase; Roll The Bones; Shadow And Bone; Hungry Cyclone; Vulnerable; Barrier.
- **Soulrift** — Movement Speed; Barrier; Distilled Anima; Soul Vortex; Frozen Wasteland; Vulnerable and Crowd Control; Ferocity.

**Core · active**

- **Blight** — Crowd Control Damage Bonus; Lucky Hit Chance; Volatile Blood; Whirlpool; Piercing Darkness; Size Bonus; Area Damage Bonus.
- **Blood Lance** — Overpower; Cost Reduction; Fortify; Ricochet; Festering Wound; Gore Quills; Blood Seeker.
- **Blood Surge** — Overpower; Weaken; Pins and Needles; You And What Army?; Bloodbath; Fortify; Damage Bonus.
- **Bone Spear** — Pierce Damage Bonus; Resolve; Shadow Splitter; Blood Spear; Bone Spikes; First Hit Damage Bonus; Cost Reduction.
- **Sever** — Damage Bonus; Cost Reduction; Reaping Lotus; Inexorable Reaper; Cold Pursuit; Crowd Control and Corpse Generation; Ferocity.
- **Skeleton Mage** — Duration Damage Bonus; Critical Strike Chance; Coven; Gift of Death; Singularity; Crowd Control Damage Bonus; Ferocity, Resolve, or Overpower.

**Macabre · active**

- **Blood Mist** — Critical Strike; Movement Speed; Blood Transfusion; Blood Rush; Devouring Mist; Overpower; Vulnerable and Slow.
- **Bone Prison** — Cooldown Reduction; Essence Generation; Bramble; Plunging Darkness; Life Imprisonment; Vulnerable; Resolve.
- **Bone Spirit** — Maximum Essence; Damage Bonus; Charges; Core Skill; Astral Projection; Unfinished Business; Poltergeists.

**Basic · active**

- **Bone Splinters** — Projectiles; Essence Generation; Vulnerable; Resolve; Bouncing Spines; Bloody Splinter; Shadow Seekers.
- **Decompose** — Damage Bonus; Barrier; Crowd Control; Lucky Hit Chance; Rip and Tear; Putrid Burst; Dry Rot.
- **Hemorrhage** — Overpower; Weaken; Cast Speed; Blood Orb; Blood Boil; Blood Runs Cold; Soul Rip.
- **Reap** — Ferocity; Corpse Generation; Essence Generation; Critical Strike Chance; Chilled To The Bone; Cull The Weak; Harvest.

**Corpse · active**

- **Corpse Explosion** — Blood Orb; Corpse Efficiency; Bloody Mess; Miasma; Shrapnel; Multiple Corpses; Essence Generation.
- **Corpse Tendrils** — Critical Strike Chance; Essence Generation; Bitter Harvest; Get Over Here!; Jaws Of Death; Vulnerable; Lucky Hit Chance.

**Curse · active**

- **Decrepify** — Resolve; Movement Speed; Unholy Frenzy; Life Tap; Dizzying Curse; Cooldown Reduction; Critical Strike Chance.
- **Iron Maiden** — Ferocity and Overpower; Vulnerable; Blood Maiden; Torture Artist; Schadenfreude; Execute and Fortify; Essence Generation.

**Minion · active**

- **Golem** — Damage Bonus; Unstoppable; Gargantua; Fel Gluttony; Gravebloom; Weaken; Resolve, Overpower, or Ferocity.
- **Skeleton Warrior** — Damage Bonus; Healing; Master of Puppets; Service and Sacrifice; Litany of Death; Vulnerable; Resolve.

Source entry: https://github.com/DiabloTools/d4data/blob/75e619f0d90a0951945747f45feef59c076a5db0/json/base/meta/Power/Necromancer_ArmyoftheDead.pow.json

**Class mechanic entries:** Skeleton Warrior; Upgrade A; Upgrade B; Sacrifice; Upgrade A; Upgrade B; Sacrifice; Upgrade A; Upgrade B; Sacrifice; Skeleton Mage; Upgrade A; Upgrade B; Sacrifice; Upgrade A; Upgrade B; Sacrifice; Upgrade A; Upgrade B; Sacrifice; Golem; Upgrade A; Upgrade B; Sacrifice; Upgrade A; Upgrade B; Sacrifice; Upgrade A; Upgrade B; Sacrifice.

**Paragon boards:** Start; Cult Leader; Hulking Monstrosity; Flesh-eater; Scent of Death; Bone Graft; Blood Begets Blood; Bloodbath; Wither; Frailty.

Repeated Upgrade A / Upgrade B / Sacrifice labels belong to individual army specializations. Their unique power IDs and specialization paths are retained in the JSON.

### Paladin

**Basic · active**

- **Advance** — Flash of the Blade; Wave Dash; Vanguard's Rush; Fortify; Unhindered; Critical Strike Chance; Weaken.
- **Brandish** — Cross Strike; Sword of Mastery; Returning Light; Faith Generation; Damage Increase; Vulnerable; Cast Speed.
- **Clash** — Seize Them; Skirmish; Punishment; Faith Generation; Resolve; Damage Increase; Crusader's March Effectiveness.
- **Holy Bolt** — Ricocheting Bolt; Divine Bolt; Storm Bolt; Cast Speed; Slow; Faith Generation; Damage Increase.

**Core · active**

- **Blessed Hammer** — Cast Speed; Slow; Mortar Combat; Disciple's Halo; Shattering Blow; Damage Bonus; Cost Reduction.
- **Blessed Shield** — Damage Bonus; Armor and Block Chance Bonus; Shield of Justice; Shield of the Revenant; Shield of Retribution; Cast Speed; Faith Generation.
- **Divine Lance** — Cast Speed; Stacking Damage; Tip of the Spear; Zealous Joust; Divine Javelin; Cost Reduction; Damage Bonus.
- **Zeal** — Zealot's Legacy; Death or Glory; Cull the Wicked; Additional Strikes; Fortify; Weaken; Critical Strike Chance.
- **Shield Bash** — Damage Bonus; Distance; Breach; Smite; Lay Siege; Size Bonus; Hits Are Blocks.

**Justice · active**

- **Condemn** — Cooldown Reduction; Weaken; Movement Speed; Size Increase; Gather the Guilty; Shepherd the Flock; Atonement.
- **Consecration** — Resource Generation; Fortify; Duration; Weaken; Sanctify; Bastion; Hallowed Ground.
- **Purify** — Faith Generation; Cooldown Reduction; Size Bonus; Echo; Absolution; Surrender; Sentence.
- **Spear of the Heavens** — Cooldown Reduction; Judgement Damage Bonus; Projectiles; Vulnerable; Pronouncement of Heaven; Part the Heavens; Fist of the Heavens .

**Aura · active**

- **Defiance Aura** — Potency; Bonus Healing; Rite of Thorns; Rite of Prayer; Rite of Might; Maximum Life; Unstoppable.
- **Holy Light Aura** — Judgement Damage Bonus; Potency; Rite of Submission; Rite of Mercy; Rite of Judgement; Additional Bounce; Additional Targets.
- **Fanaticism Aura** — Extra Passive Stack; Additional Maximum Resource; Rite of Redemption; Rite of Humility; Rite of Vengeance; Potency; Resource Generation.

**Ultimate · active**

- **Arbiter of Justice** — Cooldown Reduction; Wing Strike Recast; Movement Speed; Duration; Seraph's Wings; Reach of the Law; Divine Intervention.
- **Fortress** — Duration; Resolve Damage Bonus; Free Cast; Unstoppable; Rampart of Thorns; Entrench; Barricade.
- **Heaven's Fury** — Duration; Slow; Judgement; Damage Bonus; Final Justice; Walk With The Light; Triplicate.
- **Zenith** — Weaken; Unstoppable; Critical Strike Chance; Weaken; Empyrean Edge; Sunder; Sermon of Steel.

**Valor · active**

- **Falling Star** — Damage; Cooldown Reduction; Starfall; Fanatic Descent; Faster Than Light; Vulnerable; Additional Charge.
- **Rally** — Words of Rejuvenation; Words of Inspiration; Words of Sacrifice; Duration Bonus; Critical Strike Chance; Cost Reduction; Unhindered and Movement Speed.
- **Shield Charge** — Hit Count As Blocking; Retribution; Phalanx Charge; Virtuous Charge; Relentless Charge; Damage Bonus; Resolve.
- **Aegis** — Block Damage Reduction; Duration; Impunity; Stay Resolute; Cooldown Reduction; Unstoppable; Faith Is My Shield.

Source entry: https://github.com/DiabloTools/d4data/blob/75e619f0d90a0951945747f45feef59c076a5db0/json/base/meta/Power/Paladin_Advance_lunge.pow.json

**Class mechanic entries:** Zealot; Juggernaut; Judicator; Disciple; Retribution.

**Paragon boards:** Start; Castle; Shield Bearer; Fervent; Preacher; Divinity; Relentless; Sentencing; Endure; Beacon.

### Rogue

**Core · active**

- **Barrage** — Multishot; Broadside; Succession; Energy Cost Reduction; Vulnerable or Weakened; Cast Speed; Overpower.
- **Dance of Knives** — Fan of Knives; Star Shards; Grenade Jumper; Pierce and Size; Lucky Hit; Slow and Movement Speed; Energy Cost Reduction.
- **Flurry** — Vehemence; Encircling Blades; Transporter; Fortify; Cast Speed; Resolve; Damage Increase.
- **Penetrating Shot** — Shadow Shot; Ballista; Trick Shot; Damage Increase; Slow; Vulnerable; Ferocity.
- **Rapid Fire** — Explosive Shot; Breakneck Quiver; Ruckus; Critical Strike; Energy Gain; Ferocity; Additional Projectiles.
- **Twisting Blades** — Bladedancer; Sleight of Hand; Twisted Twin; Resolve; Crowd Controlled or Weakened; Energy Cost Reduction; Damage Increase and Execute.

**Basic · active**

- **Blade Shift** — Grenade Shift; Impossible Escape; Range of Motion; Resolve; Overpower; Energy; Resistance.
- **Forceful Arrow** — Cloudburst; Pin; Ender; Vulnerable; Energy; Ferocity; Overpower.
- **Heartseeker** — Procession; Devouring Arrow; Confluence; Pierce; Critical Strike Chance; Energy; Weakened.
- **Invigorating Strike** — Accumulating Strike; Quick Strike; Enervating Strike; Healing; Vulnerable; Cast Speed; Energy.
- **Puncture** — Propulsion; Three of a Kind; Blight Burst; Energy; Crowd Control; Overpower; Pierce.

**Agility · active**

- **Caltrops** — Mired Sharpshooter; Catch Cold; Pernicious Poisons; Duration; Ferocity; Critical Strike; Size.
- **Dash** — Detonation; Primer; Echo; Critical Strike Chance; Weaken; Charges and Cooldown Reduction; Resolve.
- **Shadow Step** — Safe Arrival; Sight Unseen; Artifice; Vulnerable; Cooldown; Unstoppable; Cleave.

**Imbuements · active**

- **Cold Imbuement** — Frozen Heart; Buffered Battery; Crystal Distillation; Overpower; Resolve; Cooldown; Energy.
- **Poison Imbuement** — Sapper; Toxic Touch; Corrosive Ring; Energy; Duration; Cooldown; Critical Strike.
- **Shadow Imbuement** — Dark Dance; Endemic; Shadow Play; Vulnerable; Duration; Ferocity; Overpower.

**Subterfuge · active**

- **Dark Shroud** — Best Served Cold; Overflow; Shadow Dancer; Healing; Consume Chance; Critical Strike; Movement Speed.
- **Poison Trap** — Infiltrator; Frostbite Trap; Contamination; Resolve; Charges; Knock Down; Vulnerable.
- **Smoke Grenade** — Silhouette; Poison Grenade; Shade Grenade; Charge or Energy Cost Reduction; Vulnerable; Size; Increased Duration and Damage.
- **Concealment** — Concealing Fate; Recuperate; Calculation; Ferocity; Unstoppable; Resolve; Energy.

**Ultimate · active**

- **Death Trap** — Death Pit; Explosive Trigger; Sudden Death; Cooldown; Dodge; Resolve; Weaken.
- **Rain of Arrows** — Flash Flood; Crashing Waves; Monsoon; Cooldown; Double Damage; Ferocity; Cast Speed and Lucky Hit.
- **Shadow Clone** — Awakening; Sentry; Relentless; Cooldown; Resolve; Unstoppable; Weaken & Vulnerable.

Source entry: https://github.com/DiabloTools/d4data/blob/75e619f0d90a0951945747f45feef59c076a5db0/json/base/meta/Power/Rogue_Barrage.pow.json

**Class mechanic entries:** Combo Points; Preparation; Inner Sight.

**Paragon boards:** Start; Eldritch Bounty; Tricks of the Trade; Cheap Shot; Deadly Ambush; Leyrana's Instinct; No Witnesses; Exploit Weakness; Cunning Stratagem; Danse Macabre.

### Sorcerer

**Basic · active**

- **Arc Lash** — Full Lash; Inescapable Reach; Icy Thrash; Ferocity; Damage Bonus; Resource Generation; Stun.
- **Fire Bolt** — Healing; Overpower; Twin Flame; Resource Generation; Damage Bonus; Shiver; Firefly.
- **Frost Bolt** — Relentless Bolt; Vulnerable; Shatter; Static Shock; Resource Generation; Barrier; Pierce.
- **Spark** — Overcharged; Firework; Electric Blast; Additional Hits; Crackling Energy; Critical Strike Chance; Additional Hits.

**Mastery · active**

- **Ball Lightning** — Crackling Energy; Damage Bonus; Multiple Projectiles; Attraction; Orbital; Implosion; Ferocity.
- **Blizzard** — Exposure; Hailstorm; Static Field; Seek and Duration Bonus; Resource Generation; Barrier; Damage Bonus.
- **Firewall** — Ring of Fire; Wildfire; Healing; Grounding; Weaken; Cost Reduction; Damage Bonus.
- **Meteor** — Resource Damage Bonus; Overpower; Shooting Star; Immobilize; Damage and Size Bonus; Comet; Air Burst.

**Core · active**

- **Chain Lightning** — Chain of Cold; Chain Whipping; Power Flux; Critical Strike Chance; Ramping Damage; Additional Chains; Damage Bonus.
- **Charged Bolts** — Damage Bonus; Weaken; Movement Speed; Surging Charge; Cast Speed; Burn Through; Overload.
- **Fireball** — Instantaneous Destruction; Illumination; Frostball; Damage Bonus; Size Bonus; Damage Over Time; Critical Strike Damage.
- **Frozen Orb** — Explosion Damage Bonus; Duration Bonus; Barrier; Seeking Orb; Shattered Path; Great Orbs of Fire; Damage Over Time Bonus.
- **Ice Shards** — Thunderbolt; Glacial Spike; Piercing Cold; Pierce Damage Bonus; Hit Damage Bonus; Barrier; Cost Reduction.
- **Incinerate** — Flamethrower; Disintegration; Overpower; Focused Flame; Execute; Healing; Healthy Damage Bonus.

**Ultimate · active**

- **Deep Freeze** — Duration; Glacier; Bonechill; Frigid Ward; Vulnerable and Weakened; Cooldown with Barrier ; Reduce Cooldown.
- **Inferno** — Direct Damage Bonus; Cooldown Reduction; Meteor Shower; Living Bomb; Lifecoil; Pull; Overpower.
- **Unstable Currents** — Cooldown Reduction; Damage Bonus; Ramping Currents; Cost Reduction; Thunder God; Boundless; Ferocity.

**Defensive · active**

- **Flame Shield** — Duration Bonus; Scorch; Cooldown Reduction; Healing; Overpower; Flash; Firebird.
- **Frost Nova** — Freezing Mist; Vulnerable; Size Bonus; Cold Front; Shatter; Resource Generation; Charge.
- **Ice Armor** — Resource Regeneration; Blast Chill; Razor Ice; Permafrost; Cooldown Reduction; Frozen and Chill; Duration Bonus and Barrier.
- **Teleport** — Wormhole; Blaze; Ferocity; Movement Speed; Electrobolt; Crackling Energy; Weaken.

**Conjuration · active**

- **Hydra** — Cooldown Reduction; Cast Speed; Healing; Overpower; Bathed in Flame; Behemoth; Frigid Hydra.
- **Ice Blades** — Cooldown Reduction; Blazing Blades; Frozen Tomb; Ice Crack; Area Bonus; Duration Bonus; Cast Speed.
- **Lightning Spear** — Duration Bonus; Projectile Speed; Reverse Polarity; Chain Reaction; Flame Spear; Critical Strike; Damage Bonus.
- **Familiar** — Charge; Size Bonus; Duration Bonus; Healing Flames; Bonus Effect; Freezing Familiar; Fast As Lightning; Prismatic Familiar.

Source entry: https://github.com/DiabloTools/d4data/blob/75e619f0d90a0951945747f45feef59c076a5db0/json/base/meta/Power/Sorcerer_ArcLash.pow.json

**Class mechanic entries:**  Enchantment;  Enchantment;  Enchantment;  Enchantment;  Enchantment;  Enchantment;  Enchantment;  Enchantment;  Enchantment;  Enchantment;  Enchantment;  Enchantment;  Enchantment;  Enchantment;  Enchantment;  Enchantment;  Enchantment;  Enchantment;  Enchantment;  Enchantment;  Enchantment; Familiar Enchantment.

**Paragon boards:** Start; Searing Heat; Burning Instinct; Frigid Fate; Icefall; Static Surge; Ceaseless Conduit; Elemental Summoner; Enchantment Master; Fundamental Release.

### Spiritborn

**Basic · active**

- **Withering Fist** — Shared Suffering; Precision Strike; Fist of the Forest; Vigor Generation; Healing; Weakened; Poisoning Duration.
- **Thunderspike** — Infected Ground; Lightning Dancer; Lightning's Grip; Damage to Sparking; Movement Speed; Critical Strike; Vigor Generation.
- **Rock Splitter** — Unbreakable; Inner Ferocity; Shattered Stone; Free Strike; Resolve; Block and Vigor; Crowd Control Damage.
- **Thrash** — Crashing and Thrashing; Wanton Rampage; Storm Conjurer; Vigor Generation; Ferocity; Cooldown Reduction; Ramping Damage.

**Core · active**

- **Stinger** — Potent Sting; Unleash the Swarm; Nimbostratal Strike; Healing and Vulnerable; Damage Bonus; Weakened Damage Bonus; Size Bonus.
- **Quill Volley** — Volatile Feathers; Quill Command; Claw Volley; Bonus Feathers; Vulnerable Damage; Returning Feathers; Evade Damage.
- **Crushing Hand** — Unrelenting Assault; Heavy Hitter; 100 Crushing Hands; Barrier; Block to Damage; Increased Cost and Damage; Resolve.
- **Rake** — Simian Swipe; Grasping Claw; Spirit Jaguar; Cast Speed; Ferocity; Cooldown Reduction; Cost Reduction and Damage.

**Defensive · active**

- **Scourge** — Scourge Commander; The Beast Within; Hive Mind; Healing; Vulnerable; Weaken; Vigor Generation.
- **Armored Hide** — Trampled Under Foot; Force of Nature; Ferocious Meal; Thorns; Blocked Damage Reduction; Barrier; Duration.
- **Concussive Stomp** — Crushing Blow; Debilitate; Crackling Earth; Barrier Generation; Additional Charge; Resolve Generation; Damage Bonus.
- **Counterattack** — Asymmetrical Warfare; Controlled Reflection; Counterattack of the Swarm; Cooldown Reduction; Damage Over Time Reduction; Duration Increase; Damage Bonus.

**Focus · active**

- **Toxic Skin** — Erupting Skin; Skuttle Bug; Fiery Soul; Duration Increase; Healing; Vigor Generation; Thorns.
- **Vortex** — Overcharged; 1000 Year Storm; Insect Swarm; Knock Down; Damage Increase; Vigor Generation; Cooldown Reduction.
- **Soar** — Free Bird; Eye of the Storm; Jaguar's Execution; Size Bonus; Cooldown Reduction; Movement Speed; Damage Increase.
- **Ravager** — Carnage; Flurry of Claws; Strength of the Forest; Duration Increase; Vigor Generation; Damage Increase; Ferocity Maximum.

**Potency · active**

- **Touch of Death** — Swarm of Swarms; Touch of Lightning; Erupting Swarm; Additional Charge; Vulnerable; Healing; Weakened Damage.
- **Razor Wings** — Wing Storm; Flock of Feathers; Brutal Blast; Movement Speed; Damage Increase; Refund Charges; Charge Cooldown.
- **Payback** — Break Free; Wrath of the Gorilla; Insidious Retribution; Area Increase; Block Chance; Resolve Generation; Damage Increase.
- **Rushing Claw** — Evasive Swipe; Colossal Claw; Evolving Claws; Incarnate Duration; Damage Bonus; Charge Increase; Damage Increase.

**Ultimate · active**

- **The Devourer** — Sky and Soil; Blast of Bile; Spew Putrefaction; Healing; Thorns; Instant Kill; Poisoning Duration.
- **The Seeker** — Lone Seeker; Stormbringer; Tag Team; Charge Refund; Vulnerable; Vigor Generation; Storm Feathers.
- **The Protector** — Canopy Shelter; Brutal Barrage; The Infected Forest; Barrier; Damage Increase; Resolve Generation; Thorns Damage.
- **The Hunter** — Shredder; Push and Pounce; Relentless Pursuit; Vigor Generation; Size Bonus; Execute; Healing.

Source entry: https://github.com/DiabloTools/d4data/blob/75e619f0d90a0951945747f45feef59c076a5db0/json/base/meta/Power/Spiritborn_Centipede_Basic.pow.json

**Class mechanic entries:** Jaguar Primary Spirit; Jaguar Secondary Spirit; Eagle Primary Spirit; Eagle Secondary Spirit; Gorilla Primary Spirit; Gorilla Secondary Spirit; Centipede Primary Spirit; Centipede Secondary Spirit.

**Paragon boards:** Start; In-Fighter; Spiney Skin; Viscous Shield; Bitter Medicine; Revealing; Drive; Convergence; Prodigy's Tempo; Swarm of Storms.

### Warlock

**Ultimate · active**

- **Metamorphosis** — Maximum Life; Damage Scaling; Stagger Bar; Dominance; Sin Demon; Destruction Demon; Terror Demon.
- **Apocalypse** — Double Blast; Epicenter Damage; Shared Damage; Survivor Damage; Minefield; Armageddon Ritual; Annihilation.
- **Terror Swarm** — Vulnerable; Extra Bites; Lesser Demons; Pull; Nightmare; Searing Infestation; Devouring Swarm.
- **Fiend of Abaddon** — Elite Damage; Dominance; Crowd Damage; Lesser Demon Bonus; Vanguard of Flames; Abyssal Colossus ; Spiteful Enrage.

**Basic · active**

- **Molten Bomb** — Ranged Damage; Wrath; Knock Down; Area Bonus; Brimstone Mortar; Demon Turret; Cluster Bomb.
- **Doom** — Assimilation; Echo; Wrath; Damage; Doomfire Ritual; Contagion; Jailer.
- **Command Fallen** — Damage Per Demon; Wrath; Dominance; Lifetime Damage; Mega Lunatic; Fallen Rush; Meat Shields.
- **Hellion Sting** — Eviscerate; Fortify; Wrath; Damage; Demonic Swipe; Tail Spikes; Multi-Strike.

**Core · active**

- **Blazing Scream** — Impact Velocity; Brimstone Hunger; Cost Reduction; Damage; Skull Splitter; Abyssal Oppression; Orbital Scream.
- **Umbral Chains** — Heavy Harpoon; Wrath Refund; Capture Damage; Long Range Slow; Chain Burst; Chain Whips; Epicenter.
- **Bombardment** — Damage; Additional Demon; Focused Target; Telekinesis ; Endless Barrage; Demon Mangler; Brutality.
- **Hell Fracture** — Ramp Up; Additional Fracture; Cost Reduction; Volatility; Eruption; Lava Fissures; Ruptured Abyss.
- **Dread Claws** — Ambush; Vulnerable; Damage; Cost Reduction; Ravenous Jaws; Enveloping Terror; Cascading Dread.

**Defensive · active**

- **Dark Prison** — Missile Slow; Fortify; Cooldown Reduction; Weaken; Tripwire; Chain Aura; Calamity Cage.
- **Tortured Wretch** — Enrage; Duration; Cost Reduction; Unstoppable; Domination; Death Saws; Punching Bag.
- **Wall of Agony** — Damage; Small Walls; Cost Reduction; Knock Down; Waller; Fallen Army; Encircle.
- **Nether Step** — Damage Reduction; Extra Charge; Movement Speed; Vacuum Swap; Portals; Gloomwalker; Shadow Recall.

**SoulShard · active**

- **Command Ae'grom**.
- **Command Laalish**.
- **Command Valloch**.
- **Command Abodian**.

**Archfiend · active**

- **Tyrant's Grasp** — Unstoppable Damage; Lesser Demon Slow; Grasp Area; Knock Down; Abyssal Pit; Demonic Grasp; Inescapable Grapple.
- **Rampage** — Killstreak Damage; Lesser Demon Wrath; Demon Masher; Elite Hit Chance; Shockwave; Abyssal Titan; Demonic Smash.
- **Infernal Breath** — Ramping Damage; Lesser Demon Volatility; Heavy Impact Damage; Expensive Damage; Demonic Breath; Explosive Death; Abyssal Torrent.
- **Profane Sentinel** — Lesser Demon Vulnerable; Single Target Damage; Wide Beam; Narrow Blast; Focused Glare; Demonic Sight; Sentry.

**Sigil · active**

- **Sigil of Chaos** — Volatility Casts; Chain Reaction; Blood Ritual; Volatility Duration; Sigil of Smoke; Diabolic Rebuke; Hellfire Aura.
- **Sigil of Subversion** — Movement Speed; Blood Ritual; Cast Range; Slow; Sigil of Lava; Seekers; Infiltration Trails.
- **Sigil of Summons** — Stronger Summons; Cooldown; Blood Ritual; Damage; Summon Hellwyrm; Feeding Frenzy; Summon Succubus.

Source entry: https://github.com/DiabloTools/d4data/blob/75e619f0d90a0951945747f45feef59c076a5db0/json/base/meta/Power/Warlock_ArchDemon.pow.json

**Class mechanic entries:** Legion Shard; Summon Ae'grom; Command Ae'grom; Spawn Fragment; Sacrificial Fragment; Evisceration Fragment; Vanguard Shard; Summon Abodian; Command Abodian; Hellguard Fragment; Warden Fragment; Inferno Fragment; Mastermind Shard; Summon Laalish; Command Laalish; Unfathomable Fragment; Blasphemous Fragment; Subjugation Fragment; Ritualist Shard; Summon Valloch; Command Valloch; Abyssal Fragment; Scorching Fragment; Twisted Fragment.

**Paragon boards:** Start; Fathomless; Demonic Spicules; Dynamism; Pyrosis; Overmind; Greater Hex; Ritualism; Chaos; Dominion.

## Diablo Immortal

public builder snapshot retrieved 2026-11-25; latest skillbook row timestamp 2026-10-21T01:37:58.940669+00:00

### Wizard

**class skill loadout · active**

- **Black Hole**.
- **Frostfield**.
- **Scorch**.
- **Lightning Nova**.
- **Ice Crystal**.
- **Teleport**.
- **Ice Armor**.
- **Arcane Wind**.
- **Arcane Torrent**.
- **Slow Time**.
- **Disintegrate**.
- **Meteor**.
- **Ray of Frost**.

**class skill loadout · primary**

- **Electrocute**.
- **Magic Missile**.

**primary attack ultimate · ultimate**

- **Ice Missile**.
- **Full Power**.

Source entry: https://diablo.tv/builder

**Named legendary essence variants:** 157; see this class's `variants` records in the JSON.

### Barbarian

**class skill loadout · active**

- **Sprint**.
- **Furious Charge**.
- **Cleave**.
- **Leap**.
- **Chained Spear**.
- **Whirlwind**.
- **Sunder**.
- **Undying Rage**.
- **Hammer of the Ancients**.
- **Grab**.
- **Wrath of the Berserker**.
- **Ground Stomp**.
- **Demoralize**.

**class skill loadout · primary**

- **Frenzy**.
- **Lacerate**.

**primary attack ultimate · ultimate**

- **Restless Demon**.
- **Blood and Rage**.

Source entry: https://diablo.tv/builder

**Named legendary essence variants:** 157; see this class's `variants` records in the JSON.

### Monk

**class skill loadout · active**

- **Wave of Light**.
- **Exploding Palm**.
- **Inner Sanctuary**.
- **Mystic Strike**.
- **Wave Strike**.
- **Seven-Sided Strike**.
- **Shield of Zen**.
- **Mystic Allies**.
- **Breath of Heaven**.
- **Imprisoned Fist**.
- **Flying Dragon**.
- **Cyclone Strike**.
- **Flying Kick**.

**class skill loadout · primary**

- **Fists of Thunder**.
- **Deadly Reach**.

**primary attack ultimate · ultimate**

- **Lightning Flux**.
- **Release Force**.

Source entry: https://diablo.tv/builder

**Named legendary essence variants:** 158; see this class's `variants` records in the JSON.

### Demon Hunter

**class skill loadout · active**

- **Escape**.
- **Spinning Chakram**.
- **Shadowsight**.
- **Smoke Screen**.
- **Multishot**.
- **Rain of Vengeance**.
- **Strafe**.
- **Knockback Shot**.
- **Vengeance**.
- **Daring Swing**.
- **Knife Trap**.
- **Sentry**.
- **Impale**.

**class skill loadout · primary**

- **Crossbow Shot**.
- **Explosive Arrow**.

**primary attack ultimate · ultimate**

- **Grenades**.
- **Volley**.

Source entry: https://diablo.tv/builder

**Named legendary essence variants:** 157; see this class's `variants` records in the JSON.

### Crusader

**class skill loadout · active**

- **Consecration**.
- **Spinning Shield**.
- **Sweep Attack**.
- **Condemn**.
- **Shield Charge**.
- **Sacred Chain**.
- **Shield Glare**.
- **Fist of the Heavens**.
- **Conjuration of Light**.
- **Draw and Quarter**.
- **Judgment**.
- **Holy Banner**.
- **Falling Sword**.

**class skill loadout · primary**

- **Sacred Fire**.
- **Punish**.

**primary attack ultimate · ultimate**

- **Burning Heart**.
- **Counterattack Storm**.

Source entry: https://diablo.tv/builder

**Named legendary essence variants:** 157; see this class's `variants` records in the JSON.

### Necromancer

**class skill loadout · active**

- **Command Skeletons**.
- **Command Golem**.
- **Bone Spikes**.
- **Corpse Lance**.
- **Bone Spirits**.
- **Wraith Form**.
- **Grim Scythe**.
- **Bone Armor**.
- **Bone Wall**.
- **Borrowed Time**.
- **Corpse Explosion**.
- **Skeletal Mage**.
- **Dark Curse**.

**class skill loadout · primary**

- **Soulfire**.
- **Bone Spear**.

**primary attack ultimate · ultimate**

- **Dread Skullscythe**.
- **Hungering Soulfire**.

Source entry: https://diablo.tv/builder

**Named legendary essence variants:** 157; see this class's `variants` records in the JSON.

### Blood Knight

**class skill loadout · active**

- **Shroud of Night**.
- **Wave of Blood**.
- **Sanguinate**.
- **Mephitic Cloud**.
- **Umbral Lance**.
- **Swarm of Bats**.
- **Siphon Blood**.
- **Tendrils of Blood**.
- **Skewer**.
- **Spear Flurry**.
- **Whirling Strike**.
- **Abomination**.
- **Transfusion**.

**class skill loadout · primary**

- **Ravage**.
- **Shadow's Edge**.

**primary attack ultimate · ultimate**

- **Annihilation**.
- **Shadow's Embrace**.

**Abomination · transformed skill**

- **Blood Rush**.
- **Shattering Fists**.

Source entry: https://diablo.tv/builder

**Named legendary essence variants:** 133; see this class's `variants` records in the JSON.

### Tempest

**class skill loadout · active**

- **Vortex**.
- **Rolling Surf**.
- **Wind Walk**.
- **Stormfury**.
- **Squall**.
- **Crosswinds**.
- **Breaker**.
- **Flowing Strike**.
- **Tidal Rush**.
- **Cascade**.
- **Riptide**.
- **Blade Dance**.
- **Zephyr Surge**.

**class skill loadout · primary**

- **Wind Edge**.
- **Wave Edge**.

**class skill loadout · passive**

- **Mist-Touched**.

**primary attack ultimate · ultimate**

- **Piercing Gale**.
- **Power of the Deeps**.

Source entry: https://diablo.tv/builder

**Named legendary essence variants:** 133; see this class's `variants` records in the JSON.

### Druid

**class skill loadout · transformed skill**

- **Crush (Werebear)**.
- **Lunge (Werewolf)**.
- **Bound (Werebear)**.
- **Howl (Werewolf)**.
- **Rend and Tear (Werewolf)**.
- **Mangle (Werebear)**.

**class skill loadout · active**

- **Earthquake**.
- **Werewolf**.
- **Fire Tornado**.
- **Stag Charge**.
- **Rabid Might**.
- **Circle of Life**.
- **Summon Wolves**.
- **Summon Oak Sage**.
- **Werebear**.
- **Thorn Armor**.
- **Raven Swarm**.
- **Summon Grizzly**.
- **Surging Stone**.

**class skill loadout · primary**

- **Landslide**.
- **Ferocious Strike**.

**Ferocious Strike · ultimate**

- **Fang and Claw**.

**Landslide · ultimate**

- **Nature’s Wrath**.

**Werewolf · transformed primary**

- **Swipe**.

**Werebear · transformed primary**

- **Maul**.

**Werewolf · transformed utility**

- **Revert**.

**Werebear · transformed utility**

- **Revert**.

**class mechanic · class resource**

- **Primal Power**.

**Werebear · transformed passive**

- **Ursine Endurance**.

Source entry: https://diablo.tv/builder

**Named legendary essence variants:** 108; see this class's `variants` records in the JSON.

### Warlock

**class skill loadout · active**

- **Lash of Pain**.
- **Hellswarm**.
- **Spiteful Sacrifice**.
- **Warp Strike**.
- **Demonic Pact**.
- **Blood Offering**.
- **Burning Ascent**.
- **Brimstone Gateway**.
- **Tyrant's Grasp**.
- **Siphon Life**.
- **Devouring Darkness**.
- **Infernal Eruption**.
- **Demonic Portal**.

**class skill loadout · passive**

- **Soulgorger**.

**class skill loadout · primary**

- **Lunatic Rush**.
- **Hellfire Bolt**.

**Lunatic Rush · ultimate**

- **Lunatic Legion**.

**Hellfire Bolt · ultimate**

- **Rain of Chaos**.

**Soulgorger · pet skill**

- **Pummeling Eruption**.

**Soulgorger · pet passive**

- **Voracious Sacrifice**.

**Soulgorger · pet command**

- **Ferocious Leap**.

**Soulgorger · pet mechanic**

- **Devour**.

Source entry: https://diablo.tv/builder

**Named legendary essence variants:** 72; see this class's `variants` records in the JSON.

### Shared Paragon groups

**Survivor:** Unyielding; Stalwart; Indestructible; Escape Artist; Precognition; Covert Operations; Coordinated Assault; Mutual Defense; Superior Coordination; Find Weakness; Recuperate; Regeneration; Battlefield Efficiency; Resilient; Always Be Prepared.

**Vanquisher:** Exorcism; Zeal; Judgment; Wrath; Heart of Wrath; Swift Learner; Gold Find; Treasure Find; Lucky; Greed; Aggression; Boundless Strides; Tenacity; Combat Trance; Delaying the Inevitable.

**Gladiator:** Uncontrollable; Quick Witted; Trapper; Rapid Recovery; Cheat Death; Hold Formation; Battle Morale; First Aid; Combat Veteran; Sacrifice; Grizzled Veteran; To the Victor; Swift Strikes; Overpower; Juggernaut.

**Slinger:** Deadly Intent; Bloodthirst; Fury; Superiority; Static Discharge; Unseen Dynamism; Spell Surge; Impulsive; Gifted; Uncanny; Careful Aim; Sharp Eye; Precision; Lone Wolf; Killing Blow.

**Brawler:** Stand Firm; Close Quarters; Bolster Ranks; Extended Reach; Rapid Remedy; Chase Down; Debilitate; Insanity; Bravery; Finishing Touch; Heavy Armor; Rugged; Combat Tricks; Combat Expertise; Defender of Sanctuary.

**Harbinger:** Quickstep; Savage Urge; Illusory Strides; Spree; Fatal Urge; Chosen One; Self Reliance; Unbreakable Will; Stand Strong; Renewed Courage; Blur; Ambush; Honed Steel; Shackle; Fate.

**Slayer:** Proficiency; Vigor; Tempo; Up the Tempo; Collateral Damage; Deterioration; Cripple; Impair; Rupture; Contagion; Amplified Shields; Convergence; Unwavering; Safeguard; Hope.

**Legendary Skills:** Lightning Rod; Roaring Strike; Benefactor; Overloaded; Shock Barrier; Death's Release; Shepherd and Protector; Curse of Weakness; Soul Harvester; Frigid Blast; Glacial Shards; Searing Flame; Brewing Storms; Elusive Strives; Concentrated Agression; Overpowering Blow.

**Maledictor:** Lashed; Double-time; Fierce Cadence; Alchemical Reaction; Lashed Detonation; Nettled; Foxtail; Sandbur; Thistle Field; Bramble Patch; Vexed; Rallying Call; Battle Cry; No Quarter; No Retreat.

## Provenance and reproduction

The catalogue stores source retrieval date, immutable commit IDs where available, input hashes and per-record URLs. The complete raw download cache is deliberately external to the repository; only factual labels and structure are published.

| Source family | Pinned source / route |
| --- | --- |
| D1 / Hellfire | `diasurgical/devilutionX` commit `e1c377775a206274f73b5f01c84a967d06e74d54`, `assets/txtdata/spells/spelldat.tsv` and `mods/hf/txtdata/spells/spelldat.tsv` |
| D2 legacy | [Arreat Summit alphabetical skill index](https://classic.battle.net/diablo2exp/skills/alphaskills.shtml) |
| D2R | `blizzhackers/d2data` commit `51e7826e67515f4a84583ebaa882339aeeb99758`, `json/skills.json`, `json/skilldesc.json`, `json/localestrings-eng.json` |
| D3 | `https://us.diablo3.blizzard.com/en-us/class/{class}/{active|passive}/`; seven classes, fourteen tables |
| D4 | `DiabloTools/d4data` commit `75e619f0d90a0951945747f45feef59c076a5db0`, build `3.2.4.74043`; SkillKit, SkillTreeRewards, Power, PlayerClass, ParagonBoard/Node/Glyph and matching enUS StringList data |
| Immortal | [diablo.tv public builder](https://diablo.tv/builder), its public `skillbook`, `classes`, `paragons_with_tree` views, official Blizzard class introductions, [Wowhead skill index](https://www.wowhead.com/diablo-immortal/skills) for the first eight classes and their ultimates |

The broad D4 recursive base-tree response was truncated and was **not** used as a completeness denominator. Targeted family-tree responses are complete; every expected family file is reconciled. English StringList enumeration was complete. One case mismatch (`Druid_landslide` / `Druid_Landslide`) was resolved against that inventory, and all current selectable modifier labels resolved.

The final cross-check found D2 locale-table problems: the Plague Poppy row incorrectly resolved to Werewolf, and Blade Mastery retained its old Sword Mastery label. The catalogue preserves each original source label and applies explicit corrections supported by the [official Druid guide](https://classic.battle.net/diablo2exp/skills/druid-summoning.shtml) and [official D2R 2.4 notes](https://news.blizzard.com/en-us/article/23788293/diablo-ii-resurrected-patch-2-4-ladder-now-live). Warlock display names (Levitation Mastery, Hex/Sigil punctuation, Miasma Chain) and Polearm Mastery are reconciled with the [first-hand 3.4.95032 extraction index](https://d2r.org/data/skills). The legacy LoD index retains its historical naming. Paladin auras with additional passive components remain classified as auras, with the passive component flagged separately.

### Source licenses

The pinned [DevilutionX LICENSE.md](https://github.com/diasurgical/devilutionX/blob/e1c377775a206274f73b5f01c84a967d06e74d54/LICENSE.md) is **Sustainable Use License 1.0**, not a permissive commercial-engine license. The pinned [d2data LICENSE](https://github.com/blizzhackers/d2data/blob/51e7826e67515f4a84583ebaa882339aeeb99758/LICENSE) and [d4data LICENSE](https://github.com/DiabloTools/d4data/blob/75e619f0d90a0951945747f45feef59c076a5db0/LICENSE) contain MIT repository notices. These repository notices do not establish ownership of underlying Diablo game content. Official and community website prose/art are not imported. The JSON records license URLs and hashes, and a complete input-hash manifest for the D4 files used. Dreadmarch remains an original implementation; these sources are factual research inputs.

Immortal's source timestamp is October 21, 2026. The public skillbook was paged to a short terminal page (1,000 then 547 rows); all IDs are unique. Its unfiltered Wowhead page returned exactly 1,000 rows and lacked Druid/Warlock, so it was not accepted as a complete class or essence inventory. Official class introductions supply the twenty ultimate names and additional form/pet names together with that first-eight-class cross-check.

Run the checked-in extraction scripts against the retained raw cache to regenerate:

```bash
python scripts/research/diablo_catalog.py --raw /path/to/research-diablo
python scripts/research/diablo_atlas.py
```

`diablo_fetch_d4.py`, `diablo_fetch_d4_mechanics.py`, and `diablo_fetch_d4_paragon.py` cache the pinned D4 references. `diablo_fetch_immortal.py` exhausts the observed public database pagination. It uses the site's public client key in memory, performs reads only, and never stores credentials in the repository.

### Remaining evidence limits

- The Immortal community snapshot needs an official or lawful current client export for exact live-build equivalence. Its source completeness is verified; unknown live differences remain explicit.
- Historical pre-3.0 D4 passive trees and prior balance revisions are not represented as current. Unreferenced localized modifiers are preserved separately where they occur in the current files.
- D1/Hellfire source-port tables are a preservation source, not a claim that every unused source row can be acquired in the retail games.
- D4 glyph eligibility bit arrays are preserved as source flags; an unverified class-index mapping is not invented.
- Global equipment affixes, every follower/monster/internal triggered power and all temporary seasonal systems are outside the stated class-and-progression scope. Immortal essences and D3 runes are included because they are explicit class-skill variant systems.
