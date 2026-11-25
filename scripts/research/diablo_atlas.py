#!/usr/bin/env python3
"""Render the readable Diablo research atlas from the complete factual catalogue."""
import argparse
import collections
import json
from pathlib import Path

PREFIX = """# Diablo class and ability atlas

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

"""

SYNTHESIS = """## Class synthesis mapped to the current prototype

The branch column below is loaded directly from **`data/gameplay/classes.json`**: three branches for most classes and **four for Spiritborn**. These are original Dreadmarch branch names, not a claim about source-game terminology. The mechanical connections are **research-backed design proposals for further development**, not a claim that the prototype already implements every listed mechanic. Rebuild timing, costs, damage, names, effects and progression for Dreadmarch's world and campaign. The Divinity atlas supplies additional exploration, social, crafting and environmental interactions for these identities.

| Class | Current prototype branches | Source mechanics to study and identity boundary |
| --- | --- | --- |
| Warrior | Arms / Guard / Duelist | D1's equipment-focused fighter and Item Repair support a grounded weapon master. Build directional blocks, stamina, dueling and field preparation here; reserve rage and supernatural earth attacks for Barbarian. Campaign utility is an original extension, not a hidden D1 tree. |
| Rogue | Marksman / Shadow / Guile | Combine D1 Trap Disarm with D4 bow/knife skills, Concealment, Smoke Grenade and traps. Emphasize scouting, precision and improvised advantage; avoid duplicating Demon Hunter's autonomous arsenal. |
| Sorcerer | Flame / Frost / Lightning | D2's Fire/Cold/Lightning trees and D4 enchantments provide coherent elemental specialization. Support interactions between these elements while keeping an explicit primary school. |
| Monk | Forms / Mantras / Spirit | D3's generators, Seven-Sided Strike, mantras and Mystic Ally combine with Immortal's mobile martial skills. Give its defensive and healing tools useful solo applications; it must not require a party to function. |
| Bard | Songs / Blades / Discord | Hellfire provides dual-weapon identity and Identify, not an extensive musical spellbook. Blades and lore utility draw on that evidence. Songs and Discord are deliberate original expansions into rhythm, sound and morale; do not attribute the new musical abilities to Hellfire. |
| Barbarian | Berserking / Warcries / Earthshaker | D2 weapon masteries and Warcries, D3 Earthquake and Wrath of the Berserker, and D4 weapon expertise support distinct rage, command and terrain-impact builds. |
| Amazon | Bowcraft / Stormspear / Battle Grace | Retain D2's Bow and Crossbow, Javelin and Spear, and Passive and Magic identities. Guided shots, lightning javelins, evasive passives and Valkyrie-style support can share a martial core without turning into Rogue. |
| Assassin | Martial Arts / Shadow / Traps | D2 already supplies three coherent trees. Study charge-up/finisher flow, mobility, shadow doubles and placed mechanisms; combine these through deliberate setup and execution windows. |
| Necromancer | Bone / Blood / Death Legion | Combine D2 curses and summons with D3/D4 bone, corpse and blood mechanics. Curses should support each branch; corpse supply and minion control are explicit systems rather than free background damage. |
| Paladin | Zeal / Auras / Judgment | D2 Combat Skills and auras combine with D4 oaths, holy weapons and judgment. Preserve oath-driven conviction and persistent support choices; separate this from Crusader's mounted, heavy-shield assault. |
| Druid | Wildshape / Primal Elements / Wildbond | D2 shapeshifting and summons, D4 earth/storm skills and Spirit Boons, and Immortal's full replacement form bars supply a strong transformation model. Forms, companions and weather need shared rules for gear and cooldowns. |
| Crusader | Shieldwall / Consecration / Crusade | D3 shield skills, Consecration and Steed Charge, plus Immortal Draw and Quarter and Holy Banner, support a durable moving front line. Shield-based area control and mounted momentum distinguish it from Paladin. |
| Demon Hunter | Arsenal / Sentry / Vengeance | D3/Immortal crossbows, grenades, Sentry, traps, evasions and Vengeance support a ranged weapons engineer. Explicit ammunition patterns and deployables distinguish it from the Rogue scout. |
| Witch Doctor | Hexes / Plague / Spirit Court | D3 Hex, Haunt, Locust Swarm, Firebats, fetishes, gargantuans and spirit movement offer curse, attrition and spirit-pet builds. Preserve living ritual and spirit themes rather than duplicating Necromancer's corpse army. |
| Wizard | Arcana / Displacement / Illusion | D3 Arcane Orb, Arcane Torrent, Slow Time, Teleport, Mirror Image and Archon support shaped arcane forces, altered space/time and deceptive copies. Sorcerer retains direct elemental-school mastery. |
| Blood Knight | Bloodcraft / Polearm / Nightform | Immortal's Siphon Blood, Tendrils of Blood, polearm attacks, bats, Sanguinate and Abomination support health tradeoffs, reach and transformation. Keep self-sustain readable and resist invulnerability loops. |
| Tempest | Windblade / Tides / Zephyrs | Immortal's dual blades, wind/water repositioning and Mist-Touched Zephyr echoes support attack mirroring and flowing mobility. Treat echoes as timed action copies, distinct from permanent necromantic minions. |
| Spiritborn | Jaguar / Eagle / Gorilla / Centipede | D4's four spirit identities remain four explicit prototype branches. Preserve Jaguar offense, Eagle movement/ranged pressure, Gorilla defense and Centipede attrition rather than deleting or hiding one spirit to force symmetry. |
| Warlock | Demon Pacts / Hellfire / Abyss | D2R Demon/Eldritch/Chaos, D4 soul shards and Greater Demons, and Immortal Soulgorger/portals/sacrifices provide pact control, destructive casting and spatial/health exchange. Preserve the distinction from undead and blood-knight systems. |

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

"""


def labels(rows):
    return "; ".join(x["name"] for x in rows if x.get("name")) or "None"


def render(data,prototype):
    games=data["games"];d4=games[5];imm=games[6]
    branch_map={x["name"]:" / ".join(x["branches"]) for x in prototype["classes"]}
    synthesis=[]
    for line in SYNTHESIS.splitlines():
        if line.startswith("| "):
            parts=line.split("|")
            if parts[1].strip() in branch_map:parts[2]=" "+branch_map[parts[1].strip()]+" ";line="|".join(parts)
        synthesis.append(line)
    text=[PREFIX,"| Source scope | Expected / discovered | Included | Unresolved / interpretation |","| --- | ---: | ---: | --- |",
      "| D1 table | 36 rows | 36 | Five unavailable/unused rows explicitly marked; 31 usable spell/innate rows. |",
      "| Hellfire table | 51 rows | 51 | Four unused rows; 47 spell/innate/rune-effect rows. Identify/Search have both shared and innate uses. |",
      "| D2 LoD official index | 210 skills | 210 | All seven legacy classes; original D2 is the five-class subset. |",
      "| D2R / Reign of the Warlock | 240 class rows | 240 | Eight classes × 30; tree grids and prerequisites retained. |",
      "| D3 active / passive / rune | 163 / 131 / 815 | 163 / 131 / 815 | All seven official class tables; footer rows rejected. |",
      f"| D4 SkillKits | 198 active entries; 194 unlocks; 1,402 modifiers | All | 2,228 graph nodes, including 632 structural nodes. No missing current names. |",
      f"| D4 class mechanics | 122 power references | 122 | Seven Barbarian rank bonuses and 21 enchantments use derived parent labels; spirit-tier labels are marked derived. |",
      f"| D4 Paragon | 80 boards; 566 nodes; 161 glyphs | All | 12,852 occupied board cells; 427 named nodes; 139 unnamed stat nodes; one unreferenced definition retained. |",
      f"| Immortal skillbook | 1,547 rows across 1,000 + 547 pages | 1,547 | 158 base rows + 1,389 essence rows; no page cap mistaken for completion. |",
      f"| Immortal core class abilities | 20 primaries; 130 equipped skills; 20 ultimates | All | Additional form, pet and resource records make 190 class records in total. |",
      f"| Immortal Paragon | 136 records | 136 | Eight 15-skill groups plus 16 Legendary Skills; source lacks client-build ID. |","","\n".join(synthesis)]
    for game in games:
        text += ["## "+game["game"],"",game["version"],""]
        if game is games[0] or game is games[1]:
            for typ,rows in group(game["records"],lambda x:x["type"]).items():text += ["**"+typ.replace("_"," ").title()+":** "+labels(rows)+".",""]
            text += ["Source: "+game["sources"][0]["url"],""]
            continue
        for cl in game["classes"]:
            text += ["### "+cl,""]
            records=[x for x in game["records"] if x["class"]==cl]
            for tree,rows in group(records,lambda x:(x["tree"],x["type"])).items():
                text += ["**"+tree[0]+" · "+tree[1].replace("_"," ")+"**",""]
                for row in rows:
                    variants=row.get("variants",[])
                    line="- **"+row["name"]+"**"
                    if variants:line+=" — "+"; ".join(x["name"] for x in variants)
                    text.append(line+".")
                text.append("")
            source=next((x["source_url"] for x in records),None)
            if source:text += ["Source entry: "+source,""]
            if game is d4:
                mechanics=[x for x in game["class_mechanic_records"] if x["class"]==cl]
                text += ["**Class mechanic entries:** "+labels(mechanics)+".","","**Paragon boards:** "+labels([x for x in game["paragon_boards"] if x["class"]==cl])+".",""]
                if cl=="Necromancer":text += ["Repeated Upgrade A / Upgrade B / Sacrifice labels belong to individual army specializations. Their unique power IDs and specialization paths are retained in the JSON.",""]
            if game is imm:
                n=game["coverage"]["per_class"][cl]["essence_variants"]
                text += [f"**Named legendary essence variants:** {n}; see this class's `variants` records in the JSON.",""]
        if game is imm:
            text += ["### Shared Paragon groups",""]
            for tree,rows in group(game["paragon_records"],lambda x:x["tree"]).items():text += ["**"+tree+":** "+labels(rows)+".",""]
    text += ["## Provenance and reproduction","",
      "The catalogue stores source retrieval date, immutable commit IDs where available, input hashes and per-record URLs. The complete raw download cache is deliberately external to the repository; only factual labels and structure are published.","",
      "| Source family | Pinned source / route |","| --- | --- |",
      "| D1 / Hellfire | `diasurgical/devilutionX` commit `e1c377775a206274f73b5f01c84a967d06e74d54`, `assets/txtdata/spells/spelldat.tsv` and `mods/hf/txtdata/spells/spelldat.tsv` |",
      "| D2 legacy | [Arreat Summit alphabetical skill index](https://classic.battle.net/diablo2exp/skills/alphaskills.shtml) |",
      "| D2R | `blizzhackers/d2data` commit `51e7826e67515f4a84583ebaa882339aeeb99758`, `json/skills.json`, `json/skilldesc.json`, `json/localestrings-eng.json` |",
      "| D3 | `https://us.diablo3.blizzard.com/en-us/class/{class}/{active|passive}/`; seven classes, fourteen tables |",
      "| D4 | `DiabloTools/d4data` commit `75e619f0d90a0951945747f45feef59c076a5db0`, build `3.2.4.74043`; SkillKit, SkillTreeRewards, Power, PlayerClass, ParagonBoard/Node/Glyph and matching enUS StringList data |",
      "| Immortal | [diablo.tv public builder](https://diablo.tv/builder), its public `skillbook`, `classes`, `paragons_with_tree` views, official Blizzard class introductions, [Wowhead skill index](https://www.wowhead.com/diablo-immortal/skills) for the first eight classes and their ultimates |","",
      "The broad D4 recursive base-tree response was truncated and was **not** used as a completeness denominator. Targeted family-tree responses are complete; every expected family file is reconciled. English StringList enumeration was complete. One case mismatch (`Druid_landslide` / `Druid_Landslide`) was resolved against that inventory, and all current selectable modifier labels resolved.","",
      "The final cross-check found D2 locale-table problems: the Plague Poppy row incorrectly resolved to Werewolf, and Blade Mastery retained its old Sword Mastery label. The catalogue preserves each original source label and applies explicit corrections supported by the [official Druid guide](https://classic.battle.net/diablo2exp/skills/druid-summoning.shtml) and [official D2R 2.4 notes](https://news.blizzard.com/en-us/article/23788293/diablo-ii-resurrected-patch-2-4-ladder-now-live). Warlock display names (Levitation Mastery, Hex/Sigil punctuation, Miasma Chain) and Polearm Mastery are reconciled with the [first-hand 3.4.95032 extraction index](https://d2r.org/data/skills). The legacy LoD index retains its historical naming. Paladin auras with additional passive components remain classified as auras, with the passive component flagged separately.","",
      "### Source licenses","",
      "The pinned [DevilutionX LICENSE.md](https://github.com/diasurgical/devilutionX/blob/e1c377775a206274f73b5f01c84a967d06e74d54/LICENSE.md) is **Sustainable Use License 1.0**, not a permissive commercial-engine license. The pinned [d2data LICENSE](https://github.com/blizzhackers/d2data/blob/51e7826e67515f4a84583ebaa882339aeeb99758/LICENSE) and [d4data LICENSE](https://github.com/DiabloTools/d4data/blob/75e619f0d90a0951945747f45feef59c076a5db0/LICENSE) contain MIT repository notices. These repository notices do not establish ownership of underlying Diablo game content. Official and community website prose/art are not imported. The JSON records license URLs and hashes, and a complete input-hash manifest for the D4 files used. Dreadmarch remains an original implementation; these sources are factual research inputs.","",
      "Immortal's source timestamp is October 21, 2026. The public skillbook was paged to a short terminal page (1,000 then 547 rows); all IDs are unique. Its unfiltered Wowhead page returned exactly 1,000 rows and lacked Druid/Warlock, so it was not accepted as a complete class or essence inventory. Official class introductions supply the twenty ultimate names and additional form/pet names together with that first-eight-class cross-check.","",
      "Run the checked-in extraction scripts against the retained raw cache to regenerate:","","```bash","python scripts/research/diablo_catalog.py --raw /path/to/research-diablo","python scripts/research/diablo_atlas.py","```","",
      "`diablo_fetch_d4.py`, `diablo_fetch_d4_mechanics.py`, and `diablo_fetch_d4_paragon.py` cache the pinned D4 references. `diablo_fetch_immortal.py` exhausts the observed public database pagination. It uses the site's public client key in memory, performs reads only, and never stores credentials in the repository.","",
      "### Remaining evidence limits","",
      "- The Immortal community snapshot needs an official or lawful current client export for exact live-build equivalence. Its source completeness is verified; unknown live differences remain explicit.",
      "- Historical pre-3.0 D4 passive trees and prior balance revisions are not represented as current. Unreferenced localized modifiers are preserved separately where they occur in the current files.",
      "- D1/Hellfire source-port tables are a preservation source, not a claim that every unused source row can be acquired in the retail games.",
      "- D4 glyph eligibility bit arrays are preserved as source flags; an unverified class-index mapping is not invented.",
      "- Global equipment affixes, every follower/monster/internal triggered power and all temporary seasonal systems are outside the stated class-and-progression scope. Immortal essences and D3 runes are included because they are explicit class-skill variant systems.",""]
    return "\n".join(text)


def group(rows,key):
    groups=collections.OrderedDict()
    for row in rows:groups.setdefault(key(row),[]).append(row)
    return groups


if __name__=="__main__":
    p=argparse.ArgumentParser();p.add_argument("--input",type=Path,default=Path("data/research/diablo-catalog.json"));p.add_argument("--output",type=Path,default=Path("docs/research/DIABLO_CLASS_ATLAS.md"));a=p.parse_args()
    a.output.parent.mkdir(parents=True,exist_ok=True);a.output.write_text(render(json.loads(a.input.read_text()),json.loads(Path("data/gameplay/classes.json").read_text())));print(a.output)
