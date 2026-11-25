# Divinity atlas: a world worth inhabiting, with builds worth mastering

Research checkpoint: **25 November 2026**. This atlas supports Dreadmarch’s massive, single-player action RPG direction. **The original Divine Divinity (2002)** is the central world-design reference. Beyond Divinity, Divinity II, both Original Sin generations and Dragon Commander contribute separately identified systems. Diablo, Lost Ark and Path of Exile remain great inspirations elsewhere in the project; this document explains the Divinity contribution.

The machine-readable companion is [divinity-catalog.json](../../data/research/divinity-catalog.json). It contains factual names, version boundaries, skill paths, class affinities, counts and source receipts. It is a research catalog, not a claim that these abilities have already been implemented in Dreadmarch. Original game artwork, audio, maps, dialogue, code and full guide descriptions are not included.

## 1. The original game the project should learn from

Divine Divinity combines one directly controlled protagonist, responsive real-time combat, broad skill learning and an authored world full of mundane objects, conversations and hidden connections. Its manual treats moving objects, using objects on other objects, containers, NPC attitudes, journals and paired teleporter stones as ordinary play systems. These details are central to its appeal: a dangerous ruin matters more when it exists near a village whose people have routines, possessions and unresolved problems. [Original 2002 manual, printed pp. 25–37](https://archive.org/download/Divine_Divinity/Divine_Divinity.pdf).

Dreadmarch should therefore develop two intertwined loops. The combat loop is movement, timing, enemy intent, abilities, loot and build growth. The world loop is exploration, conversation, investigation, discovery and consequences. Every major region needs both. A large combat map alone does not establish the desired campaign experience.

### Concrete world requirements

| Requirement for Dreadmarch | What it contributes | First implementation proof |
| --- | --- | --- |
| Authored settlements with interiors, inhabitants and local tensions | Places become memorable through relationships and use | A village, an inn, an occupied home, a workshop and a contested public space |
| Persistent objects and ownership | Moving, taking or using an object has meaning | Move a crate, save, reload; it stays moved. Theft changes a witness’s response |
| Several routes through a quest | Class choice affects approach without removing the quest | Resolve one situation through combat, evidence, a social route or an environmental route |
| Discoverable secrets | Exploration rewards attention rather than exhaustive map scraping | A concealed cellar, a clue in an ordinary object, and a shortcut with a visible cause |
| Readable journal and world state | A long single-player campaign remains understandable | Track an unresolved lead, its source and the outcome of a player decision |
| Connected travel tools | Travel itself can become a useful player plan | A deliberately placed return anchor with clear limits and persistence |
| Quiet and dangerous spaces | Combat gains contrast and towns feel inhabitable | A readable transition from settlement safety to contested outskirts and a dungeon |
| Resolvable consequences | Choices matter over the course of a campaign | Revisit a location after a quest and observe a changed NPC, service or route |

These are original Dreadmarch requirements drawn from the interaction priorities in the manual; they are not promises about an already completed campaign. The original game’s paired teleporter stones are especially useful research: the manual explicitly encourages experimenting with their placement. Dreadmarch can create its own travel-anchor fiction and rules while retaining that agency. [Manual, printed pp. 36–37](https://archive.org/download/Divine_Divinity/Divine_Divinity.pdf).

## 2. Divine Divinity’s exact class and skill structure

There are three starting identities: **Warrior, Wizard and Survivor**. Their initial skills, growth characteristics and innate moves differ, while the skill Ways can be learned across classes. This is useful for Dreadmarch’s shared exploration disciplines, but it does not require dissolving the identity of every Diablo-inspired class. [Manual, pp. 14–19 and 43–45](https://archive.org/download/Divine_Divinity/Divine_Divinity.pdf); [Larian support skill FAQ](https://forums.larian.com/ubbthreads.php?Number=46366&page=all&ubb=showflat).

### The denominator is 96 tree skills, including the eight Divine skills

| Scope | Count | Explanation |
| --- | ---: | --- |
| Warrior ordinary skills | 32 | Four paths of eight |
| Wizard ordinary skills | 32 | Four paths of eight |
| Survivor ordinary skills | 24 | Three initially visible paths of eight |
| Ordinary tree subtotal | **88** | The ordinary skills described in the original manual |
| Divine path after ascension | **8** | The fourth Survivor path, unlocked later |
| Complete visible skill-tree inventory | **96** | **88 + 8**, not 96 + 8 |
| Innate starting-class special moves | **3** | Counted separately from the skill tree |
| Named player abilities in this scope | **99** | 96 tree entries + three specials |

The 2002 manual lists the ordinary paths and three special moves. The Divine path is documented by a contemporary skill reference and a later gameplay recording that demonstrates its eight actions. [Original manual](https://archive.org/download/Divine_Divinity/Divine_Divinity.pdf); [Divine path reference](https://rpgdot.rpgwatch.com/localhost/rpgdot.rpgwatch.com/httpdocs/indexa743-2.html?game=0&gid=9&hsaction=10062&infoID=97&l1=0&pageID=381); [recorded skill demonstration](https://www.youtube.com/watch?v=WTeThE42pBk).

The innate actions are **Swirl Attack** for Warrior, **Swap Location** for Wizard and **Sneaking** for Survivor. These are not extra ranks of the 96-tree inventory. The historical six class/sex presets are recorded in the JSON as research. Dreadmarch’s body, face, hair, voice and presentation choices should be independent of combat statistics. [Manual, pp. 35–37](https://archive.org/download/Divine_Divinity/Divine_Divinity.pdf); [Larian support on Swap Places](https://forums.larian.com/ubbthreads.php?Number=338068&ubb=showthreaded).

| Way / path | Eight visible skill names |
| --- | --- |
| Survivor / Path of Lore | Identify; Alchemy; Know Creature; True Sight; Blind; Skin of Poison; Curse; NecroShift |
| Survivor / Path of Talents | Trader’s Tongue; Ranger Sight; Wisdom; Charm; Magic Barrier; Aura of Command; Heaven’s Gift; Survivors Instinct |
| Survivor / Path of the Thief | Assassin’s Kiss; Lock Pick; Pickpocket; Evade Trap; Deadly Gift; Elven Stamina; Embrace Shadows; Poison Weapon |
| Survivor / Path of the Divine | Translocate; Divine Death; Spirit Form; Scorch; Heaven’s Blessing; Divine Eye; Temporal Storm; Lightning Cascade |
| Warrior / Path of the Specialist | Sword Expertise; Mace Expertise; Axe Expertise; Hammer Expertise; Spear Expertise; Bow Expertise; Crossbow Expertise; Shield Expertise |
| Warrior / Path of the Ranger | Elven Sight; True Shot; Splitting Arrows; Poisoned Arrows; Spiritual Arrows; Elemental Arrows; Evade Arrows; Explosive Arrows |
| Warrior / Path of Warrior’s Lore | Augment Damage; Repair; Stun; Augment Defense; Poison Damage; Fire Damage; Lightning Damage; Repel Damage |
| Warrior / Path of the Warrior Gods | Enchant Weapon; Feign Death; Boomerang; Shadow Warrior; Spiritual Damage; Reflect Missiles; Death Strike; Flash Attack |
| Wizard / Elemental Powers | Meteorstrike; Lightning; Freeze; Burning Wall; Elemental Bolt; Poisonous Cloud; Elemental Hail; Elemental Strike |
| Wizard / Powers of Body and Spirit | Wizard’s Sight; Bless; Restoration; Withering Curse; Aura of Guarding; Magical Might; Spellshield; Fade from Sight |
| Wizard / Powers of Matter | Hell Spikes; Energy Cage; Limbs of Lead; Telekinesis; Deathly Discs; Polymorph; Skeletal Wall; Seeking Flame |
| Wizard / Powers of Summoning | Summon Vermin; Summon Skeleton; Life Leech; Master Summoner; Banish; Resurrect; Transfer Powers; Summon Demonic Aide |

### What should remain shared

Identification, gathering, alchemy, lore, manipulation, locks, detection and conversation should form a useful adventuring layer. They should support a class’s approach to the world without forcing every player to purchase a combat build tax just to open ordinary containers. Rare secrets can still demand investment, equipment, a companion, knowledge or an alternative route.

The Divine path suggests a campaign-scale transformation or ascension system. Dreadmarch should make that system original and attach it to earned story decisions. It should not become eight copied late-game buttons on every class. The useful idea is that the campaign can change what the player is capable of doing, including traversal and interaction, as well as damage.

## 3. Beyond Divinity: modular skills and companion investment

Beyond Divinity’s official strategy guide divides development into **Survival, Warrior, Wizard and Summoning Dolls**. Teachers and books first unlock access; skill points then activate or improve it. Its skill construction can combine elemental components and geometry parameters. Summoning dolls have their own upgrades funded through the main characters’ investment. [Official strategy guide, v1.3, pp. 311–363](https://rpgwatch.com/show/infopage?gameid=175&infoid=6); [publisher manual](https://shared.fastly.steamstatic.com/store_item_assets/steam/apps/219760/manuals/Beyond_manual_english_UK.pdf).

The catalog explicitly expands every listed branch into **269 investment or variant leaves**. This is not a claim of 269 separately castable spells. The guide’s “over three hundred single and combination skills” uses a different unit; combinations of invested components are not identical to the leaves of a tree.

| Branch | Catalog leaves | Included scope |
| --- | ---: | --- |
| Survival | 33 | Thieving 4; survivor talents 6; alchemy 2; trap variants 18; trap handling 3 |
| Summoning Dolls | 11 | Level upgrade 1; resistance upgrades 7; special upgrades 3 |
| Warrior | 125 | Fifteen melee combinations × five investments; bow/crossbow investments; arrow variants; defense; whirlwind parameters; craftsmanship; talents |
| Wizard | 100 | Elemental delivery/geometry; resistance and reflection; curses; body magic; weather, nature and ethereal magic |
| Total | **269** | All leaves enumerated from the guide’s skill appendix |

For Dreadmarch, the best contribution is a **small number of understandable ability families with significant modifications**. Delivery, area, element, duration, control and resource pressure can change a build. A different icon for every tiny parameter would make both the user interface and balancing harder. Summon development should also change behavior: formation, target preference, protection, sacrifice, infusion and recall are more expressive than giving every pet only a larger health bar.

Examples of useful Beyond Divinity families are weather attacks, enemy repositioning, trap conversion, party protection, spell reflection and doll upgrades. Their named leaves remain in the JSON; Dreadmarch should create new fiction, audiovisual treatment and balance for any corresponding mechanic.

## 4. Divinity II means Ego Draconis, not Original Sin II

These are different games and must remain distinct in project discussions. **Divinity II: Ego Draconis** uses direct real-time action and a human/dragon transformation. The human skill columns are **Dragon Slayer, Mage, Priest, Ranger and Warrior**. The modern Developer’s Cut manual states that progression is classless and describes paid skill resets. [Developer’s Cut manual, pp. 17–26](https://shared.fastly.steamstatic.com/store_item_assets/steam/apps/219780/manuals/manual_en.pdf).

The original 2009 inventory is independently represented as **54 human + seven dragon = 61** names. Dragon Knight Saga / Flames of Vengeance / Developer’s Cut research includes **54 human + nine dragon = 63**, with **Wrath of the Patriarch** and **Zeppelin Barrage** tracked as late Flames of Vengeance content. Historical labels such as *Encumbrance* and the modern *Packed and Ready* are retained as a crosswalk, not treated as two different abilities in one edition. [2009 Dragon Slayer inventory](https://www.gamepressure.com/divinityiiegodraconis/dragon-slayer/zc206d); [2009 dragon inventory](https://www.gamepressure.com/divinityiiegodraconis/dragon/zd206e); [modern manual](https://shared.fastly.steamstatic.com/store_item_assets/steam/apps/219780/manuals/manual_en.pdf); [Flames of Vengeance escort discussion](https://forums.larian.com/ubbthreads.php?Number=447666&ubb=showflat).

| Human column | Entries | Useful design contribution |
| --- | ---: | --- |
| Dragon Slayer | 9 | General expertise, discovery and utility |
| Mage | 11 | Direct elemental offense, recovery and magical specialization |
| Priest | 12 | Summoning, curses, fear and defensive magic |
| Ranger | 10 | Ranged pressure, specialized arrows and spacing |
| Warrior | 12 | Charge, burst, durability and sustained melee |

The transformation is a useful ambition for original capstones, but it changes camera, traversal, animation, targeting, environment scale and encounter design together. It should be developed as a complete playable feature. A dragon-shaped model with unchanged combat rules would not deliver the same experience. Mindreading also suggests an original information mechanic: discovering an optional route or hidden motive can be a meaningful reward in its own right.

## 5. Original Sin: interactions, skillbooks and hybrid disciplines

### Classic and Enhanced Edition are separate catalogs

Original Sin is turn-based and party-based; the relevant inheritance for Dreadmarch is its interaction vocabulary and build freedom. It is not a prescription to switch the project to turn-based combat. The Enhanced Edition manual explicitly describes classless character development, while its changes include new weapon and skill systems. [Enhanced Edition manual](https://shared.fastly.steamstatic.com/store_item_assets/steam/apps/373420/manuals/DOSEE_Manual.pdf); [Larian’s Enhanced Edition changelist](https://forums.larian.com/ubbthreads.php?Number=572102&page=all&ubb=showflat).

Both catalogs retain **Aerotheurge, Geomancer, Hydrosophist, Pyrokinetic, Witchcraft, Man-at-Arms, Expert Marksman and Scoundrel**. The specific skills differ, so Classic names are not silently substituted into Enhanced Edition trees. The inventory cross-check uses pinned structured data and an independent EE planner that identifies itself as v2.0.119.430. [Pinned planner source](https://github.com/amireh/divinity_planner/tree/b12ed5e0eeaa81bb34c6c11508e55da88cc7fa04/ui/shared/database); [independent EE planner](https://github.com/Chrest/chrest.github.io/tree/dac500dc5074b340b2f8a447a5ecbaa025802f01).

| Edition | Learned/special skill names | Other separately typed entries | Total catalog rows |
| --- | ---: | --- | ---: |
| Original Sin Classic | 142 | 45 talents, 28 ability investments, 18 dialogue traits | 233 |
| Original Sin Enhanced Edition | 132 | 50 talents, 30 ability investments, 18 traits, 17 throwable actions | 247 |

The EE skill count comprises **128 school skills, three special skills and Summon Zombie Nick**. The last is a quest reward absent from the two general school planners. Likewise **Tenebrium** is a quest-unlocked ability missing from the creation planner. These are concrete reasons why a planner alone cannot prove whole-game completeness. Larian’s changelist identifies Zombie Nick, and Larian Support confirms the EE Tenebrium ability. [EE changelist](https://forums.larian.com/ubbthreads.php?Number=572102&page=all&ubb=showflat); [Tenebrium support explanation](https://forums.larian.com/ubbthreads.php?Number=576544&ubb=showflat).

Dreadmarch can combine the useful ideas into a coherent real-time system: elemental surfaces affect ability outcomes; utility skills function outside combat; knowledge and training open new routes; a limited set of dialogue traits can express the character’s decisions. Surface interactions must be readable at action-game speed and remain deterministic in save/replay validation. They need budgets for particles, damage ticks, chain reactions and persistent state.

### Original Sin II baseline, special actions and optional content

The second Original Sin game has ten combat schools: **Aerotheurge, Geomancer, Hydrosophist, Huntsman, Necromancer, Polymorph, Pyrokinetic, Scoundrel, Summoning and Warfare**. Its learning, memory, Source and origin/racial systems are separately represented. The catalog also includes weapon actions, quest rewards, transformation child actions, summoned-creature actions, talents and combat/civil investments. [Publisher manual](https://cdn.akamai.steamstatic.com/steam/apps/435150/manuals/DOS2_Game_Manual.pdf); [skill taxonomy](https://divinity.fandom.com/wiki/Original_Sin_2_Skills).

The inventory retains Classic and Definitive rows separately: **309** Classic entries and **333** Definitive entries in the current enumerated scope. These totals include passives and investment categories; they must not be advertised as hundreds of unique active spells. Definitive-only optional features have explicit content gates. The eight Divine Talents, four extra infusion names and named Pet Power actions are not folded into the Classic baseline. [Larian gift-bag announcements](https://steamcommunity.com/app/435150/allnews/?l=english); [talent catalog](https://divinity.fandom.com/wiki/Original_Sin_2_Talents).

#### All 48 base crafted hybrid identities are reconciled

Each of the four elemental schools pairs with six other schools. Every pair contributes an ordinary skill and a Source variant: **24 pairs × two = 48**. The JSON assigns the same identity to its correct pair rather than duplicating it across two class lists. [Crafted skill taxonomy](https://divinity.fandom.com/wiki/Original_Sin_2_Skills).

| Schools | Ordinary hybrid | Source hybrid |
| --- | --- | --- |
| Aerotheurge + Huntsman | Erratic Wisp | Evasive Aura |
| Aerotheurge + Necromancer | Vacuum Touch | Vacuum Aura |
| Aerotheurge + Polymorph | Vaporize | Jellyfish Skin |
| Aerotheurge + Scoundrel | Smoke Cover | Blessed Smoke Cloud |
| Aerotheurge + Summoning | Electric Infusion | Cursed Electric Infusion |
| Aerotheurge + Warfare | Breathing Bubble | Mass Breathing Bubbles |
| Geomancer + Huntsman | Throw Dust | Dust Blast |
| Geomancer + Necromancer | Corrosive Touch | Corrosive Spray |
| Geomancer + Polymorph | Turn to Oil | Poisonous Skin |
| Geomancer + Scoundrel | Venom Coating | Venomous Aura |
| Geomancer + Summoning | Poison Infusion | Acid Infusion |
| Geomancer + Warfare | Oily Carapace | Mass Oily Carapace |
| Hydrosophist + Huntsman | Cryotherapy | Mass Cryotherapy |
| Hydrosophist + Necromancer | Raining Blood | Blood Storm |
| Hydrosophist + Polymorph | Healing Tears | Icy Skin |
| Hydrosophist + Scoundrel | Vampiric Hunger | Vampiric Hunger Aura |
| Hydrosophist + Summoning | Water Infusion | Ice Infusion |
| Hydrosophist + Warfare | Cleanse Wounds | Mass Cleanse Wounds |
| Pyrokinetic + Huntsman | Throw Explosive Trap | Deploy Mass Traps |
| Pyrokinetic + Necromancer | Corpse Explosion | Mass Corpse Explosion |
| Pyrokinetic + Polymorph | Bleed Fire | Flaming Skin |
| Pyrokinetic + Scoundrel | Sabotage | Mass Sabotage |
| Pyrokinetic + Summoning | Fire Infusion | Necrofire Infusion |
| Pyrokinetic + Warfare | Sparking Swings | Master of Sparks |

A few data-source defects were deliberately rejected. One structured catalog spells Mosquito Swarm as “Mosquito Storm” and misassigns some hybrid requirements; those requirements are not used as authoritative balance data. A second repository’s README still describes a 51-skill hybrid lookup even though its pinned live YAML now contains 206 records. The actual records were inspected and compared; the stale README number is not used as a denominator. [First structured source](https://github.com/Tsardines/divinity-2-skills/blob/f5c2e5a1aca5524d779c38f8cd6abe8322472950/divinity-skills/public/skills.json); [independent structured source](https://github.com/jguluarte/dos2-skills/blob/1d626f1a686e878a19a175fa683d087fd9bbba39/src/data/skills.yaml).

## 6. Original class synthesis for Dreadmarch

The following is an **original design proposal**, not a transcription of an existing game. It maps Divinity’s mechanics into all 19 researched Diablo class identities while retaining the distinctions between Warrior/Barbarian, Sorcerer/Wizard and Paladin/Crusader. Existing franchise spell names above are research labels; production abilities should be independently authored.

A workable structure is a clear class core, a shared adventuring layer, a bounded secondary discipline and an earned ascension. A secondary discipline should enable a new interaction without replacing another class’s entire resource loop. Utility should remain useful outside combat. Every class must be able to complete the full single-player campaign.

| Class identity | Divinity contribution that fits | Original Dreadmarch direction | Boundary that preserves identity |
| --- | --- | --- | --- |
| Warrior | Weapon expertise, defensive specialties, repair, charge | Arms, Guard and Fieldcraft; adapt equipment and position during combat | Tactical fundamentals remain distinct from Barbarian rage |
| Rogue | Thief path, backstab, invisibility, manipulation, ranged precision | Marksman, Shadow and Guile; exploit perception and prepared openings | General guile differs from Assassin martial/trap specialization |
| Sorcerer | Elemental Powers, missile/spread/circle modulation | Flame, Frost and Lightning; strong elemental commitment and combination payoffs | Elemental identity remains clearer than Wizard’s broader manipulation |
| Monk | Unarmed expertise, movement, cleansing and protective auras | Forms, Mantras and Spirit; short deliberate sequences with relocation | Keep direct bodily rhythm rather than turning it into a robed spellcaster |
| Bard | Identification, lore, charm, leadership and twin-weapon play | Twin Blades, Lore and original Resonance; information and timing support solo combat | Resonance is an original extension, not an asserted Hellfire feature |
| Barbarian | Swirl/Whirlwind parameters, rage, intimidation and earth disruption | Berserking, Warcries and Earthshaker; momentum changes nearby terrain and enemies | Reward commitment and impact rather than precision utility |
| Amazon | Bow/crossbow expertise, arrow elements, spear proficiency, ranger spacing | Bowcraft, Stormspear and Battle Grace; deliberate switching between distance and reach | Preserve the spear/javelin identity beyond a generic archer |
| Assassin | Trap conversion, shadow movement, backstab and disabling effects | Martial Arts, Shadow and Traps; prepare a route, isolate a threat, finish decisively | Powerful preparation requires readable placement and setup costs |
| Necromancer | Skeletons, resurrection, corpse skills, blood and transfer of summon properties | Bone, Blood and Death Legion; bodies and retained remains become resources | Permanent armies, blood sacrifice and bone shaping each need distinct limits |
| Paladin | Blessing, cleansing, protective fields, spiritual damage and auras | Zeal, Auras and Judgment; oath choices change protection and retaliation | Moral/oath and aura identity differs from Crusader siege defense |
| Druid | Nature/weather magic, polymorph, animal summons and terrain | Wildshape, Primal Elements and Wildbond; form changes alter combat and exploration | Forms must change controls and capabilities coherently |
| Crusader | Shield expertise, reflection, party protection and ground control | Shieldwall, Consecration and Crusade; mobile fortification and counterattack | Shield-led battlefield occupation differs from Paladin’s oath support |
| Demon Hunter | Trap families, specialized arrows, target knowledge and range control | Arsenal, Sentry and Vengeance; prepare kill zones and relocate through them | Precision weapons and devices stay central |
| Witch Doctor | Curse, poison skin, vermin, insects, doll investment and spirit control | Hexes, Plague and Spirit Court; ritual setup produces spreading, directed effects | Deliberate rituals and spirit relationships differ from Necromancer corpse economy |
| Wizard | Telekinesis, swap/teleport, matter shaping, temporal effects and hybrid spellcraft | Arcane, Temporal and Convergence; manipulate the rules of an encounter | Higher system breadth requires stronger setup and resource constraints |
| Blood Knight | Life Tap/Leech, blood surfaces, nocturnal concealment and polearm reach | Bloodcraft, Polearm and Nightform; convert health pressure into controlled aggression | Blood debt must have visible risk and recovery decisions |
| Tempest | Wind, water, movement, vapor and short-lived protective effects | Windblade, Tides and Zephyrs; rapid repositioning changes the next strike | Agile elemental melee differs from a distant storm Sorcerer |
| Spiritborn | Animal bonds, nature control, swarm effects and temporary spirit forms | Predator, Guardian and Swarm; choose a spirit’s behavior and tempo | Spirit contracts need identities beyond cosmetic animal effects |
| Warlock | Demonic summons, curses, life exchange and destructive magic | Demon Pacts, Hellfire and Abyss; binding a power creates a useful constraint | Bargains and controlled demons differ from generic fire magic or corpse armies |

### Five concrete fusion examples

**Paladin: a cleansing ward.** A shielded advance leaves a short protected path; allies or the solo hero entering it shed one eligible debuff. A secondary water discipline changes the ward’s interaction with burning ground. The class still spends and earns its own resource, and the protection does not grant universal immunity.

**Witch Doctor: an inhabited effigy.** Place a destructible vessel that stores a limited portion of nearby curse activity. Release its stored effect as a directed swarm, or keep it alive to anchor a spirit. This combines doll investment, insect effects and curses into one decision rather than three redundant damage buttons.

**Wizard: a matter exchange.** Exchange two eligible positions and briefly preserve their previous surface state. The ability has explicit target, mass and distance rules. It should work on approved puzzle objects as well as enemies, while preventing inaccessible quest-state breaks.

**Necromancer: an inherited command.** A chosen undead inherits one compatible property from its summoner; sacrificing it transfers a short version of that property to a new summon. This gives summon composition a cycle and a cost. It does not automatically copy the player’s entire passive tree.

**Amazon: a prepared volley.** Mark a lane, choose an arrow treatment and commit to a volley that rewards enemy movement through that lane. A nature discipline can alter the lane’s ground interaction; a precision discipline changes projectile behavior. The same authored ability family supports meaningfully different builds.

These examples should be prototyped through one reusable ability representation: targeting, delivery, movement, effect application, resource changes, conditions, duration, surface rules and world interaction. The effect library belongs in the reusable engine; class names, story gates, audiovisual packages and tuning belong in Dreadmarch’s game data. This is an architectural proposal, not an instruction to import another game’s code.

## 7. Dragon Commander remains in scope

Dragon Commander is a separate systems reference with strategy-map decisions, real-time armies and direct dragon control. The official manual lists **31 dragon skills** and the official unit page supplies **13 units with three abilities each: 39 unit-bound entries**. Both inventories are included, totaling **70** rows. Shared names such as Cloak appear in more than one unit binding and are counted as bindings, not newly invented spells. [Official manual, pp. 38–42](https://shared.akamai.steamstatic.com/store_item_assets/steam/apps/243950/manuals/Manual.pdf); [official units](https://www.divinitydragoncommander.com/unitsoverview.php); [official systems overview](https://www.divinitydragoncommander.com/featuresoverview.php).

Its useful ARPG contributions are readable ally commands, support versus destruction loadouts and campaign decisions with consequences. An RTS or army feature is a separate future product/design decision. It should not displace the single-player character-led campaign requested here.

## 8. A real original Divine Divinity reimplementation exists

**OpenDivine is a positive finding.** It identifies itself as a Go engine reimplementation of the 2002 game and uses original game content. The inspected source checkpoint supports world rendering, animated characters and props, walking/collision, camera movement, doors/chests and music. Its own status file still lists combat, monsters, inventory, skills, dialogue, quests and saving among missing work. It is therefore useful engine/format research, not a completed game foundation that already supplies the requested campaign. [OpenDivine repository](https://github.com/vvoland/opendivine); [pinned status](https://github.com/vvoland/opendivine/blob/6865545ca3fe5c1f3e3876f088cc2392e1d5bbf2/STATUS.md).

| Project | Actual category | Language / declared license | Relevance |
| --- | --- | --- | --- |
| [OpenDivine](https://github.com/vvoland/opendivine) | Partial original-game engine reimplementation | Go/Ebiten; GPL-3.0 | Rendering, data layouts and original-world research; missing the full gameplay loop |
| [Divine Tools](https://github.com/fstxz/divine_tools) | Original-game binary/world/font editing tools | **Rust**; GPL-3.0 | A directly relevant Rust format-tooling reference; not a game runtime |
| [divproxy](https://github.com/atom0s/divproxy) | Enhancement hook inside the original client | C++/Lua; AGPL-3.0 | Runtime investigation and Lua/ImGui extensions; requires the original executable |
| [ddcmpa](https://github.com/sormy/ddcmpa) | CMP archive tool | Python; MIT | File extraction; its “reimplementation” claim concerns an older archive tool |
| [OsirisRE](https://github.com/usernameak/OsirisRE/tree/3ea250a83b45e0d6d25e0824a0f8cef6e558a0fa) | Original Osiris VM research and compiler wrapper | C++; no license recognized in the inspected repository metadata | Uses the original OsirisDLL; script recompilation is not a full engine recompilation |
| [DivineDivinity StoryEditor](https://github.com/Raan/DivineDivinity_StoryEditor/tree/bf761eac3a755fc9796b4591a1e11e4e5977bb92) | Story decompiler/editor and original-DLL build workflow | C#; MIT | Reads original Story binaries and supports editable rules; requires original OsirisDLL and base Story |
| [Beyond Divinity Translation Tool](https://github.com/dortkoldantaciz/Beyond-Divinity-Translation-Tool) | Text extraction/translation/repacking | Python; no license assumption | Documents original localization locations and limitations |
| [D2DCTool](https://github.com/kilazz/D2DCTool/tree/f59c297a2c34e951dd2055f15031ca5704ced30f) | Divinity II DV2 archive and binary-XML tools | Rust/Slint; no license recognized in repository metadata | Another Rust tooling reference, for the later Divinity II rather than the 2002 game |
| [dv2-mod](https://github.com/ygalsk/dv2-mod/tree/9cef4c4824916b7aafd481181070522ba999be8f) | Divinity II data, world and quest editor | Python; MIT | Documents concrete skill, trainer and item-grant tables; requires the original game |
| [Norbyte LSLib](https://github.com/Norbyte/lslib) | Resource conversion/extraction | Project-specific terms | Original Sin-era data tooling; older Osiris support must not be assumed |
| [DOS2 Script Extender](https://github.com/Norbyte/ositools) | Extension to Original Sin II | Project-specific terms | Useful data inspection, unrelated to a replacement 2002 engine |

The primary repositories and author documentation were inspected. No runtime of these tools or the original games was executed in this research pass. Public searches covered original-game reimplementation/source-port terminology, Rust candidates, GitHub, GitLab and Larian discussions. **A complete original-DD automated binary recompilation or a complete Rust replacement engine was not verified in that scope.** That statement is a search boundary, not a claim that no unpublished or unindexed project exists.

Two technical distinctions matter. OpenDivine’s reconstructed internal class graph is not the same as the released UI taxonomy, and some internal names differ considerably. Its skill-cost notes also contain an interpretation that has not been validated against an installed game here. The catalog uses the original manual and visible skill references for the player inventory, keeping internal symbols out of the playable-name denominator. [Pinned internal-tree notes](https://github.com/vvoland/opendivine/blob/6865545ca3fe5c1f3e3876f088cc2392e1d5bbf2/re_docs/skill-tree.md); [pinned table-format notes](https://github.com/vvoland/opendivine/blob/6865545ca3fe5c1f3e3876f088cc2392e1d5bbf2/re_docs/formats/skillcosts.md).

## 9. Completeness, corrections and the remaining exact-data gate

The current catalog contains **1,684 versioned records**. This number includes repeated identities in different editions, parameter investments, talents, dialogue traits, item actions and unit bindings. It is useful for coverage and comparison; it is not a count of distinct spells to implement in Dreadmarch.

| Versioned catalog | Included | Count boundary |
| --- | ---: | --- |
| Divine Divinity — Original 2002; visible released-game skills | 99 | 96 tree skill, 3 class special |
| Beyond Divinity — PC; official guide 1.3 taxonomy | 269 | 7 skill, 6 passive or utility, 2 crafting skill, 18 trap variant, 11 companion investment, 170 investment node, 12 passive arrow variant, 4 passive, 4 utility skill, 12 crafting variant, 5 active or passive, 9 spell variant, 9 spell |
| Divinity II: Ego Draconis — Original 2009 named inventory | 61 | 7 dragon skill, 54 human skill |
| Divinity II — Dragon Knight Saga / Developer’s Cut manual; Ego Draconis + Flames of Vengeance | 63 | 29 active skill, 25 passive skill, 9 dragon skill |
| Divinity: Original Sin — Classic 2014 | 233 | 142 school skill, 45 talent, 28 ability investment, 18 dialogue trait |
| Divinity: Original Sin — Enhanced Edition 2015 | 247 | 128 school skill, 3 special skill, 50 talent, 29 ability investment, 1 quest ability investment, 1 quest skill, 18 dialogue trait, 17 consumable action |
| Divinity: Original Sin II — Classic 2017: named baseline with per-patch values pending | 309 | 201 school or quest skill, 7 origin skill, 5 racial skill, 5 weapon skill, 1 school skill, 1 quest skill, 4 story skill, 1 item skill, 4 granted child action, 10 summon action, 34 talent, 3 quest talent, 9 racial talent, 24 ability investment |
| Divinity: Original Sin II — Definitive Edition; optional gift bags explicitly gated | 333 | 201 school or quest skill, 7 origin skill, 5 racial skill, 5 weapon skill, 1 school skill, 1 quest skill, 4 story skill, 1 item skill, 4 granted child action, 10 summon action, 34 talent, 3 quest talent, 9 racial talent, 24 ability investment, 4 gift bag skill, 8 gift bag summon action, 3 gift bag summon variant, 8 gift bag talent, 1 gift bag item skill |
| Divinity: Dragon Commander — 2013; official website and manual | 70 | 8 dragon passive, 23 dragon active, 39 unit ability |

Within the named scopes above, all discovered entries were included. **Whole-game internal-data completeness remains unproven**, particularly for every consumable/unique-item grant, NPC template, unused/internal definition and exact patched value. The catalog explicitly stores unknown denominators as null. It does not turn an unknown count into “zero missing.” Two named candidates, *Ghost Tree* and *Accelerating Leap*, remain separated from verified player counts because their exact released grant routes are not established by the inspected sources.

The remaining evidence is specific. For DOS2 Classic and Definitive, use the corresponding edition’s complete `.pak` layers and English localization, not just one unpatched Shared archive. Skill data, statuses, weapon/armor/object/potion records, recipes, root templates and placed instances must be resolved together. This distinguishes a skill definition from a skill the player can actually obtain. Larian’s own editor documentation places skills in SkillData documents; the inspected DOS2 Forge extractor resolves inherited stat entries and localization. [Larian skill creation documentation](https://docs.larian.game/Part_1%3A_creating_a_new_skill); [DOS2 Forge pinned source](https://github.com/crazyace/dos2-forge/tree/8826e1cbc7c4dbeac6c3c6bfb6b659a9aab1e743).

The repository includes [divinity_reconcile.py](../../scripts/research/divinity_reconcile.py) to validate this catalog and compare a local DOS2 Forge export without importing commercial descriptions into it. A reconciliation report is evidence, not automatic permission to mark every unmatched NPC or template as a missing player ability.

```sh
python scripts/research/divinity_reconcile.py validate
python scripts/research/divinity_reconcile.py requirements
```

With an installed, owned copy and the pinned extractor reviewed in a separate environment, the documented export route is:

```sh
dos2forge --data-dir "/path/to/Divinity Original Sin 2/DefEd/Data" export all -o /tmp/dos2-de-reference
dos2forge --data-dir "/path/to/Divinity Original Sin 2/DefEd/Data" templates -o /tmp/dos2-de-reference/templates.json
dos2forge --data-dir "/path/to/Divinity Original Sin 2/DefEd/Data" instances -o /tmp/dos2-de-reference/instances.json
dos2forge --data-dir "/path/to/Divinity Original Sin 2/DefEd/Data" recipes -o /tmp/dos2-de-reference/recipes.json
python scripts/research/divinity_reconcile.py reconcile --game-id original-sin-ii-definitive --export-dir /tmp/dos2-de-reference --output /tmp/dos2-de-reconciliation.json
```

For Classic, point at that edition’s `Data` directory and use `original-sin-ii-classic`. The command syntax is verified against the pinned tool source; it was not run against commercial game files in this workspace. Preserve hashes, build version, localization and optional-content state with any export.

The exact file families and reasons are also stored under `original_data_requirements` in the JSON. For original Divine Divinity, the researched rank table is `dat/wgiaa.000`; the similarly named development import `dat/skillcosts.txt` is described as unshipped. For Beyond Divinity, the translation-tool author documents `Localizations/text.cmp`, act-local localization, `Common/strings.txt` and initial-save templates; these are useful paths for a future installed-data reconciliation. They are not additional files that have already been obtained here.

For Divinity II: Developer’s Cut, the author of `dv2-mod` documents the concrete archive-entry paths `Episodes/Episode_1_Extended/rpgstats_skillprototypes.xml` and `Episodes/Episode_2/rpgstats_skillprototypes.xml`, plus `rpgstats_skilltrainerlists.xml`, `rpgstats_itemprototypes.xml`, `rpgstats_values.xml` and language tables. The matching `.dv2` corpus lives under `Data/Win32/Packed/`; its archive priorities matter. The author reports **76 internal skill definitions in episode one and 78 in episode two**. This is distinct from the **63 named player skills** in the manual. Until the IDs and grant relationships are inspected, subtracting the two counts does not establish 15 missing playable skills. The exact original-2009 internal layout must still be checked independently. [Pinned data-model research](https://github.com/ygalsk/dv2-mod/blob/9cef4c4824916b7aafd481181070522ba999be8f/docs/reference/data-model.md); [pinned archive resolution](https://github.com/ygalsk/dv2-mod/blob/9cef4c4824916b7aafd481181070522ba999be8f/docs/reference/archives.md).

## 10. What implementation should prove next

The next gameplay slice should connect one original class to a small authored place. Character creation must produce a valid character; class choice must change meaningful abilities; combat must interact with the world; a quest must react to an outcome; and a save/reload must preserve it. That proves the relationship between the reusable engine and the game.

A strong first proving ground has a settlement edge, a nearby cellar/dungeon, one local dispute, a secret route, a small enemy roster, a meaningful item choice and a boss that tests movement and class identity. The content should establish atmosphere, readability, responsive controls and persistent consequences before scaling the same production path to more regions and classes. The long-term ambition remains a massive campaign and exceptional presentation; the milestone should make that ambition concrete and testable.
