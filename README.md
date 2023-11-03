Keyboard layout finder and optimizer

https://www.reddit.com/r/KeyboardLayouts/comments/176nm6u/i_made_a_program_to_find_a_new_class_of/

This is designed for a ~42 key ortholinear keyboard with a little column stagger. I use light 20gf key switches for all keys except for the 6 index, middle, and ring finger home keys.

Edit 11/3/2023:
After some real world experience and adjustments, I have settled on
```
J  B L D V W   ? ? U O Y  Alt
Q  N R T S F   ; I E A H  Enter
Z  M K G C X   ? P , . /  Ctrl
Tab Space M1   M2 Shift BackSpace
```
where the `?` spots can be customized according to common special keys for your workflow. The top right `?` could be the QMK repeat key, but in programming I find it to be rarely be used and instead use `'`, `_`, and `:` for the special spots, in order.
The bottom row goes on the thumb keys. For boards where there are only two thumb keys on each side, the tab and backspace can be moved to `?` spots, leaving the space, shift, and two modifiers. For boards without the extra pinky columns, just J, Q, Z, and the usual modifiers need to be rehomed, which could be put on the `?` spots (put Q on the upper right `?`) or be done as chords on the corners like MK, VW, CX, upper left "??", and "?P" which are rare.

The main feature of this layout is that the top 14 letters are all in the
formation
```
  L D           U O
N R T S       I E A H
      C       P
```
with only 0.23% skip bigrams, 1.12% same finger bigrams, 0.97% pinky/ring scissors, and 0.24% lateral stretch bigrams.
The top row has the advantage of being able to fast inwards roll all "you'..." and "...ould..." sequences.

The other layers are a design space that needs tuning. My current idea is that holding one or the other modifier or both leads to activating one of three other layers. Releasing goes back to the previous layer, so that every layer is the quickest movement away from each other layer.

M1 activates the numbers layer
```
 OS  @ # ? ! %   0 7 8 9 0  Alt
Esc  v < ^ > =   0 4 5 6 0  Enter
  ?  / * - + $   0 1 2 3 0  Ctrl
  Tab Space []   M3 Shift BackSpace
```
where "v < ^ >" are the arrow keys and the top `?` is the actual `?` char. Some of the zeros may be replaced. When holding down M0 with a thumb, your other four fingers are immediately in position to move with the arrow keys. The downwards arrow being on the pinky instead of a WASD-like arrangement takes some getting used to, but I think it is a strictly superior way that avoids up-down alternation cases. At the same time as you have arbitrary movement, your right hand is free to do any hold modifier, which is extremely useful when selecting text without switching to a mouse.

M2 activates the delimiters layer
```
 OS    ^ &         ' " ` ~  Alt
Esc  { } [ ] \   | ( ) < >  Enter
  ?  v < ^ >                Ctrl
  Tab Space M3   [] Shift BackSpace
```
where "v < ^ >" are the arrow keys again

M1 + M2 activates the function layer
```
F1   F2     F3       F4   F5  F6         F7      F8     F9  F10 F11  F12
Esc  PageUp PageDown Home End CapsLock   NumLock Insert Delete T? T? Alt
Ctrl+Y Ctrl+Z ..+X  ..+C  ..+V  ..+F                                Ctrl
                          Tab Space []   [] Shift BackSpace
```
If there is a need for more specialized layers, toggling modifiers could be put on the function layer.

Note: If your right alt key does not seem to work properly on Linux, it may be because of https://askubuntu.com/questions/1104092/my-right-alt-key-is-not-working
