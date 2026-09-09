# VHS recording for the Rect gallery
Output examples/vhs/generated/rect.gif

Set Theme "Aardvark Blue"
Set Width 568
Set Height 600
Set Padding 30
Set Framerate 25

Hide
Type "target/release/examples/readme rect"
Enter
Wait+Screen /Shape/
Show
Sleep 1s
Screenshot target/vhs/rect.png
Sleep 2s
