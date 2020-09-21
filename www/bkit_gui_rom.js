"use strict";

export const PaintFrame = `
# Hint: Look at the other ROM pages    ^^^^

# This is one of seven ROM pages comprising a
# graphics toolkit written in an as-yet unnamed
# stack-oriented VM language.

# Language notes:
# - Postfix with stack: T is top, S is second
# - Things like 3 (hi) <10> dup and + are words
# - Words are case sensitive
# - Words must be separated by whitespace
# - The VM opcode keywords (listed to right →)
#   cannot be redefined, but other symbols can
# - Global namespace, bindings eval at runtime

# Edits here are temporary. Reloading browser
# tab reverts changes. Editing bkit_gui_rom.js
# is the equivalent of re-flashing ROM.

# To see effect of edits, try "Soft Reboot"
# event trigger button at bottom left.

# This calls a view function to paint a frame:

drawHomeScreen
`.trim();

export const Views = `
# Views: Assemble widgets to paint a full frame

# These are defaults for slots
: wTitle (Home) ;
: wBat sprBat75 ;
: wWifi sprWifi3 ;
: wTime (12:00) ;
: note (Hello, World!) ;
: kbd kAzerty ;

# Home Screen view
: drawHomeScreen
  wDrStatusBar           # from Widget page
  wDrMainWindow          # same
  note wDrMainNotify     # same
  kbd
;

# NOTE: At boot, the PaintFrame page will run
# right after this one. But, for event-
# handlers, code from the event message gets
# spliced here to re-define slot values before
# PaintFrame page runs.
`.trim();

export const KbdAzerty = `
# Draw AZERTY key maps (base, AltL, AltR)

: kAzerty
kSetStyle
kFKeys
(1) (2) (3) (4) (5) (6) (7) (8) (9) (0) P22
kRowOfTen
(A) (Z) (E) (R) (T) (Y) (U) (I) (O) (P) P32
kRowOfTen
(Q) (S) (D) (F) (G) (H) (J) (K) (L) (M) P42
kRowOfTen
()  (W) (X) (C) (V) (B) (N) (:) (;) ()  P52
kRowOfTen
kBottomRow
;

: kAzertyAltL
kSetStyle
kFKeysAltL
(§) () () () ([) (]) () (_)  (') (") P22
kRowOfTen
()  () () () ()  ()  () ()   ()  ()  P32
kRowOfTen
()  () () () ()  ()  () (/)  ()  ()  P42
kRowOfTen
()  () () () ()  ()  () (¿)  (¡) ()  P52
kRowOfTen
kBottomRowAltL
;

: kAzertyAltR
kSetStyle
kFKeysAltR
(à)  (é) (è) (ê) (() (\\)) (&) (*) («) (») P22
kRowOfTen
(æ)  (£) (€) (\`) ({) (})  (ù) (ï) (œ) (%) P32
kRowOfTen
(@)  (ß) ($) (¤) (µ) (-)  (+) (\\\\) (|) (#) P42
kRowOfTen
(⌫) (<) (>) (ç) (^) (=)  (~) (?) (!) (⏎) P52
kRowOfTen
kBottomRowAltR
;
`.trim();

export const KbdQwerty = `
# Draw QWERTY key maps (Base, Alt)

: kQwerty
kSetStyle
kFKeys
(1) (2) (3) (4) (5) (6) (7) (8) (9) (0) P22
kRowOfTen
(Q) (W) (E) (R) (T) (Y) (U) (I) (O) (P) P32
kRowOfTen
(A) (S) (D) (F) (G) (H) (J) (K) (L) (⌫) P42
kRowOfTen
(!) (Z) (X) (C) (V) (B) (N) (M) (?) (⏎) P52
kRowOfTen
kBottomRow
;

: kQwertyAlt
kSetStyle
kFKeysAlt
()  ()  ()  ()  ()  ()  ()  ()  ()   ()  P22
kRowOfTen
(%) (^) (~) (|) ([) (]) (<) (>) ({)  (}) P32
kRowOfTen
(@) (#) (&) (*) (-) (+) (=) (() (\\)) (⌫) P42
kRowOfTen
(\`) (_) ($) (") (') (:) (;) (/) (\\\\) (⏎) P52
kRowOfTen
kBottomRowAltR  # Yes, really AltR
;
`.trim();

export const KbdCommon = `
# Kbd::Common Includes:
# - Names for key (row,col) positions
# - Drawing of key outlines and labels
# - Drawing of keys shared between layouts

# Keyboard coordinate system on screen:
# - Origin is top left (+x=right, +y=down)
# - col: key width: (-3 + 336 - 3) / 10 = 33
# - row: key height: 33
# - keyboard origin: x:2, y:536-3-(33*8)=269

# Calculate key xy from row and column
# Base key size is 33x33 px (some are wider)
: kCol 33 * 3 + ;   # x of key left, 0..10
: kRow 33 * 269 + ; # y of key top, 0..8

# Set style for key outlines
: kSetStyle 1 fill 2 stroke ;

# Draw {1,2,4}-wide key with label -- T=(utf8)
# Box dimension +1px makes border lines overlap
: key1  34 34 rect mark 16 25 +xy txtC gomark ;
: key2  67 34 rect mark 31 25 +xy txtC gomark ;
: key4 133 34 rect mark 76 25 +xy txtC gomark ;

: P2  9 kCol shr  0 kRow goxy ;  # Up
: P5  7 kCol shr  1 kRow goxy ;  # Left
: PC  9 kCol shr  1 kRow goxy ;  # Click
: P6 11 kCol shr  1 kRow goxy ;  # Right
: P3  0 kCol      2 kRow goxy ;  # F1
: P4  2 kCol      2 kRow goxy ;  # F2
: P9  9 kCol shr  2 kRow goxy ;  # Down
: P7  6 kCol      2 kRow goxy ;  # F3
: P8  8 kCol      2 kRow goxy ;  # F4

: P13 0 kCol 3 kRow goxy ; # Number row
: P14 1 kCol 3 kRow goxy ;
: P15 2 kCol 3 kRow goxy ;
: P16 3 kCol 3 kRow goxy ;
: P17 4 kCol 3 kRow goxy ;
: P18 5 kCol 3 kRow goxy ;
: P19 6 kCol 3 kRow goxy ;
: P20 7 kCol 3 kRow goxy ;
: P21 8 kCol 3 kRow goxy ;
: P22 9 kCol 3 kRow goxy ;

: P23 0 kCol 4 kRow goxy ; # Upper Letter row
: P24 1 kCol 4 kRow goxy ;
: P25 2 kCol 4 kRow goxy ;
: P26 3 kCol 4 kRow goxy ;
: P27 4 kCol 4 kRow goxy ;
: P28 5 kCol 4 kRow goxy ;
: P29 6 kCol 4 kRow goxy ;
: P30 7 kCol 4 kRow goxy ;
: P31 8 kCol 4 kRow goxy ;
: P32 9 kCol 4 kRow goxy ;

: P33 0 kCol 5 kRow goxy ; # Home letter row
: P34 1 kCol 5 kRow goxy ;
: P35 2 kCol 5 kRow goxy ;
: P36 3 kCol 5 kRow goxy ;
: P37 4 kCol 5 kRow goxy ;
: P38 5 kCol 5 kRow goxy ;
: P39 6 kCol 5 kRow goxy ;
: P40 7 kCol 5 kRow goxy ;
: P41 8 kCol 5 kRow goxy ;
: P42 9 kCol 5 kRow goxy ;

: P43 0 kCol 6 kRow goxy ; # ShiftL position
: P44 1 kCol 6 kRow goxy ; # Lower letter row
: P45 2 kCol 6 kRow goxy ;
: P46 3 kCol 6 kRow goxy ;
: P47 4 kCol 6 kRow goxy ;
: P48 5 kCol 6 kRow goxy ;
: P49 6 kCol 6 kRow goxy ;
: P50 7 kCol 6 kRow goxy ;
: P51 8 kCol 6 kRow goxy ;
: P52 9 kCol 6 kRow goxy ; # ShiftR/Return

: P53 1 kCol 7 kRow goxy ; # AltL
: P54 2 kCol 7 kRow goxy ; # Comma / SYM
: P55 3 kCol 7 kRow goxy ; # Space: 4 wide
: P56 7 kCol 7 kRow goxy ; # Period / Emoji
: P57 8 kCol 7 kRow goxy ; # AltR

# Move left and draw a key -- T=(label)
: kLK1  -33 0 +xy key1 ;   # +x: (-1*33)
: kLK2  -66 0 +xy key2 ;   # +x: (-2*33)
: kLK4 -132 0 +xy key4 ;   # +x: (-4*33)

: kFKeys
  (F1) (F2) P4 key2 kLK2
  (F3) (F4) P8 key2 kLK2
;
: kFKeysAltL
  () ()     P4 key2 kLK2
  () (Ctrl) P8 key2 kLK2
;
: kFKeysAltR
  (Tab) () P4 key2 kLK2
  ()    () P8 key2 kLK2
;
: kFKeysAlt
  (Tab) ()     P4 key2 kLK2
  ()    (Ctrl) P8 key2 kLK2
;

# Spacebar and Alt keys
: kBottomRow                 # No Modifiers
  (↑) (,) () (.) (↑) P57
  key1 kLK1 kLK4 kLK1 kLK1
;
: kBottomRowAltL             # AltL
  (↑) () () () (↑) P57
  key1 kLK1 kLK4 kLK1 kLK1
;
: kBottomRowAltR             # AltR
  (↑) (SYM) () (㋡) (↑) P57
  key1 kLK1 kLK4 kLK1 kLK1
;

# Draw row of ten 1-wide keys, right to left
# Stack: 10 key labels, T=(rightmost-label)
: kRowOfTen
  key1 kLK1 kLK1 kLK1 kLK1
  kLK1 kLK1 kLK1 kLK1 kLK1
;
`.trim();

export const Widgets = `
# Widgets: Building blocks to make views

# Bounds of screen
: wScreenX 336 ;
: wScreenY 536 ;

# Height and Y bounds of status bar
: wStatH  24 ;
: wStatY0  0 ;
: wStatY1 wStatH 1 - ;

# Height and Y bounds of keyboard
: wKbdH 33 6 * ;      # 33 px/key * 8 rows
: wKbdY0 wScreenY wKbdH - ;
: wKbdY1 wScreenY 1 - ;

# Height and Y bounds of main content area
: wMainH wKbdY0 wStatH - 4 - ;
: wMainY0 wStatH ;
: wMainY1 wKbdY0 1 - ;

# Status bar slots
: wWifi sprWifi3 ;      # Wifi sprite slot
: wBat sprBat99 ;    # Battery sprite slot
: wTime (--:--) ;        # Clock time slot
: wTitle (Home) ;        # View title slot

# Draw status bar (relies on slot :-defs)
: wStatCol 33 * 2 + ;
: wDrStatusBar
  1 wStatCol wStatY1 4 - goxy wTitle txtL
  6 wStatCol wStatY0 4 + goxy wBat image
  7 wStatCol wStatY0 4 + goxy wWifi image
  8 wStatCol wStatY1 4 - goxy wTime txtL
;

# Show message in center of main content area
: wDrMainNotify
  wScreenX shr               # Center X
  wMainH shr wMainY0 +       # Center Y
  goxy
  txtC         # Render text from stack
;

# Draw outline of main content window
: wDrMainWindow
  2 wMainY0 goxy wScreenX 4 - wMainH
  0 fill 1 stroke rect
;
`.trim();

export const Sprites = `
# Sprites: Bitmap image data
# 1. To edit a sprite, select the < ... >
#    bitmap data and drag it onto the text
#    box in the sprite editor.
# 2. You can copy and paste changes from the
#    sprite editor back to here, but the
#    changes will be temporary.
# 3. To save your changes, copy the edits to
#    bkit-rom.js (the source of this text)

: sprWifi3
< 000000011111111100000000
  000001111111111111000000
  000011110000000111100000
  000111000000000001110000
  001110000111110000111000
  011100011111111100011100
  001000111100011110001000
  000001110000000111000000
  000011100001000011100000
  000001000111110001000000
  000000001111111000000000
  000000011100011100000000
  000000001000001000000000
  000000000001000000000000
  000000000011100000000000
  000000000001000000000000 >
24 16 ;

: sprWifi2
< 000000000000000000000000
  000000000000000000000000
  000000000000000000000000
  000000000000000000000000
  000000000111110000000000
  000000011111111100000000
  000000111100011110000000
  000001110000000111000000
  000011100001000011100000
  000001000111110001000000
  000000001111111000000000
  000000011100011100000000
  000000001000001000000000
  000000000001000000000000
  000000000011100000000000
  000000000001000000000000 >
24 16 ;

: sprWifi1
< 000000000000000000000000
  000000000000000000000000
  000000000000000000000000
  000000000000000000000000
  000000000000000000000000
  000000000000000000000000
  000000000000000000000000
  000000000000000000000000
  000000000001000000000000
  000000000111110000000000
  000000001111111000000000
  000000011100011100000000
  000000001000001000000000
  000000000001000000000000
  000000000011100000000000
  000000000001000000000000 >
24 16 ;

: sprWifi0
< 000000011111111100000000
  000001100000000011000000
  000010000000000000100000
  000100000000000000010000
  001000000000000000001000
  010000000000000000000100
  001000000000000000001000
  000100000000000000010000
  000010000000000000100000
  000001000000000001000000
  000000100000000010000000
  000000010000000100000000
  000000001000001000000000
  000000000100010000000000
  000000000010100000000000
  000000000001000000000000 >
24 16 ;

: sprBat99
< 000000000000000000000000
  000000000000000000000000
  011111111111111111111100
  100000000000000000000010
  101111111111111111111010
  101111111111111111111011
  101111111111111111111011
  101111111111111111111011
  101111111111111111111011
  101111111111111111111011
  101111111111111111111011
  101111111111111111111010
  100000000000000000000010
  011111111111111111111100
  000000000000000000000000
  000000000000000000000000 >
24 16 ;

: sprBat75
< 000000000000000000000000
  000000000000000000000000
  011111111111111111111100
  100000000000000000000010
  101111111111111110000010
  101111111111111110000011
  101111111111111110000011
  101111111111111110000011
  101111111111111110000011
  101111111111111110000011
  101111111111111110000011
  101111111111111110000010
  100000000000000000000010
  011111111111111111111100
  000000000000000000000000
  000000000000000000000000 >
24 16 ;

: sprBat50
< 000000000000000000000000
  000000000000000000000000
  011111111111111111111100
  100000000000000000000010
  101111111111000000000010
  101111111111000000000011
  101111111111000000000011
  101111111111000000000011
  101111111111000000000011
  101111111111000000000011
  101111111111000000000011
  101111111111000000000010
  100000000000000000000010
  011111111111111111111100
  000000000000000000000000
  000000000000000000000000 >
24 16 ;

: sprBat25
< 000000000000000000000000
  000000000000000000000000
  011111111111111111111100
  100000000000000000000010
  101111100000000000000010
  101111100000000000000011
  101111100000000000000011
  101111100000000000000011
  101111100000000000000011
  101111100000000000000011
  101111100000000000000011
  101111100000000000000010
  100000000000000000000010
  011111111111111111111100
  000000000000000000000000
  000000000000000000000000 >
24 16 ;

: sprBat05
< 000000000000000000000000
  000000000000000000000000
  011111111111111111111100
  100000000000000000000010
  101100000000000000000010
  101100000000000000000011
  101100000000000000000011
  101100000000000000000011
  101100000000000000000011
  101100000000000000000011
  101100000000000000000011
  101100000000000000000010
  100000000000000000000010
  011111111111111111111100
  000000000000000000000000
  000000000000000000000000 >
24 16 ;

`.trim();
