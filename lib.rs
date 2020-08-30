use elvis::{
    prelude::*,
    style::traits::{JustifyContent, Margin},
    value::{layouts::FlexPosition, FontFamily, TextAlign, Unit, VecUnit},
    widgets::{
        layouts::{Center, Col, Row},
        Link, Text,
    },
};

#[page]
struct Index;

fn font() -> FontFamily {
    FontFamily::Derive(vec![FontFamily::Mix(
        Box::new(FontFamily::Helvetica),
        Box::new(FontFamily::Neue),
    )])
}

impl LifeCycle for Index {
    fn create(&self) -> Node {
        Center::with(Col::with(vec![
            Text::with("Elvis . JS")
                .size(Unit::Rem(16.0))
                .family(font())
                .align(TextAlign::Center)
                .margin(VecUnit(vec![Unit::Rem(2.0)])),
            Row::with(
                [
                    ("Contribute", "https://github.com/elvisjs/elvis#help-wanted"),
                    ("docs.rs", "https://docs.rs/elvis"),
                    ("Discord", "https://discord.gg/dxpefwy"),
                ]
                .iter()
                .map(|(text, link)| {
                    Link::with(Text::with(text).size(Unit::Rem(1.6))).href(link.to_string())
                })
                .collect::<Vec<Link>>(),
            )
            .justify_content(FlexPosition::SpaceAround)
            .margin(VecUnit(vec![Unit::Rem(2.0), Unit::Rem(10.0)])),
        ]))
        .margin(VecUnit(vec![Unit::Rem(0.0), Unit::Auto]))
    }
}
