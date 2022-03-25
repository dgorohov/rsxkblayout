Displays current Xkb layout as `i3block` block. It listens for Xkb events and detects then the keyboard layout has changed.

Example of `i3block` configuration:

```
[lang]
command=~/.config/i3blocks/language/rsxkblayout
interval=persist
format=json
```
* Actually, `format` and `interval` should be `json` and `persist` respectively :)


![Capture](capture.png)

No configurations currently available. Black text on white displaying current layout.

Haven't tested it on a different configurations yet. If you've issues, please post your keyboard configuration and the error you're getting.

X11 configuration I am using currently for the keyboard:

```
Section "InputClass"
	Identifier	"keyboard"
	Driver		"evdev"
	MatchIsKeyboard	"on"
	Option 		"XkbModel" 	"pc105"
	Option		"XkbLayout"	"se,ru"
	Option		"XkbVariant"	"nodeadkeys"
	Option 		"XkbOptions" 	"grp:alt_shift_toggle"
EndSection
```


![Build](https://github.com/dgorohov/rsxkblayout/actions/workflows/rust.yml/badge.svg)

