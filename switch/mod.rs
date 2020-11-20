use crate::uprade::Upgrade;

pub struct Port<U: Upgrade> {
  upgrader: U,
}