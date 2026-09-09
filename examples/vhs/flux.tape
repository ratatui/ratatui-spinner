# VHS recording for the Flux gallery
Output examples/vhs/generated/flux.gif

Set Theme "Aardvark Blue"
Set Width 785
Set Height 840
Set Padding 30
Set Framerate 25

Hide
Type "target/release/examples/readme flux"
Enter
Wait+Screen /Braille/
Show
Sleep 1s
Screenshot target/vhs/flux.png
Sleep 8600ms
