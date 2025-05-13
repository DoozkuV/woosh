/*
Copyright (C) 2025 George Nicholas Padron

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

pub const OPERATORS: [char; 2] = ['|', '>'];

#[derive(Debug)]
pub enum Command {
    Simple(SimpleCommand),
    Pipeline(Vec<SimpleCommand>),
}

#[derive(Debug)]
pub struct SimpleCommand {
    pub program: String,
    pub args: Vec<String>,
    pub redirection: Option<Redirection>,
}

#[derive(Debug)]
pub enum Redirection {
    Stdout(String), // > file
}
