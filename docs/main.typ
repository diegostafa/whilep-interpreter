#import "@preview/polylux:0.3.1": *
#import "theme/unipd.typ": *

#set text(font: ("Liberation sans"))

#show: unipd-theme.with(aspect-ratio: "16-9")

#title-slide(title: [Software verification])

#new-section("section 1")

#slide(title: "some text")[
```sh
some code
  ```
]

#slide(title: "Dynamic text")[
  #lorem(20)\
  #uncover("2-")[This appears after one slide]
]

#new-section("Conclusions")

#slide(title: "Qux")[
_baz_\
*Fizz*\
`Fuzz`
]
