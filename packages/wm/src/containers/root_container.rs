use std::{
  cell::{Ref, RefCell, RefMut},
  collections::VecDeque,
  rc::Rc,
};

use uuid::Uuid;

use crate::{impl_common_behavior, impl_tiling_behavior};

use super::{
  traits::{CommonBehavior, PositionBehavior, TilingBehavior},
  Container, ContainerType, TilingContainer,
};

/// Root node of the container tree.
#[derive(Clone, Debug)]
pub struct RootContainer(Rc<RefCell<RootContainerInner>>);

#[derive(Debug)]
struct RootContainerInner {
  id: Uuid,
  parent: Option<TilingContainer>,
  children: VecDeque<Container>,
  child_focus_order: VecDeque<Uuid>,
  size_percent: f32,
}

impl RootContainer {
  pub fn new() -> Self {
    let root = RootContainerInner {
      id: Uuid::new_v4(),
      parent: None,
      children: VecDeque::new(),
      child_focus_order: VecDeque::new(),
      size_percent: 1.0,
    };

    Self(Rc::new(RefCell::new(root)))
  }
}

impl_common_behavior!(RootContainer, ContainerType::Root);
impl_tiling_behavior!(RootContainer);

impl PositionBehavior for RootContainer {
  fn width(&self) -> i32 {
    0
  }

  fn height(&self) -> i32 {
    0
  }

  fn x(&self) -> i32 {
    0
  }

  fn y(&self) -> i32 {
    0
  }
}
