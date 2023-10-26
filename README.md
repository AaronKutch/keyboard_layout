Keyboard layout finder and optimizer

https://www.reddit.com/r/KeyboardLayouts/comments/176nm6u/i_made_a_program_to_find_a_new_class_of/

The board I found likes to converge on the general layout of https://github.com/rdavison/graphite-layout. Note that my cost function has issues with sfbs, I had to manually freeze some keys and mirror the vowels.

This is designed for a 42 key ortholinear keyboard with a little column stagger.

Edit 10/25/2023:
After some real world experience and adjustments, I have settled on
```
J  B L D V W   ) ; O U Y  Alt
Q  N R T S P   . H A E I  Enter
Z  M K G F C   _ , X ( /  Ctrl
Tab Space M1   M2 Shift BackSpace
```
where the parenthesis could be replaced if they are not as important to you (note that I placed the right parenthesis where it is because modern editors automatically type it when typing the left one, and it leads to the nice layout where the most common separators are all orthogonal to the right index). The bottom row goes on the thumb keys. For boards where there are only two thumb keys on each side, the tab and backspace can be moved, leaving the space, shift, and two modifiers. For boards without the extra pinky columns, just J, Q, Z, and the usual modifiers need to be rehomed, which could be done as chords on the corners like MK, VW, FC, "_,", and "(/" which are rare.

The other layers are a design space that needs tuning. My current idea is that holding one or the other modifier or both leads to activating one of three other layers. Releasing goes back to the previous layer, so that every layer is the quickest movement away from each other layer.

M1 activates the numbers layer
```
 OS  @ # ? ! %   0 7 8 9 0  Alt
Esc  v < ^ > =   0 4 5 6 0  Enter
  >  / * - + $   0 1 2 3 0  Ctrl
  Tab Space []   M3 Shift BackSpace
```
where "v < ^ >" are the arrow keys and the bottom right '>' is there for the common "->" sequence in Rust. Some of the zeros may be replaced. When holding down M0 with a thumb, your other four fingers are immediately in position to move with the arrow keys. The downwards arrow being on the pinky instead of a WASD-like arrangement takes some getting used to, but I think it is a strictly superior way that avoids up-down alternation cases. At the same time as you have arbitrary movement, your right hand is free to do any hold modifier, which is extremely useful when selecting text without switching to a mouse.

M2 activates the delimiters layer
```
 OS    ^ &         ' " ` ~  Alt
Esc  < > ( ) \   | [ ] { }  Enter
     v < ^ >                Ctrl
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
