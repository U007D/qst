extern crate hesl;

use rspec::{given, run};
use di::Container;

#[derive(Clone, Default, Debug)]
struct Environment {}

#[test]
fn tests() {
    run(&given("", Environment::default(), |ctx| {
        ctx.before(|env| {});

        ctx.when("", |ctx| {
            ctx.before(|env| {});

            ctx.then("", |env| {
            });
        });
    }));
}

