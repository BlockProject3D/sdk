// Copyright (c) 2024, BlockProject 3D
//
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
//
//     * Redistributions of source code must retain the above copyright notice,
//       this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above copyright notice,
//       this list of conditions and the following disclaimer in the documentation
//       and/or other materials provided with the distribution.
//     * Neither the name of BlockProject 3D nor the names of its contributors
//       may be used to endorse or promote products derived from this software
//       without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use std::error::Error;
use std::path::Path;

pub enum Output<'a> {
    Bin(&'a Path),
    Lib(&'a Path),
    Config(&'a Path),
    Other(&'a Path)
}

pub struct OutputList<'a>(Vec<Output<'a>>);

impl<'a> OutputList<'a> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add_bin(&mut self, path: &'a Path) {
        self.0.push(Output::Bin(path));
    }
    pub fn add_lib(&mut self, path: &'a Path) {
        self.0.push(Output::Lib(path));
    }
    pub fn add_config(&mut self, path: &'a Path) {
        self.0.push(Output::Config(path));
    }
    pub fn add_other(&mut self, path: &'a Path) {
        self.0.push(Output::Other(path));
    }
}

impl<'a> AsRef<Vec<Output<'a>>> for OutputList<'a> {
    fn as_ref(&self) -> &Vec<Output<'a>> {
        &self.0
    }
}

pub struct Module<'a> {
    pub name: &'a str,
    pub path: &'a Path
}

pub struct Context<'a> {
    pub root: &'a Path,
    pub target: &'a str,
    pub release: bool,
    pub features: &'a[&'a str]
}

pub trait Builder: Sized {
    const NAME: &'static str;
    type Error: Error;

    fn do_configure(context: &Context, module: &Module) -> Result<Self, Self::Error>;
    fn do_compile(&self, context: &Context, module: &Module) -> Result<(), Self::Error>;
    fn list_outputs(&self, context: &Context, module: &Module, paths: &mut OutputList) -> Result<(), Self::Error>;
}