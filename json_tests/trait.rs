#![feature(no_core)]
#![no_core]

pub trait Whammer {
    fn wham(&self);
}

pub trait Jammer {
    fn jam(&self);
}
