"""Authored design synthesis and source register for arpg_catalog.py.

This file records proposals, not implemented gameplay or upstream data facts.
Source labels distinguish official explanations from community extracted tables.
"""

SOURCES = [
    ("la-current", "Lost Ark", "official_release", "2026-11-18", "Echoes of Thunder", "https://www.playlostark.com/en-us/game/releases/echoes-of-thunder"),
    ("la-dimensionalist", "Lost Ark", "official_release", "2026-09-16", "Dimensions Unbound / Dimensionalist", "https://www.playlostark.com/en-us/game/releases/dimensions-unbound"),
    ("la-guardianknight", "Lost Ark", "official_release", "2026-02-04", "Guardian's Rage / Guardianknight", "https://www.playlostark.com/en-us/game/releases/guardians-rage"),
    ("la-guardianknight-design", "Lost Ark", "official_developer_article", "2026", "Guardianknight design", "https://www.playlostark.com/en-us/news/articles/guardianknight-class-behind-the-scenes"),
    ("la-valkyrie", "Lost Ark", "official_release", "2025", "Rise of the Valkyrie", "https://www.playlostark.com/en-gb/game/releases/rise-of-the-valkyrie"),
    ("la-wildsoul", "Lost Ark", "official_release", "2025", "Call of the Wildsoul / Ark Passive overhaul", "https://www.playlostark.com/en-us/game/releases/call-of-the-wildsoul"),
    ("la-passive", "Lost Ark", "official_release", "2024; historical introduction", "Arkesia Ignited / Evolution, Enlightenment, Leap", "https://www.playlostark.com/en-us/game/releases/arkesia-ignited"),
    ("la-grid", "Lost Ark", "official_release", "2025-11-19", "Day of Prophecy / Ark Grid", "https://www.playlostark.com/en-us/game/releases/day-of-prophecy"),
    ("la-grid-explainer", "Lost Ark", "official_roadmap", "2025", "Order and Chaos cores / Astrogems", "https://www.playlostark.com/en-us/news/articles/2025-roadmap-part-4"),
    ("la-tripod", "Lost Ark", "official_overview", "observed 2026-11-25", "Tripod customization", "https://www.playlostark.com/en-gb/game/about"),
    ("la-warrior", "Lost Ark", "official_class_article", "2021; historical mechanics", "Warrior advanced classes", "https://www.playlostark.com/en-us/news/articles/warrior-advanced-classes"),
    ("la-mage", "Lost Ark", "official_class_article", "2021; historical mechanics", "Mage advanced classes", "https://www.playlostark.com/en-us/news/articles/mage-advanced-classes"),
    ("la-assassin", "Lost Ark", "official_class_article", "2021; historical mechanics", "Assassin advanced classes", "https://www.playlostark.com/en-us/news/articles/assassin-advanced-classes"),
    ("la-martial", "Lost Ark", "official_class_article", "2021; historical mechanics", "Martial artist advanced classes", "https://www.playlostark.com/en-us/news/articles/martial-artist-advanced-classes"),
    ("la-gunner", "Lost Ark", "official_class_article", "2021; historical mechanics", "Gunner advanced classes", "https://www.playlostark.com/en-us/news/articles/gunner-advanced-classes"),
    ("la-arcanist", "Lost Ark", "official_class_article", "2022; historical mechanics", "Arcanist Academy", "https://www.playlostark.com/en-us/news/articles/arcanist-academy"),
    ("la-summoner", "Lost Ark", "official_class_article", "2022; historical mechanics", "Summoner Academy", "https://www.playlostark.com/en-us/news/articles/summoner-academy"),
    ("la-reaper", "Lost Ark", "official_class_article", "2022; historical mechanics", "Reaper Academy", "https://www.playlostark.com/en-us/news/articles/reaper-academy"),
    ("la-souleater", "Lost Ark", "official_class_article", "2023; historical mechanics", "Souleater Academy", "https://www.playlostark.com/en-us/news/articles/souleater-academy"),
    ("la-breaker", "Lost Ark", "official_class_article", "2024; historical mechanics", "Breaker Breakdown", "https://www.playlostark.com/en-us/news/articles/breaker-breakdown"),
    ("la-glaivier", "Lost Ark", "official_class_article", "2022; historical mechanics", "Glaivier Academy", "https://www.playlostark.com/en-gb/news/articles/lost-ark-academy-glaivier"),
    ("la-destroyer", "Lost Ark", "official_class_article", "2022; historical mechanics", "Destroyer Academy", "https://www.playlostark.com/en-gb/news/articles/lost-ark-academy-destroyer-class"),
    ("la-slayer", "Lost Ark", "official_class_article", "2023; historical mechanics", "Slayer Academy", "https://www.playlostark.com/en-us/news/articles/lost-ark-academy-slayer"),
    ("la-machinist", "Lost Ark", "official_class_article", "2022; historical mechanics", "Machinist Academy", "https://www.playlostark.com/en-us/news/articles/lost-ark-academy-machinist"),
    ("la-artist", "Lost Ark", "official_class_article", "2023; historical mechanics", "Artist Academy", "https://www.playlostark.com/en-us/news/articles/lost-ark-academy-artist"),
    ("la-aeromancer", "Lost Ark", "official_class_article", "2023; historical mechanics", "Aeromancer Academy", "https://www.playlostark.com/en-us/news/articles/lost-ark-academy-aeromancer"),
    ("la-api", "Lost Ark", "official_api_documentation", "observed 2026-11-25; Korean service", "Open API / character skill and Ark Passive endpoints", "https://developer-lostark.game.onstove.com/changelog"),
    ("la-data", "Lost Ark", "community_extracted_data", "pinned in source_receipts", "loa-logs meter data", "https://github.com/snoww/loa-logs"),
    ("poe1-overview", "Path of Exile", "official_overview", "observed 2026-11-25", "Gems, supports, passive tree, keystones and items", "https://www.pathofexile.com/game"),
    ("poe1-tree", "Path of Exile", "official_data_export", "3.29.1 stable; 3.30.0 preview separate", "GGG passive-tree export", "https://github.com/grindinggear/skilltree-export"),
    ("poe1-gems", "Path of Exile", "community_generated_data", "Path of Building 2.69.0", "Path of Building generated gem table", "https://github.com/PathOfBuildingCommunity/PathOfBuilding"),
    ("poe2-overview", "Path of Exile 2", "official_overview", "observed 2026-11-25", "Skill/support gems, endgame and equipment", "https://pathofexile2.com/"),
    ("poe2-support-change", "Path of Exile 2", "official_expansion_announcement", "The Third Edict; historical change", "Support tiers and repeated support copies", "https://pathofexile2.com/es/edict"),
    ("poe2-tree", "Path of Exile 2", "official_data_export", "0.5.5 published export", "GGG passive-tree export", "https://github.com/grindinggear/poe2-skilltree-export"),
    ("poe2-gems", "Path of Exile 2", "community_generated_data", "pinned in source_receipts", "Path of Building 2 generated gem table", "https://github.com/PathOfBuildingCommunity/PathOfBuilding-PoE2"),
]

# Mechanics are concise original summaries. Historical articles establish the
# design pattern; current tables establish observed identifiers, not balance.
LOST_ARK_IDENTITIES = [
    ("Berserker", "Warrior", "Build fury and commit to a greatsword burst state.", "la-warrior", ["Whirlwind", "Bloody Rush", "Overdrive"]),
    ("Destroyer", "Warrior", "Generate gravity cores, spend them on heavy releases, then enter a gravity state.", "la-destroyer", ["Perfect Swing", "Seismic Hammer", "Vortex Gravity"]),
    ("Gunlancer", "Warrior", "Manage a defensive stance, shield counters and deliberate lance attacks.", "la-warrior", ["Counter Gunlance", "Surge Cannon", "Charged Stinger"]),
    ("Paladin", "Warrior", "Combine sword techniques and holy magic with an identity-state decision.", "la-warrior", ["Holy Sword", "Holy Aura", "Sacred Executioner"]),
    ("Slayer", "Warrior", "Build fury into an aggressive greatsword burst, with alternative sustain and finisher paths.", "la-slayer", ["Bloodlust", "Brutal Impact", "Guillotine"]),
    ("Valkyrie", "Warrior", "Channel light through sword combat, with offensive and protective specialization.", "la-valkyrie", ["Seraphic Oath", "Shining Knight", "Judgment Stigmata"]),
    ("Arcanist", "Mage", "Apply stacks, consume them with ruin attacks and react to card effects.", "la-arcanist", ["Celestial Rain", "Secret Garden", "Return"]),
    ("Summoner", "Mage", "Build ancient energy through attacks and companions to command larger elemental summons.", "la-summoner", ["Ancient Spear", "Maririn", "Akir"]),
    ("Bard", "Mage", "Use musical attacks and protection, then choose an offensive or restorative serenade.", "la-mage", ["Serenade of Courage", "Serenade of Salvation", "Soundholic"]),
    ("Sorceress", "Mage", "Combine elemental casting with an arcane release or mobility decision.", "la-data", ["Doomsday", "Arcane Rupture", "Blink"]),
    ("Wardancer", "Martial Artist", "Turn fast martial sequences into elemental finishers.", "la-martial", ["Wind's Whisper", "Sweeping Kick", "Esoteric Skill: Blast Formation"]),
    ("Scrapper", "Martial Artist", "Alternate two interdependent energy pools for heavy gauntlet attacks.", "la-martial", ["Death Rattle", "Supernova", "Iron Cannon Blow"]),
    ("Soulfist", "Martial Artist", "Weave ranged energy with martial attacks and staged power escalation.", "la-martial", ["Hype", "World Decimation", "Energy Blast"]),
    ("Glaivier", "Martial Artist", "Build momentum in one spear stance and spend it by switching to another.", "la-glaivier", ["Red Dragon's Horn", "Raging Dragon Slash", "Dragonscale Defense"]),
    ("Striker", "Martial Artist", "Build elemental power through mobile combos for committed esoteric attacks.", "la-martial", ["Lightning Whisper", "Esoteric Skill: Lightning Tiger Strike", "Esoteric Skill: Tiger Emerges"]),
    ("Breaker", "Martial Artist", "Alternate stamina and shock while filling a separate stance resource.", "la-breaker", ["Brawl King Stance", "Asura Destruction", "Celestial Force Barrage"]),
    ("Deathblade", "Assassin", "Build a sword-combo identity state and release a decisive surge.", "la-assassin", ["Deathblade Surge", "Soul Absorber", "Blitz Rush"]),
    ("Shadowhunter", "Assassin", "Build toward demonic transformation with a distinct transformed action set.", "la-assassin", ["Demonize", "Demon Vision", "Blood Massacre"]),
    ("Reaper", "Assassin", "Choose stealth-driven persona attacks or a sustained chaos sequence.", "la-reaper", ["Persona", "Nightmare", "Dance of Fury"]),
    ("Souleater", "Assassin", "Coordinate soul stones, possession and scythe/deathlord attacks.", "la-souleater", ["Soul Drain", "Guillotine Swing", "Deathlord Mode"]),
    ("Sharpshooter", "Gunner", "Combine deliberate bow attacks with a summoned hawk.", "la-gunner", ["Snipe", "Charged Shot", "Summon Silverhawk"]),
    ("Deadeye", "Gunner", "Switch weapon sets to manage distance, timing and burst opportunities.", "la-gunner", ["Shotgun Dominator", "Perfect Shot", "Desperado"]),
    ("Artillerist", "Gunner", "Trade mobility for heavy artillery and a committed barrage state.", "la-gunner", ["Homing Barrage", "Barrage: Energy Cannon", "Summon Turret"]),
    ("Machinist", "Gunner", "Coordinate independent drone commands and personal attacks or spend energy in a powered suit.", "la-machinist", ["Command: Raid Missile", "Energy Buster", "Hypersync"]),
    ("Gunslinger", "Gunner", "Switch firearm sets while maintaining mobile positional combat.", "la-gunner", ["Focused Shot", "Target Down", "Bullet Rain"]),
    ("Artist", "Specialist", "Build harmony through brush attacks and spend it on damage or restoration.", "la-artist", ["Moonfall", "Sunrise", "Paint: Illusion Door"]),
    ("Aeromancer", "Specialist", "Blend weapon flourishes and weather spells with a sustained rain state.", "la-aeromancer", ["Sun Shower", "Piercing Wind", "Rainstorm"]),
    ("Wildsoul", "Specialist", "Alternate bear and fox forms while invoking phantom beasts.", "la-wildsoul", ["Boulder Bear", "Fox Flame", "Phantom Beast Awakening"]),
    ("Dimensionalist", "Specialist", "Use spatial displacement, timing and dimensional control.", "la-dimensionalist", ["Temporal Crush", "Spatial Manipulation", "Timeline Divergence"]),
    ("Guardianknight", "Guardianknight", "Use a halberd and guardian orb to build into an empowered incarnation.", "la-guardianknight", ["Invoke Incarnation", "Guardian's Roar", "Embereth's Breath"]),
]

# These are original Dreadmarch proposals. The named reference games do not
# implement these combinations, and class selection does not imply full kits.
CLASS_MAPPING = [
    ("Warrior", ["Gunlancer", "Destroyer"], ["Warrior", "Juggernaut"], ["Arms", "Bulwark", "Tactician"], "Guard, stance and weapon mastery; reliable physical fundamentals without a mandatory rage state."),
    ("Rogue", ["Sharpshooter", "Deadeye", "Reaper"], ["Ranger", "Deadeye", "Trickster"], ["Marksman", "Skirmisher", "Wayfinder"], "Precision, movement and fieldcraft; bow and agile weapon play remain viable throughout the campaign."),
    ("Sorcerer", ["Sorceress"], ["Elementalist", "Stormweaver"], ["Flame", "Frost", "Storm"], "Innate elemental power, elemental reactions and managed overcharge; Sorceress is an alias."),
    ("Monk", ["Wardancer", "Scrapper", "Soulfist", "Striker", "Breaker"], ["Monk", "Invoker"], ["Open Hand", "Flowing Staff", "Inner Flame"], "Rhythmic martial sequences, spirit and deliberate finishers with readable combo windows."),
    ("Bard", ["Bard", "Artist"], ["Guardian", "Hierophant"], ["War Cantor", "Dirges", "Harmonics"], "Self-sufficient offensive phrases, wards and rhythm; healing never requires another player."),
    ("Barbarian", ["Berserker", "Slayer", "Destroyer"], ["Berserker", "Slayer", "Chieftain"], ["Ravager", "Berserker", "Ancestor"], "Rage, heavy weapon commitment and forceful disruption; distinct from Warrior's stable guard play."),
    ("Amazon", ["Glaivier", "Sharpshooter"], ["Huntress", "Amazon", "Deadeye"], ["Spear Dancer", "Storm Archer", "Huntress"], "Spear and bow expertise with stance or range transitions and a strong martial identity."),
    ("Assassin", ["Deathblade", "Reaper"], ["Assassin", "Saboteur", "Trickster"], ["Shadowblade", "Venom", "Saboteur"], "Preparation, stealth, traps and payoff sequences; distinct from Rogue's sustained fieldcraft."),
    ("Necromancer", ["Summoner", "Souleater"], ["Necromancer", "Witch", "Lich"], ["Bone", "Grave Host", "Blood Rite"], "Corpses, bones and commanded undead; minion decisions matter beyond passive damage."),
    ("Paladin", ["Paladin", "Valkyrie"], ["Guardian", "Inquisitor", "Templar"], ["Oathkeeper", "Zealot", "Judicator"], "Oaths, auras and righteous weapon strikes; every protective path has a solo damage loop."),
    ("Druid", ["Wildsoul", "Aeromancer", "Summoner"], ["Druid", "Oracle", "Shaman"], ["Wildshape", "Stormcaller", "Grovekeeper"], "Persistent animal-form play, weather and living nature with meaningful form-specific actions."),
    ("Crusader", ["Gunlancer", "Paladin", "Valkyrie"], ["Juggernaut", "Guardian", "Templar"], ["Iron Wall", "Siegebreaker", "Consecrator"], "Heavy shield counters, armored advance and battlefield control; distinct from Paladin's oath emphasis."),
    ("Demon Hunter", ["Gunslinger", "Deadeye", "Machinist", "Artillerist"], ["Mercenary", "Witchhunter", "Ranger"], ["Crossbow", "Trapper", "Night Stalker"], "Ranged pursuit, traps and constructed sentries; translate reference firearms into setting-appropriate weapons."),
    ("Witch Doctor", ["Summoner", "Wildsoul", "Arcanist"], ["Occultist", "Pathfinder", "Hierophant"], ["Hexes", "Spirit Beasts", "Plague"], "Spirits, ritual objects, poison and curses; distinct from Necromancer's corpse-and-bone command."),
    ("Wizard", ["Arcanist", "Dimensionalist", "Sorceress"], ["Chronomancer", "Hierophant"], ["Arcane", "Chronomancy", "Conjuration"], "Studied arcane geometry, time, space and constructs; distinct from Sorcerer's innate elemental core."),
    ("Blood Knight", ["Shadowhunter", "Souleater"], ["Blood Mage", "Lich"], ["Blood Lance", "Sanguine Veil", "Night Form"], "Life-cost conversion, drain, reach and controlled monstrous transformation."),
    ("Tempest", ["Aeromancer", "Glaivier", "Soulfist"], ["Invoker", "Stormweaver"], ["Gale", "Tide", "Flow"], "Water and wind martial motion, afterimages and flowing attacks; no animal-form dependency."),
    ("Spiritborn", ["Wildsoul", "Guardianknight", "Soulfist"], ["Monk", "Druid", "Shaman"], ["Jaguar", "Eagle", "Gorilla", "Centipede"], "Invoke guardian aspects through martial actions; maintain a distinct identity from full Druid shapeshifting."),
    ("Warlock", ["Shadowhunter", "Summoner", "Souleater"], ["Infernalist", "Occultist", "Warlock of the Mists"], ["Pacts", "Demonology", "Malediction"], "Bind demons, bargain for power and exploit curse interactions through explicit pact costs."),
]


def design_metadata() -> dict:
    return {
        "status": "Authored proposal; not a claim of delivered gameplay or content.",
        "sources": [{"id": sid, "game": game, "provenance": kind,
                     "version_scope": version, "title": title, "url": url,
                     "observed_at": "2026-11-25"}
                    for sid, game, kind, version, title, url in SOURCES],
        "lost_ark_class_identity_reference": [
            {"class": name, "family": family, "mechanical_pattern": pattern,
             "source_id": source, "illustrative_skill_names": skills,
             "example_scope": "Examples only; complete pinned named inventory is lost_ark.named_skill_records."}
            for name, family, pattern, source, skills in LOST_ARK_IDENTITIES],
        "dreadmarch_class_mapping": [
            {"class": name, "lost_ark_reference_classes": la, "poe_reference_archetypes": poe,
             "proposed_disciplines": branches, "identity_rule": rule}
            for name, la, poe, branches, rule in CLASS_MAPPING],
        "solo_constraints": [
            "The full campaign, every class, quest route, mastery and build-defining item is achievable by one player.",
            "Local saves and pause; no login dependency for the campaign.",
            "No real-time daily or weekly progression gates, mandatory party roles, raid-only build items, or trading requirements.",
            "Class, body, voice, face and background are independent choices; no gender-locked class kits.",
            "Support-oriented disciplines must carry a tested solo offense, defense and boss-stagger loop.",
            "Respec and build previews explain compatibility and tradeoffs before points or materials are spent.",
        ],
        "proposed_production_targets": {
            "status": "Planning floor for a funded full production; validate with content throughput and player testing, not a delivery claim.",
            "classes": 19, "disciplines_per_class": 3, "spiritborn_disciplines": 4, "active_abilities_per_class": 24,
            "behavioral_modifiers_per_active": 3, "passive_nodes_per_class": 48,
            "signature_ultimates_per_class": 2, "campaign_acts": 6,
            "major_world_regions": 12, "authored_dungeons": 48, "major_bosses": 36,
            "authored_side_quest_lines": 60,
            "derived_totals": {"disciplines": 58, "active_abilities": 456,
                               "behavioral_modifiers": 1368, "passive_nodes": 912,
                               "signature_ultimates": 38},
            "world_requirements": ["Persistent quest and NPC consequences", "Distinct towns and faction hubs",
                                   "Secrets and environmental puzzles", "Hand-authored campaign routes",
                                   "Optional generated endgame encounters with authored rule sets"],
            "vertical_slice_gate": "One complete region and dungeon, an authored boss and quest consequence, persistent creator, three deeply implemented contrasting classes; all 19 remain roster goals.",
        },
        "research_gaps": [
            "The catalogue is complete for explicitly pinned extracted input tables, not a verified census of every live regional ability.",
            "Lost Ark's 704 Ark Passive records have names and levels, but their complete player-facing class/tier/prerequisite graph is not present in this source.",
            "Skill and passive identifiers do not constitute a full behavioral specification; modifier values, animation assets, tooltip prose and implementation code are not imported.",
            "PoE gem entries may contain legacy, item-granted, variant or unreleased data; official tree class slots also include placeholders.",
            "Historical official class articles establish mechanics provenance; current balance values and unlock thresholds must be checked against the chosen release before implementation.",
            "PoE1 3.30.0 is explicitly a preview and is not merged into the 3.29.1 baseline.",
        ],
    }
