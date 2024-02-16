#import "@preview/polylux:0.3.1": *

#let unipd-palette = (
  main: rgb(155, 0, 20),
  gray: rgb(72, 79, 89),
  light-gray: rgb(237, 237, 238),
)

#let palette-state = state("unipd-theme-palette", unipd-palette)
#let with-palette = f => locate(loc => f(palette-state.at(loc)))

#let header = with-palette(
  palette => {
    place(rect(width: 100%, height: 12%, fill: palette.main))
    place(
      right,
      dx: -2%,
      dy: 1%,
      image("../assets/logo_text_white.png", height: 10%),
    )
    // Section name in header
    place(dx: 2%, dy: 4.5%, text(size: 34pt, fill: white, utils.current-section))
  },
)

#let footer = with-palette(palette => {
  place(bottom, image("../assets/bg_wave.svg", width: 100%))
  // Slide number in the footer
  place(bottom + right, dx: -2.5%, dy: -2.5%, text(
    size: 18pt,
    fill: palette.main.lighten(50%),
    logic.logical-slide.display("1 of 1", both: true),
  ))
})

#let unipd-theme(aspect-ratio: "4-3", palette: unipd-palette, body) = {
  set page(paper: "presentation-" + aspect-ratio, margin: (x: 0cm, y: 0cm))
  palette-state.update(palette)
  body
}

#let title-slide(title: none, subtitle: none, authors: none, date: none) = logic.polylux-slide(
  {
    place(image("../assets/bg.svg", width: 100%))
    place(
      bottom + right,
      dx: -5%,
      dy: -5%,
      image("../assets/logo_text.png", width: 30%),
    )

    if type(subtitle) == none {
      subtitle = ""
    }
    if type(authors) != array {
      authors = (authors,)
    }
    if type(date) == "string" {
      date = (date,)
    }
    if type(date) == none {
      date = ()
    }

    v(20%)
    align(center, box(inset: (x: 2em), text(size: 46pt, fill: white, title)))
    align(center, box(inset: (x: 2em), text(size: 30pt, fill: white, subtitle)))
    v(5%)
    (authors + date)
    .map(line => h(7.5%) + text(size: 24pt, fill: white, line))
    .join(linebreak())
  },
)

#let slide(title: none, body) = logic.polylux-slide({
  header
  v(15%) // Space for header
  footer

  if title != none {
    v(7%)
    let title-text = with-palette(palette => text(palette.main, title))
    block(
      width: 100%,
      inset: (x: 4.5%, y: -.5em),
      breakable: false,
      outset: 0em,
      heading(level: 1, title-text),
    )
    v(.7em)
  }

  v(1fr)
  block(width: 100%, inset: (x: 2em), body)
  v(2fr)
})

#let new-section(title) = utils.register-section(title)

#let new-section-slide(title) = logic.polylux-slide({
  header
  footer

  new-section(title)
  set align(center + horizon)
  heading(level: 2, text(palette.main, title))
})