# Rust GUI example

## Background

I'm planning to use a GUI library for a application I'm writing.
The library has to be simple and customizable.

I've done my research, and concluded [Iced](https://github.com/hecrj/iced) and [egui](https://github.com/emilk/egui) are good candidates.

## The criteria

- The application should run natively on all desktop platforms and on the web.
- It should have two round buttons without border, like Material Design.
- The buttons should toggle dark and light theme for the whole application.
- The buttons should say what they do on hover.
- Preferably, the buttons should be a icon (SVG) and not a font (since then I'd have to bundle a large font!).
- A text, "e^πi + 1 = 0" with the correct elevation of "πi".

## Results

Egui is a ImGui-esque library for simple UI, mostly to be used in games.

Iced is a modular library with flexbox-like layouting.

In egui, everything is drawn in OpenGL on all platforms, leveraging WebGL on the web.
It's also *immediate mode* which results in less boilerplate and fast prototyping,
but leads to it's one fatal flaw; layouting.

Iced generates HTML for the web. It's a dual-edged sword; all widgets have to be created twice;
once for the WGPU and once for the web.
It's more code to get a widget and a layout working since state, updates, and drawing are split up.
You can't have layers or absolute positions; so no overlapping elements or text on top of other text,
limiting what you can show to your user.

Egui also doesn't come with SVG support out of the box, which is problematic in modern apps.
It also feels yanky to build a layout with, since it's intended to be used in game engines.

As a positive for egui, it's much nicer to work with! Everything from building the widgets to
changing theme is so much better. But the damn layouting!

> As a sidenote, it's a PAIN to add theming to Iced. I hope it improves.
> I had to add 200+ lines of boilerplate to get it to work!
> You have to implement it for every widget!

I'll use Iced for now, hoping it'll add layering.
If egui ever improves it's layout, I'll maybe give it a proper try.
