# VHS recording for the text conversion example
Output examples/vhs/generated/text.gif

Set Theme "Aardvark Blue"
Set Width 780
Set Height 430
Set Padding 30
Set Framerate 25

Hide
Type "target/release/examples/readme text"
Enter
Wait+Screen /Spinners as Text/
Show
Sleep 1s
Screenshot target/vhs/text.png
Sleep 6s
