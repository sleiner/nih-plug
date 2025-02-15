// Spectral Compressor: an FFT based compressor
// Copyright (C) 2021-2023 Robbert van der Helm
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use nih_plug::prelude::*;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;

use crate::{SpectralCompressor, SpectralCompressorParams};

// I couldn't get `LayoutType::Grid` to work as expected, so we'll fake a 4x4 grid with
// hardcoded column widths
const COLUMN_WIDTH: Units = Pixels(330.0);
const DARKER_GRAY: Color = Color::rgb(0x69, 0x69, 0x69);

#[derive(Lens)]
struct Data {
    params: Arc<SpectralCompressorParams>,
}

impl Model for Data {}

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (680, 535))
}

pub(crate) fn create(
    params: Arc<SpectralCompressorParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

        Data {
            params: params.clone(),
        }
        .build(cx);

        ResizeHandle::new(cx);

        VStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                Label::new(cx, "Spectral Compressor")
                    .font_family(vec![FamilyOwned::Name(String::from(
                        assets::NOTO_SANS_THIN,
                    ))])
                    .font_size(30.0)
                    .on_mouse_down(|_, _| {
                        // Try to open the plugin's page when clicking on the title. If this fails
                        // then that's not a problem
                        let result = open::that(SpectralCompressor::URL);
                        if cfg!(debug) && result.is_err() {
                            nih_debug_assert_failure!("Failed to open web browser: {:?}", result);
                        }
                    });
                Label::new(cx, SpectralCompressor::VERSION)
                    .color(DARKER_GRAY)
                    .top(Stretch(1.0))
                    .bottom(Pixels(4.0))
                    .left(Pixels(2.0));
            })
            .height(Pixels(30.0))
            .right(Pixels(-17.0))
            .bottom(Pixels(-5.0))
            .top(Pixels(10.0));

            HStack::new(cx, |cx| {
                make_column(cx, "Globals", |cx| {
                    GenericUi::new(cx, Data::params.map(|p| p.global.clone()));
                });

                make_column(cx, "Threshold", |cx| {
                    GenericUi::new(cx, Data::params.map(|p| p.threshold.clone()));

                    Label::new(
                        cx,
                        "Parameter ranges and overal gain staging are still subject to change. If \
                         you use this in a project, make sure to bounce things to audio just in \
                         case they'll sound different later.",
                    )
                    .font_size(11.0)
                    .left(Pixels(15.0))
                    .right(Pixels(8.0))
                    // The column isn't tall enough without this, for some reason
                    .bottom(Pixels(20.0))
                    .width(Stretch(1.0));
                });
            })
            .height(Auto)
            .width(Stretch(1.0));

            HStack::new(cx, |cx| {
                make_column(cx, "Upwards", |cx| {
                    // We don't want to show the 'Upwards' prefix here, but it should still be in
                    // the parameter name so the parameter list makes sense
                    let upwards_compressor_params =
                        Data::params.map(|p| p.compressors.upwards.clone());
                    GenericUi::new_custom(
                        cx,
                        upwards_compressor_params.clone(),
                        move |cx, param_ptr| {
                            let upwards_compressor_params = upwards_compressor_params.clone();
                            HStack::new(cx, move |cx| {
                                Label::new(
                                    cx,
                                    unsafe { param_ptr.name() }
                                        .strip_prefix("Upwards ")
                                        .expect("Expected parameter name prefix, this is a bug"),
                                )
                                .class("label");

                                GenericUi::draw_widget(cx, upwards_compressor_params, param_ptr);
                            })
                            .class("row");
                        },
                    );
                });

                make_column(cx, "Downwards", |cx| {
                    let downwards_compressor_params =
                        Data::params.map(|p| p.compressors.downwards.clone());
                    GenericUi::new_custom(
                        cx,
                        downwards_compressor_params.clone(),
                        move |cx, param_ptr| {
                            let downwards_compressor_params = downwards_compressor_params.clone();
                            HStack::new(cx, move |cx| {
                                Label::new(
                                    cx,
                                    unsafe { param_ptr.name() }
                                        .strip_prefix("Downwards ")
                                        .expect("Expected parameter name prefix, this is a bug"),
                                )
                                .class("label");

                                GenericUi::draw_widget(cx, downwards_compressor_params, param_ptr);
                            })
                            .class("row");
                        },
                    );
                });
            })
            .height(Auto)
            .width(Stretch(1.0));
        })
        .row_between(Pixels(15.0))
        .child_left(Stretch(1.0))
        .child_right(Stretch(1.0));
    })
}

fn make_column(cx: &mut Context, title: &str, contents: impl FnOnce(&mut Context)) {
    VStack::new(cx, |cx| {
        Label::new(cx, title)
            .font_family(vec![FamilyOwned::Name(String::from(
                assets::NOTO_SANS_THIN,
            ))])
            .font_size(23.0)
            .left(Stretch(1.0))
            // This should align nicely with the right edge of the slider
            .right(Pixels(7.0))
            .bottom(Pixels(-10.0));

        contents(cx);
    })
    .width(COLUMN_WIDTH)
    .height(Auto);
}
