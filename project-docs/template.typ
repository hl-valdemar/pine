#let project(
  title: "",
  abstract: [],
  authors: (),
  date: none,
  body,
) = {
  // Set the document's basic properties.
  set document(author: authors.map(a => a.name), title: title)
  set page(
    margin: (left: 20mm, right: 20mm, top: 35mm, bottom: 40mm),
    numbering: "1",
    number-align: center,
    
  )
  set text(font: "New Computer Modern", lang: "en")
  show math.equation: set text(weight: 400)

  // Set run-in subheadings, starting at level 3.
  show heading: it => {
    if it.level == 1 {
      //pad(top: 0.5em, bottom: 0.25em, it)
      parbreak()
      pad(top: 0.25em, bottom: 0.5em, text(16pt, weight: "bold", it.body))
    } else if it.level == 2 {
      //pad(top: 0.25em, bottom: 0.25em, it)
      parbreak()
      pad(top: 0.25em, bottom: 0.5em, text(14pt, weight: "bold", it.body))
    } else if it.level > 2 {
      parbreak()
      pad(top: 0.25em, bottom: 0.5em, text(12pt, style: "italic", weight: "regular", it.body + "."))
    }
  }

  // Title row.
  align(center)[
    #let title = smallcaps(text(weight: 700, 1.75em, title))
    #block(title)
    #v(1em, weak: true)
    #date
  ]

  // Author information.
  pad(
    top: 0.5em,
    bottom: 2em,
    x: 2em,
    grid(
      columns: (1fr,) * calc.min(3, authors.len()),
      gutter: 1em,
      ..authors.map(author => align(center)[
        *#author.name* \
        #author.email \
        #author.phone
      ]),
    ),
  )

  // Abstract.
  if (abstract != []) {
    pad(
      x: 2em,
      y: 1em,
      align(center)[
        #heading(
          outlined: false,
          numbering: none,
          text(0.85em, smallcaps[Abstract]),
        )
        #abstract
      ],
    )
  }

  // Main body.
  set par(first-line-indent: 1em, justify: true)
  show: columns.with(2, gutter: 2em)

  // Table of contents.
  set heading(numbering: "1.a.i")
  pad(x: 0em, y: 0.5em, outline(indent: true))

  // Code display.
  //set raw(theme: "theme/rose-pine.tmTheme")
  //show raw.where(block: true): it => block(
  //  fill: color_scheme(theme).colors.overlay,
  //  inset: 8pt,
  //  radius: 5pt,
  //  width: 100%,
  //  raw(it.text, lang: it.lang, align: left)
  //)
  //show raw.where(block: false): it => {
  //  box(inset: 2.5pt, radius: 2pt, fill: color_scheme(theme).colors.overlay, it)
  //}

  body
}
