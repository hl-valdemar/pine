#import "@preview/fletcher:0.3.0" as fletcher: node, edge
#import "template.typ": *

#show: project.with(
  title: "Pine Engine",
  authors: (
    (name: "Valdemar H. Lorenzen", email: "valdemar.lorenzen@gmail.com", phone: ""),
  ),
  abstract: [],
  date: "Started January 28, 2024",
)

= Architecture

#figure(
  caption: [Overview of the engine architecture.],
  [
  #let c = rgb("00fa")
  #v(0.5em)
  #fletcher.diagram(
    node-stroke: c,
    node-fill: rgb("aafa"),
    node-outset: 2pt,
    node((1,1), `App`),

    node((0,0), `State`),
    node((1,0), `Renderer`),
    node((2,0), `World`),

    edge((1,1), (0,0), "->"),
    edge((1,1), (1,0), "->"),
    edge((1,1), (2,0), "->"),
  )
  ]
)

== Application Context

The application context will hold all necessary components of the application.
This includes the application state, the renderer, and the world, wrapping the ECS.

== Renderer

#figure(
  caption: [Overview of the Renderer.],
  [
  #let c = rgb("00fa")
  #v(0.5em)
  #fletcher.diagram(
    node-stroke: c,
    node-fill: rgb("aafa"),
    node-outset: 2pt,
    node((1,2), `Renderer`),

    node((1,1), `Backend`),
    node((2,1), `Windows`),

    node((0,0), `OpenGL`),
    node((1,0), `Vulkan`),
    node((2,0), `WGPU`),
    node((3,0), `Metal`),

    edge((1,2), (1,1), "<-"),
    edge((1,2), (2,1), "<-"),

    edge((1,1), (0,0), "<-"),
    edge((1,1), (1,0), "<-"),
    edge((1,1), (2,0), "<-"),
    edge((1,1), (3,0), "<-"),
  )
  ]
)


The renderes includes a generic backend.
This decouples the specific rendering logic from the renderer and enables multiple backends such as OpenGL, Vulkan, WGPU, etc. to be implemented as the underlying backend.
This should make the renderer a simple interface for the actual rendering. 

=== WGPU

#figure(
  caption: [Overview of the WGPU.],
  [
  #let c = rgb("00fa")
  #v(0.5em)
  #fletcher.diagram(
    node-stroke: c,
    node-fill: rgb("aafa"),
    node-outset: 2pt,
    node((1,2), `WGPU`),

    node((0,1), `Camera`),
    node((1,1), `Renderables`),
    node((2,1), `Shaders`),

    edge((1,2), (0,1), "<-"),
    edge((1,2), (1,1), "<-"),
    edge((1,2), (2,1), "<-"),
  )
  ]
)

Right now, the WGPU backend implementation is split into a Camera module, a Renderables module (containing the Renderable trait declaration and its implementations), and a Shaders module for handling shaders.
