bug1
------
console output:

> The system has 958 fonts! First font is(these should all be equal,right?): [Helvetica] [ Arial] [ Arial] [BCourier New]

in-window output:
* (first window with font [BCourier New])

> main - bug1
> This font at index[0] is not [Helvetica], even though it should be [ Arial], but it's rather the app default font [BCourier New] unless app::set_font(...) was never called!


bug2
------
console output:
> The system has 958 fonts! First font is(these should all be equal,right?): [HelveticaItalic] [IArial] [IArial] [IArial]

* (second window with font [IArial])

> main - bug2
> This font at index[2] is not [HelveticaItalic], even though it should be [IArial], but it's rather the app default font [IArial] unless app::set_font(...) was never called!



reported at: https://github.com/fltk-rs/fltk-rs/issues/1377
