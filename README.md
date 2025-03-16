# Heliosphere Affects Calculator

A FFXIV game path parser, a generator for a database of IDs extracted from paths
and what they affect, and a calculator to combine the two and figure out what
paths affect what (item/equipment/etc.) names.

This collection of crates was designed for Heliosphere but could easy be adapted
for any project that needs to parse FFXIV game paths or figure out what they
affect.

## What does it detect?

In terms of parsed paths:

- `chara/eXXXX/equipment`
    - `/eXXXX.imc`
    - `/model/cXXXXeXXXX_SLOT.mdl`
    - `/material/vXXXX/mt_cXXXXeXXXX_SLOT_EXTRA.mtrl`
    - `/texture/vXX_cXXXXeXXXX_SLOT_EXTRA.tex`
    - `/vfx/eff/veXXXX.avfx`
- `chara/weapon`
    - will fill in the rest later, but it does catch most things in these paths
- `chara/accessory`
- `chara/human`
- `chara/demihuman`
- `chara/monster`
- `common/font`
- `ui/icon`
- `ui/map`

In terms of specific names:

- Gear
- Weapons
- Action animations
- Emote animations
- Certain miscellaneous animations (idle, movement)
- Battle NPCs
- Event NPCs
- Player character/NPC customisation (body, skin textures, skeletons, etc.)
- Mounts
- Minions
- Fashion accessories
- Decals (including Archon mark)
- Maps
- Fonts
- Icons

In terms of vague names:

- Anything in `vfx/` becomes "VFX"
- Anything in `bg/` or `bgcommon/` becomes "World"
- Anything in `ui/` becomes "Interface"
- Anything in `shaders/` becomes "Shader"
- Any `.scd` file becomes "Sound"
