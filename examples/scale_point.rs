// Copyright 2016 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate noise;

use noise::{Checkerboard, ScalePoint};

mod debug;

fn main() {
    let cboard = Checkerboard::new();
    let scale_point = ScalePoint::new(cboard).set_all_scales(1.0, 2.0, 3.0, 1.0);

    debug::render_noise_module("scale_point.png", scale_point, 1024, 1024, 50);
}