#[macro_use]
extern crate enum_primitive;

extern crate num;
use num::FromPrimitive;

mod actionscript;

#[cfg(test)]
mod tests {
    use super::actionscript::Action;

    #[test]
    fn EA_PUSHONE_eq_0x5a() {
        assert_eq!(Action::EA_PUSHONE as usize, 0x5a);
    }
}