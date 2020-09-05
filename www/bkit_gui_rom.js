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
# tab reverts changes. Editing bkit-rom.js is
# the equivalent of re-flashing ROM.

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
kNav
kFKeys
(1) P13
(2) P14
(3) P15
(4) P16
(5) P17
(6) P18
(7) P19
(8) P20
(9) P21
(0) P22

(A) P23
(Z) P24
(E) P25
(R) P26
(T) P27
(Y) P28
(U) P29
(I) P30
(O) P31
(P) P32

(Q) P33
(S) P34
(D) P35
(F) P36
(G) P37
(H) P38
(J) P39
(K) P40
(L) P41
(M) P42

()  P43  # ShL?
(W) P44
(X) P45
(C) P46
(V) P47
(B) P48
(N) P49
(:) P50
(;) P51
()  P52  # ShR?
kBottomRow
;

: kAzertyAltL
kSetStyle
kNav
kFKeysAltL
(§) P13
() P14
() P15
() P16
([) P17
(]) P18
() P19
(_) P20
(') P21
(") P22

() P23
() P24
() P25
() P26
() P27
() P28
() P29
() P30
() P31
() P32

() P33
() P34
() P35
() P36
() P37
() P38
() P39
(\\\\) P40  # js needs multi-escape
() P41
() P42

() P43
() P44
() P45
() P46
() P47
() P48
() P49
(¿) P50
(¡) P51
() P52
kBottomRowAltL
;

: kAzertyAltR
kSetStyle
kNav
kFKeysAltR
(à) P13
(é) P14
(è) P15
(ê) P16
(() P17
(\\)) P18
(&) P19
(*) P20
(«) P21
(») P22

(æ) P23
(£) P24
(€) P25
(\`) P26
({) P27
(}) P28
(ù) P29
(ï) P30
(œ) P31
(%) P32

(@) P33
(ß) P34
($) P35
(¤) P36
(µ) P37
(-) P38
(+) P39
(/) P40
(|) P41
(#) P42

(⌫ ) P43  # Backspace
(<) P44
(>) P45
(ç) P46
(^) P47
(=) P48
(~) P49
(?) P50
(!) P51
(⏎ ) P52  # Return
kBottomRowAltR
;
`.trim();

export const KbdQwerty = `
# Draw QWERTY key maps (Base, Alt)

: kQwerty
kSetStyle
kNav
kFKeys
(1) P13
(2) P14
(3) P15
(4) P16
(5) P17
(6) P18
(7) P19
(8) P20
(9) P21
(0) P22

(Q) P23
(W) P24
(E) P25
(R) P26
(T) P27
(Y) P28
(U) P29
(I) P30
(O) P31
(P) P32

(A) P33
(S) P34
(D) P35
(F) P36
(G) P37
(H) P38
(J) P39
(K) P40
(L) P41
(⌫ ) P42

(!) P43
(Z) P44
(X) P45
(C) P46
(V) P47
(B) P48
(N) P49
(M) P50
(?) P51
(⏎ ) P52  # Return
kBottomRow
;

: kQwertyAlt
kSetStyle
kNav
kFKeysAlt
() P13
() P14
() P15
() P16
() P17
() P18
() P19
() P20
() P21
() P22

(%) P23
(^) P24
(~) P25
(|) P26
([) P27
(]) P28
(<) P29
(>) P30
({) P31
(}) P32

(@) P33
(#) P34
(&) P35
(*) P36
(-) P37
(+) P38
(=) P39
(() P40
(\\)) P41
(⌫ ) P42

(\`) P43
(_) P44
($) P45
(") P46
(') P47
(:) P48
(;) P49
(/) P50
(\\\\) P51  # js needs multi-escape
(⏎ ) P52  # Return
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
: kSetStyle 1 fill 2 stroke 0 radius ;

# Draw key label -- (utf8) x=S y=T
: kLabel 16 25 +xy txtC ;

# Draw {1,2,4}-wide key -- (utf8) x=S y=T
# Box dimensions +1px makes lines overlap
: key1 goxy  34 34 rect kLabel ;
: key2 goxy  67 34 rect 15 0 +xy kLabel ;
: key4 goxy 133 34 rect kLabel ;

: P2  9 kCol shr  0 kRow key1 ;  # Up
: P5  7 kCol shr  1 kRow key1 ;  # Left
: PC  9 kCol shr  1 kRow key1 ;  # Click
: P6 11 kCol shr  1 kRow key1 ;  # Right
: P3  0 kCol      2 kRow key2 ;  # F1
: P4  2 kCol      2 kRow key2 ;  # F2
: P9  9 kCol shr  2 kRow key1 ;  # Down
: P7  6 kCol      2 kRow key2 ;  # F3
: P8  8 kCol      2 kRow key2 ;  # F4

: P13 0 kCol 3 kRow key1 ; # Number row
: P14 1 kCol 3 kRow key1 ;
: P15 2 kCol 3 kRow key1 ;
: P16 3 kCol 3 kRow key1 ;
: P17 4 kCol 3 kRow key1 ;
: P18 5 kCol 3 kRow key1 ;
: P19 6 kCol 3 kRow key1 ;
: P20 7 kCol 3 kRow key1 ;
: P21 8 kCol 3 kRow key1 ;
: P22 9 kCol 3 kRow key1 ;

: P23 0 kCol 4 kRow key1 ; # Upper Letter row
: P24 1 kCol 4 kRow key1 ;
: P25 2 kCol 4 kRow key1 ;
: P26 3 kCol 4 kRow key1 ;
: P27 4 kCol 4 kRow key1 ;
: P28 5 kCol 4 kRow key1 ;
: P29 6 kCol 4 kRow key1 ;
: P30 7 kCol 4 kRow key1 ;
: P31 8 kCol 4 kRow key1 ;
: P32 9 kCol 4 kRow key1 ;

: P33 0 kCol 5 kRow key1 ; # Home letter row
: P34 1 kCol 5 kRow key1 ;
: P35 2 kCol 5 kRow key1 ;
: P36 3 kCol 5 kRow key1 ;
: P37 4 kCol 5 kRow key1 ;
: P38 5 kCol 5 kRow key1 ;
: P39 6 kCol 5 kRow key1 ;
: P40 7 kCol 5 kRow key1 ;
: P41 8 kCol 5 kRow key1 ;
: P42 9 kCol 5 kRow key1 ;

: P43 0 kCol 6 kRow key1 ; # ShiftL position
: P44 1 kCol 6 kRow key1 ; # Lower letter row
: P45 2 kCol 6 kRow key1 ;
: P46 3 kCol 6 kRow key1 ;
: P47 4 kCol 6 kRow key1 ;
: P48 5 kCol 6 kRow key1 ;
: P49 6 kCol 6 kRow key1 ;
: P50 7 kCol 6 kRow key1 ;
: P51 8 kCol 6 kRow key1 ;
: P52 9 kCol 6 kRow key1 ; # ShiftR/Return

: P53 1 kCol 7 kRow key1 ; # AltL
: P54 2 kCol 7 kRow key1 ; # Comma / SYM
: P55 3 kCol 7 kRow key4 ; # Space: 4 wide
: P56 7 kCol 7 kRow key1 ; # Period / Emoji
: P57 8 kCol 7 kRow key1 ; # AltR

# Nav keys are same for all layouts
: kNav  # Up, left, click, right, down
        () P2
  () P5 () PC () P6
        () P9
;
: kFKeys
  (F1) P3
  (F2) P4
  (F3) P7
  (F4) P8
;
: kFKeysAltL
  ()     P3
  ()     P4
  ()     P7
  (Ctrl) P8
;
: kFKeysAltR
  (Tab) P3
  ()    P4
  ()    P7
  ()    P8
;
: kFKeysAlt
  (Tab)  P3
  ()     P4
  ()     P7
  (Ctrl) P8
;

# Spacebar and Alt keys
: kSpace ()  P55 ;
: kAltL  (↑) P53 ;
: kAltR  (↑) P57 ;

: kBottomRow  # Normal: Modifiers up
  kAltL
  (,) P54
  kSpace
  (.) P56
  kAltR
;

: kBottomRowAltL  # AltL down
  kAltL
  () P54
  kSpace
  () P56
  kAltR
;

: kBottomRowAltR
  kAltL
  (SYM) P54
  kSpace
  (😃)  P56
  kAltR
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
: wKbdH 33 8 * ;      # 33 px/key * 8 rows
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
