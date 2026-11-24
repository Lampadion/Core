<p align="center">
  <img src="https://github.com/Lampadion/Graphics/raw/refs/heads/main/icons/app/lampadion.svg" width="96"/>
</p>

$$
\begin{array}{c}
\Large \color{#e2b041}\mathbf{Lampadion} \\
\text{ } \\
\normalsize \mathbf{Extended \ Unicode \ input \ for \ many \ languages \ and \ purposes} \\
\small \text{A cross-platform input layer between your keyboard and a vast library of characters} \\
\text{ } \\
\small \text{Ænglisċ sprǣċ | Français | Română | Tiếng Việt | Hànyǔ Pīnyīn … } \\
\small \text{Ελληνική γλώσσα | 𐐔𐐯𐑅𐐨𐑉𐐯𐐻 𐐈𐑊𐑁𐐰𐐺𐐯𐐻 | ᚹᛟᛞᛁᚾᛊ᛫ᚱᚢᚾᛟ … } \\
\small \text{Словѣньскъ ѩꙁыкъ | Қазақ тілі | Рꙋмѫнѣскъ | Аԥсуа бызшәа … } \\
\end{array}
$$

<p align="center">
	<img src="https://img.shields.io/badge/status-early_development-orange" alt="Status: early development"/>
	<img src="https://img.shields.io/badge/platform-Windows-blue" alt="Platform: Windows"/>
	<img src="https://img.shields.io/badge/platform-Linux-blue" alt="Platform: Linux"/>
	<img src="https://img.shields.io/badge/platform-macOS_(later)-lightgrey" alt="Platform: macOS, later"/>
<br>
	<img src="https://img.shields.io/badge/core-Rust-b7410e?logo=rust" alt="Core: Rust"/>
	<img src="https://img.shields.io/badge/input_domains-25%2B_planned-yellow" alt="Input domains: 25+ planned"/>
	<img src="https://img.shields.io/badge/license-TBD-lightgrey" alt="License: TBD"/>
</p>


<p align="center">
🜁🜃🜂🜄🝞🝟🜲🜅🜆🜊🜍🝁🝆🝗🝘🜬🝩⚶⚴☊♉︎☄︎⚲⛤☤⚚☥♕♔♥♣⅍℆⌬⮃⬱⇶‽⁗⅝≋∰∜ …<br>☰☱☲☳☯︎☴☵☶☷
</p>

---

> **⚠️ Work in progress.** Lampadion is being built from scratch and is not yet ready for daily use. This README describes the project's goals and intended architecture; details will change as the design settles.

## What is Lampadion?

Lampadion is a tool for typing Unicode characters that are hard to enter from an ordinary keyboard: diacritics, letters of historical scripts, phonetic symbols, mathematical alphabets and many others.

It works on top of normal keyboard input. Key bindings, compose (sequences such as `AE` → `Æ`) and input modes turn what you type into the characters you need, and you can add your own bindings, compose, layouts and modes. The GUI is used to browse, search and configure all of this.

Lampadion continues the idea of an earlier AutoHotkey project. It is written from scratch in Rust and targets Windows and Linux.

## Features

### Multilayer bindings

Key bindings depend on the active layout set, so the same physical gesture can produce different results in different languages.

| Set | Binding | Result |
|---|---|---|
| Latin | `RAlt + A` / `RAlt + Z` | `Ă` / `Ż` |
| Cyrillic | `RAlt + Ф` / `RAlt + Я` | `Ѳ` / `Ѧ` |

Lampadion distinguishes the physical key, the scancode/keycode, the modifiers (left and right variants are separate: `RAlt + A` is not `LAlt + A`), the active layout, a logical key identifier and the resulting character. User-defined bindings are supported.

### Compose / Forge

Type a sequence, get a different character or sequence. Recipes are processed by a sequence engine, not looked up in a flat hotkey table, which leaves room for more complex sequences and different processing modes.

`AE` → `Æ` · `OE` → `Œ` · `TH` → `Þ` · `ІѦ` → `Ѩ` · `ЯЕ` → `Ԙ` · `ⰦⰤ` → `Ⱙ` · `Ups` → `Ʊ`

Recipes can reference other entries, so `A` + the combining acute accent is written as a reference rather than a literal. User-defined recipes are supported.

### Unicode composition

Lampadion works with sequences of code points, not only with ready-made precomposed characters.

- base character + combining mark → precomposed character where one exists (`A` + acute → `Á`);
- the combining sequence is preserved where no precomposed form exists;
- normalization (NFC/NFD) is part of data processing, not a visual effect.

### Alternative input

Dedicated input modes for scripts and special character sets. The architecture is meant to allow adding new input domains without changing the core. Planned and possible domains include:

International Phonetic Alphabet, Runic, Glagolitic, Old Turkic, Old Permic, Old Hungarian, Gothic, Caucasian Albanian, Old Italic scripts, Phoenician, Samaritan, Ancient South and North Arabian, Carian, Lycian, Lydian, Sidetic, Cypriot Syllabary, Tifinagh, Ugaritic, Old Persian, Imperial Aramaic, Palmyrene, Inscriptional Parthian, Manichaean, Deseret, Shavian.

Not all of them will be available at once.

### Glyph variations

A character can have many related variants defined by Unicode. For `A`:

`ᴬ` `𝐴` `𝐀` `𝑨` `𝙰` `Ａ` `ᴀ` `𝔄` `𝕬` `𝒜` `𝓐` `𝔸` `Ⓐ` `🅐` `🄰` `🅰` `🄐`

Special modes cover Regional Indicator Symbols and Unicode Tags.

### Language-specific input modes

Sequence-based modes in the spirit of Vietnamese Telex/VNI, for Vietnamese, Jarai and Hànyǔ Pīnyīn. The architecture leaves room for further phonetic and orthographic input schemes.

### Multiple representations

A character is data with several representations, not only a `char`:

```
Ă  →  &Abreve;  →  &#258;  →  \u{A}  →  \breve{A}
```

Where available, Lampadion can output the character itself, an HTML entity, a numeric reference or a LaTeX command.

### Symbol search

A structured local library with identifiers, code points, tags and metadata. Search accepts full names or abbreviations:

`latin small ligature turned oe with short stroke` or just `oe tur str` → `ꭂ`

Facts that Unicode already defines (official names, categories, scripts, blocks) are taken from ICU rather than duplicated in the database.

### Keyboard layouts

Lampadion keeps its own virtual layouts, separate from the system layout: QWERTY, AZERTY, Dvorak, Colemak, Workman, Norman, Graphite, Gallium, Canary and others for Latin; ЙЦУКЕН, Диктор, ЯШЕРТЫ and others for Cyrillic. Built-in layouts may be adapted rather than exact copies of the originals. User-defined layouts are supported.

### Extensions

Everything extensible is meant to be user-extensible: bindings, recipes, layouts, input modes and mods. Mods are written in [Rune](https://rune-rs.github.io/) and live in their own namespaces, so they do not conflict with the base data.

## Architecture

| Layer | Technology |
|---|---|
| Core | Rust |
| GUI | Dioxus Desktop + SCSS |
| Mod scripting | Rune |
| Non-compiled data | RON (Rusty Object Notation) |
| Localization | Fluent (`.ftl`) |

Principles:

- **The core does not depend on the operating system.** Platform-specific code (global keyboard hooks, input injection, layout detection, IME, keycode mapping) is isolated behind abstractions, because Windows and Linux need fundamentally different backends.
- **Data-driven.** Characters, bindings, recipes, layouts and input modes live in external files and are loaded at runtime.
- **Unicode first.** The model distinguishes scalar values, code points, sequences and grapheme clusters; the numeric code point stays directly accessible.
- **IME is not a synonym for a global hotkey hook.** Intercepting key events, generating Unicode input, composition and system text-service APIs are treated as separate concerns.

### Repository layout

```
apps/
    lampadion-cli          # status undecided
    lampadion-host         # application logic
    lampadion-gui          # Dioxus GUI and auxiliary windows
    lampadion-launcher     # entry point
    lampadion-website      # presentation site, not part of the app
crates/
    lampadion-assets       # graphics, styles, sounds
    lampadion-binding
    lampadion-compose
    lampadion-core
    lampadion-config
    lampadion-db
    lampadion-extensions
    lampadion-ipc
    lampadion-keyboard
    lampadion-layout
    lampadion-model
    lampadion-modes
    lampadion-scripting
    lampadion-search
    lampadion-ui           # components shared by GUI and website
    lampadion-unicode
    lampadion-l10n
data/                      # .ron
    characters/
    simulated_layout/      # latin/, cyrillic/, hellenic/
    binding/
locale/                    # .ftl, per ISO 639 / ISO 3166
```

The launcher is the entry point: it starts the host and the Dioxus GUI. The host owns all application logic and feeds data to the GUI. The GUI draws the main window and the auxiliary ones (mode selector, input hint, and so on).

### Release layout (Windows)

```
Lampadion/
    assets/
    bin/                   # host.exe, cli.exe, gui.exe
    data/                  # .ron
    locale/                # .ftl
    extensions/
    user/
        profile-<name>/
            bindings/      # default.ron is always enabled
            compose/       # default.ron, .XCompose
            simulated_layout/
            config.ron
            favorite.ron
            extensions.ron
        profile.ron
    lampadion.exe          # launcher
```

## Character database

Characters are described in RON files. Each entry is one character together with everything attached to it: collections, GUI groups, key bindings, Forge sequences and variation links. There are three kinds of entry:

- `CharacterEntry` is a single character;
- `CharactersEntry` takes the same fields, but holds several code points (`codepoints`) and characters (`characters`) in one entry;
- `TemplateEntry` generates a family of entries from an `id_template` and a list of `members`; fields set at the root act as the base for every member.
```ron
// Illustrative example
CharacterEntry(
	id: "acute",
	codepoint: 0x0301,
	collections: [Diacritic],
	ui: (
		display_groups: [
			DisplayGroup(tab: Forge, target: Diacritic),
		],
	),
	binds: [
		Mode(Default, [
			Bind(
				id: "acute_default",
				key: A,
				modifiers: [RightAlt],
			),
		]),
	],
	forge: (
		force_suggest_at_end: false,
		sequences: [
			Sequence(sequence: "acute", shown_sequence: "acute"),
		],
	),
	hidden: false,
)
```

```ron
// A family of entries from one template
TemplateEntry(
	id_template: "ipa_n_let_tone_bar_{level}",
	members: [
		Member(level: "extra_high", codepoint: Some(0x02E5)),
		Member(level: "high",       codepoint: Some(0x02E6)),
	],
)
```

Highlights of the format:

- **Bindings live in the entry, grouped by mode.** `Mode(Default, …)` is always active, `Mode(Domain(…), …)` belongs to an activable input domain (for example, Germanic runes), and `Mode(IME(…), …)` to an IME-style mode (for example, Vietnamese or Hànyǔ Pīnyīn). A bind is a key plus a list of modifiers, where left and right variants are distinct (`RightAlt`, `LeftAlt`, `RightControl`, and so on).
- **Binds can also be stored separately.** A bind has its own `id`, so a dedicated bind file can override it, and there it must name its `target_entry` explicitly. Inside an entry, the target is inferred.
- **Forge sequences.** `sequences` lists what must be typed in Forge mode to reach the character; `shown_sequence` is what the GUI displays.
- **Variations.** `variation_of` marks an entry as a variant of another one, and `hidden` keeps it out of the main lists.
- **Phantoms.** `phantoms` create duplicates of an entry under a different id and with different parameters, for example a dedicated entry for an IPA character inside the IPA input domain.
- **Templates remove repetition.** Members of a `TemplateEntry` only state what differs between them.

## Platforms

- **Windows** and **Linux** are the primary targets.
- macOS may be considered later.

Global keyboard hooks, input injection, active-layout detection, IME and keycode mapping differ substantially between platforms, so each gets its own backend.

## Status and roadmap

The project is at the design stage: the character database format (revision 2) is drafted and under discussion. Open questions include the modifier syntax in key bindings, recipe processing modes, a `patch` mode for modifying base entries, and shared axis sets for matrices.

## Notes

- **Use the extended character set for text only.** Using these characters in passwords, file names and similar places is risky.
- **Games with anti-cheat.** Tools that hook keyboard input at the system level can be flagged by some anti-cheat systems. Close Lampadion before playing such games.

## License

To be determined.

## Contributors

<table>
  <thead>
    <tr>
      <th colspan="2">Contributor</th>
      <th>Role</th>
      <th>Info</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>
        <a href="https://github.com/DemerNkardaz">
          <img src="https://avatars.githubusercontent.com/u/92405785" width="48" alt="DemerNkardaz"/>
        </a>
      </td>
      <td><a href="https://github.com/DemerNkardaz">@DemerNkardaz</a></td>
      <td>Initiator, Lead</td>
      <td><img src="https://wakatime.com/badge/user/e572f348-6192-4188-a508-7efe46e45cd5/project/1677f8bd-3eff-4888-bcf5-db7c8cdfa684.svg" alt="WakaTime"/></td>
    </tr>
  </tbody>
</table>
