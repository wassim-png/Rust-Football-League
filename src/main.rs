mod model;
mod dao;
mod sqlitedao;
mod db;
mod manager;
mod facade;
mod controller;
mod frame;

use frame::match_frame::MatchFrame;

fn main() {
    let frame = MatchFrame::new();
    frame.lancer();
}